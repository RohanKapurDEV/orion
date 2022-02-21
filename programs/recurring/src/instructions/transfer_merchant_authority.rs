use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    pub proposed_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferMerchantAuthority>) -> ProgramResult {
    let merchant_authority = &mut ctx.accounts.merchant_authority;
    let proposed_authority = &mut ctx.accounts.proposed_authority;

    merchant_authority.pending_authority = proposed_authority.key();

    Ok(())
}
