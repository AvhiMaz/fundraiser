use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_token::state::TokenAccount;

pub fn process_initialize(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, vault, _system_program, _token_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !maker.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !fundraiser.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;

    assert_eq!(vault_acc.owner(), fundraiser.key());

    Ok(())
}
