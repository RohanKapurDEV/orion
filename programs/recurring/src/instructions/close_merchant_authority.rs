use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct CloseMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"merchant_authority", &index.to_le_bytes(), init_authority.key().as_ref()],
        bump,
        close = init_authority
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(mut, constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(_ctx: Context<CloseMerchantAuthority>, _index: u8) -> ProgramResult {
    Ok(())
}
