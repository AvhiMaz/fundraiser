#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;

declare_id!("D9Du7gCDii6fUCBiuDqrcMLKQJR482BeTrMNPP7W936M");

#[program]
pub mod fundraiser_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Init>, amount: u64, duration: u8) -> Result<()> {
        ctx.accounts.initialize(amount, duration, ctx.bumps)?;
        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        ctx.accounts.contribute(amount)?;
        Ok(())
    }
}
