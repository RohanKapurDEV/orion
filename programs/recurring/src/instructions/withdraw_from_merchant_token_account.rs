use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(payment_config_index: u8, merchant_authority_index: u8, amount_to_withdraw: u64)]
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

    #[account(
        mut,
        constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount,
        constraint = payment_token_account.amount >= amount_to_withdraw @ ErrorCode::PaymentTokenAccountBalanceTooLow
    )]
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
    amount_to_withdraw: u64,
) -> ProgramResult {
    let merchant_auth_bump = *ctx.bumps.get("merchant_authority").unwrap();
    let init_authority_key = ctx.accounts.init_authority.key();

    let merchant_auth_seeds = &[
        b"merchant_authority".as_ref(),
        &merchant_authority_index.to_le_bytes(),
        init_authority_key.as_ref(),
        &[merchant_auth_bump],
    ];

    let merchant_auth_signer = &[&merchant_auth_seeds[..]];

    let transfer_accounts = Transfer {
        from: ctx.accounts.payment_token_account.to_account_info(),
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.merchant_authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        merchant_auth_signer,
    );

    transfer(cpi_ctx, amount_to_withdraw)?;

    Ok(())
}
