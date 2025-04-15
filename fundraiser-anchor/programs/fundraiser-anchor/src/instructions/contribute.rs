#![allow(clippy::needless_lifetimes)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::states::{Contributor, Fundraiser};

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    pub which_mint: Account<'info, Mint>,

    #[account(
        mut,
        has_one= which_mint,
        seeds = [b"fundraiser", fundraiser.admin.as_ref()],
        bump = fundraiser.bump
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    #[account(
        init_if_needed,
        payer = contributor,
        seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
        space = 8 + Contributor::INIT_SPACE,
    )]
    pub contributor_acc: Account<'info, Contributor>,

    #[account(
        mut,
        associated_token::mint = which_mint,
        associated_token::authority = contributor,
    )]
    pub contributor_acc_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = fundraiser.which_mint,
        associated_token::authority = fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Contribute<'info> {
    pub fn contribute(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Transfer {
            from: self.contributor_acc_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.contributor.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
