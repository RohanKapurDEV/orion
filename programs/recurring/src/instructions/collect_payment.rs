use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
use std::convert::TryFrom;

#[derive(Accounts)]
#[instruction(merchant_authority_index: u8, payment_config_index: u8)]
pub struct CollectPayment<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ RecurringError::IncorrectCollectionAuthority)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"merchant_authority", &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()],
        bump,
        constraint = merchant_authority.key() == payment_config.merchant_authority
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    /// CHECK: not used in instruction logic, just as seed for merchant_authority account. validated in constraint
    #[account(mut, constraint = init_authority.key() == merchant_authority.init_authority @ RecurringError::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    #[account(
        seeds = [b"payment_config", &payment_config_index.to_le_bytes(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.key() == payment_metadata.payment_config @ RecurringError::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        mut,
        seeds = [b"payment_metadata", owner.key().as_ref(), payment_config.key().as_ref()],
        bump
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(mut, constraint = payment_token_account.key() == payment_config.payment_token_account @ RecurringError::IncorrectPaymentTokenAccount)]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = owner_payment_account.key() == payment_metadata.owner_payment_account @ RecurringError::IncorrectOwnerPaymentAccount,
        constraint = owner_payment_account.delegate.unwrap() == program_as_signer.key()
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    /// CHECK: validated in constraint, not used in instruction logic
    #[account(constraint = owner.key() == payment_metadata.owner @ RecurringError::IncorrectOwner)]
    pub owner: UncheckedAccount<'info>,

    /// CHECK: program signer PDA
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<CollectPayment>,
    _merchant_authority_index: u8,
    _payment_config_index: u8,
) -> Result<()> {
    let payment_config = &mut ctx.accounts.payment_config;
    let owner_payment_account = &mut ctx.accounts.owner_payment_account;
    let payment_metadata = &mut ctx.accounts.payment_metadata;
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();

    let amount_being_spent = payment_config.amount_to_collect_per_period;
    let delegated_amount = owner_payment_account.delegated_amount;

    require!(
        delegated_amount >= amount_being_spent,
        RecurringError::InsufficientBalanceToDelegate
    );

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let obligation_created_at = payment_metadata.created_at;
    let spacing_period = payment_config.spacing_period;

    let applied_payments_collected = payment_metadata
        .payments_collected
        .checked_add(u16::try_from(1).unwrap())
        .unwrap();

    let time_delta = spacing_period
        .checked_mul(applied_payments_collected as i64)
        .unwrap();

    let base_value = obligation_created_at.checked_add(time_delta).unwrap();

    require!(
        base_value <= current_timestamp,
        RecurringError::PaymentUnauthorized
    );

    let transfer_accounts = Transfer {
        from: ctx.accounts.owner_payment_account.to_account_info(),
        to: ctx.accounts.payment_token_account.to_account_info(),
        authority: ctx.accounts.program_as_signer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );

    transfer(cpi_ctx, amount_being_spent)?;

    payment_metadata.payments_collected = payment_metadata
        .payments_collected
        .checked_add(u16::try_from(1).unwrap())
        .unwrap();

    Ok(())
}
