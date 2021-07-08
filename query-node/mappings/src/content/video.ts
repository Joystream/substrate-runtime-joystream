import BN from 'bn.js'
import { fixBlockTimestamp } from '../eventFix'
import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { FindConditions, In } from 'typeorm'

import {
  Content,
} from '../../../generated/types'

import {
  inconsistentState,
  logger,
  createPredictableId,
} from '../common'

import {
  convertContentActorToDataObjectOwner,
  readProtobuf,
  readProtobufWithAssets,
  RawVideoMetadata,
} from './utils'

// primary entities
import {
  AssetAvailability,
  Channel,
  Video,
  VideoCategory,
  VideoMediaEncoding,
  VideoMediaMetadata,
} from 'query-node'

// secondary entities
import { License } from 'query-node'

// Joystream types
import {
  ChannelId,
} from '@joystream/types/augment'

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoCategoryCreated(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {
    videoCategoryId,
    videoCategoryCreationParameters,
    contentActor,
  } = new Content.VideoCategoryCreatedEvent(event).data

  // read metadata
  const protobufContent = await readProtobuf(
    new VideoCategory(),
    {
      metadata: videoCategoryCreationParameters.meta,
      db,
      event,
    }
  )

  // create new video category
  const videoCategory = new VideoCategory({
    // main data
    id: videoCategoryId.toString(),
    videos: [],
    createdInBlock: event.blockNumber,

    // fill in auto-generated fields
    createdAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),
    updatedAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),

    // integrate metadata
    ...protobufContent
  })

  // save video category
  await db.save<VideoCategory>(videoCategory)

  // emit log event
  logger.info('Video category has been created', {id: videoCategoryId})
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoCategoryUpdated(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {
    videoCategoryId,
    videoCategoryUpdateParameters,
    contentActor,
  } = new Content.VideoCategoryUpdatedEvent(event).data

  // load video category
  const videoCategory = await db.get(VideoCategory, { where: { id: videoCategoryId.toString() } as FindConditions<VideoCategory> })

  // ensure video category exists
  if (!videoCategory) {
    return inconsistentState('Non-existing video category update requested', videoCategoryId)
  }

  // read metadata
  const protobufContent = await readProtobuf(
    new VideoCategory(),
    {
      metadata: videoCategoryUpdateParameters.new_meta,
      db,
      event,
    }
  )

  // update all fields read from protobuf
  for (let [key, value] of Object.entries(protobufContent)) {
    videoCategory[key] = value
  }

  // set last update time
  videoCategory.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

  // save video category
  await db.save<VideoCategory>(videoCategory)

  // emit log event
  logger.info('Video category has been updated', {id: videoCategoryId})
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoCategoryDeleted(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {videoCategoryId} = new Content.VideoCategoryDeletedEvent(event).data

  // load video category
  const videoCategory = await db.get(VideoCategory, { where: { id: videoCategoryId.toString() } as FindConditions<VideoCategory> })

  // ensure video category exists
  if (!videoCategory) {
    return inconsistentState('Non-existing video category deletion requested', videoCategoryId)
  }

  // remove video category
  await db.remove<VideoCategory>(videoCategory)

  // emit log event
  logger.info('Video category has been deleted', {id: videoCategoryId})
}

/////////////////// Video //////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoCreated(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {
    channelId,
    videoId,
    videoCreationParameters,
    contentActor,
  } = new Content.VideoCreatedEvent(event).data

  // read metadata
  const protobufContent = await readProtobufWithAssets(
    new Video(),
    {
      metadata: videoCreationParameters.meta,
      db,
      event,
      assets: videoCreationParameters.assets,
      contentOwner: convertContentActorToDataObjectOwner(contentActor, channelId.toNumber()),
    }
  )

  // load channel
  const channel = await db.get(Channel, { where: { id: channelId.toString() } as FindConditions<Channel> })

  // ensure channel exists
  if (!channel) {
    return inconsistentState('Trying to add video to non-existing channel', channelId)
  }

  // prepare video media metadata (if any)
  const fixedProtobuf = await integrateVideoMediaMetadata(db, null, protobufContent, event)

  const licenseIsEmpty = fixedProtobuf.license && !Object.keys(fixedProtobuf.license).length
  if (licenseIsEmpty) { // license deletion was requested - ignore it and consider it empty
    delete fixedProtobuf.license
  }

  // create new video
  const video = new Video({
    // main data
    id: videoId.toString(),
    isCensored: false,
    channel,
    createdInBlock: event.blockNumber,
    isFeatured: false,

    // default values for properties that might or might not be filled by metadata
    thumbnailPhotoUrls: [],
    thumbnailPhotoAvailability: AssetAvailability.INVALID,
    mediaUrls: [],
    mediaAvailability: AssetAvailability.INVALID,


    // fill in auto-generated fields
    createdAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),
    updatedAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),

    // integrate metadata
    ...fixedProtobuf
  })

  // save video
  await db.save<Video>(video)

  // emit log event
  logger.info('Video has been created', {id: videoId})
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoUpdated(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {
    videoId,
    videoUpdateParameters,
    contentActor,
  } = new Content.VideoUpdatedEvent(event).data

  // load video
  const video = await db.get(Video, { where: { id: videoId.toString() } as FindConditions<Video>, relations: ['channel', 'license'] })

  // ensure video exists
  if (!video) {
    return inconsistentState('Non-existing video update requested', videoId)
  }

  // prepare changed metadata
  const newMetadata = videoUpdateParameters.new_meta.unwrapOr(null)

  // license must be deleted AFTER video is saved - plan a license deletion by assigning it to this variable
  let licenseToDelete: License | null = null

  // update metadata if it was changed
  if (newMetadata) {
    const protobufContent = await readProtobufWithAssets(
      new Video(),
      {
        metadata: newMetadata,
        db,
        event,
        assets: videoUpdateParameters.assets.unwrapOr([]),
        contentOwner: convertContentActorToDataObjectOwner(contentActor, (new BN(video.channel.id)).toNumber()),
      }
    )

    // prepare video media metadata (if any)
    const fixedProtobuf = await integrateVideoMediaMetadata(db, video, protobufContent, event)

    // remember original license
    const originalLicense = video.license

    // update all fields read from protobuf
    for (let [key, value] of Object.entries(fixedProtobuf)) {
      video[key] = value
    }

    // license has changed - plan old license delete
    if (originalLicense && video.license != originalLicense) {
      ([video.license, licenseToDelete] = handleLicenseUpdate(originalLicense, video.license))
    } else if (!Object.keys(video.license || {}).length) { // license deletion was requested event no license exists?
      delete video.license // ensure license is empty
    }
  }

  // set last update time
  video.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

  // save video
  await db.save<Video>(video)

  // delete old license if it's planned
  if (licenseToDelete) {
    await db.remove<License>(licenseToDelete)
  }

  // emit log event
  logger.info('Video has been updated', {id: videoId})
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoDeleted(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {videoId} = new Content.VideoDeletedEvent(event).data

  // load video
  const video = await db.get(Video, { where: { id: videoId.toString() } as FindConditions<Video> })

  // ensure video exists
  if (!video) {
    return inconsistentState('Non-existing video deletion requested', videoId)
  }

  // remove video
  await db.remove<Video>(video)

  // emit log event
  logger.info('Video has been deleted', {id: videoId})
}


// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_VideoCensorshipStatusUpdated(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {videoId, isCensored} = new Content.VideoCensorshipStatusUpdatedEvent(event).data

  // load video
  const video = await db.get(Video, { where: { id: videoId.toString() } as FindConditions<Video> })

  // ensure video exists
  if (!video) {
    return inconsistentState('Non-existing video censoring requested', videoId)
  }

  // update video
  video.isCensored = isCensored.isTrue;

  // set last update time
  video.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

  // save video
  await db.save<Video>(video)

  // emit log event
  logger.info('Video censorship status has been updated', {id: videoId, isCensored: isCensored.isTrue})
}

