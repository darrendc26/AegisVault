#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

mod instructions;
mod state;

declare_id!("AYMZZjv95ipSXKcnuENKRdJnmGRgRmcNvADpaeu9ujzY");

#[program]
pub mod aegisvault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
