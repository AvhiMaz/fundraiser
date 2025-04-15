use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Fundraiser {
    pub admin: Pubkey,       // the one who will start the fundraising
    pub which_mint: Pubkey,  // which mint to rise
    pub amount_of_mint: u64, // how much to collect
    pub current_amount: u64, // current total amount collected
    pub time_started: i64,   // when it started
    pub end_time: u8,        // when it will end
    pub bump: u8,            // the bump
}
