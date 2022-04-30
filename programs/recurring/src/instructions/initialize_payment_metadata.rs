use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{approve, transfer, Approve, Token, TokenAccount, Transfer};

/// Based on the derivation seeds for the PaymentMetadata account, any given pubkey can only hold one PaymentMetadata
/// account for any given PaymentConfig since there is no index seed requirement. If a merchant notices that a payments
/// collection has failed due to insufficient balance, the merchant should close the PaymentMetadata account immediately

#[derive(Accounts)]
#[instruction(amount_delegated: u64, payment_config_index: u8, merchant_authority_index: u8)]
pub struct InitializePaymentMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_metadata".as_ref(), payer.key().as_ref(), payment_config.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 1 + 1
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(
        seeds = [b"payment_config", &payment_config_index.to_le_bytes(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.key() == payment_metadata.payment_config @ RecurringError::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        seeds = [b"merchant_authority", &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()],
        bump,
        constraint = merchant_authority.key() == payment_config.merchant_authority
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    /// CHECK: not used in instruction logic, just as seed for merchant_authority account. validated in constraint
    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ RecurringError::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = owner_payment_account.mint == payment_config.payment_mint @ RecurringError::IncorrectMint,
        constraint = owner_payment_account.owner == payer.key() @ RecurringError::IncorrectAuthority
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = payment_token_account.key() == payment_config.payment_token_account @ RecurringError::IncorrectPaymentTokenAccount
    )]
    pub payment_token_account: Account<'info, TokenAccount>,

    /// CHECK: program signer PDA
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Enforce that the owner_payment_account has enough for the amount_delegated + amount_to_collect_on_init (if it exists)
impl<'info> InitializePaymentMetadata<'info> {
    fn accounts(ctx: &Context<InitializePaymentMetadata>, amount_delegated: u64) -> Result<()> {
        let payment_config = &ctx.accounts.payment_config;
        let minimum_balance_requirement: u64;

        let owner_payment_account = &ctx.accounts.owner_payment_account;

        if payment_config.collect_on_init {
            minimum_balance_requirement =
                payment_config.amount_to_collect_on_init + amount_delegated;
        } else {
            minimum_balance_requirement = amount_delegated;
        }

        if owner_payment_account.amount < minimum_balance_requirement {
            return Err(RecurringError::InsufficientBalanceToDelegate.into());
        }

        Ok(())
    }
}

/// In most cases, amount_delegated should be some multiple of payment_config.amount_to_collect_per_period. This should probably be
/// enforced at the contract level but for now it seems fine to not implement it.
#[access_control(InitializePaymentMetadata::accounts(&ctx, amount_delegated))]
pub fn handler(
    ctx: Context<InitializePaymentMetadata>,
    amount_delegated: u64,
    _payment_config_index: u8,
    _merchant_authority_index: u8,
) -> Result<()> {
    let bump = *ctx.bumps.get("payment_metadata").unwrap();
    let payment_metadata = &mut ctx.accounts.payment_metadata;
    let payment_config = &mut ctx.accounts.payment_config;
    let program_as_signer = &mut ctx.accounts.program_as_signer;

    let init_amount = payment_config.amount_to_collect_on_init;
    let payment_config_key: Pubkey = payment_config.key();

    // Enforce amount being delegated is enough for at least 1 payment
    require!(
        amount_delegated >= payment_config.amount_to_collect_per_period,
        RecurringError::AmountToDelegateIsSmallerThanMinimum
    );

    if payment_config.collect_on_init {
        let transfer_accounts = Transfer {
            from: ctx.accounts.owner_payment_account.to_account_info(),
            to: ctx.accounts.payment_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let transfer_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        transfer(transfer_cpi_ctx, init_amount)?;
    }

    let cpi_accounts = Approve {
        to: ctx.accounts.owner_payment_account.to_account_info(),
        delegate: program_as_signer.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    approve(cpi_ctx, amount_delegated)?;

    let clock = Clock::get()?;
    let unix_timestamp = clock.unix_timestamp;

    payment_metadata.owner = ctx.accounts.payer.key();
    payment_metadata.created_at = unix_timestamp;
    payment_metadata.payment_config = payment_config_key;
    payment_metadata.owner_payment_account = ctx.accounts.owner_payment_account.key();
    payment_metadata.amount_delegated = amount_delegated;
    payment_metadata.payments_collected = 0;
    payment_metadata.bump = bump;

    Ok(())
}
