import BaseTransport from './base';
import { ApiPromise } from '@polkadot/api';
import CouncilTransport from './council';
import WorkingGroupsTransport from './workingGroups';
import { APIQueryCache } from './APIQueryCache';
import { Seats } from '@joystream/types/council';
import { Option, u32 } from '@polkadot/types';
import { BlockNumber, BalanceOf, Exposure } from '@polkadot/types/interfaces';
import { WorkerId } from '@joystream/types/working-group';
import { RewardRelationshipId, RewardRelationship } from '@joystream/types/recurring-rewards';
import { StakeId, Stake } from '@joystream/types/stake';
import { TokenomicsData } from '@polkadot/joy-utils/src/types/tokenomics';
import { calculateValidatorsRewardsPerEra } from '../functions/staking';
import { WorkingGroupKey, PostId, ChannelId } from '@joystream/types/common';
import { MemberId } from '@joystream/types/members';
import { CategoryId } from '@joystream/types/forum';
import { MintId, Mint } from '@joystream/types/mint';
import { genericTypes } from '../consts/tokenomics';
import { VideoId } from '@joystream/types/content';

import { ProposalId, ProposalDetails, Proposal } from '@joystream/types/proposals';

import HISTORICAL_PROPOSALS from '../transport/static/historical-proposals.json';

export default class TokenomicsTransport extends BaseTransport {
  private councilT: CouncilTransport;
  private workingGroupT: WorkingGroupsTransport;

  constructor (api: ApiPromise, cacheApi: APIQueryCache, councilTransport: CouncilTransport, workingGroups: WorkingGroupsTransport) {
    super(api, cacheApi);
    this.councilT = councilTransport;
    this.workingGroupT = workingGroups;
  }

  async councilSizeAndStake () {
    let totalCouncilStake = 0;
    const activeCouncil = await this.council.activeCouncil() as Seats;

    activeCouncil.map((member) => {
      let stakeAmount = 0;

      stakeAmount += member.stake.toNumber();
      member.backers.forEach((backer) => {
        stakeAmount += backer.stake.toNumber();
      });
      totalCouncilStake += stakeAmount;
    });

    return {
      numberOfCouncilMembers: activeCouncil.length,
      totalCouncilStake
    };
  }

  private async councilRewardsPerWeek (numberOfCouncilMembers: number) {
    const payoutInterval = Number((await this.api.query.council.payoutInterval() as Option<BlockNumber>).unwrapOr(0));
    const amountPerPayout = (await this.api.query.council.amountPerPayout() as BalanceOf).toNumber();
    const totalCouncilRewardsPerBlock = (amountPerPayout && payoutInterval)
      ? (amountPerPayout * numberOfCouncilMembers) / payoutInterval
      : 0;

    const { new_term_duration, voting_period, revealing_period, announcing_period } = await this.councilT.electionParameters();
    const termDuration = new_term_duration.toNumber();
    const votingPeriod = voting_period.toNumber();
    const revealingPeriod = revealing_period.toNumber();
    const announcingPeriod = announcing_period.toNumber();
    const weekInBlocks = 100800;

    const councilTermDurationRatio = termDuration / (termDuration + votingPeriod + revealingPeriod + announcingPeriod);
    const avgCouncilRewardPerBlock = councilTermDurationRatio * totalCouncilRewardsPerBlock;
    const avgCouncilRewardPerWeek = avgCouncilRewardPerBlock * weekInBlocks;

    return avgCouncilRewardPerWeek;
  }

  async getCouncilData () {
    const { numberOfCouncilMembers, totalCouncilStake } = await this.councilSizeAndStake();
    const totalCouncilRewardsInOneWeek = await this.councilRewardsPerWeek(numberOfCouncilMembers);

    return {
      numberOfCouncilMembers,
      totalCouncilRewardsInOneWeek,
      totalCouncilStake
    };
  }

