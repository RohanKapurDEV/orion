use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid delay format")]
    InvalidDelayFormat,
    #[msg("Incorrect mint")]
    IncorrectMint,
    #[msg("Incorrect authority")]
    IncorrectAuthority,
    #[msg("Incorrect init authority")]
    IncorrectInitAuthority,
    #[msg("Incorrect merchant authority")]
    IncorrectMerchantAuthority,
    #[msg("Incorrect authority for payment config")]
    IncorrectAuthorityForPaymentConfig,
    #[msg("Account holds insufficient balance for delegation")]
    InsufficientBalanceToDelegate,
    #[msg("Amount being delegated is lower than the specified minimum in the payment config")]
    AmountToDelegateIsSmallerThanMinimum,
    #[msg("Token account is not owned by the instruction signer")]
    TokenAccountNotOwnedBySigner,
    #[msg("Incorrect payment token account was supplied")]
    IncorrectPaymentTokenAccount,
    #[msg("Incorrect payment metadata owner was supplied")]
    IncorrectPaymentMetadataOwner,
    #[msg("Incorrect payment config account was supplied")]
    IncorrectPaymentConfigAccount,
    #[msg("Incorrect owner payment account was supplied")]
    IncorrectOwnerPaymentAccount,
    #[msg("Incorrect delegate for token account")]
    IncorrectDelegateForTokenAccount,
    #[msg("Incorrect collection authority")]
    IncorrectCollectionAuthority,
    #[msg("program_as_signer account not an authorized delegate for specified token account")]
    ProgramAsSignerNotAuthorized,
    #[msg(
        "Authority is attempting to make user pay earier than the contractual obligation dictates"
    )]
    PaymentUnauthorized,
    #[msg("Attempting to reinstate a payment metadata account that is not in failed state")]
    PaymentMetadataNotInFailedState,
}

#[macro_export]
macro_rules! print_error {
    ($err:expr) => {{
        || {
            let error_code: ErrorCode = $err;
            msg!("{:?} thrown at {}:{}", error_code, file!(), line!());
            $err
        }
    }};
}
