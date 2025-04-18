use anchor_lang::prelude::*;

declare_id!("EvoVJjZ3kcavJW7MSQNSPjP5siioeg3dqQKigezJkfde");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
