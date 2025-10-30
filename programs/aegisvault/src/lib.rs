#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
pub use instructions::*;

declare_id!("AYMZZjv95ipSXKcnuENKRdJnmGRgRmcNvADpaeu9ujzY");

#[program]
pub mod aegisvault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        init_vault_handler(ctx)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        init_user_handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit_handler(ctx, amount)
    }
}
