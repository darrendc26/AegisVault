// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Token, TokenAccount, Transfer};
// use crate::errors::ErrorCode;
// use crate::state::{user::User, vault::Vault};

// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//         mut,
//         seeds = [b"user".as_ref(), user.key().as_ref()],
//         bump = user_account.bump,
//         has_one = user @ ErrorCode::InvalidUser
//     )]
//     pub user_account: Account<'info, User>,

//     #[account(
//         mut,
//         seeds = [b"vault".as_ref(), vault.asset_mint.as_ref(), vault.collateral_mint.as_ref()],
//         bump = vault.bump,
//     )]
//     pub vault: Account<'info, Vault>,

//     #[account(
//         mut,
//         token::mint = vault.asset_mint,
//         token::authority = vault,
//     )]
//     pub vault_wsol_account: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         token::mint = vault.asset_mint,
//         token::authority = user,
//     )]
//     pub user_wsol_account: Account<'info, TokenAccount>,

//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

// pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let vault = &mut ctx.accounts.vault;

//     // Transfer WSOL from vault to user
//     let cpi_accounts = Transfer {
