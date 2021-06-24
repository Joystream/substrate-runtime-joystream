import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  CategoryStickyThreadUpdateEventFieldsFragment,
  ForumCategoryFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { ThreadId } from '@joystream/types/common'
import { WorkerId } from '@joystream/types/working-group'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'
import _ from 'lodash'

export type StickyThreadsParams = {
  categoryId: CategoryId
  stickyTreads: ThreadId[]
  asWorker?: WorkerId
}

export class SetStickyThreadsFixture extends WithForumWorkersFixture {
  protected events: EventDetails[] = []

  protected stickyThreadsParams: StickyThreadsParams[]

  public constructor(api: Api, query: QueryNodeApi, stickyThreadsParams: StickyThreadsParams[]) {
    super(api, query)
    this.stickyThreadsParams = stickyThreadsParams
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.getSignersFromInput(this.stickyThreadsParams)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.stickyThreadsParams.map((p) =>
      this.api.tx.forum.setStickiedThreads(
        p.asWorker ? { Moderator: p.asWorker } : { Lead: null },
        p.categoryId,
        p.stickyTreads
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'CategoryStickyThreadUpdate')
  }

  protected assertQueriedCategoriesAreValid(qCategories: ForumCategoryFieldsFragment[]): void {
    _.uniqBy([...this.stickyThreadsParams.reverse()], (v) => v.categoryId).forEach((params) => {
      const qCategory = qCategories.find((c) => c.id === params.categoryId.toString())
      Utils.assert(qCategory, 'Query node: Category not found')
      assert.sameDeepMembers(
        qCategory.threads.filter((t) => t.isSticky).map((t) => t.id),
        params.stickyTreads.map((threadId) => threadId.toString())
      )
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: CategoryStickyThreadUpdateEventFieldsFragment, i: number): void {
    const { categoryId, stickyTreads, asWorker } = this.stickyThreadsParams[i]
    assert.equal(qEvent.category.id, categoryId.toString())
    assert.sameDeepMembers(
      qEvent.newStickyThreads.map((t) => t.id),
      stickyTreads.map((threadId) => threadId.toString())
    )
    assert.equal(qEvent.actor.id, `forumWorkingGroup-${asWorker ? asWorker.toString() : this.forumLeadId!.toString()}`)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getCategoryStickyThreadUpdateEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the categories
    const qCategories = await this.query.getCategoriesByIds(this.stickyThreadsParams.map((e) => e.categoryId))
    this.assertQueriedCategoriesAreValid(qCategories)
  }
}
