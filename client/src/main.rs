use std::{fmt::Display, rc::Rc, str::FromStr};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
    Client, ClientError, Cluster,
};
use dotenv::dotenv;
use recurring::accounts as recurring_accounts;
use recurring::instruction as recurring_ixs;
use recurring::state::*;

mod utils;

use utils::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Note that this env var should be a base58 string representation of a private key
    let payer_private_key = dotenv::var("PAYER_PRIVATE_KEY").unwrap();
    let payer_signer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer_signer.pubkey();

    let network = NetworkSelector::Devnet;
    let rpc = network.fetch_rpc();

    let cluster = Cluster::Custom(rpc.0.to_string(), rpc.1.to_string());
    let client = Client::new_with_options(
        cluster,
        Rc::new(payer_signer),
        CommitmentConfig::processed(),
    );

    let res = initialize_merchant_authority(
        &client,
        payer_pubkey.clone(),
        payer_pubkey.clone(),
        payer_private_key.clone(),
    )
    .await;

    match res {
        Ok(_value) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
}

pub async fn initialize_merchant_authority(
    client: &Client,
    payer: Pubkey,
    authority: Pubkey,
    payer_private_key: String,
) -> Result<bool, ClientError> {
    let program_id_pubkey = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_pubkey = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
    let payer_signer = Keypair::from_base58_string(&payer_private_key);

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

pub enum NetworkSelector {
    Mainnet,
    Devnet,
}

impl NetworkSelector {
    fn fetch_rpc(self) -> (String, String) {
        match self {
            NetworkSelector::Mainnet => return ("".to_string(), "".to_string()),
            NetworkSelector::Devnet => {
                return (DEVNET_RPC_HTTP.to_string(), DEVNET_RPC_WS.to_string())
            }
        }
    }
}
