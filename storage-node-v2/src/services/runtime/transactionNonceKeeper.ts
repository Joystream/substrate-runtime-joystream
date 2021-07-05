import { KeyringPair } from '@polkadot/keyring/types'
import type { Index } from '@polkadot/types/interfaces/runtime'
import BN from 'bn.js'
import AwaitLock from 'await-lock'
import { ApiPromise } from '@polkadot/api'
import logger from '../logger'

let nonce: Index | null = null
const lock = new AwaitLock()

export async function getNonce(
  api: ApiPromise,
  account: KeyringPair
): Promise<Index> {
  await lock.acquireAsync()
  try {
    if (nonce === null) {
      nonce = await api.rpc.system.accountNextIndex(account.address)
    } else {
      nonce = nonce.add(new BN(1)) as Index
    }
  } finally {
    lock.release()
  }

  logger.debug(`Last transaction nonce:${nonce}`)

  return nonce as Index
}