  private async workingGroupSizeAndIds (group: WorkingGroupKey) {
    const workerStakeIds: StakeId[] = [];
    const workerRewardIds: RewardRelationshipId[] = [];
    let leadStakeId: StakeId | null = null;
    let leadRewardId: RewardRelationshipId | null = null;
    let numberOfWorkers = 0;
    let leadNumber = 0;
    const allWorkers = await this.workingGroupT.allWorkers(group);
    const currentLeadId = (await this.workingGroupT.queryByGroup(group).currentLead() as Option<WorkerId>)
      .unwrapOr(undefined)?.toNumber();

    allWorkers.forEach(([workerId, worker]) => {
      const stakeId = worker.role_stake_profile.isSome ? worker.role_stake_profile.unwrap().stake_id : null;
      const rewardId = worker.reward_relationship.unwrapOr(null);

      if (currentLeadId !== undefined && currentLeadId === workerId.toNumber()) {
        leadStakeId = stakeId;
        leadRewardId = rewardId;
        leadNumber += 1;
      } else {
        numberOfWorkers += 1;

        if (stakeId) {
          workerStakeIds.push(stakeId);
        }

        if (rewardId) {
          workerRewardIds.push(rewardId);
        }
      }
    });

    return {
      numberOfWorkers,
      workerStakeIds,
      workerRewardIds,
      leadNumber,
      leadRewardId,
      leadStakeId
    };
  }

  private async resolveGroupStakeAndRewards (
    workerStakeIds: StakeId[],
    leadStakeId: StakeId | null,
    workerRewardIds: RewardRelationshipId[],
    leadRewardId: RewardRelationshipId | null
  ) {
    let workersStake = 0;
    let leadStake = 0;
    let workersRewardsPerBlock = 0;
    let leadRewardsPerBlock = 0;

    (await this.api.query.stake.stakes.multi<Stake>(workerStakeIds)).forEach((stake) => {
      workersStake += stake.value.toNumber();
    });
    (await this.api.query.recurringRewards.rewardRelationships.multi<RewardRelationship>(workerRewardIds)).map((rewardRelationship) => {
      const amount = rewardRelationship.amount_per_payout.toNumber();
      const payoutInterval = rewardRelationship.payout_interval.isSome
        ? rewardRelationship.payout_interval.unwrap().toNumber()
        : null;

      if (amount && payoutInterval) {
        workersRewardsPerBlock += amount / payoutInterval;
      }
    });

    if (leadStakeId !== null) {
      leadStake += (await this.api.query.stake.stakes(leadStakeId) as Stake).value.toNumber();
    }

    if (leadRewardId !== null) {
      const leadRewardData = (await this.api.query.recurringRewards.rewardRelationships(leadRewardId) as RewardRelationship);
      const leadAmount = leadRewardData.amount_per_payout.toNumber();
      const leadRewardInterval = leadRewardData.payout_interval.isSome ? leadRewardData.payout_interval.unwrap().toNumber() : null;

      if (leadAmount && leadRewardInterval) {
        leadRewardsPerBlock += leadAmount / leadRewardInterval;
      }
    }

    return {
      workersStake,
      leadStake,
      workersRewardsPerWeek: workersRewardsPerBlock * 100800,
      leadRewardsPerWeek: leadRewardsPerBlock * 100800
    };
  }

  async getWorkingGroupData (group: WorkingGroupKey) {
    const { numberOfWorkers, leadNumber, workerStakeIds, workerRewardIds, leadRewardId, leadStakeId } =
      await this.workingGroupSizeAndIds(group);
    const { workersStake, leadStake, workersRewardsPerWeek, leadRewardsPerWeek } =
      await this.resolveGroupStakeAndRewards(workerStakeIds, leadStakeId, workerRewardIds, leadRewardId);

    return {
      numberOfWorkers,
      leadNumber,
      workersStake,
      leadStake,
      workersRewardsPerWeek,
      leadRewardsPerWeek
    };
  }

  async validatorSizeAndStake () {
    const validatorIds = await this.api.query.session.validators();
    const currentEra = (await this.api.query.staking.currentEra()).unwrapOr(null);
    let totalValidatorStake = 0; let numberOfNominators = 0;

    if (currentEra !== null) {
      const validatorStakeData = await this.api.query.staking.erasStakers.multi<Exposure>(
        validatorIds.map((validatorId) => [currentEra, validatorId])
      );

      validatorStakeData.forEach((data) => {
        if (!data.total.isEmpty) {
          totalValidatorStake += data.total.toNumber();
        }

        if (!data.others.isEmpty) {
          numberOfNominators += data.others.length;
        }
      });
    }

    return {
      numberOfValidators: validatorIds.length,
      numberOfNominators,
      totalValidatorStake
    };
  }

