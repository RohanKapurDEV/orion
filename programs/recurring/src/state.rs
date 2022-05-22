use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct MerchantAuthority {
    pub init_authority: Pubkey,
    pub current_authority: Pubkey,
    pub pending_authority: Pubkey,
    pub index: u8,
    pub bump: u8,
}

/// When you want to change the authority of a PaymentConfig, simply switch out the currrent_authority
/// on it's associated MerchantAuthority. The operator will still need to keep track of the Merchant-
/// Authority's initial authority account. This is stored in the account itself for convenience.
#[account]
#[derive(Default)]
pub struct PaymentConfig {
    pub payment_mint: Pubkey,
    pub payment_token_account: Pubkey,
    pub merchant_authority: Pubkey,
    pub payment_account: Pubkey,
    pub collect_on_init: bool,
    pub amount_to_collect_on_init: u64,
    pub amount_to_collect_per_period: u64,
    pub spacing_period: i64, // seconds in between payment collections
    pub index: u8,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct PaymentMetadata {
    pub owner: Pubkey,
    pub created_at: i64,
    pub payment_config: Pubkey,
    pub owner_payment_account: Pubkey,
    pub amount_delegated: u64,
    pub payments_collected: u16,
    pub bump: u8,
}
