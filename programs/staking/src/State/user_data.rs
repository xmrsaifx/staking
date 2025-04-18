use anchor_lang::prelaude::*;

#[account]
#[derive(InitSpace)]

pub struct user_data {
    pub user : Pubkey();
    pub timestamp : u64;
    pub amount : u64;
}
