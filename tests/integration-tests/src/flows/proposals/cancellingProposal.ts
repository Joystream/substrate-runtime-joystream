import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { CreateProposalsFixture, CancelProposalsFixture } from '../../fixtures/proposals'
import { Resource } from '../../Resources'

export default async function cancellingProposals({ api, query, lock }: FlowProps): Promise<void> {
  const debug = Debugger('flow:cancelling-proposals')
  debug('Started')
  api.enableDebugTxLogs()

  const unlock = await lock(Resource.Proposals)

  const [account] = (await api.createKeyPairs(1)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, [account])
  await new FixtureRunner(buyMembershipFixture).run()
  const [memberId] = buyMembershipFixture.getCreatedMembers()

  const createProposalFixture = new CreateProposalsFixture(api, query, [
    {
      type: 'Signal',
      details: 'Proposal to cancel',
      asMember: memberId,
      title: 'Proposal to cancel',
      description: 'Proposal to cancel',
    },
  ])
  await new FixtureRunner(createProposalFixture).run()
  const [proposalId] = createProposalFixture.getCreatedProposalsIds()

  const cancelProposalsFixture = new CancelProposalsFixture(api, query, [proposalId])
  await new FixtureRunner(cancelProposalsFixture).runWithQueryNodeChecks()

  unlock()

  debug('Done')
}
