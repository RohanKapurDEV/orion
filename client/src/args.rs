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
    /// Novel index - This should be scoped to the MerchantAuthority address
    #[clap(short, long)]
    pub index: u8,
    /// mainnet or devnet
    #[clap(short, long)]
    pub network: String,
    /// Address of merchant_authority account
    #[clap(short, long)]
    pub merchant_authority: String,
    /// The mint of which to expect tokens from as payment
    #[clap(short, long)]
    pub payment_mint: String,
    /// Amount of time in seconds in between payments
    #[clap(short, long)]
    pub spacing_period: i64,
    /// Collect money on creation of associated PaymentMetdata?
    #[clap(short, long)]
    pub collect_on_init: bool,
    /// Amount to collect on creation of associated PaymentMetdata
    #[clap(short, long)]
    pub amount_to_collect_on_init: u64,
    /// Amount to collect per period
    #[clap(short, long)]
    pub amount_to_collect_per_period: u64,
}

#[derive(Debug, Args)]
pub struct PaymentMetadataParams {
    /// The path to the JSON keypair to use the sign the transaction
    #[clap(short, long)]
    pub keypair_path: String,
}
