import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { Utils } from '../../utils'
import { Proposal, ProposalId, VoteKind } from '@joystream/types/proposals'
import { BaseQueryNodeFixture, FixtureRunner } from '../../Fixture'
import { ProposalVote } from './index'
import { CouncilMember } from '@joystream/types/council'
import { VoteOnProposalsFixture } from './VoteOnProposalsFixture'
import { ProposalFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'

export type DecisionStatus = 'Approved' | 'Rejected' | 'Slashed'

type ResultingProposalStatus =
  | 'ProposalStatusDormant'
  | 'ProposalStatusGracing'
  | 'ProposalStatusExecuted'
  | 'ProposalStatusExecutionFailed'
  | 'ProposalStatusSlashed'
  | 'ProposalStatusRejected'

export type DecideOnProposalStatusParams = {
  proposalId: ProposalId
  status: DecisionStatus
  expectExecutionFailure?: boolean
}

export class DecideOnProposalStatusFixture extends BaseQueryNodeFixture {
  protected params: DecideOnProposalStatusParams[]
  protected voteOnProposalsRunner?: FixtureRunner
  protected proposals: Proposal[] = []

  public constructor(api: Api, query: QueryNodeApi, params: DecideOnProposalStatusParams[]) {
    super(api, query)
    this.params = params
  }

  public getDormantProposalsIds(): ProposalId[] {
    if (!this.executed) {
      throw new Error('Trying to get dormant proposal ids before the fixture is executed')
    }
    return this.params
      .filter((p, i) => this.getExpectedProposalStatus(i) === 'ProposalStatusDormant')
      .map((p) => p.proposalId)
  }

  protected getVotes(
    proposalId: ProposalId,
    proposal: Proposal,
    targetStatus: DecisionStatus,
    councilMembers: CouncilMember[]
  ): ProposalVote[] {
    const councilSize = councilMembers.length
    const {
      approvalQuorumPercentage,
      approvalThresholdPercentage,
      slashingQuorumPercentage,
      slashingThresholdPercentage,
    } = proposal.parameters
    const vote = (vote: keyof typeof VoteKind['typeDefinitions'], i: number): ProposalVote => ({
      asMember: councilMembers[i].membership_id,
      proposalId,
      rationale: `Vote ${vote} by member ${i}`,
      vote,
    })
    if (targetStatus === 'Approved') {
      const minVotesN = Math.ceil((councilSize * approvalQuorumPercentage.toNumber()) / 100)
      const minApproveVotesN = Math.ceil((minVotesN * approvalThresholdPercentage.toNumber()) / 100)
      return Array.from({ length: minVotesN }, (v, i) =>
        i < minApproveVotesN ? vote('Approve', i) : vote('Abstain', i)
      )
    } else if (targetStatus === 'Slashed') {
      const minVotesN = Math.ceil((councilSize * slashingQuorumPercentage.toNumber()) / 100)
      const minSlashVotesN = Math.ceil((minVotesN * slashingThresholdPercentage.toNumber()) / 100)
      return Array.from({ length: minVotesN }, (v, i) => (i < minSlashVotesN ? vote('Slash', i) : vote('Abstain', i)))
    } else {
      const otherResultMinThreshold = Math.min(
        approvalThresholdPercentage.toNumber(),
        approvalQuorumPercentage.toNumber()
      )
      const minRejectOrAbstainVotesN = Math.ceil((councilSize * (100 - otherResultMinThreshold)) / 100)
      return Array.from({ length: minRejectOrAbstainVotesN }, (v, i) => vote('Reject', i))
    }
  }

  protected async postExecutionChecks(qProposal: ProposalFieldsFragment): Promise<void> {
    const { details } = qProposal
    if (details.__typename === 'VetoProposalDetails') {
      const [qVetoedProposal] = await this.query.getProposalsByIds([details.proposal!.id])
      Utils.assert(qVetoedProposal.status.__typename === 'ProposalStatusVetoed', 'Invalid proposal status')
    }
    // TODO: Other proposal types
  }

  protected getExpectedProposalStatus(i: number): ResultingProposalStatus {
    const params = this.params[i]
    const proposal = this.proposals[i]
    if (params.status === 'Approved') {
      if (proposal.parameters.constitutionality.toNumber() > proposal.nrOfCouncilConfirmations.toNumber() + 1) {
        return 'ProposalStatusDormant'
      } else if (proposal.parameters.gracePeriod.toNumber() || proposal.exactExecutionBlock.isSome) {
        return 'ProposalStatusGracing'
      } else {
        return params.expectExecutionFailure ? 'ProposalStatusExecutionFailed' : 'ProposalStatusExecuted'
      }
    } else if (params.status === 'Slashed') {
      return 'ProposalStatusSlashed'
    } else {
      return 'ProposalStatusRejected'
    }
  }

  protected assertProposalStatusesAreValid(qProposals: ProposalFieldsFragment[]): void {
    this.params.forEach((params, i) => {
      const qProposal = qProposals.find((p) => p.id === params.proposalId.toString())
      Utils.assert(qProposal, 'Query node: Proposal not found')
      Utils.assert(
        qProposal.status.__typename === this.getExpectedProposalStatus(i),
        `Exepected ${qProposal.status.__typename} to equal ${this.getExpectedProposalStatus(i)}`
      )
      if (
        qProposal.status.__typename === 'ProposalStatusExecuted' ||
        qProposal.status.__typename === 'ProposalStatusExecutionFailed'
      ) {
        Utils.assert(qProposal.status.proposalExecutedEvent?.id, 'Missing proposalExecutedEvent reference')
        assert.equal(qProposal.status.proposalExecutedEvent?.executionStatus.__typename, qProposal.status.__typename)
      } else if (
        qProposal.status.__typename === 'ProposalStatusDormant' ||
        qProposal.status.__typename === 'ProposalStatusGracing'
      ) {
        Utils.assert(qProposal.status.proposalStatusUpdatedEvent?.id, 'Missing proposalStatusUpdatedEvent reference')
        assert.equal(qProposal.status.proposalStatusUpdatedEvent?.newStatus.__typename, qProposal.status.__typename)
        assert.include(
          qProposal.proposalStatusUpdates.map((u) => u.id),
          qProposal.status.proposalStatusUpdatedEvent?.id
        )
      } else {
        Utils.assert(qProposal.status.proposalDecisionMadeEvent?.id, 'Missing proposalDecisionMadeEvent reference')
        assert.equal(qProposal.status.proposalDecisionMadeEvent?.decisionStatus.__typename, qProposal.status.__typename)
      }
    })
  }

  protected assertProposalExecutedAsExpected(qProposal: ProposalFieldsFragment, i: number): void {
    const params = this.params[i]
    const proposal = this.proposals[i]

    assert.equal(
      qProposal.status.__typename,
      params.expectExecutionFailure ? 'ProposalStatusExecutionFailed' : 'ProposalStatusExecuted'
    )
    if (proposal.exactExecutionBlock.isSome) {
      assert.equal(qProposal.statusSetAtBlock, proposal.exactExecutionBlock.unwrap().toNumber())
    } else if (proposal.parameters.gracePeriod.toNumber()) {
      const gracePriodStartedAt = qProposal.proposalStatusUpdates.find(
        (u) => u.newStatus.__typename === 'ProposalStatusGracing'
      )?.inBlock
      assert.equal(qProposal.statusSetAtBlock, (gracePriodStartedAt || 0) + proposal.parameters.gracePeriod.toNumber())
    }
  }

  public async execute(): Promise<void> {
    const { api, query } = this
    this.proposals = await this.api.query.proposalsEngine.proposals.multi<Proposal>(
      this.params.map((p) => p.proposalId)
    )
    const councilMembers = await this.api.query.council.councilMembers()
    Utils.assert(councilMembers.length, 'Council must be elected in order to cast proposal votes')
    let votes: ProposalVote[] = []
    this.params.forEach(({ proposalId, status }, i) => {
      const proposal = this.proposals[i]
      votes = votes.concat(this.getVotes(proposalId, proposal, status, councilMembers))
    })
    this.debug(
      'Casting votes:',
      votes.map((v) => ({ proposalId: v.proposalId.toString(), vote: v.vote.toString() }))
    )
    const voteOnProposalsFixture = new VoteOnProposalsFixture(api, query, votes)
    this.voteOnProposalsRunner = new FixtureRunner(voteOnProposalsFixture)
    await this.voteOnProposalsRunner.run()
  }

  public async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    Utils.assert(this.voteOnProposalsRunner)
    await this.voteOnProposalsRunner.runQueryNodeChecks()

    const qProposals = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalsByIds(this.params.map((p) => p.proposalId)),
      (res) => this.assertProposalStatusesAreValid(res)
    )

    await Promise.all(
      this.proposals.map(async (proposal, i) => {
        let qProposal = qProposals[i]
        if (this.getExpectedProposalStatus(i) === 'ProposalStatusGracing') {
          const proposalExecutionBlock = proposal.exactExecutionBlock.isSome
            ? proposal.exactExecutionBlock.unwrap().toNumber()
            : qProposal.statusSetAtBlock + proposal.parameters.gracePeriod.toNumber()
          await this.api.untilBlock(proposalExecutionBlock)
          ;[qProposal] = await this.query.tryQueryWithTimeout(
            () => this.query.getProposalsByIds([this.params[i].proposalId]),
            ([p]) => this.assertProposalExecutedAsExpected(p, i)
          )
          await this.postExecutionChecks(qProposal)
        }
      })
    )
  }
}
