import BaseTransport from './base';
import { ApiPromise } from '@polkadot/api';
import CouncilTransport from './council';
import WorkingGroupsTransport from './workingGroups';
import { APIQueryCache } from './APIQueryCache';
import { Seats, ElectionStage, SealedVote } from '@joystream/types/council';
import { Option, Vec } from '@polkadot/types';
import { BlockNumber,
  BalanceOf,
  Exposure,
  AccountId,
  Hash,
  ValidatorPrefs,
  ValidatorPrefsTo196 } from '@polkadot/types/interfaces';
import { WorkerId, Opening as StorageOpening } from '@joystream/types/working-group';
import { RewardRelationshipId, RewardRelationship } from '@joystream/types/recurring-rewards';
import { StakeId, Stake } from '@joystream/types/stake';
import { CuratorId, Curator, LeadId, CuratorOpening } from '@joystream/types/content-working-group';
import { TokenomicsData,
  StatusServerData,
  LandingPageWorkingGroupExtra,
  LandingPageMain,
  LandingPageCouncilExtra } from '@polkadot/joy-utils/src/types/tokenomics';
import { calculateValidatorsRewardsPerEra } from '../functions/staking';
import { Uint32 } from '@joystream/types/versioned-store/PropertyType';
import { Opening, OpeningId } from '@joystream/types/hiring';
import { formatBalance } from '@polkadot/util';
import { registry } from '@polkadot/react-api';
import BN from 'bn.js';

export default class TokenomicsTransport extends BaseTransport {
  private councilT: CouncilTransport
  private workingGroupT: WorkingGroupsTransport

  constructor (
    api: ApiPromise,
    cacheApi: APIQueryCache,
    councilTransport: CouncilTransport,
    workingGroups: WorkingGroupsTransport
  ) {
    super(api, cacheApi);
    this.councilT = councilTransport;
    this.workingGroupT = workingGroups;
  }

  async councilSizeAndStake () {
    let totalCouncilStake = 0;
    const activeCouncil = (await this.council.activeCouncil()) as Seats;

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
    const payoutInterval = Number(((await this.api.query.council.payoutInterval()) as Option<BlockNumber>).unwrapOr(0));
    const amountPerPayout = ((await this.api.query.council.amountPerPayout()) as BalanceOf).toNumber();
    const totalCouncilRewardsPerBlock =
      amountPerPayout && payoutInterval ? (amountPerPayout * numberOfCouncilMembers) / payoutInterval : 0;

    const {
      new_term_duration,
      voting_period,
      revealing_period,
      announcing_period
    } = await this.councilT.electionParameters();
    const termDuration = new_term_duration.toNumber();
    const votingPeriod = voting_period.toNumber();
    const revealingPeriod = revealing_period.toNumber();
    const announcingPeriod = announcing_period.toNumber();
    const weekInBlocks = 100800;
    const councilTermLength = termDuration + votingPeriod + revealingPeriod + announcingPeriod;

    const councilTermDurationRatio = termDuration / (termDuration + votingPeriod + revealingPeriod + announcingPeriod);
    const avgCouncilRewardPerBlock = councilTermDurationRatio * totalCouncilRewardsPerBlock;
    const avgCouncilRewardPerWeek = avgCouncilRewardPerBlock * weekInBlocks;

    return {
      councilTermLength,
      avgCouncilRewardPerWeek
    };
  }

  async getCouncilData () {
    const { numberOfCouncilMembers, totalCouncilStake } = await this.councilSizeAndStake();
    const { avgCouncilRewardPerWeek, councilTermLength } = await this.councilRewardsPerWeek(numberOfCouncilMembers);

    return {
      numberOfCouncilMembers,
      totalCouncilRewardsInOneWeek: avgCouncilRewardPerWeek,
      totalCouncilStake,
      councilTermLength
    };
  }

