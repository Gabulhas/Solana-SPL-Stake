use anchor_lang::prelude::*;

#[account]
pub struct UserInfo {
    pub last_stake_time: u64,
}

pub const USER_INFO_SIZE: usize = 8;
pub const USER_INFO_SEED: &str = "platform_info";
