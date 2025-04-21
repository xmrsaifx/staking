use anchor_lang::prelaude::*;

#[account]
#[derive(InitSpace)]

pub struct StakePool {
    pub admin: Pubkey,
    pub stake_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub stake_vault: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_rate: u64,
}