  async getValidatorData () {
    const totalIssuance = (await this.api.query.balances.totalIssuance()).toNumber();
    const { numberOfValidators, numberOfNominators, totalValidatorStake } = await this.validatorSizeAndStake();
    const validatorRewardsPerEra = calculateValidatorsRewardsPerEra(totalValidatorStake, totalIssuance);

    return {
      totalIssuance,
      numberOfValidators,
      numberOfNominators,
      totalValidatorStake,
      validatorRewardsPerWeek: validatorRewardsPerEra * 168 // Assuming 1 era = 1h
    };
  }

  async getTokenomicsData (): Promise<TokenomicsData> {
    const { numberOfCouncilMembers, totalCouncilRewardsInOneWeek, totalCouncilStake } =
      await this.getCouncilData();
    const workingGroupsData = {
      storageProviders: await this.getWorkingGroupData('Storage'),
      curators: await this.getWorkingGroupData('Content'),
      operations: await this.getWorkingGroupData('Operations')
    };
    const { numberOfValidators, numberOfNominators, totalValidatorStake, validatorRewardsPerWeek, totalIssuance } =
      await this.getValidatorData();

    const currentlyStakedTokens =
      totalCouncilStake +
      Object.values(workingGroupsData).reduce(
        (sum, { workersStake, leadStake }) => sum + workersStake + leadStake,
        0) +
      totalValidatorStake;

    const totalWeeklySpending =
      totalCouncilRewardsInOneWeek +
      Object.values(workingGroupsData).reduce(
        (sum, { workersRewardsPerWeek, leadRewardsPerWeek }) => sum + workersRewardsPerWeek + leadRewardsPerWeek,
        0) +
      validatorRewardsPerWeek;

    const totalNumberOfActors =
      numberOfCouncilMembers +
      Object.values(workingGroupsData).reduce(
        (sum, { numberOfWorkers, leadNumber }) => sum + numberOfWorkers + leadNumber,
        0) +
      numberOfValidators;

    const resolveGroupData = (data: typeof workingGroupsData[keyof typeof workingGroupsData]) => ({
      number: data.numberOfWorkers,
      totalStake: data.workersStake,
      stakeShare: data.workersStake / currentlyStakedTokens,
      rewardsPerWeek: data.workersRewardsPerWeek,
      rewardsShare: data.workersRewardsPerWeek / totalWeeklySpending,
      lead: {
        number: data.leadNumber,
        totalStake: data.leadStake,
        stakeShare: data.leadStake / currentlyStakedTokens,
        rewardsPerWeek: data.leadRewardsPerWeek,
        rewardsShare: data.leadRewardsPerWeek / totalWeeklySpending
      }
    });

    return {
      totalIssuance,
      currentlyStakedTokens,
      totalWeeklySpending,
      totalNumberOfActors,
      validators: {
        number: numberOfValidators,
        nominators: {
          number: numberOfNominators
        },
        rewardsPerWeek: validatorRewardsPerWeek,
        rewardsShare: validatorRewardsPerWeek / totalWeeklySpending,
        totalStake: totalValidatorStake,
        stakeShare: totalValidatorStake / currentlyStakedTokens
      },
      council: {
        number: numberOfCouncilMembers,
        rewardsPerWeek: totalCouncilRewardsInOneWeek,
        rewardsShare: totalCouncilRewardsInOneWeek / totalWeeklySpending,
        totalStake: totalCouncilStake,
        stakeShare: totalCouncilStake / currentlyStakedTokens
      },
      storageProviders: resolveGroupData(workingGroupsData.storageProviders),
      contentCurators: resolveGroupData(workingGroupsData.curators),
      operations: resolveGroupData(workingGroupsData.operations)
    };
  }

