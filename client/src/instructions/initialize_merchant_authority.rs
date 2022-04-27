use crate::utils::*;
use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    Client, ClientError,
};
use recurring::accounts as recurring_accounts;
use recurring::instruction as recurring_ixs;
use recurring::state::*;
use std::str::FromStr;

#[allow(dead_code)]
/// Rent + ix cost = 0.00326728 SOL
pub async fn initialize_merchant_authority(
    client: &Client,
    payer: Pubkey,
    payer_private_key: String,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
    let payer_signer = Keypair::from_base58_string(&payer_private_key);
    let authority = payer_signer.pubkey();

    let program = client.program(program_id_pubkey);
    let (merchant_authority, _bump) = Pubkey::find_program_address(
        &[
            b"merchant_authority",
            &MERCHANT_AUTHORITY_INDEX.to_le_bytes(),
            &authority.to_bytes(),
        ],
        &program_id_pubkey,
    );

    let accounts = recurring_accounts::InitializeMerchantAuthority {
        payer: payer,
        merchant_authority,
        authority,
        system_program: system_program_pubkey,
    };
    let params = recurring_ixs::InitializeMerchantAuthority {
        index: MERCHANT_AUTHORITY_INDEX,
    };

    program
        .request()
        .signer(&payer_signer)
        .accounts(accounts)
        .args(params)
        .send()?;

    let merch_auth: MerchantAuthority = program.account(merchant_authority)?;

    println!(
        "Merchant authority account: {}",
        merchant_authority.to_string()
    );
    println!("Current authority: {}", merch_auth.current_authority);
    println!("Index: {}", merch_auth.index);

    Ok(true)
}
