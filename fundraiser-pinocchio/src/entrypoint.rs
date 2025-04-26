use crate::instructions::{self, FundraiserInstruction};

use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator_variant, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(discriminator_variant)? {
        FundraiserInstruction::Initialize => {
            instructions::initialize::process_initialize(program_id, accounts, instruction_data)
        }
        FundraiserInstruction::Contribute => {
            instructions::contribute::process_contribute(program_id, accounts, instruction_data)
        }
        FundraiserInstruction::Checks => {
            instructions::checker::process_checkers(program_id, accounts, instruction_data)
        }
        FundraiserInstruction::Refund => {
            instructions::refund::process_refund(program_id, accounts, instruction_data)
        }
    }
}
