use anchor_lang::prelude::*;
use instructions::*;

mod errors;
mod instructions;
mod state;
mod utils;

declare_id!("CbhSxhkgUap1uERGMoMjzhGpiMmSZQBLTk4ZmfpEQyQe");

#[program]
pub mod token_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initialize_amount: u64) -> Result<()> {
        instructions::initialize::handler(ctx, initialize_amount)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, x_tokens_to_burn: u64) -> Result<()> {
        instructions::unstake::handler(ctx, x_tokens_to_burn)
    }

    pub fn crank_and_emit(ctx: Context<CrankAndEmit>) -> Result<()> {
        instructions::crank_and_emit::handler(ctx)
    }
}
