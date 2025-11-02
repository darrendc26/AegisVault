use crate::errors::ErrorCode;
use crate::state::{user::User, vault::Vault};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawWsol<'info> {
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
        token::mint = vault.asset_mint,
        token::authority = vault,
    )]
    pub vault_wsol_account: Account<'info, TokenAccount>, // Use Anchor's Account wrapper

    #[account(
        mut,
        token::mint = vault.asset_mint,
        token::authority = user,
    )]
    pub user_wsol_account: Account<'info, TokenAccount>, // Use Anchor's Account wrapper

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_wsol_handler(ctx: Context<WithdrawWsol>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let vault = &mut ctx.accounts.vault;

    // Check that user has enough WSOL deposited
    require!(
        amount <= user_account.total_wsol_deposits,
        ErrorCode::InvalidAmount
    );

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"vault".as_ref(),
        &vault.asset_mint.to_bytes(),
        &vault.collateral_mint.to_bytes(),
        &[vault.bump],
    ]];

    // Transfer WSOL from vault to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_wsol_account.to_account_info(),
        to: ctx.accounts.user_wsol_account.to_account_info(),
        authority: vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    transfer(cpi_ctx, amount)?;

    // Update user data
    user_account.total_wsol_deposits = user_account
        .total_wsol_deposits
        .checked_sub(amount)
        .ok_or(ErrorCode::MathOverflow)?;
    user_account.last_updated = Clock::get()?.unix_timestamp;

    // Update vault data
    vault.total_wsol_deposits = vault
        .total_wsol_deposits
        .checked_sub(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // Emit event
    emit!(WithdrawWsolEvent {
        user: ctx.accounts.user.key(),
        vault: ctx.accounts.vault.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct WithdrawWsolEvent {
    pub user: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
