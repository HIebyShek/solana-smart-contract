use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::mem::size_of;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hey, this is my first smart contract (not so smart)");

    let account = accounts.get(0).ok_or(ProgramError::AccountDataTooSmall)?;
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        msg!("owner: {}\nprogram_id: {}", account.owner, program_id);
        return Err(ProgramError::IncorrectProgramId);
    }

    let number = u64::try_from_slice(instruction_data.get(..size_of::<u64>()).unwrap())?;
    account.serialize_data(&number).unwrap();

    Ok(())
}
