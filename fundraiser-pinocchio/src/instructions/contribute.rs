#![allow(unused_unsafe)]
#![allow(dead_code)]

use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{
    instructions::TransferChecked,
    state::{Mint, TokenAccount},
};

use crate::states::{load_acc_mut_unchecked, load_ix_data, Contribute, DataLen, Fundraiser};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContributeInstructionData {
    pub amount: u64,
    pub fundraiser_bump: u8,
    pub contributor_bump: u8,
}

impl DataLen for ContributeInstructionData {
    const LEN: usize = core::mem::size_of::<ContributeInstructionData>();
}

pub fn process_contribute(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [contributor, mint_to_raise, fundraiser, contributor_acc, contributor_ata, vault, _token_program, _system_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    {
        let vault_acc = TokenAccount::from_account_info(vault)?;
        assert_eq!(vault_acc.owner(), fundraiser.key());
        let contributor_ata_acc = TokenAccount::from_account_info(contributor_ata)?;
        assert_eq!(contributor_ata_acc.owner(), contributor.key());
    }

    let ix_data = unsafe { load_ix_data::<ContributeInstructionData>(instruction_data) }?;

    if contributor_acc.data_is_empty() || !contributor_acc.is_owned_by(&crate::ID) {
        let rent = Rent::get()?;
        let bump_seed = [ix_data.contributor_bump];
        let contributor_signer_seeds = [
            Seed::from(Contribute::SEED.as_bytes()),
            Seed::from(fundraiser.key().as_ref()),
            Seed::from(contributor.key().as_ref()),
            Seed::from(&bump_seed),
        ];

        let contribute_signer = Signer::from(&contributor_signer_seeds[..]);

        (CreateAccount {
            from: contributor,
            to: contributor_acc,
            lamports: rent.minimum_balance(Contribute::LEN),
            space: Contribute::LEN as u64,
            owner: &crate::ID,
        })
        .invoke_signed(&[contribute_signer])?;
    }

    let mint_state = Mint::from_account_info(mint_to_raise)?;
    let decimals = mint_state.decimals();

    let contributor_state = unsafe {
        load_acc_mut_unchecked::<Contribute>(contributor_acc.borrow_mut_data_unchecked())
    }?;
    let fundraiser_state =
        unsafe { load_acc_mut_unchecked::<Fundraiser>(fundraiser.borrow_mut_data_unchecked()) }?;

    contributor_state.initialize(ix_data.amount);

    (TransferChecked {
        amount: ix_data.amount,
        from: contributor_ata,
        to: vault,
        mint: mint_to_raise,
        authority: contributor,
        decimals,
    })
    .invoke()?;

    contributor_state.amount += ix_data.amount;
    fundraiser_state.current_amount += ix_data.amount;

    Ok(())
}
