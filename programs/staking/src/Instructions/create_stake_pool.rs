use crate::{constants::*, state::*};
use anchor_lang::prelaude::*;

#[derive(Accounts)]
pub struct CreateStakePool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + StakePool::INIT_SPACE,
        seeds = [STAKE_POOL_SEED]
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,

    pub stake_mint: Account<'info, TokenAccount>,
    pub reward_mint: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        token::authority = stake_pool,
        token::mint = stake_mint, 
        seeds = [STAKE_TOKEN_VAULT_SEED],
        bump,
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        token::authority = stake_pool,
        token::mint = reward_mint, 
        seeds = [REWARD_TOKEN_VAULT_SEED],
        bump,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info , System>,
    pub rent: Sysvar<'info,Rent>,

}


pub fn create_stake_pool_handler(
  ctx:Context<CreateStakePool>,
  reward_rate: u64,
) -> Result<()>
{
    let stake_pool = &ctx.accounts.stake_pool;

    stake_pool.admin = ctx.accounts.admin.key();
    stake_pool.stake_mint = ctx.accounts.stake_mint.key();
    stake_pool.reward_mint = ctx.accounts.reward_mint.key();
    stake_pool.stake_vault = ctx.accounts.stake_vault.key();
    stake_pool.reward_vault = ctx.accounts.reward_vault.key();
    stake_pool.reward_rate = reward_rate;

    Ok(())
}