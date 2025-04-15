#![allow(clippy::needless_lifetimes)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::states::{Contributor, Fundraiser};

#[derive(Accounts)]
pub struct Contribute<'info> {
    // contributor
    #[account(mut)]
    pub contributor: Signer<'info>,

    // token needed for fundraiser
    pub which_mint: Account<'info, Mint>,

    // fundraiser pda
    #[account(
        mut,
        has_one= which_mint,
        seeds = [b"fundraiser", fundraiser.admin.as_ref()],
        bump = fundraiser.bump
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    // contributor_acc
    #[account(
        init_if_needed,
        payer = contributor,
        seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
        space = 8 + Contributor::INIT_SPACE,
    )]
    pub contributor_acc: Account<'info, Contributor>,

    //contributor_acc_ata which holds the token
    #[account(
        mut,
        associated_token::mint = which_mint,
        associated_token::authority = contributor,
    )]
    pub contributor_acc_ata: Account<'info, TokenAccount>,

    // vault which hold token of fundraiser
    #[account(
        mut,
        associated_token::mint = fundraiser.which_mint,
        associated_token::authority = fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    // req programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Contribute<'info> {
    pub fn contribute(&mut self, amount: u64) -> Result<()> {
        // transfer logic
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Transfer {
            from: self.contributor_acc_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.contributor.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        transfer(cpi_ctx, amount)?;

        self.fundraiser.current_amount += amount;
        self.contributor_acc.amount += amount;

        Ok(())
    }
}
