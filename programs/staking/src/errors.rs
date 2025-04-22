use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Not Authorized")]
    NotAuthorized,

    #[msg("No Amount Staked")]
    NoAmountStaked,

    #[msg("Invalid Amount")]
    InvalidAmount,
}
