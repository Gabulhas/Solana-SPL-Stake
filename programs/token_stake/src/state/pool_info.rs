use anchor_lang::prelude::*;

#[account]
pub struct PoolInfo {
    pub last_refill_block: u64, // 8
}

pub const POOL_INFO_SIZE: usize = 8;
pub const POOL_INFO_SEED: &str = "pool_info";
