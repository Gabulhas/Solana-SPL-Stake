use crate::state::*;
use crate::utils::initialize_mint;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

/*
   Figure out something for the initial fill/bootstrapping of the pool
   maybe the funder gets a single token (meaning that it has the full pool)
*/
pub fn handler(ctx: Context<Initialize>, initial_funding: u64) -> Result<()> {
    let current_block = Clock::get().unwrap().slot;

    let authority = &mut ctx.accounts.authority;
    let mint = &mut ctx.accounts.mint;
    let platform_info = &mut ctx.accounts.platform;
    let pool_info = &mut ctx.accounts.pool;
    let vault_info = &mut ctx.accounts.vault;

    platform_info.authority = authority.key();
    platform_info.token_mint = mint.key();

    pool_info.last_refill_block = current_block;

    vault_info.initial_fill = initial_funding;

    let cpi_accounts = Transfer {
        from: ctx.accounts.authority_ata.to_account_info(),
        to: ctx.accounts.vault_ata.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, initial_funding)?;

    initialize_mint(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.x_mint.to_account_info(),
        &ctx.accounts.rent,
        &vault_info.to_account_info(),
        ctx.accounts.mint.decimals,
    )
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        token::mint = mint,
        token::authority = authority,
    )]
    pub authority_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      init,
      space= 8 + POOL_INFO_SIZE,
      seeds= [   POOL_INFO_SEED.as_bytes()],
      bump,
      payer= authority
    )]
    pub pool: Account<'info, PoolInfo>,
    #[account(
        init_if_needed,
        payer = authority,
        token::mint = mint,
        token::authority = pool
    )]
    pub pool_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      init,
      space= 8 + VAULT_INFO_SIZE,
      seeds= [   VAULT_INFO_SEED.as_bytes()],
      bump,
      payer= authority
    )]
    pub vault: Account<'info, VaultInfo>,
    #[account(
        init_if_needed,
        payer = authority,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      init,
      space= 8 + PLATFORM_INFO_SIZE,
      seeds= [   PLATFORM_INFO_SEED.as_bytes()],
      bump,
      payer= authority
    )]
    pub platform: Account<'info, PlatformInfo>,

    #[account(init_if_needed,
              payer = authority,
              seeds = [mint.key().as_ref()],
              bump,
              mint::decimals = mint.decimals,
              mint::authority = vault_ata,
              mint::freeze_authority = vault_ata)]
    pub x_mint: InterfaceAccount<'info, Mint>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub rent: Sysvar<'info, Rent>,
}
