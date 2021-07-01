import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { Utils } from '../../utils'
import fs from 'fs'
import {
  CreateProposalsFixture,
  DecideOnProposalStatusFixture,
  AllProposalsOutcomesFixture,
  TestedProposal,
} from '../../fixtures/proposals'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { assert } from 'chai'
import { Resource } from '../../Resources'

export default async function runtimeUpgradeProposal({ api, query, lock, env }: FlowProps): Promise<void> {
  const debug = Debugger('flow:runtime-upgrade-proposal')
  debug('Started')
  api.enableVerboseTxLogs()

  const unlocks = await Promise.all(Array.from({ length: 2 }, () => lock(Resource.Proposals)))

  const runtimeUpgradeWasmPath = env.RUNTIME_UPGRADE_TARGET_WASM_PATH

  Utils.assert(
    runtimeUpgradeWasmPath && fs.existsSync(runtimeUpgradeWasmPath),
    'Invalid RUNTIME_UPGRADE_TARGET_WASM_PATH'
  )

  // Proposals to be "CancelledByRuntime"
  const [memberAcc] = (await api.createKeyPairs(1)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, [memberAcc])
  await new FixtureRunner(buyMembershipFixture).run()
  const [memberId] = buyMembershipFixture.getCreatedMembers()
  const createProposalsFixture = new CreateProposalsFixture(api, query, [
    {
      type: 'RuntimeUpgrade',
      details: Utils.readRuntimeFromFile(runtimeUpgradeWasmPath),
      asMember: memberId,
      title: 'To be cancelled by runtime',
      description: 'Proposal to be cancelled by runtime',
    },
  ])
  await new FixtureRunner(createProposalsFixture).run()
  const [toBeCanceledByRuntimeProposalId] = createProposalsFixture.getCreatedProposalsIds()
  // Currently we use constitutionality === 2 for runtime upgrade, so we need to approve the proposal
  // in order for it not to get "Rejected" during new council election
  const decideOnProposalStatusFixture = new DecideOnProposalStatusFixture(api, query, [
    { proposalId: toBeCanceledByRuntimeProposalId, status: 'Approved' },
  ])
  await new FixtureRunner(decideOnProposalStatusFixture).run()

  // Runtime upgrade proposal
  const testedProposals: TestedProposal[] = [
    { details: { RuntimeUpgrade: Utils.readRuntimeFromFile(runtimeUpgradeWasmPath) } },
  ]
  const testAllOutcomesFixture = new AllProposalsOutcomesFixture(api, query, lock, testedProposals)
  await new FixtureRunner(testAllOutcomesFixture).run()

  // Check the "CancelledByRuntime" proposal status
  await query.tryQueryWithTimeout(
    () => query.getProposalsByIds([toBeCanceledByRuntimeProposalId]),
    ([proposal]) => {
      Utils.assert(
        proposal.status.__typename === 'ProposalStatusCanceledByRuntime',
        `Proposal expected to be CanceledByRuntime. Actual status: ${proposal.status.__typename}`
      )
      Utils.assert(proposal.status.proposalDecisionMadeEvent, 'Missing proposalDecisionMadeEvent reference')
      assert.equal(
        proposal.status.proposalDecisionMadeEvent.decisionStatus.__typename,
        'ProposalStatusCanceledByRuntime'
      )
    }
  )

  unlocks.map((unlock) => unlock())

  debug('Done')
}
