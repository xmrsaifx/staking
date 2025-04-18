use anchor_lang::prelaude::*

#[error_code]

pub enum Errors{
    ![msg("Not Authorized")]
    NotAuthorized

    ![msg("No Amount Staked")]
    NoAmountStaked
} 