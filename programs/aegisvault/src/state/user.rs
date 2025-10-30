#![allow(unused)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user: Pubkey,

    // WSOL tracking
    pub total_wsol_deposits: u64, // total WSOL deposited
    pub total_wsol_locked: u64,   // WSOL locked as collateral
    pub total_wsol_borrowed: u64, // WSOL debt

    // USDC tracking
    pub total_usdc_deposits: u64, // total USDC deposited
    pub total_usdc_locked: u64,   // USDC locked as collateral
    pub total_usdc_borrowed: u64, // USDC debt

    pub last_updated: i64, // last time the user's position was updated
    pub bump: u8,
}
