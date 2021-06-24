import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, PostPath } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumPostFieldsFragment, PostDeletedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import { CategoryId } from '@joystream/types/forum'
import _ from 'lodash'

const DEFAULT_RATIONALE = 'State cleanup'

type SinglePostRemovalInput = PostPath & {
  hide?: boolean // defaults to "true"
}

export type PostsRemovalInput = {
  posts: SinglePostRemovalInput[]
  asMember: MemberId
  rationale?: string
}

export class DeletePostsFixture extends StandardizedFixture {
  protected removals: PostsRemovalInput[]

  public constructor(api: Api, query: QueryNodeApi, removals: PostsRemovalInput[]) {
    super(api, query)
    this.removals = removals
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.removals.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.removals.map((r) =>
      this.api.tx.forum.deletePosts(
        r.asMember,
        r.posts.map(
          ({ categoryId, threadId, postId, hide }) =>
            [categoryId, threadId, postId, hide === undefined || hide] as [CategoryId, ThreadId, PostId, boolean]
        ),
        r.rationale || DEFAULT_RATIONALE
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'PostDeleted')
  }

  protected assertQueriedPostsAreValid(
    qPosts: ForumPostFieldsFragment[],
    qEvents: PostDeletedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const removal = this.removals[i]
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      removal.posts.forEach((postRemoval) => {
        const hidden = postRemoval.hide === undefined || postRemoval.hide
        const expectedStatus = hidden ? 'PostStatusRemoved' : 'PostStatusLocked'
        const qPost = qPosts.find((p) => p.id === postRemoval.postId.toString())
        Utils.assert(qPost, 'Query node: Post not found')
        Utils.assert(qPost.status.__typename === expectedStatus, `Invalid post status. Expected: ${expectedStatus}`)
        Utils.assert(qPost.status.postDeletedEvent, 'Query node: Missing PostDeletedEvent ref')
        assert.equal(qPost.status.postDeletedEvent.id, qEvent.id)
      })
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: PostDeletedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.actor.id, this.removals[i].asMember.toString())
    assert.sameMembers(
      qEvent.posts.map((p) => p.id),
      _.uniq(this.removals[i].posts.map((p) => p.postId.toString()))
    )
    assert.equal(qEvent.rationale, this.removals[i].rationale || DEFAULT_RATIONALE)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getPostDeletedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qPosts = await this.query.getPostsByIds(
      this.removals.reduce((allPostsIds, { posts }) => allPostsIds.concat(posts.map((p) => p.postId)), [] as PostId[])
    )
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
