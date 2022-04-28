use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "Rohan Kapur | @0xrohan on Twitter",
    version,
    about = "A CLI application to interact with the recurring smart contract"
)]
pub struct ClientArgs {
    /// Initialize Merchant Account
    #[clap(subcommand)]
    pub subcommand: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Initialize a MerchantAuthority account
    InitMerchantAccount(MerchantAccountParams),
    /// Initialize a PaymentConfig account
    InitPaymentConfig(PaymentConfigParams),
    /// Initialize a PaymentMetadata account
    InitPaymentMetadata(PaymentMetadataParams),
}

#[derive(Debug, Args)]
pub struct MerchantAccountParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(short, long)]
    pub keypair_path: String,
    /// Novel index - This should be scoped to the init_authority field
    #[clap(short, long)]
    pub index: u8,
    /// mainnet or devnet
    #[clap(short, long)]
    pub network: String,
}

#[derive(Debug, Args)]
pub struct PaymentConfigParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(short, long)]
    pub keypair_path: String,
}

#[derive(Debug, Args)]
pub struct PaymentMetadataParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(short, long)]
    pub keypair_path: String,
}
