use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid delay format")]
    InvalidDelayFormat,
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
