use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use crate::StakePool;
use crate::UserData;
use crate::errors::CustomError;
use crate::constants::*;


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct StakeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKE_POOL_SEED],
        bump 
    )]
    pub stake_pool: Account<'info,StakePool>,

    #[account(
        mut,
        seeds = [STAKE_TOKEN_VAULT_SEED],
        bump  
    )]
    pub stake_token_vault: Account<'info,TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account <'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + UserData::INIT_SPACE,
        seeds = [USER_DATA_SEED, user.key().as_ref()],
        bump
    )]
    pub user_data: Account<'info,UserData>,

    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,System>,
    pub rent : Sysvar<'info,Rent>,

}


pub fn stake_token_handler(
    ctx: Context<StakeToken>,
    amount: u64,
) -> Result<()> {

    let user_data = &mut ctx.accounts.user_data;

    let clock = Clock::get()?;

    require!(amount> 0 , CustomError::InvalidAmount );

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer{
            from : ctx.accounts.user.to_account_info(),
            to : ctx.accounts.stake_token_vault.to_account_info(),
            authority : ctx.accounts.user.to_account_info(),
        },
    );

    let _ = transfer(cpi_ctx,amount);

    user_data.user = ctx.accounts.user.key();
    user_data.timestamp = clock.unix_timestamp;
    user_data.stake_amount = user_data.stake_amount + amount;

    Ok(())
}