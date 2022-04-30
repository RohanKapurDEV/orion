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

pub async fn initialize_payment_config(
    client: &Client,
    keypair_path: String,
    merchant_authority: String,
    payment_mint: String,
    index: u8,
    spacing_period: i64,
    collect_on_init: bool,
    amount_to_collect_on_init: u64,
    amount_to_collect_per_period: u64,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
    let token_program_pubkey = Pubkey::from_str(TOKEN_PROGRAM).unwrap();
    let rent_pubkey = Pubkey::from_str(RENT).unwrap();

    let data = fs::read_to_string(keypair_path).expect("Unable to read file");
    let json: Vec<u8> = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let payer_signer = Keypair::from_bytes(json.as_slice()).unwrap();
    let authority = payer_signer.pubkey();

    let program = client.program(program_id_pubkey);

    let payment_mint_pubkey =
        Pubkey::from_str(&payment_mint).expect("Unable to build pubkey from payment_mint param");

    let merchant_authority_pubkey = Pubkey::from_str(&merchant_authority)
        .expect("Unable to build pubkey from merchant_authority param");

    let merchant_authority_account: MerchantAuthority =
        program.account(merchant_authority_pubkey)?;

    if merchant_authority_account.current_authority != authority {
        panic!("{}", INCORRECT_AUTH_FOR_PAYMENT_CONFIG)
    }

    let (payment_config_pubkey, _bump) = Pubkey::find_program_address(
        &[
            b"payment_config",
            &index.to_le_bytes(),
            &merchant_authority_pubkey.to_bytes(),
        ],
        &program_id_pubkey,
    );

    let payment_token_account_keypair = Keypair::new();
    let payment_token_account = payment_token_account_keypair.pubkey();

    let accounts = recurring_accounts::InitializePaymentConfig {
        payer: authority,
        merchant_auth: merchant_authority_pubkey,
        payment_config: payment_config_pubkey,
        payment_mint: payment_mint_pubkey,
        payment_token_account,
        rent: rent_pubkey,
        system_program: system_program_pubkey,
        token_program: token_program_pubkey,
    };

    let params = recurring_ixs::InitializePaymentConfig {
        amount_to_collect_on_init,
        amount_to_collect_per_period,
        collect_on_init,
        index,
        spacing_period,
    };

    let tx = program
        .request()
        .signer(&payer_signer)
        .signer(&payment_token_account_keypair)
        .accounts(accounts)
        .args(params)
        .send()?;

    let payment_conf: PaymentConfig = program.account(payment_config_pubkey)?;

    println!("Successfully initialized PaymentConfig account!");

    println!("Tx Sig: {}", tx.to_string());

    println!(
        "Payment Config account address: {}",
        payment_config_pubkey.to_string()
    );

    println!("Bump: {}", payment_conf.bump);
    println!("Index: {}", payment_conf.index);
    println!("Collect on init: {}", payment_conf.collect_on_init);
    println!(
        "Amount to collect on init: {}",
        payment_conf.amount_to_collect_on_init
    );
    println!(
        "Amount to collect per period: {}",
        payment_conf.amount_to_collect_per_period
    );
    println!("Merchant authority: {}", payment_conf.merchant_authority);
    println!("Spacing period: {}", payment_conf.spacing_period);
    println!(
        "Payment token account: {}",
        payment_conf.payment_token_account
    );

    Ok(true)
}
