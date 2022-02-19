use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid delay format")]
    InvalidDelayFormat,
    #[msg("Incorrect mint")]
    IncorrectMint,
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
