import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, MemberContext, PostPath } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumPostFieldsFragment, PostTextUpdatedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import _ from 'lodash'
import { PostTextUpdatedEvent } from '../../graphql/generated/schema'

export type PostTextUpdate = PostPath & {
  newText: string
}

export class UpdatePostsTextFixture extends StandardizedFixture {
  protected postAuthors: MemberContext[] = []
  protected updates: PostTextUpdate[]

  public constructor(api: Api, query: QueryNodeApi, updates: PostTextUpdate[]) {
    super(api, query)
    this.updates = updates
  }

  protected async loadAuthors(): Promise<void> {
    this.postAuthors = await Promise.all(
      this.updates.map(async (u) => {
        const post = await this.api.query.forum.postById(u.threadId, u.postId)
        const member = await this.api.query.members.membershipById(post.author_id)
        return { account: member.controller_account.toString(), memberId: post.author_id }
      })
    )
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.postAuthors.map((a) => a.account)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.updates.map((u, i) =>
      this.api.tx.forum.editPostText(this.postAuthors[i].memberId, u.categoryId, u.threadId, u.postId, u.newText)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'PostTextUpdated')
  }

  public async execute(): Promise<void> {
    await this.loadAuthors()
    await super.execute()
  }

  protected assertQueriedPostsAreValid(
    qPosts: ForumPostFieldsFragment[],
    qEvents: PostTextUpdatedEventFieldsFragment[]
  ): void {
    // Check update events are included in posts One-to-Many relation
    this.events.forEach((e, i) => {
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const qPost = qPosts.find((p) => p.id === this.updates[i].postId.toString())
      Utils.assert(qPost, 'Query node: Post not found')
      assert.include(
        qPost.edits.map((e) => e.id),
        qEvent.id
      )
    })

    // Check post text against lastest update per post
    _.uniqBy([...this.updates].reverse(), (v) => v.postId).map((update) => {
      const qPost = qPosts.find((p) => p.id === update.postId.toString())
      Utils.assert(qPost, 'Query node: Post not found')
      assert.equal(qPost.text, update.newText)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: PostTextUpdatedEvent, i: number): void {
    const { postId, newText } = this.updates[i]
    assert.equal(qEvent.post.id, postId.toString())
    assert.equal(qEvent.newText, newText)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getPostTextUpdatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qPosts = await this.query.getPostsByIds(this.updates.map((u) => u.postId))
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
