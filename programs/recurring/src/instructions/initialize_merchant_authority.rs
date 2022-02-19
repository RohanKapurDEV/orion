use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeMerchantAuthority<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"merchant_authority", merchant_authority.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMerchantAuthority>) -> ProgramResult {
    let bump = *ctx.bumps.get("merchant_authority").unwrap();
    let merchant_authority = &mut ctx.accounts.merchant_authority;

    merchant_authority.init_authority = ctx.accounts.authority.key(); // Stored for derivation purposes
    merchant_authority.current_authority = ctx.accounts.authority.key();
    merchant_authority.pending_authority = Pubkey::default();
    merchant_authority.bump = bump;

    Ok(())
}
