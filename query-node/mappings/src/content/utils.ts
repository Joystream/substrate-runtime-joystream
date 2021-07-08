// TODO: finish db cascade on save/remove; right now there is manually added `cascade: ["insert", "update"]` directive
//       to all relations in `query-node/generated/graphql-server/src/modules/**/*.model.ts`. That should ensure all records
//       are saved on one `db.save(...)` call. Missing features
//       - find a proper way to cascade on remove or implement custom removals for every entity
//       - convert manual changes done to `*model.ts` file into some patch or bash commands that can be executed
//         every time query node codegen is run (that will overwrite said manual changes)
//       - verify in integration tests that the records are trully created/updated/removed as expected

import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { Bytes } from '@polkadot/types'
import ISO6391 from 'iso-639-1'
import { u64 } from '@polkadot/types/primitive'
import { FindConditions } from 'typeorm'
import * as jspb from "google-protobuf"

// protobuf definitions
import {
  ChannelMetadata,
  ChannelCategoryMetadata,
  PublishedBeforeJoystream as PublishedBeforeJoystreamMetadata,
  License as LicenseMetadata,
  MediaType as MediaTypeMetadata,
  VideoMetadata,
  VideoCategoryMetadata,
} from '@joystream/content-metadata-protobuf'

import {
  Content,
} from '../../../generated/types'

import {
  invalidMetadata,
  inconsistentState,
  logger,
  prepareDataObject,
  createPredictableId,
} from '../common'


import {
  // primary entities
  CuratorGroup,
  Channel,
  ChannelCategory,
  Video,
  VideoCategory,

  // secondary entities
  Language,
  License,
  VideoMediaEncoding,
  VideoMediaMetadata,

  // asset
  DataObjectOwner,
  DataObjectOwnerMember,
  DataObjectOwnerChannel,
  DataObject,
  LiaisonJudgement,
  AssetAvailability,

  Membership,
} from 'query-node'

// Joystream types
import {
  ChannelId,
  ContentParameters,
  NewAsset,
  ContentActor,
} from '@joystream/types/augment'

import { ContentParameters as Custom_ContentParameters } from '@joystream/types/storage'
import { registry } from '@joystream/types'

/*
  Asset either stored in storage or describing list of URLs.
*/
type AssetStorageOrUrls = DataObject | string[]

/*
  Type guard differentiating asset stored in storage from asset describing a list of URLs.
*/
function isAssetInStorage(dataObject: AssetStorageOrUrls): dataObject is DataObject {
  if (Array.isArray(dataObject)) {
    return false
  }

  return true
}

export interface IReadProtobufArguments {
  metadata: Bytes
  db: DatabaseManager
  event: SubstrateEvent
}

export interface IReadProtobufArgumentsWithAssets extends IReadProtobufArguments {
  assets: NewAsset[] // assets provided in event
  contentOwner: typeof DataObjectOwner
}

/*
  This class represents one of 3 possible states when changing property read from metadata.
  NoChange - don't change anything (used when invalid metadata are encountered)
  Unset - unset the value (used when the unset is requested in runtime)
  Change - set the new value
*/
export class PropertyChange<T> {

  static newUnset<T>(): PropertyChange<T> {
    return new PropertyChange<T>('unset')
  }

  static newNoChange<T>(): PropertyChange<T> {
    return new PropertyChange<T>('nochange')
  }

  static newChange<T>(value: T): PropertyChange<T> {
    return new PropertyChange<T>('change', value)
  }

  /*
    Determines property change from the given object property.
  */
  static fromObjectProperty<
    T,
    Key extends string,
    ChangedObject extends {[key in Key]?: T}
  >(object: ChangedObject, key: Key): PropertyChange<T> {
    if (!(key in object)) {
      return PropertyChange.newNoChange<T>()
    }

    if (object[key] === undefined) {
      return PropertyChange.newUnset<T>()
    }

    return PropertyChange.newChange<T>(object[key] as T)
  }

