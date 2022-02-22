use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePaymentConfig<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(seeds = [b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ ErrorCode::IncorrectAuthorityForPaymentConfig,        
        close = init_authority
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<ClosePaymentConfig>) -> ProgramResult {
    Ok(())
}
