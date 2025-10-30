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

    pub fn deposit_wsol(ctx: Context<DepositWsol>, amount: u64) -> Result<()> {
        deposit_wsol_handler(ctx, amount)
    }
    pub fn deposit_usdc(ctx: Context<DepositUsdc>, amount: u64) -> Result<()> {
        deposit_usdc_handler(ctx, amount)
    }

    pub fn withdraw_wsol(ctx: Context<WithdrawWsol>, amount: u64) -> Result<()> {
        withdraw_wsol_handler(ctx, amount)
    }

    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount: u64) -> Result<()> {
        withdraw_usdc_handler(ctx, amount)
    }
}
