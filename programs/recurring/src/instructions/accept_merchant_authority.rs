use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[event]
pub struct AcceptMerchantAuthorityEvent {
    pub merchant_authority: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

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

pub fn handler(ctx: Context<AcceptMerchantAuthority>, _index: u8) -> Result<()> {
    let merchant_authority = &mut ctx.accounts.merchant_authority;
    let new_authority = &mut ctx.accounts.payer;

    let previous_authority = merchant_authority.current_authority;

    merchant_authority.current_authority = new_authority.key();
    merchant_authority.pending_authority = Pubkey::default();

    let clock = Clock::get()?;
    let unix_timestamp = clock.unix_timestamp;

    emit!(AcceptMerchantAuthorityEvent {
        merchant_authority: merchant_authority.key(),
        new_authority: new_authority.key(),
        old_authority: previous_authority,
        timestamp: unix_timestamp
    });

    Ok(())
}
