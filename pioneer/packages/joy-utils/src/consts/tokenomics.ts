import { ProposalType } from '../types/proposals';
import { GenericProposalType } from '../types/tokenomics';

export const genericTypes: { [k in ProposalType]: GenericProposalType } = {
  Text: 'text',
  Spending: 'spending',
  RuntimeUpgrade: 'networkChanges',
  SetElectionParameters: 'networkChanges',
  SetValidatorCount: 'networkChanges',
  SetLead: 'workingGroups',
  SetContentWorkingGroupMintCapacity: 'workingGroups',
  EvictStorageProvider: 'workingGroups',
  SetStorageRoleParameters: 'workingGroups',
  AddWorkingGroupLeaderOpening: 'workingGroups',
  SetWorkingGroupMintCapacity: 'workingGroups',
  BeginReviewWorkingGroupLeaderApplication: 'workingGroups',
  FillWorkingGroupLeaderOpening: 'workingGroups',
  SlashWorkingGroupLeaderStake: 'workingGroups',
  DecreaseWorkingGroupLeaderStake: 'workingGroups',
  SetWorkingGroupLeaderReward: 'workingGroups',
  TerminateWorkingGroupLeaderRole: 'workingGroups'
};
