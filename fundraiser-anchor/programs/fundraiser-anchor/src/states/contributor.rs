use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
    pub amount: u64, // amount a contributor contributed
}
