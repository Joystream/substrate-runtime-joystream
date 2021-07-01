import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { CreateProposalsFixture, ExpireProposalsFixture } from '../../fixtures/proposals'
import { Resource } from '../../Resources'

export default async function expireProposal({ api, query, lock }: FlowProps): Promise<void> {
  const debug = Debugger('flow:expire-proposal')
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
      details: `Proposal to be Expired`,
      asMember: memberId,
      title: `Proposal to be Expired`,
      description: `Proposal to be Expired`,
    },
  ])
  await new FixtureRunner(createProposalFixture).run()
  const [proposalId] = createProposalFixture.getCreatedProposalsIds()

  const approveProposalFixture = new ExpireProposalsFixture(api, query, [proposalId])
  await new FixtureRunner(approveProposalFixture).runWithQueryNodeChecks()

  unlock()

  debug('Done')
}
