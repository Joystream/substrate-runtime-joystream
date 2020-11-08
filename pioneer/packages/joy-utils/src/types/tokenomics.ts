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
  storageProviders: {
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
  };
  contentCurators: {
    number: number;
    contentCuratorLead: number;
    rewardsPerWeek: number;
    rewardsShare: number;
    totalStake: number;
    stakeShare: number;
  };
}

export type StatusServerData = {
  dollarPool: {
    size: number;
    replenishAmount: number;
  };
  price: string;
};

export type LandingPageMain = {
  validator: [string, string][];
  nominator: [string, string][];
  storageProviders: [string, string][];
  contentCurators: [string, string][];
  council: [string, string][];
}

export type LandingPageExtraBase = {
  title: {
    value: string;
    color: string;
  };
  cellData: [string, string][];
}

export type LandingPageWorkingGroupExtra = LandingPageExtraBase & {
  type: 'WorkingGroup';
}

export type LandingPageCouncilExtra = LandingPageExtraBase & {
  type: 'Council';
  extra: {
    termReward: string;
    termMaxBonus: string;
    text: string;
    button: {
      href: string;
      text: string;
    }
  }
}
