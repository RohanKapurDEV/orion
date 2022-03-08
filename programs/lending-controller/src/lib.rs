use anchor_lang::prelude::*;

mod instructions;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lending_controller {
    use super::*;

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
