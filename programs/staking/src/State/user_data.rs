use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct UserData {
    pub user: Pubkey,
    pub timestamp: i64,
    pub stake_amount: u64,
}
