import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { ThreadCreatedEventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumThreadWithPostsFieldsFragment, ThreadCreatedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { CategoryId, Poll } from '@joystream/types/forum'
import { MemberId, ThreadId } from '@joystream/types/common'
import { CreateInterface } from '@joystream/types'
import { POST_DEPOSIT, THREAD_DEPOSIT } from '../../consts'

export type PollParams = {
  description: string
  endTime: Date
  alternatives: string[]
}

export type ThreadParams = {
  title: string
  text: string
  categoryId: CategoryId
  asMember: MemberId
  poll?: PollParams
}

export class CreateThreadsFixture extends StandardizedFixture {
  protected events: ThreadCreatedEventDetails[] = []

  protected threadsParams: ThreadParams[]

  public constructor(api: Api, query: QueryNodeApi, threadsParams: ThreadParams[]) {
    super(api, query)
    this.threadsParams = threadsParams
  }

  public getCreatedThreadsIds(): ThreadId[] {
    if (!this.events.length) {
      throw new Error('Trying to get created threads ids before they were created!')
    }
    return this.events.map((e) => e.threadId)
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.threadsParams.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  public async execute(): Promise<void> {
    const accounts = await this.getSignerAccountOrAccounts()
    // Send required funds to accounts (ThreadDeposit + PostDeposit)
    await Promise.all(accounts.map((a) => this.api.treasuryTransferBalance(a, THREAD_DEPOSIT.add(POST_DEPOSIT))))
    await super.execute()
  }

  protected parsePollParams(pollParams?: PollParams): CreateInterface<Poll> | null {
    if (!pollParams) {
      return null
    }

    return {
      description_hash: pollParams.description,
      end_time: pollParams.endTime.getTime(),
      poll_alternatives: pollParams.alternatives.map((a) => ({
        alternative_text_hash: a,
        vote_count: 0,
      })),
    }
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.threadsParams.map((params) =>
      this.api.tx.forum.createThread(
        params.asMember,
        params.categoryId,
        params.title,
        params.text,
        this.parsePollParams(params.poll)
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<ThreadCreatedEventDetails> {
    return this.api.retrieveThreadCreatedEventDetails(result)
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ForumThreadWithPostsFieldsFragment[],
    qEvents: ThreadCreatedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const qThread = qThreads.find((t) => t.id === e.threadId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const threadParams = this.threadsParams[i]
      Utils.assert(qThread, 'Query node: Thread not found')
      assert.equal(qThread.title, threadParams.title)
      assert.equal(qThread.category.id, threadParams.categoryId.toString())
      assert.equal(qThread.author.id, threadParams.asMember.toString())
      assert.equal(qThread.status.__typename, 'ThreadStatusActive')
      assert.equal(qThread.isSticky, false)
      assert.equal(qThread.createdInEvent.id, qEvent.id)
      const initialPost = qThread.posts.find((p) => p.origin.__typename === 'PostOriginThreadInitial')
      Utils.assert(initialPost, "Query node: Thread's initial post not found!")
      assert.equal(initialPost.text, threadParams.text)
      Utils.assert(initialPost.origin.__typename === 'PostOriginThreadInitial')
      // FIXME: Temporarly not working (https://github.com/Joystream/hydra/issues/396)
      // Utils.assert(initialPost.origin.threadCreatedEvent, 'Query node: Post origin ThreadCreatedEvent ref missing')
      // assert.equal(initialPost.origin.threadCreatedEvent.id, qEvent.id)
      assert.equal(initialPost.author.id, threadParams.asMember.toString())
      assert.equal(initialPost.status.__typename, 'PostStatusActive')
      if (threadParams.poll) {
        Utils.assert(qThread.poll, 'Query node: Thread poll is missing')
        assert.equal(qThread.poll.description, threadParams.poll.description)
        assert.equal(new Date(qThread.poll.endTime).getTime(), threadParams.poll.endTime.getTime())
      }
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ThreadCreatedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.thread.id, this.events[i].threadId.toString())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getThreadCreatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.events.map((e) => e.threadId))
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
