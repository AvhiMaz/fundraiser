use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{errors::FundraiserError, states::Fundraiser};

#[derive(Accounts)]

pub struct Done<'info> {
    // the one who started the fundraising
    #[account(mut)]
    pub admin: Signer<'info>,

    // which mint to collect
    pub which_mint: Account<'info, Mint>,

    // finding fundraiser pda
    #[account(
        mut,
        seeds = [b"fundraiser".as_ref(), admin.key().as_ref()],
        bump = fundraiser.bump,
        close = admin,
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    // vault account where the token is stored
    #[account(
        mut,
        associated_token::mint = which_mint,
        associated_token::authority = fundraiser,
    )]
    pub vault: Account<'info, TokenAccount>,

    // ata account of the admin
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = which_mint,
        associated_token::authority = admin,
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    //req program
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Done<'info> {
    pub fn done(&mut self) -> Result<()> {
        // check if target is met or not ?
        require!(
            self.vault.amount >= self.fundraiser.amount_of_mint,
            FundraiserError::TargetNotMet
        );

        // transfer from vault to admin's ata account
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.admin_ata.to_account_info(),
            authority: self.fundraiser.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"fundraiser".as_ref(),
            self.admin.to_account_info().key.as_ref(),
            &[self.fundraiser.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, &signer_seeds);

        transfer(cpi_ctx, self.vault.amount)?;

        Ok(())
    }
}
