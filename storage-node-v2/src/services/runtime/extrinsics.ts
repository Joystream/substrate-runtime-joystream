import { sendAndFollowSudoNamedTx, sendAndFollowNamedTx } from './api'
import { getAlicePair } from './accounts'
import { KeyringPair } from '@polkadot/keyring/types'
import { CodecArg } from '@polkadot/types/types'
import { ApiPromise } from '@polkadot/api'
import { BagId, DynamicBagType } from '@joystream/types/storage'
import logger from '../../services/logger'

export async function createStorageBucket(
  api: ApiPromise,
  account: KeyringPair,
  invitedWorker: number | null = null,
  allowedNewBags = true,
  sizeLimit = 0,
  objectsLimit = 0
): Promise<boolean> {
  return await extrinsicWrapper(() => {
    const invitedWorkerValue = api.createType('Option<WorkerId>', invitedWorker)

    return sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'createStorageBucket',
      [invitedWorkerValue, allowedNewBags, sizeLimit, objectsLimit]
    )
  })
}

export async function acceptStorageBucketInvitation(
  api: ApiPromise,
  account: KeyringPair,
  workerId: number,
  storageBucketId: number
): Promise<boolean> {
  return await extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'acceptStorageBucketInvitation',
      [workerId, storageBucketId]
    )
  )
}

export async function updateStorageBucketsForBag(
  api: ApiPromise,
  bagId: BagId,
  account: KeyringPair,
  bucketId: number,
  removeBucket: boolean
): Promise<boolean> {
  return await extrinsicWrapper(() => {
    let addBuckets: CodecArg
    let removeBuckets: CodecArg

    if (removeBucket) {
      removeBuckets = api.createType('StorageBucketIdSet', [bucketId])
    } else {
      addBuckets = api.createType('StorageBucketIdSet', [bucketId])
    }

    return sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateStorageBucketsForBag',
      [bagId, addBuckets, removeBuckets]
    )
  })
}

export async function uploadDataObjects(
  api: ApiPromise,
  objectSize: number,
  objectCid: string
): Promise<boolean> {
  return await extrinsicWrapper(() => {
    const alice = getAlicePair()

    const data = api.createType('UploadParameters', {
      deletionPrizeSourceAccountId: alice.address,
      objectCreationList: [
        {
          Size: objectSize,
          IpfsContentId: objectCid,
        },
      ],
    })

    return sendAndFollowSudoNamedTx(
      api,
      alice,
      'storage',
      'sudoUploadDataObjects',
      [data]
    )
  })
}

export async function acceptPendingDataObjects(
  api: ApiPromise,
  bagId: BagId,
  account: KeyringPair,
  workerId: number,
  storageBucketId: number,
  dataObjects: number[]
): Promise<boolean> {
  return await extrinsicWrapper(() => {
    const dataObjectSet: CodecArg = api.createType(
      'DataObjectIdSet',
      dataObjects
    )

    return sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'acceptPendingDataObjects',
      [workerId, storageBucketId, bagId, dataObjectSet]
    )
  }, true)
}

export async function updateStorageBucketsPerBagLimit(
  api: ApiPromise,
  account: KeyringPair,
  newLimit: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateStorageBucketsPerBagLimit',
      [newLimit]
    )
  )
}

export async function updateStorageBucketsVoucherMaxLimits(
  api: ApiPromise,
  account: KeyringPair,
  newSizeLimit: number,
  newObjectLimit: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateStorageBucketsVoucherMaxLimits',
      [newSizeLimit, newObjectLimit]
    )
  )
}

export async function deleteStorageBucket(
  api: ApiPromise,
  account: KeyringPair,
  bucketId: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(api, account, 'storage', 'deleteStorageBucket', [
      bucketId,
    ])
  )
}

export async function inviteStorageBucketOperator(
  api: ApiPromise,
  account: KeyringPair,
  bucketId: number,
  operatorId: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'inviteStorageBucketOperator',
      [bucketId, operatorId]
    )
  )
}

async function extrinsicWrapper(
  extrinsic: () => Promise<void>,
  throwErr = false
): Promise<boolean> {
  try {
    await extrinsic()
  } catch (err) {
    logger.error(`Api Error: ${err}`)

    if (throwErr) {
      throw err
    }
    return false
  }

  return true
}

export async function cancelStorageBucketOperatorInvite(
  api: ApiPromise,
  account: KeyringPair,
  bucketId: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'cancelStorageBucketOperatorInvite',
      [bucketId]
    )
  )
}

export async function removeStorageBucketOperator(
  api: ApiPromise,
  account: KeyringPair,
  bucketId: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'removeStorageBucketOperator',
      [bucketId]
    )
  )
}

export async function updateDataSizeFee(
  api: ApiPromise,
  account: KeyringPair,
  fee: number
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(api, account, 'storage', 'updateDataSizeFee', [fee])
  )
}

export async function setStorageOperatorMetadata(
  api: ApiPromise,
  account: KeyringPair,
  operatorId: number,
  bucketId: number,
  metadata: string
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'setStorageOperatorMetadata',
      [operatorId, bucketId, metadata]
    )
  )
}

export async function updateUploadingBlockedStatus(
  api: ApiPromise,
  account: KeyringPair,
  newStatus: boolean
): Promise<boolean> {
  return extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateUploadingBlockedStatus',
      [newStatus]
    )
  )
}

export async function updateStorageBucketStatus(
  api: ApiPromise,
  account: KeyringPair,
  workerId: number,
  storageBucketId: number,
  newStatus: boolean
): Promise<boolean> {
  return await extrinsicWrapper(() =>
    sendAndFollowNamedTx(api, account, 'storage', 'updateStorageBucketStatus', [
      workerId,
      storageBucketId,
      newStatus,
    ])
  )
}

export async function setStorageBucketVoucherLimits(
  api: ApiPromise,
  account: KeyringPair,
  workerId: number,
  storageBucketId: number,
  newSizeLimit: number,
  newObjectLimit: number
): Promise<boolean> {
  return await extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'setStorageBucketVoucherLimits',
      [workerId, storageBucketId, newSizeLimit, newObjectLimit]
    )
  )
}

export async function updateNumberOfStorageBucketsInDynamicBagCreationPolicy(
  api: ApiPromise,
  account: KeyringPair,
  dynamicBagType: DynamicBagType,
  newNumber: number
): Promise<boolean> {
  return await extrinsicWrapper(() =>
    sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateNumberOfStorageBucketsInDynamicBagCreationPolicy',
      [dynamicBagType, newNumber]
    )
  )
}

export async function updateBlacklist(
  api: ApiPromise,
  account: KeyringPair,
  cid: string,
  removeCid: boolean
): Promise<boolean> {
  return await extrinsicWrapper(() => {
    let addHashes: CodecArg
    let removeHashes: CodecArg

    if (removeCid) {
      removeHashes = api.createType('ContentIdSet', [cid])
    } else {
      addHashes = api.createType('ContentIdSet', [cid])
    }

    return sendAndFollowNamedTx(
      api,
      account,
      'storage',
      'updateBlacklist',
      [removeHashes, addHashes]
    )
  })
}