import { Text, u32, Tuple, u8, u128, Vec, Option, Null, Bytes } from '@polkadot/types'
import { BlockNumber, Balance } from '@polkadot/types/interfaces'
import { Constructor, ITuple } from '@polkadot/types/types'
import { AccountId, MemberId, WorkingGroup, JoyEnum, JoyStructDecorated, BalanceKind, PostId } from './common'
import { ApplicationId, OpeningId, StakePolicy, WorkerId } from './working-group'

export type IVotingResults = {
  abstensions: u32
  approvals: u32
  rejections: u32
  slashes: u32
}

export class VotingResults extends JoyStructDecorated({
  abstensions: u32,
  approvals: u32,
  rejections: u32,
  slashes: u32,
}) {}

export type ProposalParametersType = {
  // During this period, votes can be accepted
  votingPeriod: BlockNumber
  /* A pause before execution of the approved proposal. Zero means approved proposal would be
     executed immediately. */
  gracePeriod: BlockNumber
  // Quorum percentage of approving voters required to pass the proposal.
  approvalQuorumPercentage: u32
  // Approval votes percentage threshold to pass the proposal.
  approvalThresholdPercentage: u32
  // Quorum percentage of voters required to slash the proposal.
  slashingQuorumPercentage: u32
  // Slashing votes percentage threshold to slash the proposal.
  slashingThresholdPercentage: u32
  // Proposal stake
  requiredStake: Option<Balance>
  // The number of councils in that must approve the proposal in a row before it has its
  // intended effect. Integer no less than 1.
  constitutionality: u32
}

export class ProposalParameters
  extends JoyStructDecorated({
    votingPeriod: u32,
    gracePeriod: u32,
    approvalQuorumPercentage: u32,
    approvalThresholdPercentage: u32,
    slashingQuorumPercentage: u32,
    slashingThresholdPercentage: u32,
    requiredStake: Option.with(u128),
    constitutionality: u32,
  })
  implements ProposalParametersType {}

export type IProposal = {
  parameters: ProposalParameters
  proposerId: MemberId
  activatedAt: BlockNumber
  status: ProposalStatus
  votingResults: VotingResults
  exactExecutionBlock: Option<BlockNumber>
  nrOfCouncilConfirmations: u32
  stakingAccountId: Option<AccountId>
}

export const IProposalStatus: { [key: string]: string } = {
  Active: 'Active',
  Canceled: 'Canceled',
  Expired: 'Expired',
  Approved: 'Approved',
  Rejected: 'Rejected',
  Vetoed: 'Vetoed',
  PendingExecution: 'PendingExecution',
  Executed: 'Executed',
  ExecutionFailed: 'ExecutionFailed',
  Finalized: 'Finalized',
  Slashed: 'Slashed',
}

export class ExecutionFailedStatus extends JoyStructDecorated({
  error: Text,
}) {}

export class ExecutionFailed extends ExecutionFailedStatus {}

export const ApprovedProposalDecisionDef = {
  PendingExecution: Null,
  PendingConstitutionality: Null,
} as const
export type ApprovedProposalDecisions = keyof typeof ApprovedProposalDecisionDef
export class ApprovedProposalDecision extends JoyEnum(ApprovedProposalDecisionDef) {}

export class Approved extends ApprovedProposalDecision {}

export const ProposalDecisionDef = {
  Canceled: Null,
  CanceledByRuntime: Null,
  Vetoed: Null,
  Rejected: Null,
  Slashed: Null,
  Expired: Null,
  Approved,
} as const
export type ProposalDecisions = keyof typeof ProposalDecisionDef
export class ProposalDecision extends JoyEnum(ProposalDecisionDef) {}

export class Active extends Null {}
export class PendingExecution extends u32 {}
export class PendingConstitutionality extends Null {}

export class ProposalStatus extends JoyEnum({
  Active,
  PendingExecution,
  PendingConstitutionality,
} as const) {}

export class ExecutionStatus extends JoyEnum({
  Executed: Null,
  ExecutionFailed,
} as const) {}

export const VoteKinds = ['Approve', 'Reject', 'Slash', 'Abstain'] as const
export type VoteKindKey = typeof VoteKinds[number]
export const VoteKindDef = {
  Approve: Null,
  Reject: Null,
  Slash: Null,
  Abstain: Null,
} as const
export class VoteKind extends JoyEnum(VoteKindDef) {}

export type ProposalVotes = [MemberId, VoteKind][]

export class ProposalId extends u32 {}

export class SpendingParams extends Tuple.with(['Balance', 'AccountId']) {}

export class SetLeadParams extends Tuple.with([MemberId, AccountId]) {}

export class SetLead extends Option.with(SetLeadParams) {}

export class Proposal
  // FIXME: Snake case for consistency?
  extends JoyStructDecorated({
    parameters: ProposalParameters,
    proposerId: MemberId,
    activatedAt: u32,
    status: ProposalStatus,
    votingResults: VotingResults,
    exactExecutionBlock: Option.with(u32),
    nrOfCouncilConfirmations: u32,
    stakingAccountId: Option.with(AccountId),
  })
  implements IProposal {}

export type IProposalCreationParameters = {
  account_id: AccountId
  proposer_id: MemberId
  proposal_parameters: ProposalParameters
  title: Text
  description: Text
  staking_account_id: Option<AccountId>
  encoded_dispatchable_call_code: Vec<u8>
  exact_execution_block: Option<u32>
}

export class ProposalCreationParameters extends JoyStructDecorated({
  account_id: AccountId,
  proposer_id: MemberId,
  proposal_parameters: ProposalParameters,
  title: Text,
  description: Text,
  staking_account_id: Option.with(AccountId),
  encoded_dispatchable_call_code: Vec.with(u8),
  exact_execution_block: Option.with(u32),
}) {}