  async networkStatistics () {
    const blockHeight = (await this.api.derive.chain.bestNumber()).toNumber();
    const numberOfMembers = (await this.api.query.members.nextMemberId() as MemberId).toNumber();
    const content = (await this.api.query.content.nextVideoId() as VideoId).toNumber() - 1;
    const numberOfChannels = (await this.api.query.content.nextChannelId() as ChannelId).toNumber() - 1;
    const proposalCount = (await this.api.query.proposalsEngine.proposalCount() as u32).toNumber();
    const numberOfForumCategories = (await this.api.query.forum.nextCategoryId() as CategoryId).toNumber() - 1;
    const numberOfForumPosts = (await this.api.query.forum.nextPostId() as PostId).toNumber() - 1;
    const [councilMintId, contentCuratorMintId, storageMintId] = await Promise.all([
      (await this.api.query.council.councilMint()) as MintId,
      (await this.api.query.contentDirectoryWorkingGroup.mint()) as MintId,
      (await this.api.query.storageWorkingGroup.mint()) as MintId
    ]);
    const [councilMint, contentMint, storageMint] = await this.api.query.minting.mints.multi<Mint>([
      councilMintId,
      contentCuratorMintId,
      storageMintId
    ]);

    return {
      blockHeight,
      numberOfMembers,
      content,
      numberOfChannels,
      proposalCount,
      historicalProposals: HISTORICAL_PROPOSALS.length,
      numberOfForumCategories,
      numberOfForumPosts,
      councilMintCapacity: councilMint.capacity.toNumber(),
      councilMintSpent: councilMint.total_minted.toNumber(),
      contentCuratorMintCapacity: contentMint.capacity.toNumber(),
      contentCuratorMintSpent: contentMint.total_minted.toNumber(),
      storageProviderMintCapacity: storageMint.capacity.toNumber(),
      storageProviderMintSpent: storageMint.total_minted.toNumber()
    };
  }

  async proposalStatistics () {
    const proposalCounters = {
      all: 0,
      Active: 0,
      Approved: 0,
      Rejected: 0,
      Expired: 0,
      Slashed: 0,
      Canceled: 0,
      Vetoed: 0
    };

    const returnData = {
      text: { ...proposalCounters },
      spending: { ...proposalCounters },
      workingGroups: { ...proposalCounters },
      networkChanges: { ...proposalCounters },
      all: { ...proposalCounters }
    };

    const proposals = await this.entriesByIds<ProposalId, ProposalDetails>(this.api.query.proposalsCodex.proposalDetailsByProposalId);
    const proposalData = await this.api.query.proposalsEngine.proposals.multi<Proposal>(proposals.map(([id, _]) => id));

    proposalData.map((proposal, index) => {
      const proposalType = proposals[index][1].type;
      const definedProposalType = genericTypes[proposalType];
      const proposalStatus = proposal.status.isOfType('Active') ? 'Active' as const : proposal.status.asType('Finalized').proposalStatus.type;

      returnData[definedProposalType].all++;
      returnData[definedProposalType][proposalStatus]++;
      returnData.all.all++;
      returnData.all[proposalStatus]++;
    });

    return returnData;
  }

  historicalProposalStatistics () {
    const proposalCounters = {
      all: 0,
      Active: 0,
      Approved: 0,
      Rejected: 0,
      Expired: 0,
      Slashed: 0,
      Canceled: 0,
      Vetoed: 0
    };

    const historicalProposalData = {
      text: { ...proposalCounters },
      spending: { ...proposalCounters },
      workingGroups: { ...proposalCounters },
      networkChanges: { ...proposalCounters },
      all: { ...proposalCounters }
    };

    HISTORICAL_PROPOSALS.map(({ proposal }) => {
      let proposalType: 'text' | 'spending' | 'networkChanges' | 'workingGroups';
      const proposalStatus = proposal.status.Finalized
        ? Object.keys(proposal.status.Finalized.proposalStatus)[0] as 'Approved' | 'Rejected' | 'Expired' | 'Slashed' | 'Canceled' | 'Vetoed'
        : 'Active' as const;

      if (proposal.type === 'Text') {
        proposalType = 'text';
      } else if (proposal.type === 'Spending') {
        proposalType = 'spending';
      } else if (proposal.type === 'RuntimeUpgrade' || proposal.type === 'SetElectionParameters' || proposal.type === 'SetValidatorCount') {
        proposalType = 'networkChanges';
      } else {
        proposalType = 'workingGroups';
      }

      historicalProposalData[proposalType][proposalStatus]++;
      historicalProposalData[proposalType].all++;
      historicalProposalData.all[proposalStatus]++;
      historicalProposalData.all.all++;
    });

    return historicalProposalData;
  }
}
