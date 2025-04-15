use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Contributor {
    pub amount: u64, // amount a contributor contributed
}
