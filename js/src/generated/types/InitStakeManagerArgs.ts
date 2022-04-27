/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
export type InitStakeManagerArgs = {
  minStakeTime: beet.bignum;
};

/**
 * @category userTypes
 * @category generated
 */
export const initStakeManagerArgsBeet = new beet.BeetArgsStruct<InitStakeManagerArgs>(
  [['minStakeTime', beet.u64]],
  'InitStakeManagerArgs',
);
