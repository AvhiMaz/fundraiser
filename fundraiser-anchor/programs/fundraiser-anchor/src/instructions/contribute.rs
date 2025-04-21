use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS},
    errors::FundraiserError,
    states::{Contributor, Fundraiser},
};

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
        seeds = [b"fundraiser".as_ref(), fundraiser.admin.as_ref()],
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
        // Check if the amount to contribute meets the minimum amount required
        require!(
            amount > 1_u8.pow(self.which_mint.decimals as u32) as u64,
            FundraiserError::ContributionTooSmall
        );

        // Check if the amount to contribute is less than the maximum allowed contribution
        require!(
            amount
                <= (self.fundraiser.amount_of_mint * MAX_CONTRIBUTION_PERCENTAGE)
                    / PERCENTAGE_SCALER,
            FundraiserError::ContributionTooBig
        );

        // Check if the fundraising duration has been reached
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            self.fundraiser.end_time
                <= ((current_time - self.fundraiser.time_started) / SECONDS_TO_DAYS) as u8,
            FundraiserError::FundraiserEnded
        );

        // Check if the maximum contributions per contributor have been reached
        require!(
            (self.contributor_acc.amount
                <= (self.fundraiser.amount_of_mint * MAX_CONTRIBUTION_PERCENTAGE)
                    / PERCENTAGE_SCALER)
                && (self.contributor_acc.amount + amount
                    <= (self.fundraiser.amount_of_mint * MAX_CONTRIBUTION_PERCENTAGE)
                        / PERCENTAGE_SCALER),
            FundraiserError::MaximumContributionsReached
        );

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
