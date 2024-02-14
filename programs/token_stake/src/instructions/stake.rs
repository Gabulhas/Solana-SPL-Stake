use crate::state::*;
use crate::utils::{calculate_x_tokens_to_mint, crank, mint_x_tokens};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    // This basically just adds more tokens to the pool and mints new xTokens to the user, no matter if the user has staked before or not
    let current_block = Clock::get().unwrap().slot;

    let token_program = &ctx.accounts.token_program;
    let pool = &mut ctx.accounts.pool;
    let pool_ata = &ctx.accounts.pool_ata;
    let x_mint = &mut ctx.accounts.x_mint;

    crank(
        token_program.to_account_info(),
        &ctx.accounts.vault,
        &ctx.accounts.vault_ata,
        pool,
        &ctx.accounts.pool_ata,
        &ctx.accounts.platform,
        current_block,
    )?;

    // Transfer the user's tokens to the pool
    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.user_ata.to_account_info(),
        to: ctx.accounts.pool_ata.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let transfer_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_cpi_accounts,
    );
    transfer(transfer_cpi_ctx, amount)?;

    // Calculate the amount of xTokens to mint to the user
    let x_tokens_to_mint = calculate_x_tokens_to_mint(amount, pool_ata.amount, x_mint.supply);

    // Mint xTokens to the user
    mint_x_tokens(
        token_program.to_account_info(),
        x_mint.to_account_info(),
        &ctx.accounts.user_ata.to_account_info(),
        &ctx.accounts.vault.to_account_info(),
        x_tokens_to_mint,
    )
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::mint = platform.token_mint,
        token::authority = user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      init_if_needed,
      space= 8 + USER_INFO_SIZE,
      seeds= [   USER_INFO_SEED.as_bytes()],
      bump,
      payer= user
      )]
    pub user_info: Account<'info, UserInfo>,

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
