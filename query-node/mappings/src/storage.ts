import { fixBlockTimestamp } from './eventFix'
import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { FindConditions, In } from 'typeorm'

import { inconsistentState, logger, prepareDataObject } from './common'

import { DataDirectory } from '../../generated/types'
import { ContentId, ContentParameters, StorageObjectOwner } from '@joystream/types/augment'

import { ContentId as Custom_ContentId, ContentParameters as Custom_ContentParameters } from '@joystream/types/storage'
import { registry } from '@joystream/types'

import {
  Channel,
  Video,
  AssetAvailability,
  DataObject,
  DataObjectOwner,
  DataObjectOwnerMember,
  DataObjectOwnerChannel,
  DataObjectOwnerDao,
  DataObjectOwnerCouncil,
  DataObjectOwnerWorkingGroup,
  LiaisonJudgement,
  Worker,
  WorkerType,
} from 'query-node'

export async function dataDirectory_ContentAdded(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { contentParameters, storageObjectOwner } = new DataDirectory.ContentAddedEvent(event).data

  // save all content objects
  for (const parameters of contentParameters) {
    const owner = convertStorageObjectOwner(storageObjectOwner)
    const dataObject = await prepareDataObject(db, parameters, event, owner)

    // fill in auto-generated fields
    dataObject.createdAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())
    dataObject.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

    await db.save<DataObject>(dataObject)
  }

  // emit log event
  logger.info('Storage content has beed added', {
    ids: contentParameters.map((item) => encodeContentId(item.content_id)),
  })
}

export async function dataDirectory_ContentRemoved(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { contentId: contentIds } = new DataDirectory.ContentRemovedEvent(event).data

  // load assets
  const dataObjects = await db.getMany(DataObject, {
    where: {
      joystreamContentId: In(contentIds.map((item) => encodeContentId(item))),
    } as FindConditions<DataObject>,
  })

  // store dataObject ids before they are deleted (for logging purposes)
  const dataObjectIds = dataObjects.map((item) => item.id)

  // remove assets from database
  for (const item of dataObjects) {
    // ensure dataObject is nowhere used to prevent db constraint error
    await disconnectDataObjectRelations(db, item)

    // remove data object
    await db.remove<DataObject>(item)
  }

  // emit log event
  logger.info('Storage content have been removed', { id: contentIds, dataObjectIds })
}

export async function dataDirectory_ContentAccepted(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { contentId, storageProviderId } = new DataDirectory.ContentAcceptedEvent(event).data
  const encodedContentId = encodeContentId(contentId)

  // load asset
  const dataObject = await db.get(DataObject, {
    where: { joystreamContentId: encodedContentId } as FindConditions<DataObject>,
  })

  // ensure object exists
  if (!dataObject) {
    return inconsistentState('Non-existing content acceptation requested', encodedContentId)
  }

  // load storage provider
  const worker = await db.get(Worker, {
    where: {
      workerId: storageProviderId.toString(),
      type: WorkerType.STORAGE,
    } as FindConditions<Worker>,
  })

  // ensure object exists
  if (!worker) {
    return inconsistentState('Missing Storage Provider Id', storageProviderId)
  }

  // update object
  dataObject.liaison = worker
  dataObject.liaisonJudgement = LiaisonJudgement.ACCEPTED

  // set last update time
  dataObject.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

  // save object
  await db.save<DataObject>(dataObject)

  // emit log event
  logger.info('Storage content has been accepted', { id: encodedContentId })

  // update asset availability for all connected channels and videos
  // this will not be needed after redudant AssetAvailability will be removed (after some Hydra upgrades)
  await updateConnectedAssets(db, dataObject)
}

/// ///////////////// Updating connected entities ////////////////////////////////

async function updateConnectedAssets(db: DatabaseManager, dataObject: DataObject) {
  await updateSingleConnectedAsset(db, new Channel(), 'avatarPhoto', dataObject)
  await updateSingleConnectedAsset(db, new Channel(), 'coverPhoto', dataObject)

  await updateSingleConnectedAsset(db, new Video(), 'thumbnailPhoto', dataObject)
  await updateSingleConnectedAsset(db, new Video(), 'media', dataObject)
}

