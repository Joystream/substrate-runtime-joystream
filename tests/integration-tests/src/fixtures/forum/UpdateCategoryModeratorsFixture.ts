import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  CategoryMembershipOfModeratorUpdatedEventFieldsFragment,
  ForumCategoryFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { WorkerId } from '@joystream/types/working-group'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'
import _ from 'lodash'

export type CategoryModeratorStatusUpdate = {
  categoryId: CategoryId
  moderatorId: WorkerId
  canModerate: boolean
}

export class UpdateCategoryModeratorsFixture extends WithForumWorkersFixture {
  protected events: EventDetails[] = []

  protected updates: CategoryModeratorStatusUpdate[]

  public constructor(api: Api, query: QueryNodeApi, updates: CategoryModeratorStatusUpdate[]) {
    super(api, query)
    this.updates = updates
  }

  protected async getSignerAccountOrAccounts(): Promise<string> {
    return await this.api.getLeadRoleKey('forumWorkingGroup')
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.updates.map((u) =>
      this.api.tx.forum.updateCategoryMembershipOfModerator(u.moderatorId, u.categoryId, u.canModerate)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'CategoryMembershipOfModeratorUpdated')
  }

  protected assertQueriedCategoriesAreValid(qCategories: ForumCategoryFieldsFragment[]): void {
    for (const [categoryId, updates] of _.entries(_.groupBy(this.updates, (u) => u.categoryId.toString()))) {
      const finalUpdates = _.uniqBy([...updates.reverse()], (u) => u.moderatorId)
      const qCategory = qCategories.find((c) => c.id === categoryId.toString())
      Utils.assert(qCategory, 'Query node: Category not found!')
      finalUpdates.forEach((finalUpdate) => {
        if (finalUpdate.canModerate) {
          assert.include(
            qCategory.moderators.map((m) => m.id),
            `forumWorkingGroup-${finalUpdate.moderatorId.toString()}`
          )
        } else {
          assert.notInclude(
            qCategory.moderators.map((m) => m.id),
            `forumWorkingGroup-${finalUpdate.moderatorId.toString()}`
          )
        }
      })
    }
  }

  protected assertQueryNodeEventIsValid(
    qEvent: CategoryMembershipOfModeratorUpdatedEventFieldsFragment,
    i: number
  ): void {
    const { categoryId, moderatorId, canModerate } = this.updates[i]
    assert.equal(qEvent.category.id, categoryId.toString())
    assert.equal(qEvent.moderator.id, `forumWorkingGroup-${moderatorId.toString()}`)
    assert.equal(qEvent.newCanModerateValue, canModerate)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getCategoryMembershipOfModeratorUpdatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the categories
    const qCategories = await this.query.getCategoriesByIds(this.updates.map((u) => u.categoryId))
    this.assertQueriedCategoriesAreValid(qCategories)
  }
}
