use crate::{constants::*, state::*};
use crate::{errors::CustomError, state::StakePool};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct UnstakeToken<'info> {
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
        seeds = [STAKE_TOKEN_VAULT_SEED],
        bump
    )]
    pub stake_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED, user.key().as_ref()],
        bump
    )]
    pub user_data: Account<'info, UserData>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn unstake_token_handler(ctx: Context<UnstakeToken>) -> Result<()> {
    let user_data = &mut ctx.accounts.user_data;

    require!(user_data.stake_amount > 0, CustomError::InvalidAmount);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.stake_token_vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    let _ = transfer(cpi_ctx, user_data.stake_amount);

    user_data.user = ctx.accounts.user.key();
    user_data.timestamp = 0;
    user_data.stake_amount = 0;
    Ok(())
}
