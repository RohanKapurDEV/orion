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

#[allow(dead_code)]
/// Rent + ix cost = 0.00326728 SOL
pub async fn initialize_merchant_authority(
    client: &Client,
    keypair_path: String,
    index: u8,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();

    let data = fs::read_to_string(keypair_path).expect("Unable to read file");
    let json: Vec<u8> = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let payer_signer = Keypair::from_bytes(json.as_slice()).unwrap();
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
        payer: authority,
        merchant_authority,
        authority,
        system_program: system_program_pubkey,
    };
    let params = recurring_ixs::InitializeMerchantAuthority { index };

    let tx = program
        .request()
        .signer(&payer_signer)
        .accounts(accounts)
        .args(params)
        .send()?;

    let merch_auth: MerchantAuthority = program.account(merchant_authority)?;

    println!("Successfully initialized MerchantAuthority account!");

    println!("TX Sig: {}", tx.to_string());

    println!(
        "Merchant authority account address: {}",
        merchant_authority.to_string()
    );
    println!("Bump: {}", merch_auth.bump);
    println!("Index: {}", merch_auth.index);
    println!("Init authority: {}", merch_auth.init_authority);
    println!("Pending authority: {}", merch_auth.pending_authority);
    println!("Current authority: {}", merch_auth.current_authority);

    Ok(true)
}
