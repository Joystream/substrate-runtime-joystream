import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { WorkerId } from '@joystream/types/working-group'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumPostFieldsFragment, PostModeratedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'
import { PostId, ThreadId } from '@joystream/types/common'

export type PostModerationInput = {
  categoryId: CategoryId
  threadId: ThreadId
  postId: PostId
  rationale?: string
  asWorker?: WorkerId
}

export const DEFAULT_RATIONALE = 'Bad post'

export class ModeratePostsFixture extends WithForumWorkersFixture {
  protected moderations: PostModerationInput[]

  public constructor(api: Api, query: QueryNodeApi, moderations: PostModerationInput[]) {
    super(api, query)
    this.moderations = moderations
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.getSignersFromInput(this.moderations)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.moderations.map((u) =>
      this.api.tx.forum.moderatePost(
        u.asWorker ? { Moderator: u.asWorker } : { Lead: null },
        u.categoryId,
        u.threadId,
        u.postId,
        u.rationale || DEFAULT_RATIONALE
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'PostModerated')
  }

  protected assertQueriedPostsAreValid(
    qPosts: ForumPostFieldsFragment[],
    qEvents: PostModeratedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const moderation = this.moderations[i]
      const qPost = qPosts.find((p) => p.id === moderation.postId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qPost, 'Query node: Post not found')
      Utils.assert(qPost.status.__typename === 'PostStatusModerated', 'Invalid post status')
      Utils.assert(qPost.status.postModeratedEvent, 'Query node: Missing PostModeratedEvent ref')
      assert.equal(qPost.status.postModeratedEvent.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: PostModeratedEventFieldsFragment, i: number): void {
    const { postId, asWorker, rationale } = this.moderations[i]
    assert.equal(qEvent.post.id, postId.toString())
    assert.equal(qEvent.actor.id, `forumWorkingGroup-${asWorker ? asWorker.toString() : this.forumLeadId!.toString()}`)
    assert.equal(qEvent.rationale, rationale || DEFAULT_RATIONALE)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getPostModeratedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qPosts = await this.query.getPostsByIds(this.moderations.map((m) => m.postId))
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
