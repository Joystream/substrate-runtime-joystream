import { StoreContext } from '@joystream/hydra-common'
import { StorageSystemParameters } from 'query-node/dist/model'
import { storageSystem } from './genesis-data'

export async function loadGenesisData({ store }: StoreContext): Promise<void> {
  // Storage system
  await store.save<StorageSystemParameters>(
    new StorageSystemParameters({
      ...storageSystem,
    })
  )
}
