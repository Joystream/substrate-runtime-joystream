import { Null, u64, Text, Vec, GenericAccountId as AccountId, BTreeSet } from '@polkadot/types'
import { RegistryTypes } from '@polkadot/types/types'
import { JoyBTreeSet, JoyEnum, JoyStructDecorated, WorkingGroup } from './common'
import { ChannelId } from './content-working-group'
import { MemberId } from './members'

export class DataObjectId extends u64 {}
export class StorageBucketId extends u64 {}

export type StorageBucketsPerBagValueConstraintType = {
  min: u64
  max_min_diff: u64
}

export class StorageBucketsPerBagValueConstraint
  extends JoyStructDecorated({
    min: u64,
    max_min_diff: u64,
  })
  implements StorageBucketsPerBagValueConstraintType {}

export const DynamicBagIdDef = {
  Member: MemberId,
  Channel: ChannelId,
}
export type DynamicBagIdKey = keyof typeof DynamicBagIdDef
export class DynamicBagIdType extends JoyEnum(DynamicBagIdDef) {}

// Runtime alias
export class DynamicBagId extends DynamicBagIdType {}

// TODO: implement these types:
export class DynamicBag extends u64 {}
export class StaticBag extends u64 {}
export class StorageBucket extends u64 {}
//

export type DynamicBagCreationPolicyType = {
  numberOfStorageBuckets: u64
}

export class DynamicBagCreationPolicy
  extends JoyStructDecorated({
    numberOfStorageBuckets: u64,
  })
  implements DynamicBagCreationPolicyType {}

export const DynamicBagTypeDef = {
  Member: Null,
  Channel: Null,
} as const
export type DynamicBagTypeKey = keyof typeof DynamicBagTypeDef
export class DynamicBagType extends JoyEnum(DynamicBagTypeDef) {}

export const StaticBagIdDef = {
  Council: Null,
  WorkingGroup: WorkingGroup,
} as const
export type StaticBagIdKey = keyof typeof StaticBagIdDef
export class StaticBagId extends JoyEnum(StaticBagIdDef) {}

export class Static extends StaticBagId {}

export const BagIdDef = {
  Static,
  Dynamic: DynamicBagIdType,
} as const
export type BagIdKey = keyof typeof BagIdDef
export class BagId extends JoyEnum(BagIdDef) {}

// Alias
export class BagIdType extends BagId {}

export type VoucherType = {
  sizeLimit: u64
  objectsLimit: u64
  sizeUsed: u64
  objectsUsed: u64
}

export class Voucher
  extends JoyStructDecorated({
    sizeLimit: u64,
    objectsLimit: u64,
    sizeUsed: u64,
    objectsUsed: u64,
  })
  implements VoucherType {}

export class StorageBucketIdSet extends JoyBTreeSet(StorageBucketId) {}

export class DataObjectIdSet extends JoyBTreeSet(DataObjectId) {}

export type DataObjectCreationParametersType = {
  size: u64
  ipfsContentId: Text
}

export class DataObjectCreationParameters
  extends JoyStructDecorated({
    size: u64,
    ipfsContentId: Text,
  })
  implements DataObjectCreationParametersType {}

export type UploadParametersType = {
  authenticationKey: Text
  bagId: BagId
  objectCreationList: Vec<DataObjectCreationParameters>
  deletionPrizeSourceAccountId: AccountId
}

export class UploadParameters
  extends JoyStructDecorated({
    authenticationKey: Text,
    bagId: BagId,
    objectCreationList: Vec.with(DataObjectCreationParameters),
    deletionPrizeSourceAccountId: AccountId,
  })
  implements UploadParametersType {}

export class ContentId extends Text {}
export class ContentIdSet extends BTreeSet.with(ContentId) {}

export const storageTypes: RegistryTypes = {
  StorageBucketId,
  StorageBucketsPerBagValueConstraint,
  DataObjectId,
  DynamicBagIdType,
  DynamicBagId,
  Voucher,
  DynamicBagType,
  DynamicBagCreationPolicy,
  DynamicBag,
  StaticBag,
  StorageBucket,
  StaticBagId,
  Static,
  BagId,
  DataObjectCreationParameters,
  BagIdType,
  UploadParameters,
  StorageBucketIdSet,
  DataObjectIdSet,
  ContentIdSet,
  ContentId,
}
export default storageTypes
