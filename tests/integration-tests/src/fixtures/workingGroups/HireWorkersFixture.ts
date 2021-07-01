import { WorkerId } from '@joystream/types/working-group'
import { Api } from '../../Api'
import { LEADER_OPENING_STAKE } from '../../consts'
import { BaseQueryNodeFixture, FixtureRunner } from '../../Fixture'
import { QueryNodeApi } from '../../QueryNodeApi'
import { WorkingGroupModuleName } from '../../types'
import { Utils } from '../../utils'
import { AddStakingAccountsHappyCaseFixture, BuyMembershipHappyCaseFixture } from '../membership'
import { ApplicantDetails, ApplyOnOpeningsHappyCaseFixture } from './ApplyOnOpeningsHappyCaseFixture'
import { CreateOpeningsFixture, DEFAULT_OPENING_PARAMS } from './CreateOpeningsFixture'
import { FillOpeningsFixture } from './FillOpeningsFixture'

export class HireWorkersFixture extends BaseQueryNodeFixture {
  protected group: WorkingGroupModuleName
  protected workersN: number
  protected createdWorkerIds: WorkerId[] = []

  protected fillOpeningRunner?: FixtureRunner

  constructor(api: Api, query: QueryNodeApi, group: WorkingGroupModuleName, workersN: number) {
    super(api, query)
    this.group = group
    this.workersN = workersN
  }

  public getCreatedWorkerIds(): WorkerId[] {
    Utils.assert(this.createdWorkerIds.length, 'Trying to get created workers before they were created!')
    return this.createdWorkerIds
  }

  public async execute(): Promise<void> {
    // Transfer funds to leader staking account to cover opening stake
    const leaderStakingAcc = await this.api.getLeaderStakingKey(this.group)
    await this.api.treasuryTransferBalance(leaderStakingAcc, LEADER_OPENING_STAKE)

    // Create an opening
    const createOpeningFixture = new CreateOpeningsFixture(this.api, this.query, this.group)
    const openingRunner = new FixtureRunner(createOpeningFixture)
    await openingRunner.run()
    const [openingId] = createOpeningFixture.getCreatedOpeningIds()
    const { stake: openingStake, metadata: openingMetadata } = DEFAULT_OPENING_PARAMS

    // Create the applications
    const roleAccounts = (await this.api.createKeyPairs(this.workersN)).map((kp) => kp.address)
    const stakingAccounts = (await this.api.createKeyPairs(this.workersN)).map((kp) => kp.address)
    const rewardAccounts = (await this.api.createKeyPairs(this.workersN)).map((kp) => kp.address)

    const buyMembershipFixture = new BuyMembershipHappyCaseFixture(this.api, this.query, roleAccounts)
    await new FixtureRunner(buyMembershipFixture).run()
    const memberIds = buyMembershipFixture.getCreatedMembers()

    const addStakingAccFixture = new AddStakingAccountsHappyCaseFixture(
      this.api,
      this.query,
      memberIds.map((memberId, i) => ({
        asMember: memberId,
        account: stakingAccounts[i],
        stakeAmount: openingStake,
      }))
    )
    await new FixtureRunner(addStakingAccFixture).run()

    const applicants: ApplicantDetails[] = memberIds.map((memberId, i) => ({
      memberId,
      roleAccount: roleAccounts[i],
      stakingAccount: stakingAccounts[i],
      rewardAccount: rewardAccounts[i],
    }))
    const applyOnOpeningFixture = new ApplyOnOpeningsHappyCaseFixture(this.api, this.query, this.group, [
      {
        openingId,
        openingMetadata,
        applicants,
      },
    ])
    const applyRunner = new FixtureRunner(applyOnOpeningFixture)
    await applyRunner.run()
    const applicationIds = await applyOnOpeningFixture.getCreatedApplicationsByOpeningId(openingId)

    // Fill the opening
    const fillOpeningFixture = new FillOpeningsFixture(this.api, this.query, this.group, [openingId], [applicationIds])
    const fillOpeningRunner = new FixtureRunner(fillOpeningFixture)
    await fillOpeningRunner.run()

    this.createdWorkerIds = fillOpeningFixture.getCreatedWorkerIdsByOpeningId(openingId)
  }

  public async runQueryNodeChecks(): Promise<void> {
    Utils.assert(this.fillOpeningRunner)
    await this.fillOpeningRunner.runQueryNodeChecks()
  }
}
