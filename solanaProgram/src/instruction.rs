use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::*;
use core::intrinsics::size_of;

pub enum EscrowGameInstruction {
    /// Starts the game creating a game account and transferring ownership of the given token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The main account(wallet) of the person initializing the game.
    /// 1. `[writable]` The token account of the person initializing the game.
    /// 2. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer.
    /// 3. `[writable]` The escrowGame account, it will hold all necessary info about the game.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitGameEscrow {
        /// The amount party A expects to receive of token Y.
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
    pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitGameEscrow { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            _ => { return Err(InvalidInstruction.into())}
        };
        Ok(buf)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_packing() {
        // check
        let check = EscrowGameInstruction::InitGameEscrow { amount: 1 };
        // packed
        let packed = check.pack().unwrap();
        let mut expect = Vec::from([0u8, 1]);
        assert_eq!(packed, expect);
        let unpacked = EscrowGameInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
        // unpacked
    }
}