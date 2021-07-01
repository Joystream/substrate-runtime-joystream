import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ProposalFieldsFragment, ProposalVotedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { Proposal, ProposalId, VoteKind } from '@joystream/types/proposals'
import { MemberId } from '@joystream/types/common'
import { StandardizedFixture } from '../../Fixture'
import { ProposalVoteKind } from '../../graphql/generated/schema'

export type ProposalVote = {
  asMember: MemberId
  proposalId: ProposalId
  vote: keyof typeof VoteKind['typeDefinitions']
  rationale: string
}

const voteKindToQueryNodeVoteKind: { [K in keyof typeof VoteKind['typeDefinitions']]: ProposalVoteKind } = {
  'Abstain': ProposalVoteKind.Abstain,
  'Approve': ProposalVoteKind.Approve,
  'Reject': ProposalVoteKind.Reject,
  'Slash': ProposalVoteKind.Slash,
}

export class VoteOnProposalsFixture extends StandardizedFixture {
  protected votes: ProposalVote[]
  protected proposals: Proposal[] = []

  public constructor(api: Api, query: QueryNodeApi, votes: ProposalVote[]) {
    super(api, query)
    this.votes = votes
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.votes)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.votes.map(({ asMember, proposalId, vote, rationale }) =>
      this.api.tx.proposalsEngine.vote(asMember, proposalId, vote, rationale)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveProposalsEngineEventDetails(result, 'Voted')
  }

  protected assertQueriedProposalsAreValid(
    qProposals: ProposalFieldsFragment[],
    qEvents: ProposalVotedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const vote = this.votes[i]
      const qProposal = qProposals.find((p) => p.id === vote.proposalId.toString())
      Utils.assert(qProposal, 'Query node: Proposal not found')
      assert.include(
        qProposal.votes.map((v) => v.id),
        qEvent.id
      )
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ProposalVotedEventFieldsFragment, i: number): void {
    const vote = this.votes[i]
    assert.equal(qEvent.proposal.id, vote.proposalId.toString())
    assert.equal(qEvent.voter.id, vote.asMember.toString())
    assert.equal(qEvent.votingRound, this.proposals[i].nrOfCouncilConfirmations.toNumber() + 1)
    assert.equal(qEvent.voteKind, voteKindToQueryNodeVoteKind[vote.vote])
    assert.equal(qEvent.rationale, vote.rationale)
  }

  public async execute(): Promise<void> {
    this.proposals = await this.api.query.proposalsEngine.proposals.multi<Proposal>(this.votes.map((v) => v.proposalId))
    await super.execute()
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalVotedEvents(this.events),
      (result) => this.assertQueryNodeEventsAreValid(result)
    )

    // Query the proposals
    const qProposals = await this.query.getProposalsByIds(this.votes.map((v) => v.proposalId))
    this.assertQueriedProposalsAreValid(qProposals, qEvents)
  }
}
