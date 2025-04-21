use crate::{constants::*, state::*};
use anchor_lang::prelaude::*;

#[derive(Accounts)]
#[instructions(amount: u64)]
pub struct StakeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKE_POOL_SEED],
        bump  = stake_pool.bump 
    )]
    pub stake_pool: Account<'info,StakePool>,

    #[account(
        mut,
        seeds = [stake_token_vault],
        bump  = stake_pool.bump 
    )]
    pub stake_token_vault: Account<'info,AccountToken>,

    #[account(mut)]
    pub user_token_account: Account <'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + UserData::INIT_SPACE,
        seeds = [USER_DATA_SEED],
        bump
    )]
    pub user_data: Account<'info,UserData>,

    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,System>,
    pub rent : Sysvar<'info,rent>,

}


pub fn stake_token_handler(
    ctx: Context(StakeToken),
    amount: u64,
) -> Result<()> {
    let stake_pool = &ctx.account.stake_pool;

    let user_data = &mut ctx.account.user_data;

    let clock = Clock::get()?;

    require!(amount> 0 , CustomError::InvalidAmount );

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer{
            from : ctx.accounts.user_data.to_account_info(),
            to : ctx.accounts.stake_vault.to_account_info(),
            authority : ctx.accounts.user.to_account_info(),
        },
    );

    transfer(cpi_ctx,amount);

    user_data.user = ctx.accounts.user.key();
    user_data.timestamp = clock.unix_timestamp;
    user_data.stake_amount = user_data.stake_amount + amount;

    Ok(())
}