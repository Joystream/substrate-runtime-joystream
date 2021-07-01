import { FlowProps } from '../../Flow'
import {
  ApplyOnOpeningsHappyCaseFixture,
  CreateOpeningsFixture,
  FillOpeningsFixture,
  ApplicantDetails,
  DEFAULT_OPENING_PARAMS,
} from '../../fixtures/workingGroups'

import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { AddStakingAccountsHappyCaseFixture, BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { workingGroups } from '../../consts'

export default async function leadOpening({ api, query, env }: FlowProps): Promise<void> {
  await Promise.all(
    workingGroups.map(async (group) => {
      const debug = Debugger(`flow:lead-opening:${group}`)
      debug('Started')
      api.enableDebugTxLogs()

      const createOpeningFixture = new CreateOpeningsFixture(api, query, group, undefined, true)
      const openingRunner = new FixtureRunner(createOpeningFixture)
      await openingRunner.run()
      const [openingId] = createOpeningFixture.getCreatedOpeningIds()
      const { stake: openingStake, metadata: openingMetadata } = DEFAULT_OPENING_PARAMS

      const [roleAccount, stakingAccount, rewardAccount] = (await api.createKeyPairs(3)).map((kp) => kp.address)
      const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, [roleAccount])
      await new FixtureRunner(buyMembershipFixture).run()
      const [memberId] = buyMembershipFixture.getCreatedMembers()

      const addStakingAccFixture = new AddStakingAccountsHappyCaseFixture(api, query, [
        {
          asMember: memberId,
          account: stakingAccount,
        },
      ])
      await new FixtureRunner(addStakingAccFixture).run()
      await api.treasuryTransferBalance(stakingAccount, openingStake)

      const applicantDetails: ApplicantDetails = {
        memberId,
        roleAccount,
        rewardAccount,
        stakingAccount,
      }

      const applyOnOpeningFixture = new ApplyOnOpeningsHappyCaseFixture(api, query, group, [
        {
          openingId,
          openingMetadata,
          applicants: [applicantDetails],
        },
      ])
      const applicationRunner = new FixtureRunner(applyOnOpeningFixture)
      await applicationRunner.run()
      const [applicationId] = applyOnOpeningFixture.getCreatedApplicationsByOpeningId(openingId)

      // Run query node checks once this part of the flow is done
      await Promise.all([openingRunner.runQueryNodeChecks(), applicationRunner.runQueryNodeChecks()])

      // Fill opening
      const fillOpeningFixture = new FillOpeningsFixture(api, query, group, [openingId], [[applicationId]], true)
      await new FixtureRunner(fillOpeningFixture).runWithQueryNodeChecks()

      debug('Done')
    })
  )
}
