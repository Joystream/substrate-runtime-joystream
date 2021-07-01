import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { CreateProposalsFixture, DecideOnProposalStatusFixture } from '../../fixtures/proposals'
import { Resource } from '../../Resources'

export default async function vetoProposal({ api, query, lock }: FlowProps): Promise<void> {
  const debug = Debugger('flow:creating-proposals')
  debug('Started')
  api.enableDebugTxLogs()

  const unlocks = await Promise.all(Array.from({ length: 2 }, () => lock(Resource.Proposals)))

  const [account] = (await api.createKeyPairs(1)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, [account])
  await new FixtureRunner(buyMembershipFixture).run()
  const [memberId] = buyMembershipFixture.getCreatedMembers()

  const createProposalFixture = new CreateProposalsFixture(api, query, [
    {
      type: 'Signal',
      details: 'Proposal to be vetoed',
      asMember: memberId,
      title: 'Proposal to veto',
      description: 'Proposal to be vetoed',
    },
  ])
  await new FixtureRunner(createProposalFixture).run()
  const [proposalId] = createProposalFixture.getCreatedProposalsIds()

  const createVetoProposalFixture = new CreateProposalsFixture(api, query, [
    {
      type: 'VetoProposal',
      details: proposalId,
      asMember: memberId,
      title: 'Veto proposal',
      description: 'Test veto proposal',
    },
  ])
  await new FixtureRunner(createVetoProposalFixture).run()
  const [vetoProposalId] = createVetoProposalFixture.getCreatedProposalsIds()

  const decideOnProposalStatusFixture = new DecideOnProposalStatusFixture(api, query, [
    { proposalId: vetoProposalId, status: 'Approved' },
  ])
  await new FixtureRunner(decideOnProposalStatusFixture).runWithQueryNodeChecks()

  unlocks.forEach((unlock) => unlock())

  debug('Done')
}
