use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct TransferMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ RecurringError::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [b"merchant_authority".as_ref(), &index.to_le_bytes(), init_authority.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ RecurringError::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    pub proposed_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferMerchantAuthority>, _index: u8) -> Result<()> {
    let merchant_authority = &mut ctx.accounts.merchant_authority;
    let proposed_authority = &mut ctx.accounts.proposed_authority;

    merchant_authority.pending_authority = proposed_authority.key();

    Ok(())
}
