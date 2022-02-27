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
        index: u8,
    ) -> ProgramResult {
        instructions::initialize_merchant_authority::handler(ctx, index)
    }

    pub fn transfer_merchant_authority(
        ctx: Context<TransferMerchantAuthority>,
        index: u8,
    ) -> ProgramResult {
        instructions::transfer_merchant_authority::handler(ctx, index)
    }

    pub fn accept_merchant_authority(
        ctx: Context<AcceptMerchantAuthority>,
        index: u8,
    ) -> ProgramResult {
        instructions::accept_merchant_authority::handler(ctx, index)
    }

    pub fn close_merchant_authority(
        ctx: Context<CloseMerchantAuthority>,
        index: u8,
    ) -> ProgramResult {
        instructions::close_merchant_authority::handler(ctx, index)
    }

    pub fn initialize_payment_config(
        ctx: Context<InitializePaymentConfig>,
        index: u8,
        spacing_period: i64,
        collect_on_init: bool,
        amount_to_collect_on_init: u64,
        amount_to_collect_per_period: u64,
    ) -> ProgramResult {
        instructions::initialize_payment_config::handler(
            ctx,
            index,
            spacing_period,
            collect_on_init,
            amount_to_collect_on_init,
            amount_to_collect_per_period,
        )
    }

    pub fn close_payment_config(ctx: Context<ClosePaymentConfig>, index: u8) -> ProgramResult {
        instructions::close_payment_config::handler(ctx, index)
    }

    pub fn collect_payment(ctx: Context<CollectPayment>) -> ProgramResult {
        instructions::collect_payment::handler(ctx)
    }

    pub fn reinstate_failed_payment_metadata(
        ctx: Context<ReinstateFailedPaymentMetadata>,
    ) -> ProgramResult {
        instructions::reinstate_failed_payment_metadata::handler(ctx)
    }

    // // Consumer instructions

    pub fn initialize_payment_metadata(
        ctx: Context<InitializePaymentMetadata>,
        amount_delegated: u64,
    ) -> ProgramResult {
        instructions::initialize_payment_metadata::handler(ctx, amount_delegated)
    }

    pub fn close_payment_metadata(
        ctx: Context<ClosePaymentMetadata>,
        payment_config_index: u8,
        merchant_authority_index: u8,
    ) -> ProgramResult {
        instructions::close_payment_metadata::handler(
            ctx,
            payment_config_index,
            merchant_authority_index,
        )
    }
}