  private async storageProviderSizeAndIds () {
    const stakeIds: StakeId[] = [];
    const rewardIds: RewardRelationshipId[] = [];
    let leadStakeId: StakeId | null = null;
    let leadRewardId: RewardRelationshipId | null = null;
    let numberOfStorageProviders = 0;
    let leadNumber = 0;
    const allWorkers = await this.workingGroupT.allWorkers('Storage');
    const currentLeadId = ((await this.api.query.storageWorkingGroup.currentLead()) as Option<WorkerId>)
      .unwrapOr(null)
      ?.toNumber();

    allWorkers.forEach(([workerId, worker]) => {
      const stakeId = worker.role_stake_profile.isSome ? worker.role_stake_profile.unwrap().stake_id : null;
      const rewardId = worker.reward_relationship.unwrapOr(null);

      if (currentLeadId !== undefined && currentLeadId === workerId.toNumber()) {
        leadStakeId = stakeId;
        leadRewardId = rewardId;
        leadNumber += 1;
      } else {
        numberOfStorageProviders += 1;

        if (stakeId) {
          stakeIds.push(stakeId);
        }

        if (rewardId) {
          rewardIds.push(rewardId);
        }
      }
    });

    return {
      numberOfStorageProviders,
      stakeIds,
      rewardIds,
      leadNumber,
      leadRewardId,
      leadStakeId
    };
  }

  private async storageProviderStakeAndRewards (
    stakeIds: StakeId[],
    leadStakeId: StakeId | null,
    rewardIds: RewardRelationshipId[],
    leadRewardId: RewardRelationshipId | null
  ) {
    let totalStorageProviderStake = 0;
    let leadStake = 0;
    let storageProviderRewardsPerBlock = 0;
    let storageLeadRewardsPerBlock = 0

    ;(await this.api.query.stake.stakes.multi<Stake>(stakeIds)).forEach((stake) => {
      totalStorageProviderStake += stake.value.toNumber();
    })
    ;(await this.api.query.recurringRewards.rewardRelationships.multi<RewardRelationship>(rewardIds)).map(
      (rewardRelationship) => {
        const amount = rewardRelationship.amount_per_payout.toNumber();
        const payoutInterval = rewardRelationship.payout_interval.isSome
          ? rewardRelationship.payout_interval.unwrap().toNumber()
          : null;

        if (amount && payoutInterval) {
          storageProviderRewardsPerBlock += amount / payoutInterval;
        }
      }
    );

    if (leadStakeId !== null) {
      leadStake += ((await this.api.query.stake.stakes(leadStakeId)) as Stake).value.toNumber();
    }

    if (leadRewardId !== null) {
      const leadRewardData = (await this.api.query.recurringRewards.rewardRelationships(
        leadRewardId
      )) as RewardRelationship;
      const leadAmount = leadRewardData.amount_per_payout.toNumber();
      const leadRewardInterval = leadRewardData.payout_interval.isSome
        ? leadRewardData.payout_interval.unwrap().toNumber()
        : null;

      if (leadAmount && leadRewardInterval) {
        storageLeadRewardsPerBlock += leadAmount / leadRewardInterval;
      }
    }

    return {
      totalStorageProviderStake,
      leadStake,
      storageProviderRewardsPerWeek: storageProviderRewardsPerBlock * 100800,
      storageProviderLeadRewardsPerWeek: storageLeadRewardsPerBlock * 100800
    };
  }

  async getStorageProviderData () {
    const {
      numberOfStorageProviders,
      leadNumber,
      stakeIds,
      rewardIds,
      leadRewardId,
      leadStakeId
    } = await this.storageProviderSizeAndIds();
    const {
      totalStorageProviderStake,
      leadStake,
      storageProviderRewardsPerWeek,
      storageProviderLeadRewardsPerWeek
    } = await this.storageProviderStakeAndRewards(stakeIds, leadStakeId, rewardIds, leadRewardId);

    return {
      numberOfStorageProviders,
      storageProviderLeadNumber: leadNumber,
      totalStorageProviderStake,
      totalStorageProviderLeadStake: leadStake,
      storageProviderRewardsPerWeek,
      storageProviderLeadRewardsPerWeek
    };
  }

