use anchor_lang::prelude::*;

#[account]
pub struct VaultInfo {
    pub initial_fill: u64, //8
}

pub const VAULT_INFO_SIZE: usize = 8;
pub const VAULT_INFO_SEED: &str = "vault_info";
