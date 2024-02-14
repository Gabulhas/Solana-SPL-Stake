use crate::state::*;
use crate::utils::crank;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

// Crank that boi
pub fn handler(ctx: Context<CrankAndEmit>) -> Result<()> {
    // This basically just adds more tokens to the pool and mints new xTokens to the user, no matter if the user has staked before or not
    let current_block = Clock::get().unwrap().slot;

    let token_program = &ctx.accounts.token_program;
    let pool = &mut ctx.accounts.pool;
    let pool_ata = &ctx.accounts.pool_ata;
    let x_mint = &mut ctx.accounts.x_mint;

    let previous_pool_size = pool_ata.amount;
    let last_crank = pool.last_refill_block;

    crank(
        token_program.to_account_info(),
        &ctx.accounts.vault,
        &ctx.accounts.vault_ata,
        pool,
        &ctx.accounts.pool_ata,
        &ctx.accounts.platform,
        current_block,
    )?;
    emit!(CurrentPoolInfo {
        previous_pool_size,
        cranked_pool_size: pool_ata.amount,
        last_refill: last_crank,
        current_x_minted: x_mint.supply,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CrankAndEmit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
      mut,
      seeds= [   POOL_INFO_SEED.as_bytes()],
      bump,
    )]
    pub pool: Account<'info, PoolInfo>,
    #[account(
        token::mint = mint,
        token::authority = pool
    )]
    pub pool_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      mut,
      seeds= [   VAULT_INFO_SEED.as_bytes()],
      bump,
    )]
    pub vault: Account<'info, VaultInfo>,
    #[account(
        token::mint = mint,
        token::authority = vault
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      seeds= [PLATFORM_INFO_SEED.as_bytes()],
      bump,
    )]
    pub platform: Account<'info, PlatformInfo>,

    #[account(
              seeds = [mint.key().as_ref()],
              bump,
              mint::decimals = mint.decimals,
              mint::authority = vault_ata,
              mint::freeze_authority = vault_ata)]
    pub x_mint: InterfaceAccount<'info, Mint>,

    //TODO: for this mint, and for every mint, it should always check if the mint.key is the one on PlatformInfo
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

#[event]
struct CurrentPoolInfo {
    previous_pool_size: u64,
    cranked_pool_size: u64,
    last_refill: u64,
    current_x_minted: u64,
}
