use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct ClosePaymentConfig<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config".as_ref(), &index.to_le_bytes(),  merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ ErrorCode::IncorrectAuthorityForPaymentConfig,        
        close = init_authority
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(mut, constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<ClosePaymentConfig>, _index: u8) -> ProgramResult {
    Ok(())
}
