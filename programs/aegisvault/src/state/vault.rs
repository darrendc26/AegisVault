#![allow(unused)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub asset_mint: Pubkey,           // WSOL
    pub collateral_mint: Pubkey,      // USDC
    pub collateralization_ratio: u64, // Collateralization ratio (e.g., 15000 = 150%)
    pub total_wsol_deposits: u64,     // total WSOL supplied
    pub total_wsol_borrowed: u64,     // total USDC borrowed
    pub total_wsol_collateral: u64,   // total WSOL as collateral
    pub total_usdc_deposits: u64,     // total USDC supplied
    pub total_usdc_borrowed: u64,     // total USDC borrowed
    pub total_usdc_collateral: u64,   // total USDC as collateral

    pub interest_rate: u64, // current borrow rate (basis points)

    pub bump: u8,
}
