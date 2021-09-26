use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct GameScrow {
    pub is_initialized: bool,
    pub is_waiting_player: bool,
    pub initializer_pubkey: Pubkey,
    pub initializer_temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub joiner_pubkey: Pubkey,
    pub joiner_temp_token_account_pubkey: Pubkey,
    pub joiner_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
    pub secret_num: u64,
}
impl GameScrow {
    pub(crate) fn is_waiting_player(&self) -> bool {
        self.is_waiting_player
    }
}
impl Sealed for GameScrow {}

impl IsInitialized for GameScrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GameScrow {
    const LEN: usize = 210;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            is_waiting_player,
            initializer_pubkey,
            initializer_temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            joiner_pubkey,
            joiner_temp_token_account_pubkey,
            joiner_token_to_receive_account_pubkey,
            expected_amount,
            secret_num
        ) = array_refs![src, 1, 1, 32, 32, 32, 32, 32, 32, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        let is_waiting_player = match is_waiting_player {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(GameScrow {
            is_initialized,
            is_waiting_player,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            initializer_temp_token_account_pubkey: Pubkey::new_from_array(
                *initializer_temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initializer_token_to_receive_account_pubkey,
            ),
            joiner_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            joiner_temp_token_account_pubkey: Pubkey::new_from_array(*joiner_temp_token_account_pubkey),
            joiner_token_to_receive_account_pubkey: Pubkey::new_from_array(*joiner_token_to_receive_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
            secret_num: u64::from_le_bytes(*secret_num),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GameScrow::LEN];
        let (
            is_initialized_dst,
            is_waiting_player_dst,
            initializer_pubkey_dst,
            initializer_temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            joiner_pubkey_dst,
            joiner_temp_token_account_pubkey_dst,
            joiner_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
            secret_num_dst
        ) = array_refs![dst, 1, 1, 32, 32, 32, 32, 32, 32, 8, 8];


        let GameScrow {
            is_initialized,
            is_waiting_player,
            initializer_pubkey,
            initializer_temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            joiner_pubkey,
            joiner_temp_token_account_pubkey,
            joiner_token_to_receive_account_pubkey,
            expected_amount,
            secret_num
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;

        is_waiting_player_dst[0] = *is_waiting_player as u8;

        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        //initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        initializer_temp_token_account_pubkey_dst
            .copy_from_slice(initializer_temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst
            .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());

        joiner_pubkey_dst.copy_from_slice(joiner_pubkey.as_ref());
        joiner_temp_token_account_pubkey_dst
            .copy_from_slice(joiner_temp_token_account_pubkey.as_ref());
        joiner_token_to_receive_account_pubkey_dst
            .copy_from_slice(joiner_token_to_receive_account_pubkey.as_ref());

        *expected_amount_dst = expected_amount.to_le_bytes();
        *secret_num_dst = secret_num.to_le_bytes();

    }
}