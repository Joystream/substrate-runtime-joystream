import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, PostPath } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumPostFieldsFragment, PostReactedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import _ from 'lodash'
import { MemberId } from '@joystream/types/common'
import { ForumPostReaction } from '@joystream/metadata-protobuf'
import { PostReaction } from '../../graphql/generated/schema'

export type PostReactionParams = PostPath & {
  reactionId: number
  asMember: MemberId
}

export class ReactToPostsFixture extends StandardizedFixture {
  protected reactions: PostReactionParams[]

  public constructor(api: Api, query: QueryNodeApi, reactions: PostReactionParams[]) {
    super(api, query)
    this.reactions = reactions
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.reactions.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.reactions.map((r) =>
      this.api.tx.forum.reactPost(r.asMember, r.categoryId, r.threadId, r.postId, r.reactionId)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'PostReacted')
  }

  protected getExpectedReaction(reactionId: number): PostReaction | null {
    if (reactionId === ForumPostReaction.Reaction.LIKE) {
      return PostReaction.Like
    }

    return null
  }

  protected assertQueriedPostsAreValid(qPosts: ForumPostFieldsFragment[]): void {
    // Check against lastest reaction per user per post
    _.uniqBy([...this.reactions].reverse(), (v) => `${v.postId.toString()}:${v.asMember.toString()}`).map(
      (reaction) => {
        const expectedReaction = this.getExpectedReaction(reaction.reactionId)
        const qPost = qPosts.find((p) => p.id === reaction.postId.toString())
        Utils.assert(qPost, 'Query node: Post not found')

        const qReaction = qPost.reactions.find((r) => r.member.id === reaction.asMember.toString())
        if (expectedReaction) {
          Utils.assert(
            qReaction,
            `Query node: Expected post reaction by member ${reaction.asMember.toString()} not found!`
          )
          assert.equal(qReaction.reaction, expectedReaction)
        } else {
          assert.isUndefined(qReaction)
        }
      }
    )
  }

  protected assertQueryNodeEventIsValid(qEvent: PostReactedEventFieldsFragment, i: number): void {
    const { postId, asMember, reactionId } = this.reactions[i]
    const expectedReaction = this.getExpectedReaction(reactionId)
    assert.equal(qEvent.post.id, postId.toString())
    assert.equal(qEvent.reactingMember.id, asMember.toString())
    if (reactionId && expectedReaction === null) {
      Utils.assert(
        qEvent.reactionResult.__typename === 'PostReactionResultInvalid',
        'Query node: Invalid reaction result'
      )
      assert.equal(qEvent.reactionResult.reactionId, reactionId)
    } else if (!reactionId) {
      Utils.assert(
        qEvent.reactionResult.__typename === 'PostReactionResultCancel',
        'Query node: Invalid reaction result'
      )
    } else {
      Utils.assert(
        qEvent.reactionResult.__typename === 'PostReactionResultValid',
        'Query node: Invalid reaction result'
      )
      assert.equal(qEvent.reactionResult.reaction, expectedReaction)
      assert.equal(qEvent.reactionResult.reactionId, reactionId)
    }
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    await this.query.tryQueryWithTimeout(
      () => this.query.getPostReactedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qPosts = await this.query.getPostsByIds(this.reactions.map((r) => r.postId))
    this.assertQueriedPostsAreValid(qPosts)
  }
}
