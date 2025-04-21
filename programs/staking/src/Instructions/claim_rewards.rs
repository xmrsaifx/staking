use crate::{constants::*, state::*};
use anchor_lang::prelaude::*;

#[derive(Accounts)]
#[instructions()]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer(),

    #[account(
        mut,
        seeds = [STAKE_POOL_SEED],
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        mut,
        seeds = [REWARD_TOKEN_VAULT_SEED],
        bump
    )]
    pub reward_token_vault: Account<'info, TokenAccount>,

    #[account{
        mut,
        seeds = [USER_DATA_SEED],
        bump
    }]
    pub user_data: Account<'info, UserData>,

    pub token_program: Program<'info, Account>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, rent>,
}

pub fn claim_reward_handler(ctx: Context<ClaimReward>) -> Result<()> {
    let stake_pool = ctx.accounts.stake_pool;
    let user_data = ctx.accounts.user_data;

    require!(user_data.user == ctx.accounts.user_data.key())
}
