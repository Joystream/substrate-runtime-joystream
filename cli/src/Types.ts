import { Codec } from '@polkadot/types/types'
import { Balance, AccountId } from '@polkadot/types/interfaces'
import { DeriveBalancesAll } from '@polkadot/api-derive/types'
import { KeyringPair } from '@polkadot/keyring/types'
import { WorkerId, OpeningType } from '@joystream/types/working-group'
import { Membership } from '@joystream/types/members'
import { MemberId } from '@joystream/types/common'
import { Validator } from 'inquirer'
import { ApiPromise } from '@polkadot/api'
import { SubmittableModuleExtrinsics, QueryableModuleStorage, QueryableModuleConsts } from '@polkadot/api/types'

// KeyringPair type extended with mandatory "meta.name"
// It's used for accounts/keys management within CLI.
// If not provided in the account json file, the meta.name value is set to "Unnamed Account"
export type NamedKeyringPair = KeyringPair & {
  meta: {
    name: string
  }
}

// Summary of the account information fetched from the api for "account:current" purposes (currently just balances)
export type AccountSummary = {
  balances: DeriveBalancesAll
}

// Object with "name" and "value" properties, used for rendering simple CLI tables like:
// Total balance:   100 JOY
// Free calance:     50 JOY
export type NameValueObj = { name: string; value: string }

// Working groups related types
export enum WorkingGroups {
  StorageProviders = 'storageProviders',
  Curators = 'curators',
  Forum = 'forum',
  Membership = 'membership',
}

// In contrast to Pioneer, currently only StorageProviders group is available in CLI
export const AvailableGroups: readonly WorkingGroups[] = [
  WorkingGroups.StorageProviders,
  WorkingGroups.Curators,
  WorkingGroups.Forum,
  WorkingGroups.Membership,
] as const

export type Reward = {
  totalMissed?: Balance
  valuePerBlock?: Balance
}

// Compound working group types
export type GroupMember = {
  workerId: WorkerId
  memberId: MemberId
  roleAccount: AccountId
  stakingAccount: AccountId
  profile: MemberDetails
  stake: Balance
  reward: Reward
}

export type ApplicationDetails = {
  applicationId: number
  member: MemberDetails
  roleAccout: AccountId
  stakingAccount: AccountId
  rewardAccount: AccountId
  descriptionHash: string
  openingId: number
}

export type OpeningDetails = {
  openingId: number
  stake: {
    value: Balance
    unstakingPeriod: number
  }
  applications: ApplicationDetails[]
  type: OpeningType
  createdAtBlock: number
  rewardPerBlock?: Balance
}

// Extended membership information (including optional query node data)
export type MemberDetails = {
  id: MemberId
  name?: string | null
  handle?: string
  membership: Membership
}

// Api-related

// Additional options that can be passed to ApiCommandBase.promptForParam in order to override
// its default behaviour, change param name, add validation etc.
export type ApiParamOptions<ParamType = Codec> = {
  forcedName?: string
  value?: {
    default: ParamType
    locked?: boolean
  }
  validator?: Validator
  nestedOptions?: ApiParamsOptions // For more complex params, like structs
}
export type ApiParamsOptions = {
  [paramName: string]: ApiParamOptions
}

export type ApiMethodArg = Codec
export type ApiMethodNamedArg = {
  name: string
  value: ApiMethodArg
}
export type ApiMethodNamedArgs = ApiMethodNamedArg[]

// Api without TypeScript augmentations for "query", "tx" and "consts" (useful when more type flexibility is needed)
export type UnaugmentedApiPromise = Omit<ApiPromise, 'query' | 'tx' | 'consts'> & {
  query: { [key: string]: QueryableModuleStorage<'promise'> }
  tx: { [key: string]: SubmittableModuleExtrinsics<'promise'> }
  consts: { [key: string]: QueryableModuleConsts }
}
