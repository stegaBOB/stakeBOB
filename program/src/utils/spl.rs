use crate::utils::assert_keys_equal;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

pub fn assert_token_account(token_account: &AccountInfo) -> ProgramResult {
    match token_account.owner != &spl_token::id()
        && token_account.data_len() != spl_token::state::Account::get_packed_len()
    {
        true => Err(ProgramError::InvalidAccountData),
        false => Ok(()),
    }
}

pub fn assert_valid_token_account(
    token_account: TokenAccount,
    mint: &Pubkey,
    owner: &Pubkey,
) -> ProgramResult {
    assert_keys_equal(&token_account.mint, mint)?;
    assert_keys_equal(&token_account.owner, owner)?;
    Ok(())
}

pub fn delegate_nft<'a>(
    token_program_info: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    delegate: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
) -> ProgramResult {
    let instruction = spl_token::instruction::approve(
        token_program_info.key,
        source.key,
        delegate.key,
        owner.key,
        &[owner.key],
        1,
    )?;

    invoke(
        &instruction,
        &[
            source.clone(),
            delegate.clone(),
            owner.clone(),
            token_program_info.clone(),
        ],
    )?;
    Ok(())
}
