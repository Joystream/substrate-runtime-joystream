import { MemberId, PostId } from '@joystream/types/common'
import { ApplicationId, OpeningId, WorkerId, ApplyOnOpeningParameters } from '@joystream/types/working-group'
import { Event } from '@polkadot/types/interfaces/system'
import { BTreeMap } from '@polkadot/types'
import { MembershipBoughtEvent } from './graphql/generated/schema'
import { ProposalDetails, ProposalId } from '@joystream/types/proposals'
import { CreateInterface } from '@joystream/types'

export type MemberContext = {
  account: string
  memberId: MemberId
}

export type MetadataInput<T> = {
  value: T | string
  expectFailure?: boolean
}

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

// Proposals:

export interface ProposalCreatedEventDetails extends EventDetails {
  proposalId: ProposalId
}

export type ProposalsEngineEventName =
  | 'ProposalCreated'
  | 'ProposalStatusUpdated'
  | 'ProposalDecisionMade'
  | 'ProposalExecuted'
  | 'Voted'
  | 'ProposalCancelled'

export type ProposalsDiscussionEventName =
  | 'ThreadCreated'
  | 'PostCreated'
  | 'PostUpdated'
  | 'ThreadModeChanged'
  | 'PostDeleted'

export interface ProposalDiscussionPostCreatedEventDetails extends EventDetails {
  postId: PostId
}

export type ProposalType = keyof typeof ProposalDetails.typeDefinitions
export type ProposalDetailsJsonByType<T extends ProposalType = ProposalType> = CreateInterface<
  InstanceType<ProposalDetails['typeDefinitions'][T]>
>
