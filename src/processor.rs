use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::mem::size_of;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccountWithNumber(u64);

impl AccountWithNumber {
    pub fn new(num: u64) -> Self {
        Self { 0: num }
    }

    pub fn store(&mut self, num: u64) {
        self.0 = num;
    }

    pub fn read(&self) -> u64 {
        self.0
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hey, this is my first smart contract (not so smart)");

    let account = accounts.get(0).ok_or(ProgramError::AccountDataTooSmall)?;
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut acc_with_num = AccountWithNumber::try_from_slice(*account.try_borrow_data()?)?;
    let number = u64::try_from_slice(instruction_data.get(..size_of::<u64>()).unwrap())?;

    acc_with_num.store(number);

    Ok(())
}
