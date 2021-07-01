import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { CreateProposalsFixture, DecideOnProposalStatusFixture } from '../../fixtures/proposals'
import { Resource } from '../../Resources'

export default async function exactExecutionBlock({ api, query, lock }: FlowProps): Promise<void> {
  const debug = Debugger('flow:proposal-exact-execution-block')
  debug('Started')
  api.enableDebugTxLogs()

  const unlock = await lock(Resource.Proposals)

  const [account] = (await api.createKeyPairs(1)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, [account])
  await new FixtureRunner(buyMembershipFixture).run()
  const [memberId] = buyMembershipFixture.getCreatedMembers()

  const currentBlock = (await api.getBestBlock()).toNumber()
  const exactExecutionBlock = currentBlock + 50
  const createProposalFixture = new CreateProposalsFixture(api, query, [
    {
      type: 'Signal',
      details: `Proposal to be executed at block ${exactExecutionBlock}`,
      asMember: memberId,
      title: `Executes at #${exactExecutionBlock}`,
      description: `Proposal to be executed at block ${exactExecutionBlock}`,
      exactExecutionBlock,
    },
  ])
  await new FixtureRunner(createProposalFixture).run()
  const [proposalId] = createProposalFixture.getCreatedProposalsIds()

  const approveProposalFixture = new DecideOnProposalStatusFixture(api, query, [{ proposalId, status: 'Approved' }])
  await new FixtureRunner(approveProposalFixture).runWithQueryNodeChecks()

  unlock()

  debug('Done')
}