  private async contentCuratorSizeAndIds () {
    const stakeIds: StakeId[] = [];
    const rewardIds: RewardRelationshipId[] = [];
    let numberOfContentCurators = 0;
    const contentCurators = await this.entriesByIds<CuratorId, Curator>(this.api.query.contentWorkingGroup.curatorById);
    const currentLeadId = ((await this.api.query.contentWorkingGroup.currentLeadId()) as Option<LeadId>)
      .unwrapOr(null)
      ?.toNumber();

    contentCurators.forEach(([curatorId, curator]) => {
      const stakeId = curator.role_stake_profile.isSome ? curator.role_stake_profile.unwrap().stake_id : null;
      const rewardId = curator.reward_relationship.unwrapOr(null);

      if (curator.is_active) {
        numberOfContentCurators += 1;

        if (stakeId) {
          stakeIds.push(stakeId);
        }

        if (rewardId) {
          rewardIds.push(rewardId);
        }
      }
    });

    return {
      stakeIds,
      rewardIds,
      numberOfContentCurators,
      contentCuratorLeadNumber: currentLeadId !== undefined ? 1 : 0
    };
  }

  private async contentCuratorStakeAndRewards (stakeIds: StakeId[], rewardIds: RewardRelationshipId[]) {
    let totalContentCuratorStake = 0;
    let contentCuratorRewardsPerBlock = 0

    ;(await this.api.query.stake.stakes.multi<Stake>(stakeIds)).forEach((stake) => {
      totalContentCuratorStake += stake.value.toNumber();
    })
    ;(await this.api.query.recurringRewards.rewardRelationships.multi<RewardRelationship>(rewardIds)).map(
      (rewardRelationship) => {
        const amount = rewardRelationship.amount_per_payout.toNumber();
        const payoutInterval = rewardRelationship.payout_interval.isSome
          ? rewardRelationship.payout_interval.unwrap().toNumber()
          : null;

        if (amount && payoutInterval) {
          contentCuratorRewardsPerBlock += amount / payoutInterval;
        }
      }
    );

    return {
      totalContentCuratorStake,
      contentCuratorRewardsPerBlock
    };
  }

  async getContentCuratorData () {
    const {
      stakeIds,
      rewardIds,
      numberOfContentCurators,
      contentCuratorLeadNumber
    } = await this.contentCuratorSizeAndIds();
    const { totalContentCuratorStake, contentCuratorRewardsPerBlock } = await this.contentCuratorStakeAndRewards(
      stakeIds,
      rewardIds
    );

    return {
      numberOfContentCurators,
      contentCuratorLeadNumber,
      totalContentCuratorStake,
      contentCuratorRewardsPerWeek: contentCuratorRewardsPerBlock * 100800
    };
  }

  async validatorSizeAndStake () {
    const validatorIds = await this.api.query.session.validators();
    const currentEra = (await this.api.query.staking.currentEra()).unwrapOr(null);
    const currentMaxValidatorCount = (await this.api.query.staking.validatorCount()).toNumber();
    const emptyValidatorSlots = currentMaxValidatorCount - validatorIds.length;
    let totalValidatorStake = 0;
    let numberOfNominators = 0;
    let leastPossibleStakingAmount = 0;

    if (currentEra !== null) {
      const validatorStakeData = await this.api.query.staking.erasStakers.multi<Exposure>(
        validatorIds.map((validatorId) => [currentEra, validatorId])
      );

      leastPossibleStakingAmount = validatorStakeData[0].total.toNumber();

      validatorStakeData.forEach((data) => {
        if (!data.total.isEmpty) {
          totalValidatorStake += data.total.toNumber();
        }

        if (!data.others.isEmpty) {
          numberOfNominators += data.others.length;
        }

        if (data.total.toNumber() < leastPossibleStakingAmount) {
          leastPossibleStakingAmount = data.total.toNumber();
        }
      });
    }

    return {
      numberOfValidators: validatorIds.length,
      numberOfNominators,
      totalValidatorStake,
      leastPossibleStakingAmount: emptyValidatorSlots ? 1 : leastPossibleStakingAmount + 1
    };
  }

  async getValidatorData () {
    const totalIssuance = (await this.api.query.balances.totalIssuance()).toNumber();
    const {
      numberOfValidators,
      numberOfNominators,
      totalValidatorStake,
      leastPossibleStakingAmount
    } = await this.validatorSizeAndStake();
    const validatorRewardsPerEra = calculateValidatorsRewardsPerEra(totalValidatorStake, totalIssuance);

    return {
      totalIssuance,
      numberOfValidators,
      numberOfNominators,
      totalValidatorStake,
      validatorRewardsPerWeek: validatorRewardsPerEra * 168, // Assuming 1 era = 1h
      leastPossibleStakingAmount
    };
  }

