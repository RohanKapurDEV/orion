use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(
    index: u8,
    _spacing_period: i64,
    _collect_on_init: bool,
    _amount_to_collect_on_init: u64,
    _amount_to_collect_per_period: u64
)]
pub struct InitializePaymentConfig<'info> {
    #[account(mut, constraint = payer.key() == merchant_auth.current_authority @ RecurringError::IncorrectAuthorityForPaymentConfig)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_config".as_ref(), &index.to_le_bytes(),  merchant_auth.key().as_ref()],
        bump
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    pub merchant_auth: Account<'info, MerchantAuthority>,

    pub payment_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = payment_mint,
        token::authority = merchant_auth
    )]
    pub payment_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializePaymentConfig>,
    index: u8,
    spacing_period: i64,
    collect_on_init: bool,
    amount_to_collect_on_init: u64,
    amount_to_collect_per_period: u64,
) -> Result<()> {
    let bump = *ctx.bumps.get("payment_config").unwrap();
    let payment_config = &mut ctx.accounts.payment_config;

    payment_config.payment_mint = ctx.accounts.payment_mint.key();
    payment_config.payment_token_account = ctx.accounts.payment_token_account.key();
    payment_config.merchant_authority = ctx.accounts.merchant_auth.key();
    payment_config.spacing_period = spacing_period;
    payment_config.amount_to_collect_per_period = amount_to_collect_per_period;
    payment_config.collect_on_init = collect_on_init;
    payment_config.amount_to_collect_on_init = amount_to_collect_on_init;
    payment_config.index = index;
    payment_config.bump = bump;

    Ok(())
}
