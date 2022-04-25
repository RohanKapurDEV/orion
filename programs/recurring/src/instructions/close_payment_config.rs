use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8, merchant_authority_index: u8)]
pub struct ClosePaymentConfig<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ RecurringError::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(seeds = [b"merchant_authority", &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config", &index.to_le_bytes(),  merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ RecurringError::IncorrectAuthorityForPaymentConfig,
        close = init_authority
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    /// CHECK: not used in instruction logic, just as close target for payment_config. validated in constraint
    #[account(mut, constraint = init_authority.key() == merchant_authority.init_authority @ RecurringError::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(
    _ctx: Context<ClosePaymentConfig>,
    _index: u8,
    _merchant_authority_index: u8,
) -> Result<()> {
    Ok(())
}
