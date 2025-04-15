use anchor_lang::prelude::*;

declare_id!("D9Du7gCDii6fUCBiuDqrcMLKQJR482BeTrMNPP7W936M");

#[program]
pub mod fundraiser_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
