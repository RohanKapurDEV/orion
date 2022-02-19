use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct CollectPayment<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority)]
    pub payer: Signer<'info>, // This account is the merchant_authority.current_authority

    #[account(seeds = [b"merchant_authority", merchant_authority.key().as_ref(), payer.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.key() == payment_metadata.payment_config @ ErrorCode::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(seeds = [b"payment_metadata", payment_metadata_owner.key().as_ref(), payment_config.key().as_ref()], bump)]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(mut, constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount)]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(constraint = payment_metadata_owner.key() == payment_metadata.owner @ ErrorCode::IncorrectPaymentMetadataOwner)]
    pub payment_metadata_owner: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = owner_payment_account.key() == payment_metadata.owner_payment_account @ ErrorCode::IncorrectOwnerPaymentAccount,
        constraint = owner_payment_account.delegate.unwrap() == program_as_signer.key() @ ErrorCode::IncorrectDelegateForTokenAccount
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<CollectPayment>) -> ProgramResult {
    let owner_payment_account = &mut ctx.accounts.owner_payment_account;

    let delegate = owner_payment_account.delegate;

    Ok(())
}
