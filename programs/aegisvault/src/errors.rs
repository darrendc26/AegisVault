use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Math Overflow")]
    MathOverflow,

    #[msg("Invalid User")]
    InvalidUser,

    #[msg("Invalid Vault")]
    InvalidVault,

    #[msg("Invalid Collateral")]
    InvalidCollateral,

    #[msg("Invalid Asset")]
    InvalidAsset,
}
