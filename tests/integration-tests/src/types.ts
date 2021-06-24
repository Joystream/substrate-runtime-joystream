import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import { ApplicationId, OpeningId, WorkerId, ApplyOnOpeningParameters } from '@joystream/types/working-group'
import { Event } from '@polkadot/types/interfaces/system'
import { BTreeMap } from '@polkadot/types'
import { CategoryId } from '@joystream/types/forum'
import { MembershipBoughtEvent } from './graphql/generated/schema'

export type AnyQueryNodeEvent = Pick<
  MembershipBoughtEvent,
  'createdAt' | 'updatedAt' | 'id' | 'inBlock' | 'inExtrinsic' | 'indexInBlock' | 'network'
>
export interface EventDetails {
  event: Event
  blockNumber: number
  blockTimestamp: number
  blockHash: string
  indexInBlock: number
}

export type MemberContext = {
  account: string
  memberId: MemberId
}

export type MetadataInput<T> = {
  value: T | string
  expectFailure?: boolean
}

// Membership

export interface MembershipBoughtEventDetails extends EventDetails {
  memberId: MemberId
}

export interface MemberInvitedEventDetails extends EventDetails {
  newMemberId: MemberId
}

export type MembershipEventName =
  | 'MembershipBought'
  | 'MemberProfileUpdated'
  | 'MemberAccountsUpdated'
  | 'MemberVerificationStatusUpdated'
  | 'InvitesTransferred'
  | 'MemberInvited'
  | 'StakingAccountAdded'
  | 'StakingAccountConfirmed'
  | 'StakingAccountRemoved'
  | 'InitialInvitationCountUpdated'
  | 'MembershipPriceUpdated'
  | 'ReferralCutUpdated'
  | 'InitialInvitationBalanceUpdated'
  | 'LeaderInvitationQuotaUpdated'

// Working groups

export interface OpeningAddedEventDetails extends EventDetails {
  openingId: OpeningId
}

export interface AppliedOnOpeningEventDetails extends EventDetails {
  applicationId: ApplicationId
  params: ApplyOnOpeningParameters
}

export interface OpeningFilledEventDetails extends EventDetails {
  applicationIdToWorkerIdMap: BTreeMap<ApplicationId, WorkerId>
}

export type WorkingGroupsEventName =
  | 'OpeningAdded'
  | 'AppliedOnOpening'
  | 'OpeningFilled'
  | 'LeaderSet'
  | 'WorkerRoleAccountUpdated'
  | 'LeaderUnset'
  | 'WorkerExited'
  | 'TerminatedWorker'
  | 'TerminatedLeader'
  | 'StakeSlashed'
  | 'StakeDecreased'
  | 'StakeIncreased'
  | 'ApplicationWithdrawn'
  | 'OpeningCanceled'
  | 'BudgetSet'
  | 'WorkerRewardAccountUpdated'
  | 'WorkerRewardAmountUpdated'
  | 'StatusTextChanged'
  | 'BudgetSpending'
  | 'WorkerStartedLeaving'
  | 'RewardPaid'
  | 'NewMissedRewardLevelReached'

export type WorkingGroupModuleName =
  | 'storageWorkingGroup'
  | 'contentDirectoryWorkingGroup'
  | 'forumWorkingGroup'
  | 'membershipWorkingGroup'

// Forum

export type ThreadPath = {
  categoryId: CategoryId
  threadId: ThreadId
}

export type PostPath = {
  categoryId: CategoryId
  threadId: ThreadId
  postId: PostId
}

export interface CategoryCreatedEventDetails extends EventDetails {
  categoryId: CategoryId
}

export interface ThreadCreatedEventDetails extends EventDetails {
  threadId: ThreadId
}

export interface PostAddedEventDetails extends EventDetails {
  postId: PostId
}

export type ForumEventName =
  | 'CategoryCreated'
  | 'CategoryArchivalStatusUpdated'
  | 'CategoryDeleted'
  | 'ThreadCreated'
  | 'ThreadModerated'
  | 'ThreadUpdated'
  | 'ThreadTitleUpdated'
  | 'ThreadDeleted'
  | 'ThreadMoved'
  | 'PostAdded'
  | 'PostModerated'
  | 'PostDeleted'
  | 'PostTextUpdated'
  | 'PostReacted'
  | 'VoteOnPoll'
  | 'CategoryStickyThreadUpdate'
  | 'CategoryMembershipOfModeratorUpdated'
