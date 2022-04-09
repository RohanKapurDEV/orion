use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct InitializeMerchantAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"merchant_authority".as_ref(), &index.to_le_bytes(), authority.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 32 + 1 + 1
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    /// CHECK: only used as field in merchant_authority account
    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMerchantAuthority>, index: u8) -> Result<()> {
    let bump = *ctx.bumps.get("merchant_authority").unwrap();
    let merchant_authority = &mut ctx.accounts.merchant_authority;

    merchant_authority.init_authority = ctx.accounts.authority.key();
    merchant_authority.current_authority = ctx.accounts.authority.key();
    merchant_authority.pending_authority = Pubkey::default();
    merchant_authority.index = index;
    merchant_authority.bump = bump;

    Ok(())
}
