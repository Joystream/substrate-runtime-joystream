/*
eslint-disable @typescript-eslint/naming-convention
*/
import { DatabaseManager, EventContext, StoreContext } from '@joystream/hydra-common'
import { Storage } from './generated/types/storage'
import {
  StorageBag,
  StorageBagOwner,
  StorageBagOwnerChannel,
  StorageBagOwnerCouncil,
  StorageBagOwnerMember,
  StorageBagOwnerWorkingGroup,
  StorageBucket,
  StorageBucketOperatorStatusActive,
  StorageBucketOperatorStatusInvited,
  StorageBucketOperatorStatusMissing,
  StorageDataObject,
  StorageSystemParameters,
} from 'query-node/dist/model'
import BN from 'bn.js'
import { getById, getWorkingGroupModuleName } from './common'
import { BTreeSet } from '@polkadot/types'
import { DataObjectCreationParameters } from '@joystream/types/storage'
import { registry } from '@joystream/types'
import { In } from 'typeorm'
import _ from 'lodash'
import { DataObjectId, BagId, DynamicBagId, StaticBagId } from '@joystream/types/augment/all'

async function getDataObjectsInBag(
  store: DatabaseManager,
  bagId: BagId,
  dataObjectIds: BTreeSet<DataObjectId>
): Promise<StorageDataObject[]> {
  const dataObjects = await store.getMany(StorageDataObject, {
    where: {
      id: In(Array.from(dataObjectIds).map((id) => id.toString())),
      storageBag: { id: getBagId(bagId) },
    },
  })
  if (dataObjects.length !== Array.from(dataObjectIds).length) {
    throw new Error(
      `Missing data objects: ${_.difference(
        Array.from(dataObjectIds).map((id) => id.toString()),
        dataObjects.map((o) => o.id)
      )} in bag ${getBagId(bagId)}`
    )
  }
  return dataObjects
}

function getStaticBagOwner(bagId: StaticBagId): typeof StorageBagOwner {
  if (bagId.isCouncil) {
    return new StorageBagOwnerCouncil()
  } else if (bagId.isWorkingGroup) {
    const owner = new StorageBagOwnerWorkingGroup()
    owner.workingGroupId = getWorkingGroupModuleName(bagId.asWorkingGroup)
    return owner
  } else {
    throw new Error(`Unexpected static bag type: ${bagId.type}`)
  }
}

function getDynamicBagOwner(bagId: DynamicBagId) {
  if (bagId.isChannel) {
    const owner = new StorageBagOwnerChannel()
    owner.channelId = bagId.asChannel.toNumber()
    return owner
  } else if (bagId.isMember) {
    const owner = new StorageBagOwnerMember()
    owner.memberId = bagId.asMember.toNumber()
    return owner
  } else {
    throw new Error(`Unexpected dynamic bag type: ${bagId.type}`)
  }
}

function getStaticBagId(bagId: StaticBagId): string {
  if (bagId.isCouncil) {
    return `CO`
  } else if (bagId.isWorkingGroup) {
    return `WG-${bagId.asWorkingGroup.type}`
  } else {
    throw new Error(`Unexpected static bag type: ${bagId.type}`)
  }
}

function getDynamicBagId(bagId: DynamicBagId): string {
  if (bagId.isChannel) {
    return `CH-${bagId.asChannel.toString()}`
  } else if (bagId.isMember) {
    return `M-${bagId.asMember.toString()}`
  } else {
    throw new Error(`Unexpected dynamic bag type: ${bagId.type}`)
  }
}

function getBagId(bagId: BagId) {
  return bagId.isStatic ? getStaticBagId(bagId.asStatic) : getDynamicBagId(bagId.asDynamic)
}

async function getDynamicBag(
  store: DatabaseManager,
  bagId: DynamicBagId,
  relations?: ('storedBy' | 'objects')[]
): Promise<StorageBag> {
  return getById(store, StorageBag, getDynamicBagId(bagId), relations)
}

async function getStaticBag(
  store: DatabaseManager,
  bagId: StaticBagId,
  relations?: ('storedBy' | 'objects')[]
): Promise<StorageBag> {
  const id = getStaticBagId(bagId)
  const bag = await store.get(StorageBag, { where: { id }, relations })
  if (!bag) {
    console.log(`Creating new static bag: ${id}`)
    const newBag = new StorageBag({
      id,
      owner: getStaticBagOwner(bagId),
    })
    await store.save<StorageBag>(newBag)
    return newBag
  }
  return bag
}

