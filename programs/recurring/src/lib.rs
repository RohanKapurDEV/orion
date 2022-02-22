use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod recurring {
    use super::*;

    // Merchant instructions

    pub fn initialize_merchant_authority(
        ctx: Context<InitializeMerchantAuthority>,
    ) -> ProgramResult {
        instructions::initialize_merchant_authority::handler(ctx)
    }

    pub fn transfer_merchant_authority(ctx: Context<TransferMerchantAuthority>) -> ProgramResult {
        instructions::transfer_merchant_authority::handler(ctx)
    }

    pub fn accept_merchant_authority(ctx: Context<AcceptMerchantAuthority>) -> ProgramResult {
        instructions::accept_merchant_authority::handler(ctx)
    }

    pub fn close_merchant_authority(ctx: Context<CloseMerchantAuthority>) -> ProgramResult {
        instructions::close_merchant_authority::handler(ctx)
    }

    pub fn initialize_payment_config(
        ctx: Context<InitializePaymentConfig>,
        minimum_amount_to_delegate: u64,
        spacing_period: i64,
        collect_on_init: bool,
        amount_to_collect_on_init: u64,
        amount_to_collect_per_period: u64,
    ) -> ProgramResult {
        instructions::initialize_payment_config::handler(
            ctx,
            minimum_amount_to_delegate,
            spacing_period,
            collect_on_init,
            amount_to_collect_on_init,
            amount_to_collect_per_period,
        )
    }

    pub fn close_payment_config(ctx: Context<ClosePaymentConfig>) -> ProgramResult {
        instructions::close_payment_config::handler(ctx)
    }

    pub fn collect_payment(ctx: Context<CollectPayment>) -> ProgramResult {
        instructions::collect_payment::handler(ctx)
    }

    // Consumer instructions

    pub fn initialize_payment_metadata(
        ctx: Context<InitializePaymentMetadata>,
        amount_delegated: u64,
    ) -> ProgramResult {
        instructions::initialize_payment_metadata::handler(ctx, amount_delegated)
    }

    pub fn close_payment_metadata(ctx: Context<ClosePaymentMetadata>) -> ProgramResult {
        instructions::close_payment_metadata::handler(ctx)
    }
}
