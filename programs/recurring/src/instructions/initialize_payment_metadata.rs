use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{approve, transfer, Approve, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(amount_delegated: u64)]
pub struct InitializePaymentMetadata<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_metadata", payer.key().as_ref(), payment_config.key().as_ref()],
        bump
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        mut,
        constraint = owner_payment_account.mint == payment_config.payment_mint @ ErrorCode::IncorrectMint,
        constraint = owner_payment_account.amount >= amount_delegated @ ErrorCode::InsufficientBalanceToDelegate
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount
    )]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<InitializePaymentMetadata>, amount_delegated: u64) -> ProgramResult {
    let bump = *ctx.bumps.get("payment_metadata").unwrap();
    let payment_metadata = &mut ctx.accounts.payment_metadata;
    let payment_config = &mut ctx.accounts.payment_config;
    let program_as_signer = &mut ctx.accounts.program_as_signer;

    let init_amount = payment_config.amount_to_collect;
    let payment_config_key: Pubkey = payment_config.key();

    require!(
        amount_delegated > payment_config.minimum_amount_to_delegate,
        ErrorCode::AmountToDelegateIsSmallerThanMinimum
    );

    payment_metadata.owner = ctx.accounts.payer.key();
    payment_metadata.payment_config = payment_config_key;
    payment_metadata.owner_payment_account = ctx.accounts.owner_payment_account.key();
    payment_metadata.amount_delegated = amount_delegated;
    payment_metadata.bump = bump;

    if payment_config.collect_on_init == true {
        let transfer_accounts = Transfer {
            from: ctx.accounts.owner_payment_account.to_account_info(),
            to: ctx.accounts.payment_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        transfer(cpi_ctx, init_amount)?
    }

    let cpi_accounts = Approve {
        to: ctx.accounts.owner_payment_account.to_account_info(),
        delegate: program_as_signer.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    approve(cpi_ctx, amount_delegated)?;

    Ok(())
}
