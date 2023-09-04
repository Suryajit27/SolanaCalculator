use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::Instruction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    pub result: u32,
}
fn add(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

fn subtract(num1: u32, num2: u32) -> u32 {
    num1 - num2
}
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Iterating accounts is safer than indexing
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        let instruction = Instruction::unpack(instruction_data)?;

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;

        match instruction {
            Instruction::Add { num1, num2 } => {
                calculator_account.result = add(num1, num2)
            }
            Instruction::Subtract { num1, num2 } => {
                calculator_account.result = subtract(num1, num2)
            }
        };

        calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

        msg!("Result is: {}", calculator_account.result);

        Ok(())
    }
}
