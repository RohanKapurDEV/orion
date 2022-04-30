use crate::{args::MerchantAccountParams, utils::*};
use anchor_client::{
    anchor_lang::Key,
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    Client, ClientError,
};
use recurring::accounts as recurring_accounts;
use recurring::instruction as recurring_ixs;
use recurring::state::*;
use std::fs;
use std::str::FromStr;

pub async fn initialize_payment_metadata(
    client: &Client,
    keypair_path: String,
    merchant_authority: String,
    payment_config: String,
    amount_delegated: u64,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
    let token_program_pubkey = Pubkey::from_str(TOKEN_PROGRAM).unwrap();
    let rent_pubkey = Pubkey::from_str(RENT).unwrap();
    let assoc_token_program_pubkey = Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM).unwrap();

    let data = fs::read_to_string(keypair_path).expect("Unable to read file");
    let json: Vec<u8> = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let payer_signer = Keypair::from_bytes(json.as_slice()).unwrap();
    let authority = payer_signer.pubkey();

    let program = client.program(program_id_pubkey);

    let merchant_authority_pubkey = Pubkey::from_str(&merchant_authority)
        .expect("Unable to build pubkey from merchant_authority param");

    let payment_config_pubkey = Pubkey::from_str(&payment_config)
        .expect("Unable to build pubkey from payment_config param");

    let merchant_authority_account: MerchantAuthority =
        program.account(merchant_authority_pubkey)?;

    let payment_config_account: PaymentConfig = program.account(payment_config_pubkey)?;

    let merchant_authority_index = merchant_authority_account.index;
    let init_authority = merchant_authority_account.init_authority;
    let payment_config_index = payment_config_account.index;
    let payment_token_mint = payment_config_account.payment_mint;
    let payment_token_account = payment_config_account.payment_token_account;

    let (payment_metadata_pubkey, _bump) = Pubkey::find_program_address(
        &[
            b"payment_metadata",
            &authority.to_bytes(),
            &payment_config_pubkey.to_bytes(),
        ],
        &program_id_pubkey,
    );

    let (owner_payment_account, _bump) = Pubkey::find_program_address(
        &[
            &authority.to_bytes(),
            &token_program_pubkey.to_bytes(),
            &payment_token_mint.to_bytes(),
        ],
        &assoc_token_program_pubkey,
    );

    let (program_as_signer, _bump) =
        Pubkey::find_program_address(&[b"program", b"signer"], &program_id_pubkey);

    let accounts = recurring_accounts::InitializePaymentMetadata {
        init_authority,
        merchant_authority: merchant_authority_pubkey,
        owner_payment_account,
        payer: authority,
        payment_config: payment_config_pubkey,
        payment_metadata: payment_metadata_pubkey,
        payment_token_account,
        program_as_signer,
        system_program: system_program_pubkey,
        token_program: token_program_pubkey,
    };

    let params = recurring_ixs::InitializePaymentMetadata {
        merchant_authority_index,
        payment_config_index,
        amount_delegated,
    };

    let tx = program
        .request()
        .accounts(accounts)
        .args(params)
        .signer(&payer_signer)
        .send()?;

    let payment_meta: PaymentMetadata = program.account(payment_metadata_pubkey)?;

    println!("Successfully initialized PaymentConfig account!");

    println!("Tx Sig: {}", tx.to_string());

    println!(
        "Payment metadata account address: {}",
        payment_metadata_pubkey.to_string()
    );

    println!("Bump: {}", payment_meta.bump);

    println!("Amount delegated: {}", payment_meta.amount_delegated);

    println!("Created at:: {}", payment_meta.created_at);

    println!("Owner: {}", payment_meta.owner);

    println!(
        "Owner payment account: {}",
        payment_meta.owner_payment_account
    );

    println!("Payment config: {}", payment_meta.payment_config);

    println!("Payments collected: {}", payment_meta.payments_collected);

    Ok(true)
}
