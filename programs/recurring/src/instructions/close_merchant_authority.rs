use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[event]
pub struct CloseMerchantAuthorityEvent {
    pub merchant_authority: Pubkey,
    pub closing_authority: Pubkey,
    pub timestamp: i64,
}

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

pub fn handler(ctx: Context<CloseMerchantAuthority>, _index: u8) -> Result<()> {
    let clock = Clock::get()?;
    let unix_timestamp = clock.unix_timestamp;

    emit!(CloseMerchantAuthorityEvent {
        merchant_authority: ctx.accounts.merchant_authority.key(),
        closing_authority: ctx.accounts.payer.key(),
        timestamp: unix_timestamp
    });

    Ok(())
}
