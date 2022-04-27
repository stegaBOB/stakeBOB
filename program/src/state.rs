use crate::{
    error::StakeBobError,
    utils::{cmp_pubkeys, try_from_slice_checked},
};
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub const STATUS: &str = "status";
pub const MANAGER: &str = "manager";

pub const STAKE_MANAGER_SIZE: usize = 1 // key
    + 32 // collection mint
    + 8; // min stake time

pub const STAKE_STATUS_SIZE: usize = 1 // key
    + 32 // collection mint
    + 32 // mint
    + 8; // stake start

#[repr(C)]
#[non_exhaustive]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    StakeManagerV1,
    StakeStatusV1,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct StakeManager {
    pub key: Key,
    pub min_stake_time: u64,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct StakeStatus {
    pub key: Key,
    pub collection_mint: Pubkey,
    pub mint: Pubkey,
    pub stake_start: u64,
}

// pub trait FromAccountInfo {
//     type T;
//
//     fn from_account_info(account: &AccountInfo) -> Result<Self::T, ProgramError>;
// }

impl StakeManager {
    pub fn from_account_info(account_info: &AccountInfo) -> Result<StakeStatus, ProgramError> {
        let status: StakeStatus = try_from_slice_checked(
            &account_info.data.borrow_mut(),
            Key::StakeStatusV1,
            STAKE_STATUS_SIZE,
        )?;
        Ok(status)
    }

    // pub fn get_seeds(collection_mint: &Pubkey) -> [&[u8]; 2] {
    //     [MANAGER.as_ref(), collection_mint.as_ref()]
    // }

    pub fn find_address(collection_mint: &Pubkey, bump: Option<u8>) -> (Pubkey, u8) {
        match bump {
            Some(bump) => Pubkey::find_program_address(
                &[b"StakeManager", collection_mint.as_ref(), &[bump]],
                &crate::id(),
            ),
            None => Pubkey::find_program_address(
                &[b"StakeManager", collection_mint.as_ref()],
                &crate::id(),
            ),
        }
    }

    pub fn assert_address(
        pda: &Pubkey,
        collection_mint: &Pubkey,
        bump: Option<u8>,
    ) -> ProgramResult {
        let (address, _) = StakeManager::find_address(collection_mint, bump);
        if cmp_pubkeys(pda, &address) {
            Err(StakeBobError::DerivedKeyInvalid.into())
        } else {
            Ok(())
        }
    }
}

impl StakeStatus {
    pub fn from_account_info(account_info: &AccountInfo) -> Result<StakeStatus, ProgramError> {
        let status: StakeStatus = try_from_slice_checked(
            &account_info.data.borrow_mut(),
            Key::StakeStatusV1,
            STAKE_STATUS_SIZE,
        )?;
        Ok(status)
    }

    pub fn find_address(mint: &Pubkey, bump: Option<u8>) -> (Pubkey, u8) {
        match bump {
            Some(bump) => Pubkey::find_program_address(
                &[MANAGER.as_ref(), mint.as_ref(), &[bump]],
                &crate::id(),
            ),
            None => Pubkey::find_program_address(&[MANAGER.as_ref(), mint.as_ref()], &crate::id()),
        }
    }

    pub fn assert_address(pda: &Pubkey, mint: &Pubkey, bump: Option<u8>) -> ProgramResult {
        let (address, _) = StakeStatus::find_address(mint, bump);
        if cmp_pubkeys(pda, &address) {
            Err(StakeBobError::DerivedKeyInvalid.into())
        } else {
            Ok(())
        }
    }
}
