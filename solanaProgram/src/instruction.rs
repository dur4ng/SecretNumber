use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowGameInstruction {

    InitGameEscrow {
        amount: u64,
    },
    JoinGameEscrow {
        amount: u64,
    },
    TryGuessSecretNumber {
        number: u64,
    },
}

impl EscrowGameInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitGameEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::JoinGameEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}