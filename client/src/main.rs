use clap::Parser;

mod args;
mod error;
mod instructions;
mod utils;

use args::*;
use instructions::*;
use utils::*;

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();

    match args {
        ClientArgs { subcommand } => match subcommand {
            EntityType::InitMerchantAccount(params) => {
                let network = params.network;
                let keypair_path = params.keypair_path;

                let client = build_client(keypair_path.clone(), network);
                let res = initialize_merchant_authority(&client, keypair_path, params.index).await;

                match res {
                    Ok(_) => {
                        println!("Successful tx")
                    }
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }
            _ => {}
        },
    }
}
