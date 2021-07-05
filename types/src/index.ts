import { Codec, RegistryTypes } from '@polkadot/types/types'
import common from './common'
import members from './members'
import council from './council'
import roles from './roles'
import forum from './forum'
import stake from './stake'
import mint from './mint'
import recurringRewards from './recurring-rewards'
import hiring from './hiring'
import contentWorkingGroup from './content-working-group'
import workingGroup from './working-group'
import discovery from './discovery'
import media from './media'
import proposals from './proposals'
import contentDirectory from './content-directory'
import storage from './storage'

import { InterfaceTypes } from '@polkadot/types/types/registry'
import { TypeRegistry, Text, UInt, Null, bool, Option, Vec, BTreeSet, BTreeMap } from '@polkadot/types'
import { ExtendedEnum } from './JoyEnum'
import { ExtendedStruct } from './JoyStruct'
import BN from 'bn.js'

export {
  common,
  members,
  council,
  roles,
  forum,
  stake,
  mint,
  recurringRewards,
  hiring,
  contentWorkingGroup,
  workingGroup,
  discovery,
  media,
  proposals,
  contentDirectory,
  storage,
}

export const types: RegistryTypes = {
  ...common,
  ...members,
  ...council,
  ...roles,
  ...forum,
  ...stake,
  ...mint,
  ...recurringRewards,
  ...hiring,
  ...contentWorkingGroup,
  ...workingGroup,
  ...discovery,
  ...media,
  ...proposals,
  ...contentDirectory,
  ...storage,
}

// Allows creating types without api instance (it's not a recommended way though, so should be used just for mocks)
export const registry = new TypeRegistry()
registry.register(types)

// Tweaked version of https://stackoverflow.com/a/62163715 for handling enum variants
// Based on type (T) like: { a: string; b: number; c: Null; }
// will create a type like: { a: string } | { b: number } | { c: Null } | "c"
type EnumVariant<T> = keyof T extends infer K
  ? K extends keyof T
    ? T[K] extends Null
      ? K
      : { [I in K]: T[I] }
    : never
  : never

// Create simple interface for any Codec type (inlcuding JoyEnums and JoyStructs)
// Cannot handle Option here, since that would cause circular reference error
type CreateInterface_NoOption<T extends Codec> =
  | T
  | (T extends ExtendedEnum<infer S>
      ? EnumVariant<{ [K in keyof S]: CreateInterface<InstanceType<T['typeDefinitions'][K]>> }>
      : T extends ExtendedStruct<infer S>
      ? { [K in keyof S]?: CreateInterface<InstanceType<T['typeDefs'][K]>> }
      : T extends Text
      ? string
      : T extends UInt
      ? number | BN
      : T extends bool
      ? boolean
      : T extends Vec<infer S> | BTreeSet<infer S>
      ? CreateInterface<S>[]
      : T extends BTreeMap<infer K, infer V>
      ? Map<K, V>
      : any)

// Wrapper for CreateInterface_NoOption that includes resolving an Option
// (nested Options like Option<Option<Codec>> will resolve to Option<any>, but there are very edge case)
type CreateInterface<T extends Codec> =
  | T
  | (T extends Option<infer S> ? undefined | null | S | CreateInterface_NoOption<S> : CreateInterface_NoOption<T>)

export function createType<TypeName extends keyof InterfaceTypes>(
  type: TypeName,
  value: InterfaceTypes[TypeName] extends Codec ? CreateInterface<InterfaceTypes[TypeName]> : any
): InterfaceTypes[TypeName] {
  return registry.createType(type, value)
}
