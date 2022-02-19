use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitializePaymentConfig<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_auth.key().as_ref()],
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
    pub payment_pda: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializePaymentConfig>,
    minimum_amount_to_delegate: u64,
    spacing_period: i64,
    delay_format: u8,
) -> ProgramResult {
    let bump = *ctx.bumps.get("payment_config").unwrap();
    let payment_config = &mut ctx.accounts.payment_config;
    let merchant_authority = &mut ctx.accounts.merchant_auth;

    require!(
        ctx.accounts.payer.key() == merchant_authority.current_authority,
        ErrorCode::IncorrectAuthorityForPaymentConfig
    );

    payment_config.payment_mint = ctx.accounts.payment_mint.key();
    payment_config.payment_pda = ctx.accounts.payment_pda.key();
    payment_config.merchant_authority = ctx.accounts.merchant_auth.key();
    payment_config.minimum_amount_to_delegate = minimum_amount_to_delegate;
    payment_config.spacing_period = spacing_period;
    payment_config.delay_format = delay_format;
    payment_config.bump = bump;

    Ok(())
}
