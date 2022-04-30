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
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }

            EntityType::InitPaymentConfig(params) => {
                let network = params.network;
                let keypair_path = params.keypair_path;

                let client = build_client(keypair_path.clone(), network);
                let res = initialize_payment_config(
                    &client,
                    keypair_path,
                    params.merchant_authority,
                    params.payment_mint,
                    params.index,
                    params.spacing_period,
                    params.collect_on_init,
                    params.amount_to_collect_on_init,
                    params.amount_to_collect_per_period,
                )
                .await;

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }

            EntityType::InitPaymentMetadata(params) => {
                let network = params.network;
                let keypair_path = params.keypair_path;

                let client = build_client(keypair_path.clone(), network);
                let res = initialize_payment_metadata(
                    &client,
                    keypair_path,
                    params.merchant_authority,
                    params.payment_config,
                    params.amount_delegated,
                )
                .await;

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e.to_string())
                    }
                }
            }
            #[allow(unreachable_patterns)]
            _ => {}
        },
    }
}
