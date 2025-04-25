#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
#[cfg(feature = "std")]
extern crate std;

mod instructions;
mod states;

pinocchio_pubkey::declare_id!("CFMyDXxFozMqDnpgefi9iuKkzxydRWbWumiDxSFwwUxz");
