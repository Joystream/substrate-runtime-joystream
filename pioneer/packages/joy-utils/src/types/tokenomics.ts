export type WorkingGroupTokenomicsData = {
  number: number;
  totalStake: number;
  stakeShare: number;
  rewardsPerWeek: number;
  rewardsShare: number;
  lead: {
    number: number;
    totalStake: number;
    stakeShare: number;
    rewardsPerWeek: number;
    rewardsShare: number;
  };
}

export type TokenomicsData = {
  totalIssuance: number;
  currentlyStakedTokens: number;
  totalWeeklySpending: number;
  totalNumberOfActors: number;
  validators: {
    number: number;
    nominators: {
      number: number;
    };
    rewardsPerWeek: number;
    rewardsShare: number;
    totalStake: number;
    stakeShare: number;
  };
  council: {
    number: number;
    rewardsPerWeek: number;
    rewardsShare: number;
    totalStake: number;
    stakeShare: number;
  };
  storageProviders: WorkingGroupTokenomicsData;
  contentCurators: WorkingGroupTokenomicsData;
  operations: WorkingGroupTokenomicsData;
}

export type StatusServerData = {
  dollarPool: {
    size: number;
    replenishAmount: number;
  };
  price: string;
};

export type NetworkStatisticsData = {
  blockHeight: number;
  numberOfMembers: number;
  content: number;
  numberOfChannels: number;
  proposalCount: number;
  historicalProposals: number;
  numberOfForumCategories: number;
  numberOfForumPosts: number;
  councilMintCapacity: number;
  councilMintSpent: number;
  contentCuratorMintCapacity: number;
  contentCuratorMintSpent: number;
  storageProviderMintCapacity: number;
  storageProviderMintSpent: number;
};

export type ProposalStatistics = {
  all: number;
  Active: number;
  Approved: number;
  Rejected: number;
  Expired: number;
  Slashed: number;
  Canceled: number;
  Vetoed: number;
}

export type ProposalStatisticsData = {
  text: ProposalStatistics;
  spending: ProposalStatistics;
  workingGroups: ProposalStatistics;
  networkChanges: ProposalStatistics;
  all: ProposalStatistics
}

export type GenericProposalType = 'text' | 'spending' | 'networkChanges' | 'workingGroups';
