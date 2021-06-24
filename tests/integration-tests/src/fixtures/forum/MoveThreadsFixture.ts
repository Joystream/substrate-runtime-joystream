import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumThreadWithPostsFieldsFragment, ThreadMovedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { ThreadId } from '@joystream/types/common'
import _ from 'lodash'
import { WorkerId } from '@joystream/types/working-group'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'

export type MoveThreadParams = {
  categoryId: CategoryId
  threadId: ThreadId
  newCategoryId: CategoryId
  asWorker?: WorkerId
}

export class MoveThreadsFixture extends WithForumWorkersFixture {
  protected updates: MoveThreadParams[]

  public constructor(api: Api, query: QueryNodeApi, updates: MoveThreadParams[]) {
    super(api, query)
    this.updates = updates
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.getSignersFromInput(this.updates)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.updates.map((u) =>
      this.api.tx.forum.moveThreadToCategory(
        u.asWorker ? { Moderator: u.asWorker } : { Lead: null },
        u.categoryId,
        u.threadId,
        u.newCategoryId
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'ThreadMoved')
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ForumThreadWithPostsFieldsFragment[],
    qEvents: ThreadMovedEventFieldsFragment[]
  ): void {
    // Check movedInEvents array
    this.events.forEach((e, i) => {
      const update = this.updates[i]
      const qThread = qThreads.find((t) => t.id === update.threadId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qThread, 'Query node: Thread not found')
      assert.include(
        qThread.movedInEvents.map((e) => e.id),
        qEvent.id
      )
    })

    // Check updated categories (against lastest update per thread)
    _.uniqBy([...this.updates].reverse(), (v) => v.threadId).map((update) => {
      const qThread = qThreads.find((t) => t.id === update.threadId.toString())
      Utils.assert(qThread, 'Query node: Thread not found')
      assert.equal(qThread.category.id, update.newCategoryId.toString())
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ThreadMovedEventFieldsFragment, i: number): void {
    const { threadId, categoryId, newCategoryId, asWorker } = this.updates[i]
    assert.equal(qEvent.thread.id, threadId.toString())
    assert.equal(qEvent.oldCategory.id, categoryId.toString())
    assert.equal(qEvent.newCategory.id, newCategoryId.toString())
    assert.equal(qEvent.actor.id, `forumWorkingGroup-${asWorker ? asWorker.toString() : this.forumLeadId!.toString()}`)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getThreadMovedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.updates.map((u) => u.threadId))
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
