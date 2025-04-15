#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod states;

declare_id!("D9Du7gCDii6fUCBiuDqrcMLKQJR482BeTrMNPP7W936M");

#[program]
pub mod fundraiser_anchor {
    use super::*;
}
