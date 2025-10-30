#![allow(unused)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user: Pubkey,        // user's wallet
    pub total_deposits: u64, // total WSOL deposited
    pub total_locked: u64,   // total WSOL locked
    pub total_borrowed: u64, // total USDC borrowed
    pub last_updated: i64,   // last time the user's position was updated
    pub bump: u8,
}
