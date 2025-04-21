use crate::{constants::*, state::*};
use anchor_lang::prelaude::*;

#[derive(Accounts)]
#[instructions(amount: u64)]
pub struct unstake_token<'info> {}
