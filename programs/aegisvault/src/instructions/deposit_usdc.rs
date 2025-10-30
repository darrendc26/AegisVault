use crate::errors::ErrorCode;
use crate::state::{user::User, vault::Vault};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositUsdc<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
        has_one = user @ ErrorCode::InvalidUser
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
        token::mint = vault.collateral_mint,
        token::authority = user,
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = vault.collateral_mint,
        token::authority = vault,
    )]
    pub vault_usdc_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn deposit_usdc_handler(ctx: Context<DepositUsdc>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let vault = &mut ctx.accounts.vault;

    // Transfer WSOL from user to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_usdc_account.to_account_info(),
        to: ctx.accounts.vault_usdc_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update user data
    user_account.total_usdc_deposits = user_account
        .total_usdc_deposits
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    user_account.last_updated = Clock::get()?.unix_timestamp;

    // Update vault data
    vault.total_usdc_deposits = vault
        .total_usdc_deposits
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // Emit event
    emit!(DepositUsdcEvent {
        user: ctx.accounts.user.key(),
        vault: ctx.accounts.vault.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DepositUsdcEvent {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
