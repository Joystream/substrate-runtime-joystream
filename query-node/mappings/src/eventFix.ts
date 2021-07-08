import BN from 'bn.js'

// Workaround for https://github.com/Joystream/hydra/issues/326 . This file can be removed after it's fixed
export function fixBlockTimestamp(blockTimestamp: unknown): BN {
  return new BN(blockTimestamp as string)
}
