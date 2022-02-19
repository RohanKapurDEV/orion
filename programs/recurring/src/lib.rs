use anchor_lang::prelude::*;
use anchor_spl::token::{approve, transfer, Approve, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod recurring {
    use super::*;

    pub fn initialize_merchant_authority(
        ctx: Context<InitializeMerchantAuthority>,
        index: u16,
    ) -> ProgramResult {
        let bump = *ctx.bumps.get("merchant_authority").unwrap();
        let merchant_authority = &mut ctx.accounts.merchant_authority;

        merchant_authority.init_authority = ctx.accounts.authority.key(); // Stored for derivation purposes
        merchant_authority.current_authority = ctx.accounts.authority.key();
        merchant_authority.pending_authority = Pubkey::default();
        merchant_authority.bump = bump;

        Ok(())
    }

    pub fn initialize_payment_config(ctx: Context<InitializePaymentConfig>) -> ProgramResult {
        let bump = *ctx.bumps.get("payment_config").unwrap();
        let payment_config = &mut ctx.accounts.payment_config;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct InitializeMerchantAuthority<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"merchant_authority", merchant_authority.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    pub authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePaymentConfig<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_auth.key().as_ref()],
        bump
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    pub merchant_auth: Account<'info, MerchantAuthority>,

    pub payment_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = payment_mint,
        token::authority = merchant_auth
    )]
    pub payment_pda: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct MerchantAuthority {
    pub init_authority: Pubkey,
    pub current_authority: Pubkey,
    pub pending_authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct PaymentConfig {
    pub payment_mint: Pubkey,
    pub payment_pda: Pubkey,
    pub merchant_authority: Pubkey,
    pub time_delay: i64,
    pub delay_format: u8, // delay format for payment schedule (start of next minute, hour, day, week, etc...)
    pub bump: u8,
}
