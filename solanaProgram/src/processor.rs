use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::Account as TokenAccount;

use crate::{error::EscrowError, instruction::EscrowGameInstruction, state::GameScrow};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowGameInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowGameInstruction::InitGameEscrow { amount } => {
                msg!("Instruction: InitGameEscrow");
                Self::process_init_game_escrow(accounts, amount, program_id)
            }
            EscrowGameInstruction::JoinGameEscrow { amount } => {
                msg!("Instruction: JoinGameEscrow");
                Self::process_join_game_escrow(accounts, amount, program_id)
            }
            EscrowGameInstruction::TryGuessSecretNumber { number } => {
                msg!("Instruction: TryGuessSecretNumber");
                Self::process_try_guess_secret_number(accounts, number, program_id)
            }
        }
    }

    pub fn process_init_game_escrow() {
        todo!();
    }

    pub fn process_join_game_escrow() {
        todo!();
    }

    pub fn process_try_guess_secret_number() {
        todo!();
    }
}