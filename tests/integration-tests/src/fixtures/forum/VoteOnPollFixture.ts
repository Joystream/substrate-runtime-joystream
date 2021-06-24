import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumThreadWithPostsFieldsFragment, VoteOnPollEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { CategoryId } from '@joystream/types/forum'
import { MemberId, ThreadId } from '@joystream/types/common'
import { Utils } from '../../utils'

export type VoteParams = {
  categoryId: CategoryId
  threadId: ThreadId
  index: number
  asMember: MemberId
}

export class VoteOnPollFixture extends StandardizedFixture {
  protected votes: VoteParams[]

  public constructor(api: Api, query: QueryNodeApi, votes: VoteParams[]) {
    super(api, query)
    this.votes = votes
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.votes.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.votes.map((params) =>
      this.api.tx.forum.voteOnPoll(params.asMember, params.categoryId, params.threadId, params.index)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'VoteOnPoll')
  }

  protected assertQueryNodeEventIsValid(qEvent: VoteOnPollEventFieldsFragment, i: number): void {
    assert.equal(qEvent.pollAlternative.poll.thread.id, this.votes[i].threadId.toString())
    assert.equal(qEvent.pollAlternative.index, this.votes[i].index)
    assert.equal(qEvent.votingMember.id, this.votes[i].asMember.toString())
  }

  protected assertQueriedThreadsAreValid(qThreads: ForumThreadWithPostsFieldsFragment[]): void {
    this.votes.forEach(({ asMember, threadId, index }) => {
      const qThread = qThreads.find((t) => t.id === threadId.toString())
      Utils.assert(qThread, 'Query node: Thread not found')
      Utils.assert(
        qThread.poll?.pollAlternatives[index].votes.find((v) => v.votingMember.id === asMember.toString()),
        'Query node: Member vote not found'
      )
    })
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getVoteOnPollEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.votes.map((v) => v.threadId))
    this.assertQueriedThreadsAreValid(qThreads)
  }
}