// eslint-disable-next-line @typescript-eslint/naming-convention
export async function content_FeaturedVideosSet(
  db: DatabaseManager,
  event: SubstrateEvent
) {
  // read event data
  const {videoId: videoIds} = new Content.FeaturedVideosSetEvent(event).data

  // load old featured videos
  const existingFeaturedVideos = await db.getMany(Video, { where: { isFeatured: true } as FindConditions<Video> })

  // comparsion utility
  const isSame = (videoIdA: string) => (videoIdB: string) => videoIdA == videoIdB

  // calculate diff sets
  const toRemove = existingFeaturedVideos.filter(existingFV =>
    !videoIds
      .map(item => item.toHex())
      .some(isSame(existingFV.id))
  )
  const toAdd = videoIds.filter(video =>
    !existingFeaturedVideos
      .map(item => item.id)
      .some(isSame(video.toHex()))
  )

  // mark previously featured videos as not-featured
  for (let video of toRemove) {
    video.isFeatured = false;

    // set last update time
    video.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

    await db.save<Video>(video)
  }

  // escape if no featured video needs to be added
  if (!toAdd) {
    // emit log event
    logger.info('Featured videos unchanged')

    return
  }

  // read videos previously not-featured videos that are meant to be featured
  const videosToAdd = await db.getMany(Video, { where: {
    id: In(toAdd.map(item => item.toString()))
  } as FindConditions<Video> })

  if (videosToAdd.length != toAdd.length) {
    return inconsistentState('At least one non-existing video featuring requested', toAdd)
  }

  // mark previously not-featured videos as featured
  for (let video of videosToAdd) {
    video.isFeatured = true;

    // set last update time
    video.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

    await db.save<Video>(video)
  }

  // emit log event
  logger.info('New featured videos have been set', {videoIds})
}

