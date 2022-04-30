use crate::utils::{assert_keys_equal, cmp_pubkeys};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

pub fn assert_token_account(token_account: &AccountInfo) -> ProgramResult {
    match !cmp_pubkeys(token_account.owner, &spl_token::id())
        && token_account.data_len() != spl_token::state::Account::get_packed_len()
    {
        true => Err(ProgramError::InvalidAccountData),
        false => Ok(()),
    }
}

pub fn assert_mint(mint: &AccountInfo) -> ProgramResult {
    match mint.owner != &spl_token::id()
        && mint.data_len() != spl_token::state::Mint::get_packed_len()
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

pub fn undelegate_nft<'a>(
    token_program_info: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    signer: &AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = spl_token::instruction::revoke(
        token_program_info.key,
        source.key,
        owner.key,
        &[signer.key],
    )?;

    invoke_signed(
        &instruction,
        &[
            source.clone(),
            owner.clone(),
            signer.clone(),
            token_program_info.clone(),
        ],
        signer_seeds,
    )?;
    Ok(())
}
