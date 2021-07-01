import {
  GenericAccountId,
  Struct,
  Option,
  Text,
  bool,
  u16,
  u32,
  u64,
  Null,
  U8aFixed,
  BTreeSet,
  UInt,
} from '@polkadot/types'
import { BlockNumber, Hash as PolkadotHash, Moment } from '@polkadot/types/interfaces'
import { Codec, Constructor, RegistryTypes } from '@polkadot/types/types'
// we get 'moment' because it is a dependency of @polkadot/util, via @polkadot/keyring
import moment from 'moment'
import { JoyStructCustom, JoyStructDecorated } from './JoyStruct'
import { JoyEnum } from './JoyEnum'

export { JoyEnum, JoyStructCustom, JoyStructDecorated }

// Adds sorting during BTreeSet toU8a encoding (required by the runtime)
// Currently only supports values that extend UInt
// FIXME: Will not cover cases where BTreeSet is part of extrinsic args metadata
export interface ExtendedBTreeSet<V extends UInt> extends BTreeSet<V> {
  toArray(): V[]
}

export function JoyBTreeSet<V extends UInt>(valType: Constructor<V>): Constructor<ExtendedBTreeSet<V>> {
  return class extends BTreeSet.with(valType) {
    public forEach(callbackFn: (value: V, value2: V, set: Set<V>) => void, thisArg?: any): void {
      const sorted = this.toArray()
      return new Set(sorted).forEach(callbackFn, thisArg)
    }

    public toArray() {
      return Array.from(this).sort((a, b) => (a.lt(b) ? -1 : 1))
    }
  }
}

export class ActorId extends u64 {}
export class MemberId extends u64 {}

// Indentical type names for Forum and Proposal Discussions modules
// Ensure they are both configured in runtime to have same type
export class ThreadId extends u64 {}
export class PostId extends u64 {}

// Which module uses this?
export class Hash extends U8aFixed implements PolkadotHash {}

export type BlockAndTimeType = {
  block: BlockNumber
  time: Moment
}

export class BlockAndTime extends JoyStructDecorated({ block: u32, time: u64 }) implements BlockAndTimeType {
  get momentDate(): moment.Moment {
    const YEAR_2000_MILLISECONDS = 946684801000

    // overflowing in ~270,000 years
    const timestamp = this.time.toNumber()

    // TODO: remove once https://github.com/Joystream/joystream/issues/705 is resolved
    // due to a bug, timestamp can be either in seconds or milliseconds
    let timestampInMillis = timestamp
    if (timestamp < YEAR_2000_MILLISECONDS) {
      // timestamp is in seconds
      timestampInMillis = timestamp * 1000
    }

    return moment(timestampInMillis)
  }
}

export function getTextPropAsString(struct: Struct, fieldName: string): string {
  return (struct.get(fieldName) as Text).toString()
}

export function getBoolPropAsBoolean(struct: Struct, fieldName: string): boolean {
  return (struct.get(fieldName) as bool).valueOf()
}

export function getOptionPropOrUndefined<T extends Codec>(struct: Struct, fieldName: string): T | undefined {
  return (struct.get(fieldName) as Option<T>).unwrapOr(undefined)
}

export class OptionText extends Option.with(Text) {}

export type InputValidationLengthConstraintType = {
  min: u16
  max_min_diff: u16
}

export class InputValidationLengthConstraint
  extends JoyStructDecorated({
    min: u16,
    max_min_diff: u16,
  })
  implements InputValidationLengthConstraintType {
  get max(): u16 {
    return this.registry.createType('u16', this.min.add(this.max_min_diff))
  }
}

export const WorkingGroupDef = {
  Forum: Null,
  Storage: Null,
  Content: Null,
  Membership: Null,
} as const
export type WorkingGroupKey = keyof typeof WorkingGroupDef
export class WorkingGroup extends JoyEnum(WorkingGroupDef) {}

export class MemoText extends Text {}

export class BalanceKind extends JoyEnum({
  Positive: Null,
  Negative: Null,
}) {}

// @polkadot/types overrides required since migration to Substrate 2.0,
// see: https://polkadot.js.org/docs/api/FAQ#i-cannot-send-transactions-sending-yields-address-decoding-failures
export class AccountId extends GenericAccountId {}
export class Address extends AccountId {}
export class LookupSource extends AccountId {}

export const commonTypes: RegistryTypes = {
  ActorId,
  MemberId,
  BlockAndTime,
  ThreadId,
  PostId,
  InputValidationLengthConstraint,
  WorkingGroup,
  MemoText,
  BalanceKind,
  // Customize Address type for joystream chain
  Address,
  LookupSource,
}

export default commonTypes
