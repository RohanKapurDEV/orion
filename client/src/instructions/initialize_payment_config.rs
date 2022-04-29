use crate::utils::*;
use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    Client, ClientError,
};
use recurring::accounts as recurring_accounts;
use recurring::instruction as recurring_ixs;
use recurring::state::*;
use std::fs;
use std::str::FromStr;

pub async fn initialize_payment_config() -> Result<bool, ClientError> {
    todo!()
}
