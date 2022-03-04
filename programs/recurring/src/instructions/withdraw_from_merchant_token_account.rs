use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFromMerchantTokenAccount<'info> {
    pub payer: Signer<'info>,
}

pub fn handler(ctx: Context<WithdrawFromMerchantTokenAccount>) -> ProgramResult {
    Ok(())
}
