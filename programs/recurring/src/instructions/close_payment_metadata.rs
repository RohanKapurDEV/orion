use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use spl_token::instruction::revoke;

#[derive(Accounts)]
pub struct ClosePaymentMetadata<'info> {
    #[account(constraint = payer.key() == payment_metadata.owner @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"payment_metadata", payment_metadata.owner.as_ref(), payment_config.key().as_ref()],
        bump,
        close = payer
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    #[account(
        mut,
        seeds = [b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref()],
        bump,
    )]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(
        mut,
        seeds = [b"payment_config", payment_config.key().as_ref(), merchant_authority.key().as_ref()],
        bump,
    )]
    pub payment_config: Account<'info, PaymentConfig>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_singer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

// impl<'info> ClosePaymentMetadata<'info> {
//     fn accounts(ctx: &Context<ClosePaymentMetadata>) -> ProgramResult {
//         let payment_metadata = &ctx.accounts.payment_metadata;
//         let current_owner = payment_metadata.owner;

//         let merchant_authority = &ctx.accounts.merchant_authority;
//         let current_authority = merchant_authority.current_authority;

//         let signer_is_owner = ctx.accounts.payer.key() == current_owner;
//         let signer_is_merchant = ctx.accounts.payer.key() == current_authority;

//         require!(
//             signer_is_owner || signer_is_merchant,
//             ErrorCode::IncorrectAuthority
//         );

//         Ok(())
//     }
// }

// #[access_control(ClosePaymentMetadata::accounts(&ctx))]
pub fn handler(ctx: Context<ClosePaymentMetadata>) -> ProgramResult {
    let ix = revoke(
        &ctx.accounts.token_program.key(),
        source_pubkey,
        owner_pubkey,
        signer_pubkeys,
    );

    Ok(())
}
