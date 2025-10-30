use crate::state::vault::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault".as_ref(), asset_mint.key().as_ref(), collateral_mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub asset_mint: Account<'info, Mint>,
    pub collateral_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn init_vault_handler(ctx: Context<InitializeVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.asset_mint = ctx.accounts.asset_mint.key();
    vault.collateral_mint = ctx.accounts.collateral_mint.key();
    vault.total_deposits = 0;
    vault.total_borrowed = 0;
    vault.total_collateral = 0;
    vault.interest_rate = 750;
    vault.bump = ctx.bumps.vault;
    Ok(())
}