  private type: string
  private value?: T

  private constructor(type: 'change' | 'nochange' | 'unset', value?: T) {
    this.type = type
    this.value = value
  }

  public isUnset(): boolean {
    return this.type == 'unset'
  }

  public isNoChange(): boolean {
    return this.type == 'nochange'
  }

  public isValue(): boolean {
    return this.type == 'change'
  }

  public getValue(): T | undefined {
    return this.type == 'change'
      ? this.value
      : undefined
  }

  /*
    Integrates the value into the given dictionary.
  */
  public integrateInto(object: Object, key: string): void {
    if (this.isNoChange()) {
      return
    }

    if (this.isUnset()) {
      delete object[key]
      return
    }

    object[key] = this.value
  }
}

export interface RawVideoMetadata {
  encoding: {
    codecName: PropertyChange<string>
    container: PropertyChange<string>
    mimeMediaType: PropertyChange<string>
  }
  pixelWidth: PropertyChange<number>
  pixelHeight: PropertyChange<number>
  size: PropertyChange<number>
}

/*
  Reads information from the event and protobuf metadata and constructs changeset that's fit to be used when saving to db.
*/
export async function readProtobuf<T extends ChannelCategory | VideoCategory>(
  type: T,
  parameters: IReadProtobufArguments,
): Promise<Partial<T>> {
  // true option here is crucial, it indicates that we want just the underlying bytes (by default it will also include bytes encoding the length)
  const metaU8a = parameters.metadata.toU8a(true)

  // process channel category
  if (type instanceof ChannelCategory) {
    const meta = ChannelCategoryMetadata.deserializeBinary(metaU8a)
    const result = convertMetadataToObject<ChannelCategoryMetadata.AsObject>(meta) as Partial<T>

    return result
  }

  // process video category
  if (type instanceof VideoCategory) {
    const meta = VideoCategoryMetadata.deserializeBinary(metaU8a)
    const result = convertMetadataToObject<VideoCategoryMetadata.AsObject>(meta) as Partial<T>

    return result
  }

  // this should never happen
  logger.error('Not implemented metadata type', {type})
  throw `Not implemented metadata type`
}

/*
  Reads information from the event and protobuf metadata and constructs changeset that's fit to be used when saving to db.
  In addition it handles any assets associated with the metadata.
*/