export type IGeneralProposalParameters = {
  member_id: MemberId
  title: Text
  description: Text
  staking_account_id: Option<AccountId>
  exact_execution_block: Option<BlockNumber>
}

export class GeneralProposalParameters
  extends JoyStructDecorated({
    member_id: MemberId,
    title: Text,
    description: Text,
    staking_account_id: Option.with(AccountId),
    exact_execution_block: Option.with(u32),
  })
  implements IGeneralProposalParameters {}

export type ICreateOpeningParameters = {
  description: Bytes
  stake_policy: StakePolicy
  reward_per_block: Option<Balance>
  working_group: WorkingGroup
}

export class CreateOpeningParameters
  extends JoyStructDecorated({
    description: Bytes,
    stake_policy: StakePolicy,
    reward_per_block: Option.with(u128),
    working_group: WorkingGroup,
  })
  implements ICreateOpeningParameters {}

export type IFillOpeningParameters = {
  opening_id: OpeningId
  successful_application_id: ApplicationId
  working_group: WorkingGroup
}

export class FillOpeningParameters
  extends JoyStructDecorated({
    opening_id: OpeningId,
    successful_application_id: ApplicationId,
    working_group: WorkingGroup,
  })
  implements IFillOpeningParameters {}

export type ITerminateRoleParameters = {
  worker_id: WorkerId
  slashing_amount: Option<Balance>
  working_group: WorkingGroup
}

export class TerminateRoleParameters
  extends JoyStructDecorated({
    worker_id: WorkerId,
    slashing_amount: Option.with(u128),
    working_group: WorkingGroup,
  })
  implements ITerminateRoleParameters {}

export type IFundingRequestParameters = {
  account: AccountId
  amount: Balance
}

export class FundingRequestParameters
  extends JoyStructDecorated({
    account: AccountId,
    amount: u128,
  })
  implements IFundingRequestParameters {}

// Typesafe tuple workarounds
const UpdateWorkingGroupBudget = (Tuple.with(['Balance', WorkingGroup, BalanceKind]) as unknown) as Constructor<
  ITuple<[Balance, WorkingGroup, BalanceKind]>
>
const DecreaseWorkingGroupLeadStake = (Tuple.with([WorkerId, 'Balance', WorkingGroup]) as unknown) as Constructor<
  ITuple<[WorkerId, Balance, WorkingGroup]>
>
const SlashWorkingGroupLead = (Tuple.with([WorkerId, 'Balance', WorkingGroup]) as unknown) as Constructor<
  ITuple<[WorkerId, Balance, WorkingGroup]>
>
const SetWorkingGroupLeadReward = (Tuple.with([WorkerId, 'Option<Balance>', WorkingGroup]) as unknown) as Constructor<
  ITuple<[WorkerId, Option<Balance>, WorkingGroup]>
>
const CancelWorkingGroupLeadOpening = (Tuple.with([OpeningId, WorkingGroup]) as unknown) as Constructor<
  ITuple<[OpeningId, WorkingGroup]>
>
const CreateBlogPost = (Tuple.with([Text, Text]) as unknown) as Constructor<ITuple<[Text, Text]>>
const EditBlogPost = (Tuple.with([PostId, 'Option<Text>', 'Option<Text>']) as unknown) as Constructor<
  ITuple<[PostId, Option<Text>, Option<Text>]>
>

export class ProposalDetails extends JoyEnum({
  Signal: Text,
  RuntimeUpgrade: Bytes,
  FundingRequest: Vec.with(FundingRequestParameters),
  SetMaxValidatorCount: u32,
  CreateWorkingGroupLeadOpening: CreateOpeningParameters,
  FillWorkingGroupLeadOpening: FillOpeningParameters,
  UpdateWorkingGroupBudget,
  DecreaseWorkingGroupLeadStake,
  SlashWorkingGroupLead,
  SetWorkingGroupLeadReward,
  TerminateWorkingGroupLead: TerminateRoleParameters,
  AmendConstitution: Text,
  CancelWorkingGroupLeadOpening,
  SetMembershipPrice: u128,
  SetCouncilBudgetIncrement: u128,
  SetCouncilorReward: u128,
  SetInitialInvitationBalance: u128,
  SetInitialInvitationCount: u32,
  SetMembershipLeadInvitationQuota: u32,
  SetReferralCut: u8,
  CreateBlogPost,
  EditBlogPost,
  LockBlogPost: PostId,
  UnlockBlogPost: PostId,
  VetoProposal: ProposalId,
} as const) {}

// Discussions

export class ThreadAuthorId extends MemberId {}
export class PostAuthorId extends MemberId {}

export class ThreadMode extends JoyEnum({
  Open: Null,
  Closed: Vec.with(MemberId),
} as const) {}

export class DiscussionThread extends JoyStructDecorated({
  activated_at: u32, // BlockNumber
  author_id: ThreadAuthorId,
  mode: ThreadMode,
}) {}

export class DiscussionPost extends JoyStructDecorated({
  author_id: PostAuthorId,
}) {}

// export default proposalTypes;
export const proposalsTypes = {
  ProposalId,
  ProposalStatus,
  ProposalOf: Proposal,
  ProposalDetails,
  ProposalDetailsOf: ProposalDetails, // Runtime alias
  VotingResults,
  ProposalParameters,
  GeneralProposalParameters,
  VoteKind,
  DiscussionThread,
  DiscussionPost,
  CreateOpeningParameters,
  FillOpeningParameters,
  TerminateRoleParameters,
  ProposalDecision,
  ExecutionFailed,
  Approved,
  SetLeadParams,
  ThreadMode,
  ExecutionStatus,
  FundingRequestParameters,
}

export default proposalsTypes
