#![allow(clippy::needless_lifetimes)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::states::Fundraiser;

#[derive(Accounts)]

pub struct Init<'info> {
    // the who will init the fundraising
    #[account(mut)]
    pub admin: Signer<'info>,

    //which mint to collect
    pub which_mint: Account<'info, Mint>,

    // fundraiser account
    #[account(
        init,
        payer = admin,
        seeds = [b"fundraiser", admin.key().as_ref()],
        bump,
        space = 8 + Fundraiser::INIT_SPACE,
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    // token vault account owned by the fundraiser pda
    #[account(
        init,
        payer = admin,
        associated_token::mint = which_mint,
        associated_token::authority = fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    // required programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Init<'info> {
    pub fn initialize(&mut self, amount: u64, duration: u8, bumps: InitBumps) -> Result<()> {
        //init
        self.fundraiser.set_inner(Fundraiser {
            admin: self.admin.key(),
            which_mint: self.which_mint.key(),
            amount_of_mint: amount,
            current_amount: 0,
            time_started: Clock::get()?.unix_timestamp,
            end_time: duration,
            bump: bumps.fundraiser,
        });
        Ok(())
    }
}