export async function readProtobufWithAssets<T extends Channel | Video>(
  type: T,
  parameters: IReadProtobufArgumentsWithAssets,
): Promise<Partial<T>> {
  // true option here is crucial, it indicates that we want just the underlying bytes (by default it will also include bytes encoding the length)
  const metaU8a = parameters.metadata.toU8a(true)

  // process channel
  if (type instanceof Channel) {
    const meta = ChannelMetadata.deserializeBinary(metaU8a)
    const metaAsObject = convertMetadataToObject<ChannelMetadata.AsObject>(meta)
    const result = metaAsObject as any as Partial<Channel>

    // prepare cover photo asset if needed
    if ('coverPhoto' in metaAsObject) {
      const asset = await extractAsset({
        assetIndex: metaAsObject.coverPhoto,
        assets: parameters.assets,
        db: parameters.db,
        event: parameters.event,
        contentOwner: parameters.contentOwner,
      })
      integrateAsset('coverPhoto', result, asset) // changes `result` inline!
      delete metaAsObject.coverPhoto
    }

    // prepare avatar photo asset if needed
    if ('avatarPhoto' in metaAsObject) {
      const asset = await extractAsset({
        assetIndex: metaAsObject.avatarPhoto,
        assets: parameters.assets,
        db: parameters.db,
        event: parameters.event,
        contentOwner: parameters.contentOwner,
      })
      integrateAsset('avatarPhoto', result, asset) // changes `result` inline!
      delete metaAsObject.avatarPhoto
    }

    // prepare language if needed
    if ('language' in metaAsObject) {
      const language = await prepareLanguage(metaAsObject.language, parameters.db, parameters.event)
      delete metaAsObject.language // make sure temporary value will not interfere
      language.integrateInto(result, 'language')
    }

    return result as Partial<T>
  }

  // process video
  if (type instanceof Video) {
    const meta = VideoMetadata.deserializeBinary(metaU8a)
    const metaAsObject = convertMetadataToObject<VideoMetadata.AsObject>(meta)
    const result = metaAsObject as any as Partial<Video>

    // prepare video category if needed
    if ('category' in metaAsObject) {
      const category = await prepareVideoCategory(metaAsObject.category, parameters.db)
      delete metaAsObject.category // make sure temporary value will not interfere
      category.integrateInto(result, 'category')
    }

    // prepare media meta information if needed
    if ('mediaType' in metaAsObject || 'mediaPixelWidth' in metaAsObject || 'mediaPixelHeight' in metaAsObject) {
      // prepare video file size if poosible
      const videoSize = extractVideoSize(parameters.assets, metaAsObject.video)

      // NOTE: type hack - `RawVideoMetadata` is inserted instead of VideoMediaMetadata - it should be edited in `video.ts`
      //       see `integrateVideoMetadata()` in `video.ts` for more info
      result.mediaMetadata = prepareVideoMetadata(
        metaAsObject,
        videoSize,
        parameters.event.blockNumber,
      ) as unknown as VideoMediaMetadata

      // remove extra values
      delete metaAsObject.mediaType
      delete metaAsObject.mediaPixelWidth
      delete metaAsObject.mediaPixelHeight
    }

    // prepare license if needed
    if ('license' in metaAsObject) {
      result.license = await prepareLicense(parameters.db, metaAsObject.license, parameters.event)
    }

    // prepare thumbnail photo asset if needed
    if ('thumbnailPhoto' in metaAsObject) {
      const asset = await extractAsset({
        assetIndex: metaAsObject.thumbnailPhoto,
        assets: parameters.assets,
        db: parameters.db,
        event: parameters.event,
        contentOwner: parameters.contentOwner,
      })
      integrateAsset('thumbnailPhoto', result, asset) // changes `result` inline!
      delete metaAsObject.thumbnailPhoto
    }

    // prepare video asset if needed
    if ('video' in metaAsObject) {
      const asset = await extractAsset({
        assetIndex: metaAsObject.video,
        assets: parameters.assets,
        db: parameters.db,
        event: parameters.event,
        contentOwner: parameters.contentOwner,
      })
      integrateAsset('media', result, asset) // changes `result` inline!
      delete metaAsObject.video
    }

    // prepare language if needed
    if ('language' in metaAsObject) {
      const language = await prepareLanguage(
        metaAsObject.language,
        parameters.db,
        parameters.event,
      )
      delete metaAsObject.language // make sure temporary value will not interfere
      language.integrateInto(result, 'language')
    }

    if (metaAsObject.publishedBeforeJoystream) {
      const publishedBeforeJoystream = handlePublishedBeforeJoystream(result, metaAsObject.publishedBeforeJoystream)
      delete metaAsObject.publishedBeforeJoystream // make sure temporary value will not interfere
      publishedBeforeJoystream.integrateInto(result, 'publishedBeforeJoystream')
    }

    return result as Partial<T>
  }

  // this should never happen
  logger.error('Not implemented metadata type', {type})
  throw `Not implemented metadata type`
}

