use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct AcceptMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.pending_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [b"merchant_authority".as_ref(), &index.to_le_bytes(), init_authority.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<AcceptMerchantAuthority>, _index: u8) -> ProgramResult {
    let merchant_authority = &mut ctx.accounts.merchant_authority;
    let new_authority = &mut ctx.accounts.payer;

    let previous_authority = merchant_authority.current_authority; // for use in event

    merchant_authority.current_authority = new_authority.key();
    merchant_authority.pending_authority = Pubkey::default();

    Ok(())
}
