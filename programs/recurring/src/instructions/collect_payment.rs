use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct CollectPayment<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectCollectionAuthority)]
    pub payer: Signer<'info>, // This account is the merchant_authority.current_authority

    #[account(seeds = [b"merchant_authority", merchant_authority.key().as_ref(), payer.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_authority.key().as_ref()],
        bump,
        constraint = payment_config.key() == payment_metadata.payment_config @ ErrorCode::IncorrectPaymentConfigAccount
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(mut, seeds = [b"payment_metadata", payment_metadata_owner.key().as_ref(), payment_config.key().as_ref()], bump)]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(mut, constraint = payment_token_account.key() == payment_config.payment_token_account @ ErrorCode::IncorrectPaymentTokenAccount)]
    pub payment_token_account: Account<'info, TokenAccount>,

    #[account(constraint = payment_metadata_owner.key() == payment_metadata.owner @ ErrorCode::IncorrectPaymentMetadataOwner)]
    pub payment_metadata_owner: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = owner_payment_account.key() == payment_metadata.owner_payment_account @ ErrorCode::IncorrectOwnerPaymentAccount,
        constraint = owner_payment_account.delegate.unwrap() == program_as_signer.key() @ ErrorCode::IncorrectDelegateForTokenAccount
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CollectPayment>) -> ProgramResult {
    let payment_config = &mut ctx.accounts.payment_config;
    let owner_payment_account = &mut ctx.accounts.owner_payment_account;
    let payment_metadata = &mut ctx.accounts.payment_metadata;
    let program_as_signer = &mut ctx.accounts.program_as_signer;
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();

    let amount_being_spent = payment_config.amount_to_collect_per_period;
    // For now, assume this field is updated wheneever the delegate authority spends from the token account
    let delegated_amount = owner_payment_account.delegated_amount;

    require!(
        delegated_amount >= amount_being_spent,
        ErrorCode::InsufficientBalanceToDelegate
    );

    require!(
        owner_payment_account.delegate.unwrap() == program_as_signer.key(),
        ErrorCode::ProgramAsSignerNotAuthorized
    );

    // Make sure the authority is calling the collect function at or after the timestamp at which the current payment is due.
    // We can use payment_config.spacing_period, payment_metadata.payments_collected, and Clock to calculate the appropriate values.

    // (payment_metadata.created_at + ((payment_metadata.payments_collected + 1) * payment_config.spacing_period)) >= current_timestamp => PAYMENT AUTHORIZED, else, PAYMENT UNAUTHORIZED

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let obligation_created_at = payment_metadata.created_at;
    let spacing_period = payment_config.spacing_period;

    let applied_payments_collected = payment_metadata
        .payments_collected
        .checked_add(1 as u16)
        .unwrap();

    let time_delta = spacing_period
        .checked_mul(applied_payments_collected as i64)
        .unwrap();

    let base_value = obligation_created_at.checked_add(time_delta).unwrap();

    require!(
        base_value >= current_timestamp,
        ErrorCode::PaymentUnauthorized
    );

    let transfer_accounts = Transfer {
        from: ctx.accounts.owner_payment_account.to_account_info(),
        to: ctx.accounts.payment_token_account.to_account_info(),
        authority: ctx.accounts.program_as_signer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );

    let transfer_attempt = transfer(cpi_ctx, amount_being_spent);

    match transfer_attempt {
        Ok(_x) => (),
        Err(y) => {
            payment_metadata.payment_failure = true;
            return Err(y);
        }
    }

    payment_metadata.payments_collected = payment_metadata
        .payments_collected
        .checked_add(1 as u16)
        .unwrap();

    Ok(())
}
