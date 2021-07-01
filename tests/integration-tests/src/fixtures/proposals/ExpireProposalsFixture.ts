import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { Utils } from '../../utils'
import { Proposal, ProposalId } from '@joystream/types/proposals'
import { BaseQueryNodeFixture } from '../../Fixture'
import { ProposalFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'

export class ExpireProposalsFixture extends BaseQueryNodeFixture {
  protected proposalIds: ProposalId[]
  protected proposals: Proposal[] = []

  public constructor(api: Api, query: QueryNodeApi, proposalIds: ProposalId[]) {
    super(api, query)
    this.proposalIds = proposalIds
  }

  public async execute(): Promise<void> {
    const { api } = this
    this.proposals = await this.api.query.proposalsEngine.proposals.multi<Proposal>(this.proposalIds)
    await Promise.all(
      this.proposals.map(async (p) => {
        const activatedAt = p.activatedAt.toNumber()
        const expiresAt = activatedAt + p.parameters.votingPeriod.toNumber()
        await api.untilBlock(expiresAt)
      })
    )
  }

  protected assertQueriedProposalsStatusesAreValid(qProposals: ProposalFieldsFragment[]): void {
    this.proposalIds.forEach((id) => {
      const qProposal = qProposals.find((p) => p.id === id.toString())
      Utils.assert(qProposal, 'Query node: Proposal not found')
      Utils.assert(
        qProposal.status.__typename === 'ProposalStatusExpired',
        `Unexpected proposal status: ${qProposal.status.__typename}`
      )
      Utils.assert(qProposal.status.proposalDecisionMadeEvent, 'Missing proposalDecisionMadeEvent relation')
      assert.equal(qProposal.status.proposalDecisionMadeEvent.decisionStatus.__typename, 'ProposalStatusExpired')
    })
  }

  public async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    await this.query.tryQueryWithTimeout(
      () => this.query.getProposalsByIds(this.proposalIds),
      (res) => this.assertQueriedProposalsStatusesAreValid(res)
    )
  }
}
