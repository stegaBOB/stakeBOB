use crate::{error::StakeBobError, state::Key, utils::cmp_pubkeys};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn assert_derivation(
    program_id: &Pubkey,
    account_info: &AccountInfo,
    seeds: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(seeds, program_id);
    if account_info.key != &key {
        Err(StakeBobError::DerivedKeyInvalid.into())
    } else {
        Ok(bump)
    }
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_keys_equal(pubkey1: &Pubkey, pubkey2: &Pubkey) -> ProgramResult {
    if !cmp_pubkeys(pubkey1, pubkey2) {
        Err(ProgramError::InvalidAccountData)
    } else {
        Ok(())
    }
}

pub fn assert_uninitialized(account_info: &AccountInfo) -> ProgramResult {
    if account_info.data_is_empty() {
        Ok(())
    } else {
        Err(ProgramError::AccountAlreadyInitialized)
    }
}

pub fn assert_initialized(account_info: &AccountInfo) -> ProgramResult {
    if account_info.data.borrow_mut()[0] != Key::Uninitialized as u8 {
        Ok(())
    } else {
        Err(ProgramError::UninitializedAccount)
    }
}
