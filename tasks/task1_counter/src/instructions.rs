extern crate borsh;
extern crate solana_program;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

impl UpdateArgs {
    pub fn try_from_slice(data: &[u8]) -> Result<Self, ProgramError> {
        Self::deserialize(&mut &*data).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let value = u32::from_le_bytes(rest.try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
                Self::Increment(value)
            },
            1 => {
                let value = u32::from_le_bytes(rest.try_into().map_err(|_| ProgramError::InvalidInstructionData)?);
                Self::Decrement(value)
            },
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
