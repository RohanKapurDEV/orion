use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

use error::*;
use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod recurring {
    use super::*;

    pub fn initialize_merchant_authority(
        ctx: Context<InitializeMerchantAuthority>,
    ) -> ProgramResult {
        instructions::initialize_merchant_authority::handler(ctx)
    }

    pub fn initialize_payment_config(
        ctx: Context<InitializePaymentConfig>,
        minimum_amount_to_delegate: u64,
        spacing_period: i64,
        delay_format: u8,
    ) -> ProgramResult {
        instructions::initialize_payment_config::handler(
            ctx,
            minimum_amount_to_delegate,
            spacing_period,
            delay_format,
        )
    }

    pub fn initialize_payment_metadata(
        ctx: Context<InitializePaymentMetadata>,
        amount_delegated: u64,
    ) -> ProgramResult {
        instructions::initialize_payment_metadata::handler(ctx, amount_delegated)
    }
}
