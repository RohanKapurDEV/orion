use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(payment_config_index: u8, merchant_authority_index: u8)]
pub struct WithdrawFromMerchantTokenAccount<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"payment_metadata".as_ref(), payer.key().as_ref(), payment_config.key().as_ref()],
        bump,
        constraint = payment_metadata.payment_config == payment_config.key(),
        close = payer
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(
        mut,
        seeds = [b"merchant_authority", &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()],
        bump,
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config", &payment_config_index.to_le_bytes(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ ErrorCode::IncorrectMerchantAuthority,
        constraint = payment_metadata.payment_config == payment_config.key() @ ErrorCode::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(mut, constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount)]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = receiver_token_account.mint == payment_config.payment_mint @ ErrorCode::IncorrectMint)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<WithdrawFromMerchantTokenAccount>,
    _payment_config_index: u8,
    merchant_authority_index: u8,
) -> ProgramResult {
    Ok(())
}