/////////////////// Helpers ////////////////////////////////////////////////////

/*
  Integrates video metadata-related data into existing data (if any) or creates a new record.

  NOTE: type hack - `RawVideoMetadata` is accepted for `metadata` instead of `Partial<Video>`
        see `prepareVideoMetadata()` in `utils.ts` for more info
*/
async function integrateVideoMediaMetadata(
  db: DatabaseManager,
  existingRecord: Video | null,
  metadata: Partial<Video>,
  event: SubstrateEvent,
): Promise<Partial<Video>> {
  if (!metadata.mediaMetadata) {
    return metadata
  }

  // fix TS type
  const rawMediaMetadata = metadata.mediaMetadata as unknown as RawVideoMetadata

  // ensure encoding object
  const encoding = (existingRecord && existingRecord.mediaMetadata && existingRecord.mediaMetadata.encoding)
    || new VideoMediaEncoding({
        createdById: '1',
        updatedById: '1',
      })

  // integrate media encoding-related data
  rawMediaMetadata.encoding.codecName.integrateInto(encoding, 'codecName')
  rawMediaMetadata.encoding.container.integrateInto(encoding, 'container')
  rawMediaMetadata.encoding.mimeMediaType.integrateInto(encoding, 'mimeMediaType')

  // ensure media metadata object
  const mediaMetadata = (existingRecord && existingRecord.mediaMetadata) || new VideoMediaMetadata({
    createdInBlock: event.blockNumber,

    createdById: '1',
    updatedById: '1',
  })

  // integrate media-related data
  rawMediaMetadata.pixelWidth.integrateInto(mediaMetadata, 'pixelWidth')
  rawMediaMetadata.pixelHeight.integrateInto(mediaMetadata, 'pixelHeight')
  rawMediaMetadata.size.integrateInto(mediaMetadata, 'size')

  // connect encoding to media metadata object
  mediaMetadata.encoding = encoding

  // ensure predictable ids
  if (!mediaMetadata.encoding.id) {
    mediaMetadata.encoding.id = await createPredictableId(db)
  }
  if (!mediaMetadata.id) {
    mediaMetadata.id = await createPredictableId(db)
  }

  return {
    ...metadata,
    mediaMetadata
  }
}

// returns tuple `[newLicenseForVideo, oldLicenseToBeDeleted]`
function handleLicenseUpdate(originalLicense, newLicense): [License | undefined, License | null] {
  const isNewEmpty = !Object.keys(newLicense).length

  if (!originalLicense && isNewEmpty) {
    return [undefined, null]
  }

  if (!originalLicense) { // && !isNewEmpty
    return [newLicense, null]
  }

  if (!isNewEmpty) { // && originalLicense
    return [
      new License({
        ...originalLicense,
        ...newLicense,
      }),
      null
    ]
  }

  // originalLicense && isNewEmpty

  return [originalLicense, null]
}
