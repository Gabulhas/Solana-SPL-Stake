use anchor_lang::prelude::*;
#[error_code]
pub enum CustomError {
    #[msg("Seems like you tried to do something funny like DivisionByZero")]
    DivisionByZero,

    #[msg("CooldownWait")]
    CooldownWait,
}
