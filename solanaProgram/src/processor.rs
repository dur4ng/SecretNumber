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

    pub fn process_init_game_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let game_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(game_account.lamports(), game_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut game_info = GameScrow::unpack_unchecked(&game_account.data.borrow())?;
        if game_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        if game_info.is_waiting_player() {
            return Err(EscrowError::NotWaitingPlayers);
        }

        game_info.is_initialized = true;
        game_info.is_waiting_player = true;
        game_info.initializer_pubkey = initializer.key;
        game_info.initializer_temp_token_account_pubkey = temp_token_account.key;
        game_info.initializer_token_to_receive_account_pubkey = token_to_receive_account.key;
        game_info.expected_amount = amount;

        GameScrow::pack(game_info, &mut game_account.data.borrow_mut())?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"game"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key, 
            Some(&pda), 
            spl_token::instruction::AuthorityType::AccountOwner, 
            initializer.key, 
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer temp token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn process_join_game_escrow() {
        todo!();
    }

    pub fn process_try_guess_secret_number() {
        todo!();
    }
}