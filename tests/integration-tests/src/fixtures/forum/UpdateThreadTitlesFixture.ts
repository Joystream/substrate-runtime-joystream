import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, MemberContext } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  ForumThreadWithPostsFieldsFragment,
  ThreadTitleUpdatedEventFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { StandardizedFixture } from '../../Fixture'
import { ThreadId } from '@joystream/types/common'
import _ from 'lodash'

export type ThreadTitleUpdate = {
  categoryId: CategoryId
  threadId: ThreadId
  newTitle: string
}

export class UpdateThreadTitlesFixture extends StandardizedFixture {
  protected threadAuthors: MemberContext[] = []
  protected updates: ThreadTitleUpdate[]

  public constructor(api: Api, query: QueryNodeApi, updates: ThreadTitleUpdate[]) {
    super(api, query)
    this.updates = updates
  }

  protected async loadAuthors(): Promise<void> {
    this.threadAuthors = await Promise.all(
      this.updates.map(async (u) => {
        const thread = await this.api.query.forum.threadById(u.categoryId, u.threadId)
        const member = await this.api.query.members.membershipById(thread.author_id)
        return { account: member.controller_account.toString(), memberId: thread.author_id }
      })
    )
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.threadAuthors.map((a) => a.account)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.updates.map((u, i) =>
      this.api.tx.forum.editThreadTitle(this.threadAuthors[i].memberId, u.categoryId, u.threadId, u.newTitle)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'ThreadTitleUpdated')
  }

  public async execute(): Promise<void> {
    await this.loadAuthors()
    await super.execute()
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ForumThreadWithPostsFieldsFragment[],
    qEvents: ThreadTitleUpdatedEventFieldsFragment[]
  ): void {
    // Check titleUpdates array
    this.events.forEach((e, i) => {
      const update = this.updates[i]
      const qThread = qThreads.find((t) => t.id === update.threadId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qThread, 'Query node: Thread not found')
      assert.include(
        qThread.titleUpdates.map((u) => u.id),
        qEvent.id
      )
    })

    // Check updated titles (against lastest update per thread)
    _.uniqBy([...this.updates].reverse(), (v) => v.threadId).map((update) => {
      const qThread = qThreads.find((t) => t.id === update.threadId.toString())
      Utils.assert(qThread, 'Query node: Thread not found')
      assert.equal(qThread.title, update.newTitle)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ThreadTitleUpdatedEventFieldsFragment, i: number): void {
    const { threadId, newTitle } = this.updates[i]
    assert.equal(qEvent.thread.id, threadId.toString())
    assert.equal(qEvent.newTitle, newTitle)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getThreadTitleUpdatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.updates.map((u) => u.threadId))
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
