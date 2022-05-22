use crate::error::AppError;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair},
    Client, Cluster,
};
use std::fs;
use std::rc::Rc;

pub const PROGRAM_ID: &str = "4AWRyt6whM4M8C4rimokJxvDP6bts7NxWNxZuyFVKD31";
pub const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const ASSOCIATED_TOKEN_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
pub const RENT: &str = "SysvarRent111111111111111111111111111111111";
pub const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";
pub const DEVNET_RPC_HTTP: &str = "https://api.devnet.solana.com";
pub const DEVNET_RPC_WS: &str = "wss://api.devnet.solana.com";
pub const MAINNET_RPC_HTTP: &str = "";
pub const MAINNET_RPC_WS: &str = "";

pub const INCORRECT_AUTH_FOR_PAYMENT_CONFIG: &str =
    "Incorrect authority keypair - does not match MerchantAccount.current_authority field";

pub fn build_client(keypair_path: String, network: String) -> Client {
    let network_selector = validate_network(network).unwrap();
    let rpc = network_selector.fetch_rpc();

    let data = fs::read_to_string(keypair_path).expect("Unable to read file");
    let json: Vec<u8> = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let signer = Keypair::from_bytes(json.as_slice()).unwrap();

    let cluster = Cluster::Custom(rpc.0.to_string(), rpc.1.to_string());
    Client::new_with_options(cluster, Rc::new(signer), CommitmentConfig::processed())
}

#[allow(dead_code)]
pub enum NetworkSelector {
    Mainnet,
    Devnet,
}

impl NetworkSelector {
    pub fn fetch_rpc(self) -> (String, String) {
        match self {
            NetworkSelector::Mainnet => {
                return (MAINNET_RPC_HTTP.to_string(), MAINNET_RPC_WS.to_string())
            }
            NetworkSelector::Devnet => {
                return (DEVNET_RPC_HTTP.to_string(), DEVNET_RPC_WS.to_string())
            }
        }
    }
}

pub fn validate_network(network: String) -> Result<NetworkSelector, AppError> {
    let cmp = network.as_str();

    match cmp {
        "mainnet" => Ok(NetworkSelector::Mainnet),
        "devnet" => Ok(NetworkSelector::Devnet),
        _ => {
            return Err(AppError::NotValidNetwork);
        }
    }
}
