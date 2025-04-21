use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::SECONDS_TO_DAYS,
    errors::FundraiserError,
    states::{Contributor, Fundraiser},
};

#[derive(Accounts)]
pub struct Refund<'info> {
    // the contributor
    #[account(mut)]
    pub contributor: Signer<'info>,

    // who started the fundraiser
    pub admin: SystemAccount<'info>,

    // which token the admin is collecting
    pub which_mint: Account<'info, Mint>,

    // finding fundraiser pda
    #[account(
        mut,
        has_one = which_mint,
        seeds = [b"fundraiser", admin.key().as_ref()],
        bump = fundraiser.bump
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    //finding contributor account
    #[account(
        mut,
        close = contributor,
        seeds = [b"contributor", fundraiser.key().as_ref(), contributor.key().as_ref()],
        bump,
    )]
    pub contributor_acc: Account<'info, Contributor>,

    // ata of contributor's account
    #[account(
        mut,
        associated_token::mint = which_mint,
        associated_token::authority = contributor
    )]
    pub contributor_acc_ata: Account<'info, TokenAccount>,

    //vault account where token are stored
    #[account(
        mut,
        associated_token::mint = which_mint,
        associated_token::authority = fundraiser
    )]
    pub vault: Account<'info, TokenAccount>,

    // req program
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            self.fundraiser.end_time
                >= ((current_time - self.fundraiser.time_started) / SECONDS_TO_DAYS) as u8,
            FundraiserError::FundraiserNotEnded
        );

        require!(
            self.vault.amount < self.fundraiser.amount_of_mint,
            FundraiserError::TargetMet
        );

        //transfer
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.contributor_acc_ata.to_account_info(),
            authority: self.fundraiser.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"fundraiser".as_ref(),
            self.admin.to_account_info().key.as_ref(),
            &[self.fundraiser.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        transfer(cpi_ctx, self.contributor_acc.amount)?;

        //update current amount
        self.fundraiser.current_amount -= self.contributor_acc.amount;

        Ok(())
    }
}