export async function convertContentActorToChannelOwner(db: DatabaseManager, contentActor: ContentActor): Promise<{
  ownerMember?: Membership,
  ownerCuratorGroup?: CuratorGroup,
}> {
  if (contentActor.isMember) {
    const memberId = contentActor.asMember.toNumber()
    const member = await db.get(Membership, { where: { id: memberId.toString() } as FindConditions<Membership> })

    // ensure member exists
    if (!member) {
      return inconsistentState(`Actor is non-existing member`, memberId)
    }

    return {
      ownerMember: member,
      ownerCuratorGroup: undefined, // this will clear the field
    }
  }

  if (contentActor.isCurator) {
    const curatorGroupId = contentActor.asCurator[0].toNumber()
    const curatorGroup = await db.get(CuratorGroup, { where: { id: curatorGroupId.toString() } as FindConditions<CuratorGroup> })

    // ensure curator group exists
    if (!curatorGroup) {
      return inconsistentState('Actor is non-existing curator group', curatorGroupId)
    }

    return {
      ownerMember: undefined, // this will clear the field
      ownerCuratorGroup: curatorGroup,
    }
  }

  // TODO: contentActor.isLead

  logger.error('Not implemented ContentActor type', {contentActor: contentActor.toString()})
  throw 'Not-implemented ContentActor type used'
}

export function convertContentActorToDataObjectOwner(contentActor: ContentActor, channelId: number): typeof DataObjectOwner {
  const owner = new DataObjectOwnerChannel()
  owner.channel = channelId

  return owner

  /* contentActor is irrelevant now -> all video/channel content belongs to the channel
  if (contentActor.isMember) {
    const owner = new DataObjectOwnerMember()
    owner.member = contentActor.asMember.toBn()

    return owner
  }

  if (contentActor.isLead || contentActor.isCurator) {
    const owner = new DataObjectOwnerChannel()
    owner.channel = channelId

    return owner
  }

  logger.error('Not implemented ContentActor type', {contentActor: contentActor.toString()})
  throw 'Not-implemented ContentActor type used'
  */
}

function handlePublishedBeforeJoystream(video: Partial<Video>, metadata: PublishedBeforeJoystreamMetadata.AsObject): PropertyChange<Date> {
  // is publish being unset
  if ('isPublished' in metadata && !metadata.isPublished) {
    return PropertyChange.newUnset()
  }

  // try to parse timestamp from publish date
  const timestamp = metadata.date
    ? Date.parse(metadata.date)
    : NaN

  // ensure date is valid
  if (isNaN(timestamp)) {
    invalidMetadata(`Invalid date used for publishedBeforeJoystream`, {
      timestamp
    })
    return PropertyChange.newNoChange()
  }

  // set new date
  return PropertyChange.newChange(new Date(timestamp))
}

interface IConvertAssetParameters {
  rawAsset: NewAsset
  db: DatabaseManager
  event: SubstrateEvent
  contentOwner: typeof DataObjectOwner
}

/*
  Converts event asset into data object or list of URLs fit to be saved to db.
*/
async function convertAsset(parameters: IConvertAssetParameters): Promise<AssetStorageOrUrls> {
  // is asset describing list of URLs?
  if (parameters.rawAsset.isUrls) {
    const urls = parameters.rawAsset.asUrls.toArray().map(item => item.toString())

    return urls
  }

  // !parameters.rawAsset.isUrls && parameters.rawAsset.isUpload // asset is in storage

  // prepare data object
  const contentParameters: ContentParameters = parameters.rawAsset.asUpload
  const dataObject = await prepareDataObject(
    parameters.db,
    contentParameters,
    parameters.event,
    parameters.contentOwner,
  )

  return dataObject
}

interface IExtractAssetParameters {
  assetIndex: number | undefined
  assets: NewAsset[]
  db: DatabaseManager
  event: SubstrateEvent
  contentOwner: typeof DataObjectOwner
}

/*
  Selects asset from provided set of assets and prepares asset data fit to be saved to db.
*/
async function extractAsset(parameters: IExtractAssetParameters): Promise<PropertyChange<AssetStorageOrUrls>> {
  // is asset being unset?
  if (parameters.assetIndex === undefined) {
    return PropertyChange.newUnset()
  }

  // ensure asset index is valid
  if (parameters.assetIndex >= parameters.assets.length) {
    invalidMetadata(`Non-existing asset extraction requested`, {
      assetsProvided: parameters.assets.length,
      assetIndex: parameters.assetIndex,
    })
    return PropertyChange.newNoChange()
  }

  // convert asset to data object record
  const asset = await convertAsset({
    rawAsset: parameters.assets[parameters.assetIndex],
    db: parameters.db,
    event: parameters.event,
    contentOwner: parameters.contentOwner,
  })

  return PropertyChange.newChange(asset)
}

