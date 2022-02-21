use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref()],
        bump,
        close = payer
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<CloseMerchantAuthority>) -> ProgramResult {
    Ok(())
}
