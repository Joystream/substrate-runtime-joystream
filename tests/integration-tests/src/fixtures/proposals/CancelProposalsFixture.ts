import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ProposalCancelledEventFieldsFragment, ProposalFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { Proposal, ProposalId } from '@joystream/types/proposals'
import { StandardizedFixture } from '../../Fixture'

export class CancelProposalsFixture extends StandardizedFixture {
  protected proposalIds: ProposalId[]
  protected proposals: Proposal[] = []

  public constructor(api: Api, query: QueryNodeApi, proposalIds: ProposalId[]) {
    super(api, query)
    this.proposalIds = proposalIds
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.proposals.map((p) => ({ asMember: p.proposerId })))
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.proposalIds.map((proposalId, i) =>
      this.api.tx.proposalsEngine.cancelProposal(this.proposals[i].proposerId, proposalId)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveProposalsEngineEventDetails(result, 'ProposalCancelled')
  }

  protected assertQueriedProposalsAreValid(
    qProposals: ProposalFieldsFragment[],
    qEvents: ProposalCancelledEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const proposalId = this.proposalIds[i]
      const qProposal = qProposals.find((p) => p.id === proposalId.toString())
      Utils.assert(qProposal, 'Query node: Proposal not found')
      Utils.assert(qProposal.status.__typename === 'ProposalStatusCancelled', 'Invalid proposal status')
      assert.equal(qProposal.status.cancelledInEvent?.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ProposalCancelledEventFieldsFragment, i: number): void {
    const proposalId = this.proposalIds[i]
    assert.equal(qEvent.proposal.id, proposalId.toString())
  }

  public async execute(): Promise<void> {
    this.proposals = await this.api.query.proposalsEngine.proposals.multi<Proposal>(this.proposalIds)
    await super.execute()
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalCancelledEvents(this.events),
      (result) => this.assertQueryNodeEventsAreValid(result)
    )

    // Query the proposals
    const qProposals = await this.query.getProposalsByIds(this.proposalIds)
    this.assertQueriedProposalsAreValid(qProposals, qEvents)
  }
}
