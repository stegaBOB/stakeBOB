use crate::{error::StakeBobError, state::Key, utils::assert_keys_equal};
use borsh::BorshDeserialize;
use mpl_token_metadata::state::Metadata;
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_memory::sol_memcmp,
    program_pack::Pack,
    pubkey::{Pubkey, PUBKEY_BYTES},
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if (data[0] != data_type as u8 && data[0] != Key::Uninitialized as u8)
        || data.len() != data_size
    {
        return Err(StakeBobError::DataTypeMismatch.into());
    }

    let result = try_from_slice_unchecked(data)?;
    Ok(result)
}

pub fn assert_mint(mint: &AccountInfo) -> ProgramResult {
    match mint.owner != &spl_token::id()
        && mint.data_len() != spl_token::state::Mint::get_packed_len()
    {
        true => Err(ProgramError::InvalidAccountData),
        false => Ok(()),
    }
}

pub fn create_rent_exempt_owned<'a>(
    payer_info: &AccountInfo<'a>,
    new_account_info: &AccountInfo<'a>,
    seeds: &[&[u8]],
    space: usize,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let rent = Rent::get()?;
    let lamports = rent
        .minimum_balance(space)
        .saturating_sub(new_account_info.lamports());

    invoke_signed(
        &system_instruction::create_account(
            payer_info.key,
            new_account_info.key,
            lamports,
            space as u64,
            &crate::id(),
        ),
        &[
            payer_info.clone(),
            new_account_info.clone(),
            system_program.clone(),
        ],
        &[seeds],
    )
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

pub fn assert_uninitialized(account_info: &AccountInfo) -> ProgramResult {
    if account_info.data_is_empty() {
        Ok(())
    } else {
        Err(ProgramError::AccountAlreadyInitialized)
    }
}
