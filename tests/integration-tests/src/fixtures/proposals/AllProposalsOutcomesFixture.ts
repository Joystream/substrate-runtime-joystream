import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { ProposalDetails } from '@joystream/types/proposals'
import { BaseFixture, FixtureRunner } from '../../Fixture'
import { CreateInterface } from '@joystream/types'
import {
  DecideOnProposalStatusFixture,
  DecideOnProposalStatusParams,
  DecisionStatus,
} from './DecideOnProposalStatusFixture'
import { ProposalDetailsJsonByType, ProposalType } from '../../types'
import { BuyMembershipHappyCaseFixture } from '../membership'
import { CreateProposalsFixture, ProposalCreationParams } from './CreateProposalsFixture'
import { ElectCouncilFixture } from '../council/ElectCouncilFixture'
import _ from 'lodash'
import { Resource, ResourceLocker } from '../../Resources'

export type TestedProposal = { details: CreateInterface<ProposalDetails>; expectExecutionFailure?: boolean }
export type ProposalTestCase = {
  type: ProposalType
  details: ProposalDetailsJsonByType
  decisionStatus: DecisionStatus
  expectExecutionFailure?: boolean
}

export class AllProposalsOutcomesFixture extends BaseFixture {
  protected testedProposals: TestedProposal[]
  protected query: QueryNodeApi
  protected lock: ResourceLocker

  public constructor(api: Api, query: QueryNodeApi, lock: ResourceLocker, testedProposals: TestedProposal[]) {
    super(api)
    this.query = query
    this.lock = lock
    this.testedProposals = testedProposals
  }

  public async execute(): Promise<void> {
    const { api, query, testedProposals } = this

    const decisionStatuses: DecisionStatus[] = ['Approved', 'Rejected', 'Slashed']

    const testCases: ProposalTestCase[] = []

    for (const { details, expectExecutionFailure } of testedProposals) {
      for (const decisionStatus of decisionStatuses) {
        const proposalDetails = api.createType('ProposalDetails', details)
        testCases.push({
          type: proposalDetails.type,
          details: proposalDetails.value,
          decisionStatus,
          expectExecutionFailure,
        })
      }
    }

    const memberKeys = (await api.createKeyPairs(testCases.length)).map((key) => key.address)
    const membersFixture = new BuyMembershipHappyCaseFixture(api, query, memberKeys)
    await new FixtureRunner(membersFixture).run()
    const memberIds = membersFixture.getCreatedMembers()

    const { maxActiveProposalLimit } = api.consts.proposalsEngine
    const proposalsPerBatch = maxActiveProposalLimit.toNumber()

    let batch: ProposalTestCase[]
    let n = 0
    while ((batch = testCases.slice(n * proposalsPerBatch, (n + 1) * proposalsPerBatch)).length) {
      const unlocks = await Promise.all(batch.map(() => this.lock(Resource.Proposals)))
      const createProposalsParams: ProposalCreationParams[] = batch.map(({ type, details, decisionStatus }, i) => ({
        asMember: memberIds[i],
        title: `${_.startCase(type)}`,
        description: `Test ${type} proposal to be ${decisionStatus}`,
        type,
        details: details,
      }))
      this.debug(
        'Creating proposals:',
        createProposalsParams.map((p) => p.type)
      )
      const createProposalsFixure = new CreateProposalsFixture(api, query, createProposalsParams)
      await new FixtureRunner(createProposalsFixure).runWithQueryNodeChecks()
      const proposalIds = createProposalsFixure.getCreatedProposalsIds()

      const decideOnProposalStatusBatch: DecideOnProposalStatusParams[] = batch.map(
        ({ decisionStatus, expectExecutionFailure }, i) => ({
          proposalId: proposalIds[i],
          status: decisionStatus,
          expectExecutionFailure,
        })
      )
      const decideOnProposalsStatusFixture = new DecideOnProposalStatusFixture(api, query, decideOnProposalStatusBatch)
      this.debug(
        'Deciding on proposals:',
        decideOnProposalStatusBatch.map((p) => ({ porposalId: p.proposalId.toString(), status: p.status }))
      )
      await new FixtureRunner(decideOnProposalsStatusFixture).runWithQueryNodeChecks()

      let dormantProposals = decideOnProposalsStatusFixture.getDormantProposalsIds()
      while (dormantProposals.length) {
        // Reelect council
        this.debug('Re-electing council...')
        const electCouncilFixture = new ElectCouncilFixture(api, query)
        await new FixtureRunner(electCouncilFixture).run()
        this.debug('Council re-elected')

        const approveProposalsFixture = new DecideOnProposalStatusFixture(
          api,
          query,
          dormantProposals.map((proposalId) => ({
            proposalId,
            status: 'Approved',
          }))
        )
        this.debug(
          'Deciding on proposals:',
          dormantProposals.map((proposalId) => ({ porposalId: proposalId.toString(), status: 'Approved' }))
        )
        await new FixtureRunner(approveProposalsFixture).runWithQueryNodeChecks()
        dormantProposals = approveProposalsFixture.getDormantProposalsIds()
      }
      unlocks.forEach((unlock) => unlock())
      ++n
    }
  }
}
