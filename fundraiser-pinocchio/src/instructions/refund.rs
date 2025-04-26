#![allow(dead_code)]

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_token::{
    instructions::TransferChecked,
    state::{Mint, TokenAccount},
};

use crate::states::{load_acc_mut, Contribute, Fundraiser};

pub fn process_refund(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [contributor, maker, mint_to_raise, fundraiser, contribute_acc, contribute_ata, vault, _token_program, _system_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;

    assert_eq!(vault_acc.owner(), fundraiser.key());

    let fundraiser_state =
        unsafe { load_acc_mut::<Fundraiser>(fundraiser.borrow_mut_data_unchecked())? };

    let contribute_state =
        unsafe { load_acc_mut::<Contribute>(contribute_acc.borrow_mut_data_unchecked())? };

    let mint_state = Mint::from_account_info(mint_to_raise)?;

    let bump_seeds = [fundraiser_state.bump];
    let fundraiser_seed = [
        Seed::from(Fundraiser::SEED.as_bytes()),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump_seeds[..]),
    ];

    let fundraiser_signer_seed = Signer::from(&fundraiser_seed[..]);

    (TransferChecked {
        from: vault,
        to: contribute_ata,
        authority: fundraiser,
        mint: mint_to_raise,
        amount: contribute_state.amount,
        decimals: mint_state.decimals(),
    })
    .invoke_signed(&[fundraiser_signer_seed.clone()])?;

    unsafe {
        *contributor.borrow_mut_lamports_unchecked() +=
            *contribute_acc.borrow_mut_lamports_unchecked();
    }

    contribute_acc.close()?;

    Ok(())
}
