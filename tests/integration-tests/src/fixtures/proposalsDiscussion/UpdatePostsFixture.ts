import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  ProposalDiscussionPostFieldsFragment,
  ProposalDiscussionPostUpdatedEventFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import _ from 'lodash'

export type PostUpdateParams = {
  threadId: ThreadId | number
  postId: PostId | number
  newText: string
  asMember: MemberId // Cannot retrieve this information from the runtime currently
}

export class UpdatePostsFixture extends StandardizedFixture {
  protected postsUpdates: PostUpdateParams[]
  protected postsAuthors: MemberId[] = []

  public constructor(api: Api, query: QueryNodeApi, postsUpdates: PostUpdateParams[]) {
    super(api, query)
    this.postsUpdates = postsUpdates
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.postsUpdates)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.postsUpdates.map((params) =>
      this.api.tx.proposalsDiscussion.updatePost(params.threadId, params.postId, params.newText)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveProposalsDiscussionEventDetails(result, 'PostUpdated')
  }

  protected assertQueriedPostsAreValid(
    qPosts: ProposalDiscussionPostFieldsFragment[],
    qEvents: ProposalDiscussionPostUpdatedEventFieldsFragment[]
  ): void {
    for (const [postId, updates] of _.entries(_.groupBy(this.postsUpdates, (p) => p.postId.toString()))) {
      const finalUpdate = _.last(updates)
      const qPost = qPosts.find((p) => p.id === postId.toString())
      Utils.assert(qPost, 'Query node: Post not found!')
      assert.includeDeepMembers(
        qPost.updates.map((e) => e.id),
        qEvents.filter((e) => e.post.id === qPost.id).map((e) => e.id)
      )
      Utils.assert(finalUpdate)
      assert.equal(qPost.text, Utils.asText(finalUpdate.newText))
    }
  }

  protected assertQueryNodeEventIsValid(qEvent: ProposalDiscussionPostUpdatedEventFieldsFragment, i: number): void {
    const params = this.postsUpdates[i]
    assert.equal(qEvent.post.id, params.postId.toString())
    assert.equal(qEvent.text, Utils.asText(params.newText))
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalDiscussionPostUpdatedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qPosts = await this.query.getProposalDiscussionPostsByIds(this.postsUpdates.map((u) => u.postId))
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
