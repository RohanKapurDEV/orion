use crate::error::*;
use anchor_lang::prelude::*;

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
    pub spacing_period: i64, // seconds in between payment collections
    pub delay_format: u8, // delay format for payment schedule (start of next minute, hour, day, week, etc...)
    pub bump: u8,
}

// impl PaymentConfig {
//     fn decode_delay_format(self) -> Result<DecodedDelayFormat, ProgramError> {
//         match self.delay_format {
//             0 => Ok(DecodedDelayFormat::Minute),
//             1 => Ok(DecodedDelayFormat::Hour),
//             2 => Ok(DecodedDelayFormat::Day),
//             3 => Ok(DecodedDelayFormat::Week),
//             4 => Ok(DecodedDelayFormat::Month),
//             5 => Ok(DecodedDelayFormat::Year),
//             6..=u8::MAX => Err(ErrorCode::InvalidDelayFormat.into()),
//         }
//     }
// }

pub enum DecodedDelayFormat {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}
