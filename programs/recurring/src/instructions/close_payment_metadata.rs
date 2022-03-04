use crate::{error::*, state::*};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::token::{Token, TokenAccount};
use spl_token::instruction::revoke;

#[derive(Accounts)]
#[instruction(payment_config_index: u8, merchant_authority_index: u8)]
pub struct ClosePaymentMetadata<'info> {
    #[account(mut, constraint = payer.key() == payment_metadata.owner @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"payment_metadata", payer.key().as_ref(), payment_config.key().as_ref()],
        bump,
        constraint = payment_metadata.payment_config == payment_config.key(),
        close = payer
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(mut, constraint = owner_payment_account.key() == payment_metadata.owner_payment_account @ ErrorCode::IncorrectOwnerPaymentAccount)]
    pub owner_payment_account: Account<'info, TokenAccount>,

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

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ClosePaymentMetadata>,
    _payment_config_index: u8,
    _merchant_authority_index: u8,
) -> ProgramResult {
    if let COption::Some(_x) = ctx.accounts.owner_payment_account.delegate {
        let ix = revoke(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.owner_payment_account.key(),
            &ctx.accounts.payer.key(),
            &[],
        )
        .unwrap();

        invoke(
            &ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.owner_payment_account.to_account_info(),
                ctx.accounts.payer.to_account_info(),
            ],
        )?;
    }

    Ok(())
}
