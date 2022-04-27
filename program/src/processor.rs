use crate::{
    instruction::{InitStakeManagerArgs, StakeArgs, StakeBobInstruction, UnstakeArgs},
    state::{
        Key, StakeManager, StakeStatus, MANAGER, STAKE_MANAGER_SIZE, STAKE_STATUS_SIZE, STATUS,
    },
    utils::{
        assert_in_verified_collection, assert_keys_equal, assert_mint, assert_signer,
        assert_token_account, assert_uninitialized, assert_valid_token_account,
        create_rent_exempt_owned, delegate_nft, freeze_nft,
    },
};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    assertions::collection::assert_master_edition, state::Metadata, utils::assert_owned_by,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar::Sysvar,
};
use spl_token::state::Account as TokenAccount;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: StakeBobInstruction =
            StakeBobInstruction::try_from_slice(instruction_data)?;
        match instruction {
            StakeBobInstruction::InitStakeManager(args) => {
                msg!("Instruction: InitStakeManager");
                Self::process_init_stake_manager(accounts, args)
            }
            StakeBobInstruction::Stake(args) => {
                msg!("Instruction: Stake");
                Self::process_stake(program_id, accounts, args)
            }
            StakeBobInstruction::Unstake(args) => {
                msg!("Instruction: Unstake");
                Self::process_unstake(program_id, accounts, args)
            }
        }
    }

    pub fn process_init_stake_manager(
        accounts: &[AccountInfo],
        args: InitStakeManagerArgs,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let stake_initializer_info = next_account_info(account_info_iter)?;
        assert_signer(stake_initializer_info)?;

        let stake_manager_pda_info = next_account_info(account_info_iter)?;
        let collection_mint_info = next_account_info(account_info_iter)?;
        assert_mint(collection_mint_info)?;
        let (stake_key, bump) = StakeManager::find_address(collection_mint_info.key, None);
        assert_keys_equal(&stake_key, stake_manager_pda_info.key)?;

        let collection_metadata_info = next_account_info(account_info_iter)?;
        assert_keys_equal(collection_metadata_info.owner, &mpl_token_metadata::id())?;

        let collection_metadata = Metadata::from_account_info(collection_metadata_info)?;
        assert_keys_equal(
            &collection_metadata.update_authority,
            stake_initializer_info.key,
        )?;

        let system_program_info = next_account_info(account_info_iter)?;
        assert_keys_equal(system_program_info.key, &system_program::id())?;

        create_rent_exempt_owned(
            stake_initializer_info,
            stake_manager_pda_info,
            &[MANAGER.as_ref(), collection_mint_info.key.as_ref(), &[bump]],
            STAKE_MANAGER_SIZE,
            system_program_info,
        )?;

        StakeManager {
            key: Key::StakeManagerV1,
            min_stake_time: args.min_stake_time,
        }
        .serialize(&mut *stake_manager_pda_info.data.borrow_mut())?;
        Ok(())
    }

    pub fn process_stake(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: StakeArgs,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let staker_info = next_account_info(account_info_iter)?;
        assert_signer(staker_info)?;
        let token_metadata_program_info = next_account_info(account_info_iter)?;
        assert_keys_equal(token_metadata_program_info.key, &mpl_token_metadata::id())?;
        let token_program_info = next_account_info(account_info_iter)?;
        assert_keys_equal(token_program_info.key, &spl_token::id())?;
        let system_program_info = next_account_info(account_info_iter)?;
        assert_keys_equal(system_program_info.key, &system_program::id())?;

        let collection_mint_info = next_account_info(account_info_iter)?;
        assert_mint(collection_mint_info)?;

        let nft_metadata_info = next_account_info(account_info_iter)?;
        assert_owned_by(nft_metadata_info, &mpl_token_metadata::id())?;

        let nft_mint_info = next_account_info(account_info_iter)?;
        assert_owned_by(nft_mint_info, &mpl_token_metadata::id())?;

        let nft_metadata = Metadata::from_account_info(nft_metadata_info)?;
        assert_keys_equal(&nft_metadata.mint, nft_mint_info.key)?;
        assert_keys_equal(&nft_metadata.update_authority, staker_info.key)?;
        assert_in_verified_collection(&nft_metadata, collection_mint_info.key)?;

        let nft_edition_info = next_account_info(account_info_iter)?;
        assert_owned_by(nft_edition_info, &mpl_token_metadata::id())?;
        assert_master_edition(&nft_metadata, nft_edition_info)?;

        let nft_token_account_info = next_account_info(account_info_iter)?;
        assert_token_account(nft_token_account_info)?;

        let nft_token_account =
            TokenAccount::unpack_unchecked(&nft_token_account_info.try_borrow_data()?)?;
        assert_valid_token_account(nft_token_account, nft_mint_info.key, staker_info.key)?;

        let stake_manager_pda_info = next_account_info(account_info_iter)?;
        let (stake_manager_pda_key, _) =
            StakeManager::find_address(collection_mint_info.key, Some(args.stake_manager_pda_bump));
        assert_keys_equal(&stake_manager_pda_key, stake_manager_pda_info.key)?;

        let stake_status_pda_info = next_account_info(account_info_iter)?;
        let (stake_status_pda_key, bump) = StakeStatus::find_address(nft_mint_info.key, None);
        assert_keys_equal(&stake_status_pda_key, stake_status_pda_info.key)?;
        assert_uninitialized(stake_status_pda_info)?;

        create_rent_exempt_owned(
            staker_info,
            stake_status_pda_info,
            &[STATUS.as_ref(), nft_mint_info.key.as_ref(), &[bump]],
            STAKE_STATUS_SIZE,
            system_program_info,
        )?;

        delegate_nft(
            token_program_info,
            nft_token_account_info,
            stake_status_pda_info,
            staker_info,
        )?;

        freeze_nft(
            token_metadata_program_info,
            stake_status_pda_info,
            nft_token_account_info,
            nft_edition_info,
            nft_mint_info,
            token_program_info,
        )?;

        let timestamp = Clock::get()?.unix_timestamp as u64;

        StakeStatus {
            key: Key::StakeStatusV1,
            stake_start: timestamp,
            mint: *nft_mint_info.key,
            collection_mint: *collection_mint_info.key,
        }
        .serialize(&mut *stake_status_pda_info.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_unstake(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _args: UnstakeArgs,
    ) -> ProgramResult {
        let _account_info_iter = &mut accounts.iter();
        Ok(())
    }
}
