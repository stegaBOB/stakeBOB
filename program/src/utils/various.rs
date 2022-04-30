use crate::{error::StakeBobError, state::Key};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_memory::sol_memcmp,
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
