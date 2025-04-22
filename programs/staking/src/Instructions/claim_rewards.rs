use crate::errors::CustomError;
use crate::{constants::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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
        seeds = [USER_DATA_SEED, user.key().as_ref()],
        bump
    }]
    pub user_data: Account<'info, UserData>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn claim_reward_handler(ctx: Context<ClaimReward>) -> Result<()> {
    let user_data = &mut ctx.accounts.user_data;
    let clock = Clock::get()?;

    require!(
        user_data.user == ctx.accounts.user.key(),
        CustomError::NotAuthorized
    );
    require!(user_data.stake_amount > 0, CustomError::NoAmountStaked);

    let reward_amount = (clock.unix_timestamp - user_data.timestamp) / 60 * 60; // 1 hour 1 token staking logic

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.reward_token_vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    user_data.timestamp = clock.unix_timestamp;
    let _ = transfer(cpi_ctx, reward_amount.try_into().unwrap()); //you can convert an `i64` to a `u64` and panic if the converted value doesn't fit
    Ok(())
}