// async function updateSingleConnectedAsset(db: DatabaseManager, type: typeof Channel | typeof Video, propertyName: string, dataObject: DataObject) {
async function updateSingleConnectedAsset<T extends Channel | Video>(
  db: DatabaseManager,
  type: T,
  propertyName: string,
  dataObject: DataObject
) {
  // prepare lookup condition
  const condition = {
    where: {
      [propertyName + 'DataObject']: dataObject,
    },
  } // as FindConditions<T>

  // NOTE: we don't need to retrieve multiple channels/videos via `db.getMany()` because dataObject
  //       is allowed to be associated only with one channel/video in runtime

  // in therory the following condition(s) can be generalized `... db.get(type, ...` but in practice it doesn't work :-\
  const item = type instanceof Channel ? await db.get(Channel, condition) : await db.get(Video, condition)

  // escape when no dataObject association found
  if (!item) {
    return
  }

  item[propertyName + 'Availability'] = AssetAvailability.ACCEPTED

  if (type instanceof Channel) {
    await db.save<Channel>(item)

    // emit log event
    logger.info('Channel using Content has been accepted', {
      channelId: item.id.toString(),
      joystreamContentId: dataObject.joystreamContentId,
    })
  } else {
    await db.save<Video>(item)

    // emit log event
    logger.info('Video using Content has been accepted', {
      videoId: item.id.toString(),
      joystreamContentId: dataObject.joystreamContentId,
    })
  }
}

// removes connection between dataObject and other entities
async function disconnectDataObjectRelations(db: DatabaseManager, dataObject: DataObject) {
  await disconnectSingleDataObjectRelation(db, new Channel(), 'avatarPhoto', dataObject)
  await disconnectSingleDataObjectRelation(db, new Channel(), 'coverPhoto', dataObject)

  await disconnectSingleDataObjectRelation(db, new Video(), 'thumbnailPhoto', dataObject)
  await disconnectSingleDataObjectRelation(db, new Video(), 'media', dataObject)
}

async function disconnectSingleDataObjectRelation<T extends Channel | Video>(
  db: DatabaseManager,
  type: T,
  propertyName: string,
  dataObject: DataObject
) {
  // prepare lookup condition
  const condition = {
    where: {
      [propertyName + 'DataObject']: dataObject,
    },
  } // as FindConditions<T>

  // NOTE: we don't need to retrieve multiple channels/videos via `db.getMany()` because dataObject
  //       is allowed to be associated only with one channel/video in runtime

  // in therory the following condition(s) can be generalized `... db.get(type, ...` but in practice it doesn't work :-\
  const item = type instanceof Channel ? await db.get(Channel, condition) : await db.get(Video, condition)

  // escape when no dataObject association found
  if (!item) {
    return
  }

  item[propertyName + 'Availability'] = AssetAvailability.INVALID
  item[propertyName + 'DataObject'] = null

  if (type instanceof Channel) {
    await db.save<Channel>(item)

    // emit log event
    logger.info('Content has been disconnected from Channel', {
      channelId: item.id.toString(),
      joystreamContentId: dataObject.joystreamContentId,
    })
  } else {
    // type instanceof Video
    await db.save<Video>(item)

    // emit log event
    logger.info('Content has been disconnected from Video', {
      videoId: item.id.toString(),
      joystreamContentId: dataObject.joystreamContentId,
    })
  }
}

/// ///////////////// Helpers ////////////////////////////////////////////////////

function convertStorageObjectOwner(objectOwner: StorageObjectOwner): typeof DataObjectOwner {
  if (objectOwner.isMember) {
    const owner = new DataObjectOwnerMember()
    owner.member = objectOwner.asMember.toNumber()

    return owner
  }

  if (objectOwner.isChannel) {
    const owner = new DataObjectOwnerChannel()
    owner.channel = objectOwner.asChannel.toNumber()

    return owner
  }

  if (objectOwner.isDao) {
    const owner = new DataObjectOwnerDao()
    owner.dao = objectOwner.asDao.toNumber()

    return owner
  }

  if (objectOwner.isCouncil) {
    return new DataObjectOwnerCouncil()
  }

  if (objectOwner.isWorkingGroup) {
    const owner = new DataObjectOwnerWorkingGroup()
    owner.workingGroup = objectOwner.asWorkingGroup.toNumber()

    return owner
  }

  logger.error('Not implemented StorageObjectOwner type', { objectOwner: objectOwner.toString() })
  throw new Error('Not implemented StorageObjectOwner type')
}

function encodeContentId(contentId: ContentId) {
  const customContentId = new Custom_ContentId(registry, contentId)

  return customContentId.encode()
}
