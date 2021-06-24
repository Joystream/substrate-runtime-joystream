import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { CategoryCreatedEventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { CategoryCreatedEventFieldsFragment, ForumCategoryFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { CategoryId } from '@joystream/types/forum'

export type CategoryParams = {
  title: string
  description: string
  parentId?: CategoryId
}

export class CreateCategoriesFixture extends StandardizedFixture {
  protected events: CategoryCreatedEventDetails[] = []

  protected categoriesParams: CategoryParams[]

  public constructor(api: Api, query: QueryNodeApi, categoriesParams: CategoryParams[]) {
    super(api, query)
    this.categoriesParams = categoriesParams
  }

  public getCreatedCategoriesIds(): CategoryId[] {
    if (!this.events.length) {
      throw new Error('Trying to get created categories ids before they were created!')
    }
    return this.events.map((e) => e.categoryId)
  }

  protected async getSignerAccountOrAccounts(): Promise<string> {
    return await this.api.getLeadRoleKey('forumWorkingGroup')
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.categoriesParams.map((params) =>
      this.api.tx.forum.createCategory(params.parentId || null, params.title, params.description)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<CategoryCreatedEventDetails> {
    return this.api.retrieveCategoryCreatedEventDetails(result)
  }

  protected assertQueriedCategoriesAreValid(qCategories: ForumCategoryFieldsFragment[]): void {
    this.events.map((e, i) => {
      const qCategory = qCategories.find((c) => c.id === e.categoryId.toString())
      const categoryParams = this.categoriesParams[i]
      Utils.assert(qCategory, 'Query node: Category not found')
      assert.equal(qCategory.description, categoryParams.description)
      assert.equal(qCategory.title, categoryParams.title)
      if (categoryParams.parentId) {
        Utils.assert(qCategory.parent, 'Query node: Category parent was expected, but not set')
        assert.equal(qCategory.parent.id, categoryParams.parentId.toString())
      }
      assert.equal(qCategory.status.__typename, 'CategoryStatusActive')
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: CategoryCreatedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.category.id, this.events[i].categoryId.toString())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getCategoryCreatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the categories
    const qCategories = await this.query.getCategoriesByIds(this.events.map((e) => e.categoryId))
    this.assertQueriedCategoriesAreValid(qCategories)
  }
}
