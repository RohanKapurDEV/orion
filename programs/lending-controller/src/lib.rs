use anchor_lang::prelude::*;

mod instructions;
mod utils;

use instructions::*;
use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lending_controller {
    use super::*;

    // Wrapper function over recurring.withdraw_from_merchant_token_account to additionally deposit
    // merchant payments directly into Solend
    pub fn withdraw_to_solend_pool(
        ctx: Context<WithdrawToSolendPool>,
        payment_config_index: u8,
        merchant_authority_index: u8,
        amount_to_withdraw: u64,
    ) -> ProgramResult {
        instructions::withdraw_to_solend_pool::handler(
            ctx,
            payment_config_index,
            merchant_authority_index,
            amount_to_withdraw,
        )
    }
}
