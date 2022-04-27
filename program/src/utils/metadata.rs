use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};

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
