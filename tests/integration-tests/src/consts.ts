import { WorkingGroup } from '@joystream/types/common'
import { AugmentedConsts } from '@polkadot/api/types'
import BN from 'bn.js'
import { ProposalType, WorkingGroupModuleName } from './types'

// Dummy const type validation function (see: https://stackoverflow.com/questions/57069802/as-const-is-ignored-when-there-is-a-type-definition)
export const validateType = <T>(obj: T) => obj

// Test chain blocktime
export const BLOCKTIME = 1000

export const MINIMUM_STAKING_ACCOUNT_BALANCE = 200
export const MIN_APPLICATION_STAKE = new BN(2000)
export const MIN_UNSTANKING_PERIOD = 43201
export const LEADER_OPENING_STAKE = new BN(2000)
export const PROPOSALS_POST_DEPOSIT = new BN(2000)
export const ALL_BYTES = '0x' + Array.from({ length: 256 }, (v, i) => Buffer.from([i]).toString('hex')).join('')

export const lockIdByWorkingGroup: { [K in WorkingGroupModuleName]: string } = {
  storageWorkingGroup: '0x0606060606060606',
  contentDirectoryWorkingGroup: '0x0707070707070707',
  forumWorkingGroup: '0x0808080808080808',
  membershipWorkingGroup: '0x0909090909090909',
}

export const workingGroups: WorkingGroupModuleName[] = [
  'storageWorkingGroup',
  'contentDirectoryWorkingGroup',
  'forumWorkingGroup',
  'membershipWorkingGroup',
]

export function getWorkingGroupModuleName(group: WorkingGroup): WorkingGroupModuleName {
  if (group.isOfType('Content')) {
    return 'contentDirectoryWorkingGroup'
  } else if (group.isOfType('Membership')) {
    return 'membershipWorkingGroup'
  } else if (group.isOfType('Forum')) {
    return 'forumWorkingGroup'
  } else if (group.isOfType('Storage')) {
    return 'storageWorkingGroup'
  }

  throw new Error(`Unsupported working group: ${group}`)
}

// Proposals

export const proposalTypeToProposalParamsKey = {
  'AmendConstitution': 'amendConstitutionProposalParameters',
  'CancelWorkingGroupLeadOpening': 'cancelWorkingGroupLeadOpeningProposalParameters',
  'CreateBlogPost': 'createBlogPostProposalParameters',
  'CreateWorkingGroupLeadOpening': 'createWorkingGroupLeadOpeningProposalParameters',
  'DecreaseWorkingGroupLeadStake': 'decreaseWorkingGroupLeadStakeProposalParameters',
  'EditBlogPost': 'editBlogPostProoposalParamters',
  'FillWorkingGroupLeadOpening': 'fillWorkingGroupOpeningProposalParameters',
  'FundingRequest': 'fundingRequestProposalParameters',
  'LockBlogPost': 'lockBlogPostProposalParameters',
  'RuntimeUpgrade': 'runtimeUpgradeProposalParameters',
  'SetCouncilBudgetIncrement': 'setCouncilBudgetIncrementProposalParameters',
  'SetCouncilorReward': 'setCouncilorRewardProposalParameters',
  'SetInitialInvitationBalance': 'setInitialInvitationBalanceProposalParameters',
  'SetInitialInvitationCount': 'setInvitationCountProposalParameters',
  'SetMaxValidatorCount': 'setMaxValidatorCountProposalParameters',
  'SetMembershipLeadInvitationQuota': 'setMembershipLeadInvitationQuotaProposalParameters',
  'SetMembershipPrice': 'setMembershipPriceProposalParameters',
  'SetReferralCut': 'setReferralCutProposalParameters',
  'SetWorkingGroupLeadReward': 'setWorkingGroupLeadRewardProposalParameters',
  'Signal': 'signalProposalParameters',
  'SlashWorkingGroupLead': 'slashWorkingGroupLeadProposalParameters',
  'TerminateWorkingGroupLead': 'terminateWorkingGroupLeadProposalParameters',
  'UnlockBlogPost': 'unlockBlogPostProposalParameters',
  'UpdateWorkingGroupBudget': 'updateWorkingGroupBudgetProposalParameters',
  'VetoProposal': 'vetoProposalProposalParameters',
} as const

type ProposalTypeToProposalParamsKeyMap = { [K in ProposalType]: keyof AugmentedConsts<'promise'>['proposalsCodex'] }
validateType<ProposalTypeToProposalParamsKeyMap>(proposalTypeToProposalParamsKey)
