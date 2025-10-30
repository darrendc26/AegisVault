use crate::state::user::User;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_user_handler(ctx: Context<InitializeUser>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.user = ctx.accounts.user.key();
    user_account.total_wsol_deposits = 0;
    user_account.total_wsol_locked = 0;
    user_account.total_wsol_borrowed = 0;

    user_account.total_usdc_deposits = 0;
    user_account.total_usdc_locked = 0;
    user_account.total_usdc_borrowed = 0;

    user_account.last_updated = Clock::get()?.unix_timestamp;
    user_account.bump = ctx.bumps.user_account;
    Ok(())
}
