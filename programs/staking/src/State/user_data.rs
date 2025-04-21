use anchor_lang::prelaude::*;

#[account]
#[derive(InitSpace)]

pub struct UserData {
    pub user: Pubkey,
    pub timestamp: u64,
    pub stake_amount: u64,
}
