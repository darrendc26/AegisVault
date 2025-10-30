#![allow(unused)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub asset_mint: Pubkey,      // WSOL
    pub collateral_mint: Pubkey, // USDC
    pub total_deposits: u64,     // total WSOL supplied
    pub total_borrowed: u64,     // total USDC borrowed
    pub total_collateral: u64,   // total WSOL as collateral
    pub interest_rate: u64,      // current borrow rate (basis points)
    pub bump: u8,
}
