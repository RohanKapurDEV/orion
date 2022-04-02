use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReinstateFailedPaymentMetadata<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"payment_metadata", payment_metadata.owner.as_ref(), payment_config.key().as_ref()],
        bump,
        constraint = payment_metadata.payment_config == payment_config.key()
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(
        mut,
        seeds = [b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref()],
        bump,
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ ErrorCode::IncorrectAuthorityForPaymentConfig
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ReinstateFailedPaymentMetadata>) -> Result<()> {
    let clock = Clock::get()?;
    let unix_timestamp = clock.unix_timestamp;

    let payment_metadata = &mut ctx.accounts.payment_metadata;

    if payment_metadata.payment_failure == true {
        payment_metadata.created_at = unix_timestamp;
        payment_metadata.payment_failure = false;
        payment_metadata.payments_collected = 0;
    } else {
        return Err(ErrorCode::PaymentMetadataNotInFailedState.into());
    }

    Ok(())
}
