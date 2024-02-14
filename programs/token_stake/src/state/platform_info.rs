use anchor_lang::prelude::*;

#[account]
pub struct PlatformInfo {
    pub token_mint: Pubkey, // 32
    pub authority: Pubkey,  // 32
    // Rate at which we drain the pool daily
    pub drain_rate: f64, // 8
}

pub const PLATFORM_INFO_SIZE: usize = 32 + 32 + 8;
pub const PLATFORM_INFO_SEED: &str = "platform_info";
