use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[event]
pub struct InitializePaymentConfigEvent {
    pub payment_config_pubkey: Pubkey,
    pub merchant_authority: Pubkey,
    pub payment_mint: Pubkey,
    pub payment_token_account: Pubkey,
}

/// An important thing to note is that if a MerchantAuthority intends on issuing multiple PaymentConfigs, they need to keep track of the
/// current index; it's a number that refers to the amount of PaymentConfigs issued by a specific MerchantAuthority account.
///
/// Another gotcha here is when a MerchantAuthority does switch out it's current authority,
#[derive(Accounts)]
#[instruction(
    index: u8,
    merchant_authority_index: u8,
    _spacing_period: i64,
    _collect_on_init: bool,
    _amount_to_collect_on_init: u64,
    _amount_to_collect_per_period: u64
)]
pub struct InitializePaymentConfig<'info> {
    #[account(mut, constraint = payer.key() == merchant_authority.current_authority @ RecurringError::IncorrectAuthorityForPaymentConfig)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_config".as_ref(), &index.to_le_bytes(),  merchant_authority.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 1 + 1
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        seeds = [b"merchant_authority", &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()],
        bump
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    /// CHECK: not used in instruction logic, just as seed for merchant_authority account. validated in constraint
    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ RecurringError::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    pub payment_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = payment_mint,
        token::authority = merchant_authority
    )]
    pub payment_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializePaymentConfig>,
    index: u8,
    _merchant_authority_index: u8,
    spacing_period: i64,
    collect_on_init: bool,
    amount_to_collect_on_init: u64,
    amount_to_collect_per_period: u64,
) -> Result<()> {
    let bump = *ctx.bumps.get("payment_config").unwrap();
    let payment_config = &mut ctx.accounts.payment_config;

    payment_config.payment_mint = ctx.accounts.payment_mint.key();
    payment_config.payment_token_account = ctx.accounts.payment_token_account.key();
    payment_config.merchant_authority = ctx.accounts.merchant_authority.key();
    payment_config.spacing_period = spacing_period;
    payment_config.amount_to_collect_per_period = amount_to_collect_per_period;
    payment_config.collect_on_init = collect_on_init;
    payment_config.amount_to_collect_on_init = amount_to_collect_on_init;
    payment_config.index = index;
    payment_config.bump = bump;

    emit!(InitializePaymentConfigEvent {
        payment_config_pubkey: payment_config.key(),
        payment_mint: ctx.accounts.payment_mint.key(),
        merchant_authority: ctx.accounts.merchant_authority.key(),
        payment_token_account: ctx.accounts.payment_token_account.key()
    });

    Ok(())
}
