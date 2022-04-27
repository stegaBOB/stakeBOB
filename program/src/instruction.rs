use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitStakeManagerArgs {
    pub min_stake_time: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct StakeArgs {
    pub stake_manager_pda_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UnstakeArgs {
    pub stake_manager_pda_bump: u8,
    pub stake_status_bump: u8,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum StakeBobInstruction {
    /// Creates a new stakeBOB stake manager
    #[account(0, writable, signer, name="stake_initializer", desc = "Account that initializes the stake manager. Also has to be the authority of the provided collection NFT")]
    #[account(1, writable, name="stake_manager_pda", desc = "The stakeBOB stake manager account")]
    #[account(2, name="collection_mint", desc = "The mint of the collection NFT")]
    #[account(3, name="collection_metadata", desc = "The token metadata for the collection NFT")]
    #[account(4, name="system_program", desc = "The system program")]
    InitStakeManager(InitStakeManagerArgs),

    /// Stakes an NFT
    #[account(0, writable, signer, name="staker", desc = "Account that is about to stake their NFT")]
    #[account(1, name="token_metadata_program", desc = "The Token Metadata Program")]
    #[account(2, name="token_program", desc = "The SPL Token Program")]
    #[account(3, name="system_program", desc = "The system program")]
    #[account(4, name="collection_mint", desc = "The mint of the collection NFT")]
    #[account(5, name="nft_metadata", desc = "The token metadata for the NFT to be staked")]
    #[account(6, name="nft_mint", desc = "The mint for the NFT to be staked")]
    #[account(7, name="nft_token_account", desc = "The token account for the NFT to be staked")]
    #[account(8, writable, name="stake_manager_pda", desc = "The stakeBOB stake manager account")]
    #[account(9, writable, name="stake_status_pda", desc = "The new stake status PDA account")]
    Stake(StakeArgs),

    /// Unstakes an NFT
    #[account(0, writable, signer, name="staker", desc = "Account that is about to unstake their NFT")]
    #[account(1, writable, name="stake_manager", desc = "The stakeBOB stake manager account")]
    #[account(2, writable, name="stake_status_pda", desc = "The stake status PDA account")]
    #[account(3, name="collection_mint", desc = "The mint of the collection NFT")]
    #[account(4, name="collection_metadata", desc = "The token metadata for the collection NFT")]
    #[account(5, name="token_metadata_program", desc = "The Token Metadata Program")]
    #[account(6, name="token_program", desc = "The SPL Token Program")]
    Unstake(UnstakeArgs),
}