  async getTokenomicsData (): Promise<TokenomicsData> {
    const { numberOfCouncilMembers, totalCouncilRewardsInOneWeek, totalCouncilStake } = await this.getCouncilData();
    const {
      numberOfStorageProviders,
      storageProviderLeadNumber,
      totalStorageProviderStake,
      totalStorageProviderLeadStake,
      storageProviderLeadRewardsPerWeek,
      storageProviderRewardsPerWeek
    } = await this.getStorageProviderData();
    const {
      numberOfContentCurators,
      contentCuratorLeadNumber,
      totalContentCuratorStake,
      contentCuratorRewardsPerWeek
    } = await this.getContentCuratorData();
    const {
      numberOfValidators,
      numberOfNominators,
      totalValidatorStake,
      validatorRewardsPerWeek,
      totalIssuance
    } = await this.getValidatorData();
    const currentlyStakedTokens =
      totalCouncilStake +
      totalStorageProviderStake +
      totalStorageProviderLeadStake +
      totalContentCuratorStake +
      totalValidatorStake;
    const totalWeeklySpending =
      totalCouncilRewardsInOneWeek +
      storageProviderRewardsPerWeek +
      storageProviderLeadRewardsPerWeek +
      contentCuratorRewardsPerWeek +
      validatorRewardsPerWeek;
    const totalNumberOfActors =
      numberOfCouncilMembers +
      numberOfStorageProviders +
      storageProviderLeadNumber +
      numberOfContentCurators +
      contentCuratorLeadNumber +
      numberOfValidators;

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
      storageProviders: {
        number: numberOfStorageProviders,
        totalStake: totalStorageProviderStake,
        stakeShare: totalStorageProviderStake / currentlyStakedTokens,
        rewardsPerWeek: storageProviderRewardsPerWeek,
        rewardsShare: storageProviderRewardsPerWeek / totalWeeklySpending,
        lead: {
          number: storageProviderLeadNumber,
          totalStake: totalStorageProviderLeadStake,
          stakeShare: totalStorageProviderLeadStake / currentlyStakedTokens,
          rewardsPerWeek: storageProviderLeadRewardsPerWeek,
          rewardsShare: storageProviderLeadRewardsPerWeek / totalWeeklySpending
        }
      },
      contentCurators: {
        number: numberOfContentCurators,
        contentCuratorLead: contentCuratorLeadNumber,
        rewardsPerWeek: contentCuratorRewardsPerWeek,
        rewardsShare: contentCuratorRewardsPerWeek / totalWeeklySpending,
        totalStake: totalContentCuratorStake,
        stakeShare: totalContentCuratorStake / currentlyStakedTokens
      }
    };
  }

  async optimalNomination (calcWith: BN = new BN(10000)) {
    const electedInfo = await this.api.derive.staking.electedInfo();
    const lastEra = await this.api.derive.session.indexes();
    const lastReward = await this.api.query.staking.erasValidatorReward(
      lastEra.activeEra.gtn(0) ? lastEra.activeEra.subn(1).toNumber() : new BN(0)
    );

    const perValidatorReward = lastReward.unwrapOrDefault().divn(electedInfo.info.length);

    const rewardData = electedInfo.info
      .map(({ exposure: _exposure, stakingLedger, validatorPrefs }) => {
        const exposure = _exposure || {
          others: registry.createType('Vec<IndividualExposure>'),
          own: registry.createType('Compact<Balance>'),
          total: registry.createType('Compact<Balance>')
        };

        const prefs = (validatorPrefs as ValidatorPrefs | ValidatorPrefsTo196) || {
          commission: registry.createType('Compact<Perbill>')
        };

        let bondTotal = exposure.total.unwrap();
        const skipRewards = bondTotal.isZero();

        if (bondTotal.isZero() && stakingLedger) {
          bondTotal = stakingLedger.total.unwrap();
        }

        const validatorPayment = (prefs as ValidatorPrefsTo196).validatorPayment
          ? ((prefs as ValidatorPrefsTo196).validatorPayment.unwrap() as BN)
          : (prefs as ValidatorPrefs).commission.unwrap().mul(perValidatorReward).div(new BN(1_000_000_000));

        const rewardSplit = perValidatorReward.sub(validatorPayment);
        const rewardPayout =
          calcWith.isZero() || rewardSplit.isZero() ? new BN(0) : calcWith.mul(rewardSplit).div(calcWith.add(bondTotal));

        return skipRewards ? new BN(0) : rewardPayout;
      })
      .sort((a, b) => b.toNumber() - a.toNumber());

    return rewardData[0];
  }

  async landingPageData (userAddress?: string): Promise<LandingPageMain> {
    let userFreeBalance = 10000;
    let profitPerEra = 0;
    const statusData = await fetch('https://status.joystream.org/status')
      .then((data) => data.json())
      .then((data) => data as StatusServerData);
    const { validatorRewardsPerWeek, numberOfValidators, leastPossibleStakingAmount } = await this.getValidatorData();
    const {
      numberOfCouncilMembers,
      totalCouncilRewardsInOneWeek,
      totalCouncilStake,
      councilTermLength
    } = await this.getCouncilData();
    const {
      numberOfStorageProviders,
      storageProviderLeadNumber,
      totalStorageProviderStake,
      totalStorageProviderLeadStake,
      storageProviderLeadRewardsPerWeek,
      storageProviderRewardsPerWeek
    } = await this.getStorageProviderData();
    const {
      numberOfContentCurators,
      contentCuratorLeadNumber,
      totalContentCuratorStake,
      contentCuratorRewardsPerWeek
    } = await this.getContentCuratorData();
    const currentBlockheight = (await this.api.derive.chain.bestNumber()).toNumber();
    const councilTermEndsAt = ((await this.api.query.council.termEndsAt()) as BlockNumber).toNumber();

    const averageStorageProviderStake =
      (totalStorageProviderLeadStake + totalStorageProviderStake) /
      (numberOfStorageProviders + storageProviderLeadNumber);
    const averageContentCuratorStake = totalContentCuratorStake / numberOfContentCurators;
    const averageCouncilStake = totalCouncilStake / numberOfCouncilMembers;

    if (userAddress) {
      const accountBalanceData = await this.api.derive.balances.account(userAddress);

      profitPerEra = (
        await this.optimalNomination(
          accountBalanceData.freeBalance.isZero() ? undefined : accountBalanceData.freeBalance.toBn()
        )
      ).toNumber();

      if (!accountBalanceData.freeBalance.isZero()) {
        userFreeBalance = accountBalanceData.freeBalance.toNumber();
      }
    } else {
      profitPerEra = (await this.optimalNomination()).toNumber();
    }

    return {
      validator: [
        ['REQUIRED STAKE', `${Math.trunc(leastPossibleStakingAmount)}`],
        [
          'WEEKLY REWARD',
          statusData
            ? `$${((validatorRewardsPerWeek / numberOfValidators) * Number(statusData.price)).toFixed(2)}`
            : 'Not available..'
        ]
      ],
      nominator: [
        ['YOUR STAKE', formatBalance(userFreeBalance).split('.000').join('')],
        [
          'WEEKLY REWARD',
          statusData ? `$${(Number(statusData.price) * profitPerEra * 168).toFixed(2)}` : 'Not available..'
        ]
      ],
      storageProviders: [
        ['AVERAGE STAKE', `${formatBalance(Math.trunc(averageStorageProviderStake)).split('.000').join('')}`],
        [
          'WEEKLY REWARD',
          statusData
            ? `$${(
              (storageProviderLeadRewardsPerWeek + storageProviderRewardsPerWeek) *
                Number(statusData.price)
            ).toFixed(2)}`
            : ' Not available..'
        ],
        ['POSITIONS', `${numberOfStorageProviders} (${storageProviderLeadNumber})`]
      ],
      contentCurators: [
        ['AVERAGE STAKE', `${formatBalance(Math.trunc(averageContentCuratorStake)).split('.000').join('')}`],
        [
          'WEEKLY REWARD',
          statusData ? `$${(contentCuratorRewardsPerWeek * Number(statusData.price)).toFixed(2)}` : 'Not available..'
        ],
        ['POSITIONS', `${numberOfContentCurators} (${contentCuratorLeadNumber})`]
      ],
      council: [
        ['AVERAGE STAKE', `${formatBalance(Math.trunc(averageCouncilStake)).split('.000').join('')}`],
        [
          'WEEKLY REWARD',
          statusData ? `$${(totalCouncilRewardsInOneWeek * Number(statusData.price)).toFixed(2)}` : 'Not available..'
        ],
        ['MAX BONUS', '$30'],
        ['POSITIONS', `${numberOfCouncilMembers}`],
        ['TERM LENGTH', `${Math.trunc((councilTermLength * 6) / 3600 / 24)} days`],
        [
          'NEXT ELECTION STARTS',
          currentBlockheight < councilTermEndsAt
            ? `${Math.trunc(((councilTermEndsAt - currentBlockheight) * 6) / 3600)}h`
            : 'Running..'
        ]
      ]
    };
  }

  async councilStageData (): Promise<LandingPageCouncilExtra[] | undefined> {
    let termReward = 0;
    const {
      new_term_duration,
      voting_period,
      revealing_period,
      announcing_period,
      min_council_stake,
      candidacy_limit,
      council_size
    } = await this.councilT.electionParameters();
    const termLength =
      ((new_term_duration.toNumber() +
        voting_period.toNumber() +
        revealing_period.toNumber() +
        announcing_period.toNumber()) *
        6) /
      3600 /
      24;
    const stage = ((await this.api.query.councilElection.stage()) as Option<ElectionStage>).value as ElectionStage;
    const round = (await this.api.query.councilElection.round()) as Uint32;
    const statusData = await fetch('https://status.joystream.org/status')
      .then((data) => data.json())
      .then((data) => data as StatusServerData);
    const currentBlockheight = (await this.api.derive.chain.bestNumber()).toNumber();
    const amountPerPayout = (await this.api.query.council.amountPerPayout()) as BalanceOf;
    const payoutInterval = (await this.api.query.council.payoutInterval()) as Option<BlockNumber>;

    if (payoutInterval.isSome) {
      termReward = (amountPerPayout.toNumber() / payoutInterval.unwrap().toNumber()) * new_term_duration.toNumber();
    }

    if (stage.isOfType('Announcing')) {
      const applicants = (await this.api.query.councilElection.applicants()) as Vec<AccountId>;

      return [
        {
          type: 'Council',
          title: {
            value: `Council Election #${Number(round)} - Announcing stage`,
            color: 'green'
          },
          cellData: [
            ['MIN STAKE', `${min_council_stake.toNumber()}`],
            ['MAX APPLICANTS', `${candidacy_limit.toNumber()}`],
            ['CURRENT APPLICANTS', `${applicants.length}`],
            ['POSITIONS', `${council_size.toNumber()}`],
            ['TERM LENGTH', `${Math.trunc(termLength)} days`],
            [
              'STAGE ENDS IN',
              `${Math.trunc(((stage.asType('Announcing').toNumber() - currentBlockheight) * 6) / 3600)}h`
            ]
          ],
          extra: {
            termReward: statusData ? `$${(Number(statusData.price) * termReward).toFixed(2)}` : 'Not available..',
            termMaxBonus: '$60',
            text: "If you think you have what it takes to become a <a href='/#/council'>Council Member</a>:",
            button: {
              href: '/#/council/applicants',
              text: 'Apply Here'
            }
          }
        }
      ];
    } else if (stage.isOfType('Voting')) {
      const commitments = (await this.api.query.councilElection.commitments()) as Vec<Hash>;
      const applicants = (await this.api.query.councilElection.applicants()) as Vec<AccountId>;
      const votes = (await this.api.query.councilElection.votes.multi<SealedVote>(commitments));
      const stake = votes.reduce((result, vote) => result + (vote.stake.transferred.toNumber() + vote.stake.new.toNumber()), 0);

      return [
        {
          type: 'Council',
          title: {
            value: `Council Election #${Number(round)} - Voting stage`,
            color: '#2196f3'
          },
          cellData: [
            ['STAKE', `${formatBalance(stake).split('.000').join('')}`],
            ['CURRENT VOTES', `${commitments.length}`],
            ['CURRENT APPLICANTS', `${applicants.length}`],
            ['POSITIONS', `${council_size.toNumber()}`],
            ['TERM LENGTH', `${Math.trunc(termLength)} days`],
            ['STAGE ENDS IN', `${Math.trunc(((stage.asType('Voting').toNumber() - currentBlockheight) * 6) / 3600)}h`]
          ],
          extra: {
            termReward: statusData ? `$${(Number(statusData.price) * termReward).toFixed(2)}` : 'Not available..',
            termMaxBonus: '$60',
            text: "Both <a href='/#/council'>Council Members</a> and Voters can earn rewards:",
            button: {
              href: '/#/council/votes',
              text: 'Vote Here'
            }
          }
        }
      ];
    } else if (stage.isOfType('Revealing')) {
      const commitments = (await this.api.query.councilElection.commitments()) as Vec<Hash>;
      const votes = (await this.api.query.councilElection.votes.multi<SealedVote>(commitments));
      const numberOfRevealedVotes = votes.reduce(
        (revealedVoteCount, vote) => (vote.vote.unwrapOr(null) === null ? revealedVoteCount : revealedVoteCount + 1),
        0
      );
      const applicants = (await this.api.query.councilElection.applicants()) as Vec<AccountId>;
      const stake = votes.reduce((result, vote) => result + (vote.stake.transferred.toNumber() + vote.stake.new.toNumber()), 0);

      return [
        {
          type: 'Council',
          title: {
            value: `Council election #${Number(round)} - Revealing stage`,
            color: 'red'
          },
          cellData: [
            ['STAKE', `${formatBalance(stake).split('.000').join('')}`],
            ['REVEALED VOTES', `${numberOfRevealedVotes}/${votes.length}`],
            ['CURRENT APPLICANTS', `${applicants.length}`],
            ['POSITIONS', `${council_size.toNumber()}`],
            ['TERM LENGTH', `${Math.trunc(termLength)} days`],
            [
              'STAGE ENDS IN',
              `${Math.trunc(((stage.asType('Revealing').toNumber() - currentBlockheight) * 6) / 3600)}h`
            ]
          ],
          extra: {
            termReward: statusData ? `$${(Number(statusData.price) * termReward).toFixed(2)}` : 'Not available..',
            termMaxBonus: '$60',
            text: "If you've voted for a <a href='/#/council'>Council Member</a>:",
            button: {
              href: '/#/council',
              text: 'Reveal Here'
            }
          }
        }
      ];
    }

    return undefined;
  }

  async openingData (group: 'storageWorkingGroup' | 'contentWorkingGroup'): Promise<LandingPageWorkingGroupExtra[]> {
    let openingsToShow: Opening[];

    if (group === 'storageWorkingGroup') {
      const storageOpenings = await this.entriesByIds<OpeningId, StorageOpening>(
        this.api.query.storageWorkingGroup.openingById
      );

      openingsToShow = await this.api.query.hiring.openingById.multi<Opening>(
        storageOpenings.map(([_, storageOpening]) => storageOpening.hiring_opening_id)
      );
    } else {
      const contentOpenings = await this.entriesByIds<OpeningId, CuratorOpening>(
        this.api.query.contentWorkingGroup.curatorOpeningById
      );

      openingsToShow = await this.api.query.hiring.openingById.multi<Opening>(
        contentOpenings.map(([_, curatorOpening]) => curatorOpening.opening_id)
      );
    }

    return openingsToShow
      .filter(
        (opening) =>
          opening.stage.isOfType('Active') && opening.stage.asType('Active').stage.isOfType('AcceptingApplications')
      )
      .map((opening, idx) => {
        return {
          type: 'WorkingGroup',
          title: {
            color: 'green',
            value: `${group === 'storageWorkingGroup' ? 'Storage Provider' : 'Content Curator'} Opening #${idx + 1}` // may need to be changed
          },
          cellData: [
            [
              'MIN APPLICATION STAKE',
              formatBalance(opening.application_staking_policy.unwrapOrDefault().amount.toString())
                .split('.000')
                .join('')
            ],
            [
              'MIN ROLE STAKE',
              formatBalance(opening.role_staking_policy.unwrapOrDefault().amount.toString()).split('.000').join('')
            ],
            ['REWARDS', opening.parse_human_readable_text_with_fallback().reward]
          ]
        };
      });
  }
}
