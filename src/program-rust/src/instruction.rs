use solana_program::program_error::ProgramError;
use std::convert::TryInto;

pub enum Instruction {
    Add { num1: u32, num2: u32 },
    Subtract { num1: u32, num2: u32 },
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        if rest.len() != 8 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(match tag {
            0 => Self::Add {
                num1: Self::unpack_num1(rest)?,
                num2: Self::unpack_num2(rest)?,
            },
            1 => Self::Subtract {
                num1: Self::unpack_num1(rest)?,
                num2: Self::unpack_num2(rest)?,
            },

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_num1(input: &[u8]) -> Result<u32, ProgramError> {
        let num1 = input
            .get(..4)
            .and_then(|slice| slice.try_into().ok())
            .map(u32::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(num1)
    }
    fn unpack_num2(input: &[u8]) -> Result<u32, ProgramError> {
        let num1 = input
            .get(4..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u32::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(num1)
    }
}
