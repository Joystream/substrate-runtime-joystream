import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { WorkerId } from '@joystream/types/working-group'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  CategoryArchivalStatusUpdatedEventFieldsFragment,
  ForumCategoryFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'
import _ from 'lodash'

export type CategoryStatusUpdate = {
  categoryId: CategoryId
  archived: boolean
  asWorker?: WorkerId
}

export class UpdateCategoriesStatusFixture extends WithForumWorkersFixture {
  protected updates: CategoryStatusUpdate[]

  public constructor(api: Api, query: QueryNodeApi, updates: CategoryStatusUpdate[]) {
    super(api, query)
    this.updates = updates
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return Promise.all(
      this.updates.map(async (u) => {
        const workerId = u.asWorker || (await this.getForumLeadId())
        return (await this.api.query.forumWorkingGroup.workerById(workerId)).role_account_id.toString()
      })
    )
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.updates.map((u) =>
      this.api.tx.forum.updateCategoryArchivalStatus(
        u.asWorker ? { Moderator: u.asWorker } : { Lead: null },
        u.categoryId,
        u.archived
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'CategoryArchivalStatusUpdated')
  }

  protected assertQueriedCategoriesAreValid(
    qCategories: ForumCategoryFieldsFragment[],
    qEvents: CategoryArchivalStatusUpdatedEventFieldsFragment[]
  ): void {
    // Check against latest update per category
    _.uniqBy([...this.updates].reverse(), (v) => v.categoryId).map((update) => {
      const updateIndex = this.updates.indexOf(update)
      const qCategory = qCategories.find((c) => c.id === update.categoryId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(this.events[updateIndex], qEvents)
      Utils.assert(qCategory, 'Query node: Category not found')
      if (update.archived) {
        Utils.assert(qCategory.status.__typename === 'CategoryStatusArchived', 'Invalid category status')
        Utils.assert(
          qCategory.status.categoryArchivalStatusUpdatedEvent,
          'Query node: Missing CategoryArchivalStatusUpdatedEvent ref'
        )
        assert.equal(qCategory.status.categoryArchivalStatusUpdatedEvent.id, qEvent.id)
      } else {
        assert.equal(qCategory.status.__typename, 'CategoryStatusActive')
      }
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: CategoryArchivalStatusUpdatedEventFieldsFragment, i: number): void {
    const { categoryId, archived, asWorker } = this.updates[i]
    assert.equal(qEvent.category.id, categoryId.toString())
    assert.equal(qEvent.newArchivalStatus, archived)
    assert.equal(qEvent.actor.id, `forumWorkingGroup-${asWorker ? asWorker.toString() : this.forumLeadId!.toString()}`)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getCategoryArchivalStatusUpdatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the categories
    const qCategories = await this.query.getCategoriesByIds(this.updates.map((u) => u.categoryId))
    this.assertQueriedCategoriesAreValid(qCategories, qEvents)
  }
}
