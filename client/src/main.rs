use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::Keypair,
        signer::{keypair, Signer},
    },
    Client, Cluster,
};
use clap::Parser;
use dotenv::dotenv;
use std::fs;
use std::rc::Rc;

mod args;
mod error;
mod instructions;
mod utils;

use args::*;
use instructions::*;
use utils::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = ClientArgs::parse();

    match args {
        ClientArgs { subcommand } => match subcommand {
            EntityType::InitMerchantAccount(params) => {}
            _ => {}
        },
    }

    // Note that this env var should be a base58 string representation of a private key
    // let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    // let payer_signer = Keypair::from_base58_string(&payer_private_key);
    // let payer_pubkey = payer_signer.pubkey();

    // let network = NetworkSelector::Devnet;
    // let rpc = network.fetch_rpc();

    // let cluster = Cluster::Custom(rpc.0.to_string(), rpc.1.to_string());
    // let client = Client::new_with_options(
    //     cluster,
    //     Rc::new(payer_signer),
    //     CommitmentConfig::processed(),
    // );

    // let res =
    //     initialize_merchant_authority(&client, payer_pubkey.clone(), payer_private_key.clone())
    //         .await;

    // match res {
    //     Ok(_value) => {}
    //     Err(e) => {
    //         println!("{}", e)
    //     }
    // }
}
