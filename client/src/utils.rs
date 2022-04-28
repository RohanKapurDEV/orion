use crate::error::AppError;

// Some useful constants
pub const PROGRAM_ID: &str = "CFHiFGAChg829XSFBRhswft7Vnmc9tQdR3Esiqcxmeef";
pub const PAYER: &str = "4puafxtL1437aibBy4pCteADWjja9aQvygD9LhkwRMG5";
pub const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const ASSOCIATED_TOKEN_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
pub const RENT: &str = "SysvarRent111111111111111111111111111111111";
pub const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";
pub const DEVNET_RPC_HTTP: &str = "https://api.devnet.solana.com";
pub const DEVNET_RPC_WS: &str = "wss://api.devnet.solana.com";
pub const MAINNET_RPC_HTTP: &str = "";
pub const MAINNET_RPC_WS: &str = "";
pub const MERCHANT_AUTHORITY_INDEX: u8 = 2;

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