/*
  As a temporary messure to overcome yet-to-be-implemented features in Hydra, we are using redudant information
  to describe asset state. This function introduces all redudant data needed to be saved to db.

  Changes `result` argument!
*/
function integrateAsset<T>(propertyName: string, result: Object, asset: PropertyChange<AssetStorageOrUrls>): void {
  // helpers - property names
  const nameUrl = propertyName + 'Urls'
  const nameDataObject = propertyName + 'DataObject'
  const nameAvailability = propertyName + 'Availability'

  if (asset.isNoChange()) {
    return
  }

  if (asset.isUnset()) {
    result[nameUrl] = []
    result[nameAvailability] = AssetAvailability.INVALID
    result[nameDataObject] = undefined // plan deletion (will have effect when saved to db)

    return
  }

  const newValue = asset.getValue() as AssetStorageOrUrls

  // is asset available on external URL(s)
  if (!isAssetInStorage(newValue)) {
    // (un)set asset's properties
    result[nameUrl] = newValue
    result[nameAvailability] = AssetAvailability.ACCEPTED
    result[nameDataObject] = undefined // plan deletion (will have effect when saved to db)

    return
  }

  // asset saved in storage

  // prepare conversion table between liaison judgment and asset availability
  const conversionTable = {
    [LiaisonJudgement.ACCEPTED]: AssetAvailability.ACCEPTED,
    [LiaisonJudgement.PENDING]: AssetAvailability.PENDING,
  }

  // (un)set asset's properties
  result[nameUrl] = [] // plan deletion (will have effect when saved to db)
  result[nameAvailability] = conversionTable[newValue.liaisonJudgement]
  result[nameDataObject] = newValue
}

function extractVideoSize(assets: NewAsset[], assetIndex: number | undefined): number | undefined {
  // escape if no asset is required
  if (assetIndex === undefined) {
    return undefined
  }

  // ensure asset index is valid
  if (assetIndex > assets.length) {
    invalidMetadata(`Non-existing asset video size extraction requested`, {assetsProvided: assets.length, assetIndex})
    return undefined
  }

  const rawAsset = assets[assetIndex]

  // escape if asset is describing URLs (can't get size)
  if (rawAsset.isUrls) {
    return undefined
  }

  // !rawAsset.isUrls && rawAsset.isUpload // asset is in storage

  // convert generic content parameters coming from processor to custom Joystream data type
  const customContentParameters = new Custom_ContentParameters(registry, rawAsset.asUpload.toJSON() as any)
  // extract video size
  const videoSize = customContentParameters.size_in_bytes.toNumber()

  return videoSize
}

async function prepareLanguage(
  languageIso: string | undefined,
  db: DatabaseManager,
  event: SubstrateEvent,
): Promise<PropertyChange<Language>> {
  // is language being unset?
  if (languageIso === undefined) {
    return PropertyChange.newUnset()
  }

  // validate language string
  const isValidIso = ISO6391.validate(languageIso)

  // ensure language string is valid
  if (!isValidIso) {
    invalidMetadata(`Invalid language ISO-639-1 provided`, languageIso)
    return PropertyChange.newNoChange()
  }

  // load language
  const language = await db.get(Language, { where: { iso: languageIso } as FindConditions<Language> })

  // return existing language if any
  if (language) {
    return PropertyChange.newChange(language)
  }


  // create new language
  const newLanguage = new Language({
    id: await createPredictableId(db),
    iso: languageIso,
    createdInBlock: event.blockNumber,

    // TODO: remove these lines after Hydra auto-fills the values when cascading save (remove them on all places)
    createdById: '1',
    updatedById: '1',
  })

  await db.save<Language>(newLanguage)

  return PropertyChange.newChange(newLanguage)
}

