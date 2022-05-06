use anchor_client::{
    solana_client::rpc_config::{
        RpcTransactionConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
    },
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client, ClientError, Cluster, EventContext,
};
use recurring::instructions::InitializeMerchantAuthorityEvent;
use std::str::FromStr;
use std::time::Duration;

use anchor_client::solana_client::rpc_response::RpcKeyedAccount;
use solana_client_async::{prelude::*, rpc_message::RpcResponse};

mod error;
mod utils;

use error::*;
use utils::*;

#[tokio::main]
async fn main() {
    let mut client = ClientBuilder::new()
        .ws_url("wss://api.mainnet-beta.solana.com")
        .build()
        .await
        .unwrap();

    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

    let logs_filter = RpcTransactionLogsFilter::All;
    let logs_config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::processed()),
    };

    let _subscription_id: u64 = client
        .logs_subscribe(logs_filter, logs_config)
        .await
        .unwrap()
        .await
        .unwrap(); // Double await because the first await is for `Send` and the second one for `Receive`. It is fine to drop the second one.

    loop {
        let slot = client.recv::<RpcResponse>().await.unwrap();
        println!("slot {:?}", slot);
    }

    // let client = build_client(
    //     "/Users/rohan/.config/solana/id.json".to_string(),
    //     "devnet".to_string(),
    // );

    // let res = listen_initialize_merchant_authority_event(&client).await;
}

async fn listen_initialize_merchant_authority_event(client: &Client) -> Result<(), ClientError> {
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

    let program = client.program(program_id);

    println!("1");

    let (sender, receiver) = std::sync::mpsc::channel();
    let handle = program.on(
        move |_ctx: &EventContext, event: InitializeMerchantAuthorityEvent| {
            sender
                .send(event)
                .expect("Could not send value back to channel");
        },
    )?;

    let event = receiver.recv().unwrap();

    println!("Address: {}", event.merchant_authority_pubkey);
    println!("Index: {}", event.index);
    println!("Bump: {}", event.bump);
    println!("Authority: {}", event.authority);

    // TODO: remove once https://github.com/solana-labs/solana/issues/16102
    //       is addressed. Until then, drop the subscription handle in another
    //       thread so that we deadlock in the other thread as to not block
    //       this thread.
    std::thread::spawn(move || {
        drop(handle);
    });

    println!("Events success!");

    Ok(())
}