async function getBag(store: DatabaseManager, bagId: BagId, relations?: 'storedBy'[]): Promise<StorageBag> {
  return bagId.isStatic
    ? getStaticBag(store, bagId.asStatic, relations)
    : getDynamicBag(store, bagId.asDynamic, relations)
}

// BUCKETS

export async function storage_StorageBucketCreated({ event, store }: EventContext & StoreContext): Promise<void> {
  const [
    bucketId,
    invitedWorkerId,
    acceptingNewBags,
    dataObjectSizeLimit,
    dataObjectCountLimit,
  ] = new Storage.StorageBucketCreatedEvent(event).params

  const storageBucket = new StorageBucket({
    id: bucketId.toString(),
    acceptingNewBags: acceptingNewBags.isTrue,
    dataObjectCountLimit: new BN(dataObjectCountLimit.toString()),
    dataObjectsSizeLimit: new BN(dataObjectSizeLimit.toString()),
  })
  if (invitedWorkerId.isSome) {
    const operatorStatus = new StorageBucketOperatorStatusInvited()
    operatorStatus.workerId = invitedWorkerId.unwrap().toNumber()
    storageBucket.operatorStatus = operatorStatus
  } else {
    storageBucket.operatorStatus = new StorageBucketOperatorStatusMissing()
  }
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageOperatorMetadataSet({ event, store }: EventContext & StoreContext): Promise<void> {
  const [bucketId, , metadataBytes] = new Storage.StorageOperatorMetadataSetEvent(event).params
  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  storageBucket.operatorMetadata = Buffer.from(metadataBytes.toU8a(true))
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketStatusUpdated({ event, store }: EventContext & StoreContext): Promise<void> {
  const [bucketId, , acceptingNewBags] = new Storage.StorageBucketStatusUpdatedEvent(event).params

  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  storageBucket.acceptingNewBags = acceptingNewBags.isTrue
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketInvitationAccepted({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [bucketId, workerId] = new Storage.StorageBucketInvitationAcceptedEvent(event).params
  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  const operatorStatus = new StorageBucketOperatorStatusActive()
  operatorStatus.workerId = workerId.toNumber()
  storageBucket.operatorStatus = operatorStatus
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketInvitationCancelled({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [bucketId] = new Storage.StorageBucketInvitationCancelledEvent(event).params
  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  const operatorStatus = new StorageBucketOperatorStatusMissing()
  storageBucket.operatorStatus = operatorStatus
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketOperatorInvited({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [bucketId, workerId] = new Storage.StorageBucketOperatorInvitedEvent(event).params
  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  const operatorStatus = new StorageBucketOperatorStatusInvited()
  operatorStatus.workerId = workerId.toNumber()
  storageBucket.operatorStatus = operatorStatus
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketOperatorRemoved({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [bucketId] = new Storage.StorageBucketInvitationCancelledEvent(event).params
  const storageBucket = await getById(store, StorageBucket, bucketId.toString())
  const operatorStatus = new StorageBucketOperatorStatusMissing()
  storageBucket.operatorStatus = operatorStatus
  await store.save<StorageBucket>(storageBucket)
}

export async function storage_StorageBucketsUpdatedForBag({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [bagId, addedBucketsIds, removedBucketsIds] = new Storage.StorageBucketsUpdatedForBagEvent(event).params
  const storageBag = await getBag(store, bagId, ['storedBy'])
  storageBag.storedBy = (storageBag.storedBy || [])
    .filter((b) => !Array.from(removedBucketsIds).some((id) => id.eq(b.id)))
    .concat(Array.from(addedBucketsIds).map((id) => new StorageBucket({ id: id.toString() })))

  await store.save<StorageBag>(storageBag)
}

export async function storage_StorageBucketDeleted({ event, store }: EventContext & StoreContext): Promise<void> {
  const [bucketId] = new Storage.StorageBucketDeletedEvent(event).params
  // TODO: Delete or just change status?
  // TODO: Cascade remove on db level?
  // We shouldn't have to worry about deleting DataObjects, since this is already enforced by the runtime
  const storageBucket = await getById(store, StorageBucket, bucketId.toString(), ['storedBags'])
  await Promise.all((storageBucket.storedBags || []).map((b) => store.remove<StorageBag>(b)))
  await store.remove<StorageBucket>(storageBucket)
}

// DYNAMIC BAGS
export async function storage_DynamicBagCreated({ event, store }: EventContext & StoreContext): Promise<void> {
  const [bagId] = new Storage.DynamicBagCreatedEvent(event).params
  const storageBag = new StorageBag({
    id: getDynamicBagId(bagId),
    owner: getDynamicBagOwner(bagId),
  })
  await store.save<StorageBag>(storageBag)
}

export async function storage_DynamicBagDeleted({ event, store }: EventContext & StoreContext): Promise<void> {
  const [, bagId] = new Storage.DynamicBagDeletedEvent(event).params
  // TODO: Delete or just change status?
  // TODO: Cascade remove on db level?
  const storageBag = await getDynamicBag(store, bagId, ['objects'])
  await Promise.all((storageBag.objects || []).map((o) => store.remove<StorageDataObject>(o)))
  await store.remove<StorageBag>(storageBag)
}

// DATA OBJECTS

// Note: "Uploaded" here actually means "created" (the real upload happens later)
export async function storage_DataObjectdUploaded({ event, store }: EventContext & StoreContext): Promise<void> {
  const [dataObjectIds, uploadParams] = new Storage.DataObjectdUploadedEvent(event).params
  const { bagId, authenticationKey, objectCreationList } = uploadParams
  const storageBag = await getBag(store, bagId)
  const dataObjects = dataObjectIds.map((objectId, i) => {
    const objectParams = new DataObjectCreationParameters(registry, objectCreationList[i].toJSON() as any)
    return new StorageDataObject({
      id: objectId.toString(),
      authenticationKey: authenticationKey.toString(),
      isAccepted: false,
      ipfsHash: objectParams.ipfsContentId.toString(),
      size: new BN(objectParams.getField('size').toString()),
      storageBag,
    })
  })
  await Promise.all(dataObjects.map((o) => store.save<StorageDataObject>(o)))
}

export async function storage_PendingDataObjectsAccepted({ event, store }: EventContext & StoreContext): Promise<void> {
  const [, , bagId, dataObjectIds] = new Storage.PendingDataObjectsAcceptedEvent(event).params
  const dataObjects = await getDataObjectsInBag(store, bagId, dataObjectIds)
  await Promise.all(
    dataObjects.map(async (dataObject) => {
      dataObject.isAccepted = true
      // TODO: Do we still want other storage providers to accept it? How long should the key be valid?
      // dataObject.authenticationKey = null as any
      await store.save<StorageDataObject>(dataObject)
    })
  )
}

export async function storage_DataObjectsMoved({ event, store }: EventContext & StoreContext): Promise<void> {
  const [srcBagId, destBagId, dataObjectIds] = new Storage.DataObjectsMovedEvent(event).params
  const dataObjects = await getDataObjectsInBag(store, srcBagId, dataObjectIds)
  const destBag = await getBag(store, destBagId)
  await Promise.all(
    dataObjects.map(async (dataObject) => {
      dataObject.storageBag = destBag
      await store.save<StorageDataObject>(dataObject)
    })
  )
}

export async function storage_DataObjectsDeleted({ event, store }: EventContext & StoreContext): Promise<void> {
  const [, bagId, dataObjectIds] = new Storage.DataObjectsDeletedEvent(event).params
  const dataObjects = await getDataObjectsInBag(store, bagId, dataObjectIds)
  // TODO: Delete them or just change status?
  // (may not be so optimal if we expect a large amount of data objects)
  await Promise.all(dataObjects.map((o) => store.remove<StorageDataObject>(o)))
}

// BLACKLIST
export async function storage_UpdateBlacklist({ event, store }: EventContext & StoreContext): Promise<void> {
  const [removedContentIds, addedContentIds] = new Storage.UpdateBlacklistEvent(event).params
  const storageSystem = await store.get(StorageSystemParameters, {})
  if (!storageSystem) {
    throw new Error('StorageSystemParameters entity not found!')
  }
  storageSystem.blacklist = storageSystem.blacklist
    .filter((cid) => !Array.from(removedContentIds).some((id) => id.eq(cid)))
    .concat(Array.from(addedContentIds).map((id) => id.toString()))

  await store.save<StorageSystemParameters>(storageSystem)
}

export async function storage_StorageBucketVoucherLimitsSet({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_UploadingBlockStatusUpdated({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_DataObjectPerMegabyteFeeUpdated({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_StorageBucketsPerBagLimitUpdated({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_StorageBucketsVoucherMaxLimitsUpdated({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_DeletionPrizeChanged({ event, store }: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_VoucherChanged({ event, store }: EventContext & StoreContext): Promise<void> {
  // To be implemented
}

export async function storage_NumberOfStorageBucketsInDynamicBagCreationPolicyUpdated({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  // To be implemented
}