async function prepareLicense(
  db: DatabaseManager,
  licenseProtobuf: LicenseMetadata.AsObject | undefined,
  event: SubstrateEvent,
): Promise<License | undefined> {
  // NOTE: Deletion of any previous license should take place in appropriate event handling function
  //       and not here even it might appear so.

  // is license being unset?
  if (licenseProtobuf === undefined) {
    return undefined
  }

  // license is meant to be deleted
  if (isLicenseEmpty(licenseProtobuf)) {
    return new License({})
  }

  // crete new license
  const license = new License({
    ...licenseProtobuf,
    id: await createPredictableId(db),

    createdById: '1',
    updatedById: '1',
  })

  return license
}

/*
  Checks if protobof contains license with some fields filled or is empty object (`{}` or `{someKey: undefined, ...}`).
  Empty object means deletion is requested.
*/
function isLicenseEmpty(licenseObject: LicenseMetadata.AsObject): boolean {
    let somePropertySet = Object.entries(licenseObject).reduce((acc, [key, value]) => {
        return acc || value !== undefined
    }, false)

    return !somePropertySet
}


function prepareVideoMetadata(videoProtobuf: VideoMetadata.AsObject, videoSize: number | undefined, blockNumber: number): RawVideoMetadata {
  const rawMeta = {
    encoding: {
      codecName: PropertyChange.fromObjectProperty<string, 'codecName', MediaTypeMetadata.AsObject>(videoProtobuf.mediaType || {}, 'codecName'),
      container: PropertyChange.fromObjectProperty<string, 'container', MediaTypeMetadata.AsObject>(videoProtobuf.mediaType || {}, 'container'),
      mimeMediaType: PropertyChange.fromObjectProperty<string, 'mimeMediaType', MediaTypeMetadata.AsObject>(videoProtobuf.mediaType || {}, 'mimeMediaType'),
    },
    pixelWidth: PropertyChange.fromObjectProperty<number, 'mediaPixelWidth', VideoMetadata.AsObject>(videoProtobuf, 'mediaPixelWidth'),
    pixelHeight: PropertyChange.fromObjectProperty<number, 'mediaPixelHeight', VideoMetadata.AsObject>(videoProtobuf, 'mediaPixelHeight'),
    size: videoSize === undefined
      ? PropertyChange.newNoChange()
      : PropertyChange.newChange(videoSize)
  } as RawVideoMetadata

  return rawMeta
}

async function prepareVideoCategory(categoryId: number | undefined, db: DatabaseManager): Promise<PropertyChange<VideoCategory>> {
  // is category being unset?
  if (categoryId === undefined) {
    return PropertyChange.newUnset()
  }

  // load video category
  const category = await db.get(VideoCategory, { where: { id: categoryId.toString() } as FindConditions<VideoCategory> })

  // ensure video category exists
  if (!category) {
    invalidMetadata('Non-existing video category association with video requested', categoryId)
    return PropertyChange.newNoChange()
  }

  return PropertyChange.newChange(category)
}

function convertMetadataToObject<T extends Object>(metadata: jspb.Message): T {
  const metaAsObject = metadata.toObject()
  const result = {} as T

  for (const key in metaAsObject) {
    const funcNameBase = key.charAt(0).toUpperCase() + key.slice(1)
    const hasFuncName = 'has' + funcNameBase
    const isSet = funcNameBase == 'PersonsList' // there is no `VideoMetadata.hasPersonsList` method from unkown reason -> create exception
      ? true
      : metadata[hasFuncName]()

    if (!isSet) {
      continue
    }


    const getFuncName = 'get' + funcNameBase
    const value = metadata[getFuncName]()

    // TODO: check that recursion trully works
    if (value instanceof jspb.Message) {
      result[key] = convertMetadataToObject(value)
      continue
    }

    result[key] = metaAsObject[key]
  }

  return result
}
