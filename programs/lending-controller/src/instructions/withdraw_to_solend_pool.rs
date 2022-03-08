use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use recurring::cpi::accounts::WithdrawFromMerchantTokenAccount;
use recurring::program::Recurring;
use recurring::{self, error::*, state::*};

// If you're using rust-analyzer with VSCode, ignore the import errors at the top here.
// The code compiles just fine. r-a just acts up about the imports for some reason :)

#[derive(Accounts)]
#[instruction(payment_config_index: u8, merchant_authority_index: u8, amount_to_withdraw: u64)]
pub struct WithdrawToSolendPool<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority )]
    pub payer: Signer<'info>,

    #[account(constraint = metadata_owner.key() == payment_metadata.owner)]
    pub metadata_owner: UncheckedAccount<'info>,

    #[account(
        seeds = [b"payment_metadata".as_ref(), metadata_owner.key().as_ref(), payment_config.key().as_ref()],
        bump,
        constraint = payment_metadata.payment_config == payment_config.key() @ ErrorCode::IncorrectPaymentConfigAccount,
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(
        seeds = [b"merchant_authority".as_ref(), &merchant_authority_index.to_le_bytes(), init_authority.key().as_ref()],
        bump,
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        seeds = [b"payment_config".as_ref(), &payment_config_index.to_le_bytes(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.merchant_authority == merchant_authority.key() @ ErrorCode::IncorrectMerchantAuthority,
        constraint = payment_metadata.payment_config == payment_config.key() @ ErrorCode::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        mut,
        constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount,
        constraint = payment_token_account.amount >= amount_to_withdraw @ ErrorCode::PaymentTokenAccountBalanceTooLow
    )]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = receiver_token_account.mint == payment_config.payment_mint @ ErrorCode::IncorrectMint)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub recurring_program: Program<'info, Recurring>,
}

impl<'info> WithdrawToSolendPool<'info> {
    fn set_data_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, WithdrawFromMerchantTokenAccount<'info>> {
        let cpi_program = self.recurring_program.to_account_info();
        let cpi_accounts = WithdrawFromMerchantTokenAccount {
            payer: self.payer.to_account_info(),
            metadata_owner: self.metadata_owner.to_account_info(),
            payment_metadata: self.payment_metadata.to_account_info(),
            merchant_authority: self.merchant_authority.to_account_info(),
            payment_config: self.payment_config.to_account_info(),
            payment_token_account: self.payment_token_account.to_account_info(),
            receiver_token_account: self.receiver_token_account.to_account_info(),
            init_authority: self.init_authority.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handler(
    ctx: Context<WithdrawToSolendPool>,
    payment_config_index: u8,
    merchant_authority_index: u8,
    amount_to_withdraw: u64,
) -> ProgramResult {
    // Step 1: Call CPI on recurring program
    recurring::cpi::withdraw_from_merchant_token_account(
        ctx.accounts.set_data_ctx(),
        payment_config_index,
        merchant_authority_index,
        amount_to_withdraw,
    );

    // Step 2: Call CPI on solend to deposit from receiver_token_account

    Ok(())
}
