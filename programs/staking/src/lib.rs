use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("EvoVJjZ3kcavJW7MSQNSPjP5siioeg3dqQKigezJkfde");

#[program]
pub mod staking {
    use super::*;

    pub fn create_stake_pool(ctx: Context<CreateStakePool>, reward_rate: u64) -> Result<()> {
        instructions::create_stake_pool_handler(ctx, reward_rate)
    }

    pub fn stake_token(ctx: Context<StakeToken>, amount: u64) -> Result<()> {
        instructions::stake_token_handler(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward_handler(ctx)
    }

    pub fn unstake_token(ctx: Context<UnstakeToken>) -> Result<()> {
        instructions::unstake_token_handler(ctx)
    }
}
