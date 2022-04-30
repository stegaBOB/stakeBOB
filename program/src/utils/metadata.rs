use crate::{error::StakeBobError, utils::assert_keys_equal};
use mpl_token_metadata::state::Metadata;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn freeze_nft<'a>(
    token_metadata_info: &AccountInfo<'a>,
    delegate_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    edition_account_info: &AccountInfo<'a>,
    mint_account_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let instruction = mpl_token_metadata::instruction::freeze_delegated_account(
        *token_metadata_info.key,
        *delegate_info.key,
        *token_account_info.key,
        *edition_account_info.key,
        *mint_account_info.key,
    );
    invoke(
        &instruction,
        &[
            delegate_info.clone(),
            token_account_info.clone(),
            edition_account_info.clone(),
            spl_token_info.clone(),
            token_metadata_info.clone(),
        ],
    )?;
    Ok(())
}

pub fn thaw_nft<'a>(
    token_metadata_info: &AccountInfo<'a>,
    delegate_info: &AccountInfo<'a>,
    token_account_info: &AccountInfo<'a>,
    edition_account_info: &AccountInfo<'a>,
    mint_account_info: &AccountInfo<'a>,
    spl_token_info: &AccountInfo<'a>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = mpl_token_metadata::instruction::thaw_delegated_account(
        *token_metadata_info.key,
        *delegate_info.key,
        *token_account_info.key,
        *edition_account_info.key,
        *mint_account_info.key,
    );
    invoke_signed(
        &instruction,
        &[
            delegate_info.clone(),
            token_account_info.clone(),
            edition_account_info.clone(),
            spl_token_info.clone(),
            token_metadata_info.clone(),
        ],
        seeds,
    )?;
    Ok(())
}

pub fn assert_in_verified_collection(
    metadata: &Metadata,
    collection_pubkey: &Pubkey,
) -> ProgramResult {
    match &metadata.collection {
        Some(collection) => {
            assert_keys_equal(&collection.key, collection_pubkey).map_err(|_| {
                let to_return: ProgramError = StakeBobError::NotInCollection.into();
                to_return
            })?;
            if collection.verified {
                Ok(())
            } else {
                Err(StakeBobError::NotInCollection.into())
            }
        }
        None => Err(StakeBobError::NotInCollection.into()),
    }
}
