#![allow(dead_code)]

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_token::{
    instructions::{CloseAccount, TransferChecked},
    state::Mint,
};

use crate::states::{load_acc_mut, Fundraiser};

pub fn process_checkers(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [maker, maker_ata, mint_to_raise, fundraiser, vault, _token_program, _system_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let fundraiser_state =
        unsafe { load_acc_mut::<Fundraiser>(fundraiser.borrow_mut_data_unchecked())? };

    let mint_state = Mint::from_account_info(mint_to_raise)?;

    let bump_seed = [fundraiser_state.bump];
    let fundraiser_seeds = [
        Seed::from(Fundraiser::SEED.as_bytes()),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump_seed[..]),
    ];

    let fundraiser_signer_seed = Signer::from(&fundraiser_seeds[..]);

    (TransferChecked {
        from: vault,
        to: maker_ata,
        amount: fundraiser_state.current_amount,
        mint: mint_to_raise,
        decimals: mint_state.decimals(),
        authority: fundraiser,
    })
    .invoke_signed(&[fundraiser_signer_seed.clone()])?;

    (CloseAccount {
        account: vault,
        destination: maker,
        authority: fundraiser,
    })
    .invoke_signed(&[fundraiser_signer_seed.clone()])?;

    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *fundraiser.borrow_lamports_unchecked();
    }

    fundraiser.close()?;

    Ok(())
}
