use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    errors::ErrorCode,
    state::{user::User, vault::Vault},
};

#[derive(Accounts)]
pub struct BorrowSol<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), borrower.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), vault.asset_mint.as_ref(), vault.collateral_mint.as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        token::mint = vault.asset_mint,
        token::authority = borrower,
    )]
    pub borrower_wsol_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = vault.asset_mint,
        token::authority = vault,
    )]
    pub vault_wsol_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = vault.collateral_mint,
        token::authority = vault,
    )]
    pub vault_usdc_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = vault.collateral_mint,
        token::authority = borrower,
    )]
    pub borrower_usdc_token_account: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// User borrows SOL by locking USDC as collateral
// params: amount: u64 - amount of SOL to borrow in lamports
// User borrows WSOL by locking USDC as collateral
pub fn borrow_sol_handler(ctx: Context<BorrowSol>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    require!(amount > 0, ErrorCode::InvalidAmount);

    // Get SOL/USD price from Pyth
    let price_update = &ctx.accounts.price_update;
    let maximum_age: u64 = 60;
    let feed_id: [u8; 32] =
        get_feed_id_from_hex("0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d")?; // SOL/USD
    let sol_price_data = price_update.get_price_no_older_than(&clock, maximum_age, &feed_id)?;

    let sol_price_usd = (sol_price_data.price as u128)
        .checked_mul(10u128.pow(sol_price_data.exponent as u32))
        .ok_or(ErrorCode::MathOverflow)?;

    let wsol_value_in_usdc = (amount as u128)
        .checked_mul(sol_price_usd)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10u128.pow(9)) // Convert from lamports
        .ok_or(ErrorCode::MathOverflow)?;

    let collateralization_ratio = vault.collateralization_ratio; // e.g., 15000 = 150%
    let required_usdc_collateral = (wsol_value_in_usdc as u128)
        .checked_mul(collateralization_ratio as u128)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10000) // Ratio is in basis points
        .ok_or(ErrorCode::MathOverflow)? as u64;

    // 7. Check if user has enough USDC deposits to lock as collateral
    let available_usdc = user_account
        .total_usdc_deposits
        .checked_sub(user_account.total_usdc_locked)
        .ok_or(ErrorCode::InsufficientCollateral)?;

    require!(
        available_usdc >= required_usdc_collateral,
        ErrorCode::InsufficientCollateral
    );

    // 8. Check vault has enough WSOL liquidity
    require!(
        ctx.accounts.vault_wsol_token_account.amount >= amount,
        ErrorCode::InsufficientVaultLiquidity
    );

    // 9. Transfer WSOL from vault to borrower
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"vault".as_ref(),
        vault.asset_mint.as_ref(),
        vault.collateral_mint.as_ref(),
        &[vault.bump],
    ]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_wsol_token_account.to_account_info(),
        to: ctx.accounts.borrower_wsol_token_account.to_account_info(),
        authority: vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    // 10. Update user account - LOCK USDC, not WSOL!
    user_account.total_wsol_borrowed = user_account
        .total_wsol_borrowed
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    user_account.total_usdc_locked = user_account // ← Lock USDC!
        .total_usdc_locked
        .checked_add(required_usdc_collateral)
        .ok_or(ErrorCode::MathOverflow)?;

    user_account.last_updated = clock.unix_timestamp;

    // 11. Update vault
    vault.total_wsol_borrowed = vault
        .total_wsol_borrowed
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    Ok(())
}
