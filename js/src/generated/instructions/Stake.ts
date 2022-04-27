/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import { StakeArgs, stakeArgsBeet } from '../types/StakeArgs';

/**
 * @category Instructions
 * @category Stake
 * @category generated
 */
export type StakeInstructionArgs = {
  stakeArgs: StakeArgs;
};
/**
 * @category Instructions
 * @category Stake
 * @category generated
 */
const StakeStruct = new beet.BeetArgsStruct<
  StakeInstructionArgs & {
    instructionDiscriminator: number;
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['stakeArgs', stakeArgsBeet],
  ],
  'StakeInstructionArgs',
);
/**
 * Accounts required by the _Stake_ instruction
 *
 * @property [_writable_, **signer**] staker Account that is about to stake their NFT
 * @property [] tokenMetadataProgram The Token Metadata Program
 * @property [] collectionMint The mint of the collection NFT
 * @property [] nftMetadata The token metadata for the NFT to be staked
 * @property [] nftMint The mint for the NFT to be staked
 * @property [] nftTokenAccount The token account for the NFT to be staked
 * @property [_writable_] stakeManagerPda The stakeBOB stake manager account
 * @property [_writable_] stakeStatusPda The new stake status PDA account
 * @category Instructions
 * @category Stake
 * @category generated
 */
export type StakeInstructionAccounts = {
  staker: web3.PublicKey;
  tokenMetadataProgram: web3.PublicKey;
  collectionMint: web3.PublicKey;
  nftMetadata: web3.PublicKey;
  nftMint: web3.PublicKey;
  nftTokenAccount: web3.PublicKey;
  stakeManagerPda: web3.PublicKey;
  stakeStatusPda: web3.PublicKey;
};

const stakeInstructionDiscriminator = 1;

/**
 * Creates a _Stake_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Stake
 * @category generated
 */
export function createStakeInstruction(
  accounts: StakeInstructionAccounts,
  args: StakeInstructionArgs,
) {
  const {
    staker,
    tokenMetadataProgram,
    collectionMint,
    nftMetadata,
    nftMint,
    nftTokenAccount,
    stakeManagerPda,
    stakeStatusPda,
  } = accounts;

  const [data] = StakeStruct.serialize({
    instructionDiscriminator: stakeInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: staker,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: tokenMetadataProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: collectionMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: nftMetadata,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: nftMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: nftTokenAccount,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: stakeManagerPda,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: stakeStatusPda,
      isWritable: true,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey('MyProgram1111111111111111111111111111111111'),
    keys,
    data,
  });
  return ix;
}
