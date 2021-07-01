import { SubstrateEvent } from '@dzlzv/hydra-common'
import { Network } from 'query-node/dist/src/modules/enums/enums'
import { Event } from 'query-node/dist/src/modules/event/event.model'
import { Bytes } from '@polkadot/types'
import { WorkingGroup, ProposalId, ThreadId } from '@joystream/types/augment/all'
import { BaseModel } from 'warthog'

export const CURRENT_NETWORK = Network.OLYMPIA

// FIXME: See issues like: https://github.com/Joystream/joystream/issues/2457
type MappingsMemoryCache = {
  lastCreatedProposalId?: ProposalId
  lastCreatedProposalThreadId?: ThreadId
}

export const MemoryCache: MappingsMemoryCache = {}

export function genericEventFields(substrateEvent: SubstrateEvent): Partial<BaseModel & Event> {
  const { blockNumber, indexInBlock, extrinsic, blockTimestamp } = substrateEvent
  const eventTime = new Date(blockTimestamp)
  return {
    createdAt: eventTime,
    updatedAt: eventTime,
    id: `${CURRENT_NETWORK}-${blockNumber}-${indexInBlock}`,
    inBlock: blockNumber,
    network: CURRENT_NETWORK,
    inExtrinsic: extrinsic?.hash,
    indexInBlock,
  }
}

type AnyMessage<T> = T & {
  toJSON(): Record<string, unknown>
}

type AnyMetadataClass<T> = {
  name: string
  decode(binary: Uint8Array): AnyMessage<T>
  encode(obj: T): { finish(): Uint8Array }
  toObject(obj: AnyMessage<T>): Record<string, unknown>
}

export function deserializeMetadata<T>(metadataType: AnyMetadataClass<T>, metadataBytes: Bytes): T | null {
  try {
    // We use `toObject()` to get rid of .prototype defaults for optional fields
    return metadataType.toObject(metadataType.decode(metadataBytes.toU8a(true))) as T
  } catch (e) {
    console.error(`Cannot deserialize ${metadataType.name}! Provided bytes: (${metadataBytes.toHex()})`)
    return null
  }
}

export function bytesToString(b: Bytes): string {
  return (
    Buffer.from(b.toU8a(true))
      .toString()
      // eslint-disable-next-line no-control-regex
      .replace(/\u0000/g, '')
  )
}

export function perpareString(s: string): string {
  // eslint-disable-next-line no-control-regex
  return s.replace(/\u0000/g, '')
}

export function hasValuesForProperties<
  T extends Record<string, unknown>,
  P extends keyof T & string,
  PA extends readonly P[]
>(obj: T, props: PA): obj is T & { [K in PA[number]]: NonNullable<K> } {
  props.forEach((p) => {
    if (obj[p] === null || obj[p] === undefined) {
      return false
    }
  })
  return true
}

export type WorkingGroupModuleName =
  | 'storageWorkingGroup'
  | 'contentDirectoryWorkingGroup'
  | 'forumWorkingGroup'
  | 'membershipWorkingGroup'

export function getWorkingGroupModuleName(group: WorkingGroup): WorkingGroupModuleName {
  if (group.isContent) {
    return 'contentDirectoryWorkingGroup'
  } else if (group.isMembership) {
    return 'membershipWorkingGroup'
  } else if (group.isForum) {
    return 'forumWorkingGroup'
  } else if (group.isStorage) {
    return 'storageWorkingGroup'
  }

  throw new Error(`Unsupported working group: ${group.type}`)
}
