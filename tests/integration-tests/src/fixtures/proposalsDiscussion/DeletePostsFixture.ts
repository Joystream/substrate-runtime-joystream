import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  ProposalDiscussionPostDeletedEventFieldsFragment,
  ProposalDiscussionPostFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { MemberId, PostId, ThreadId } from '@joystream/types/common'

export type DeletePostParams = {
  threadId: ThreadId | number
  postId: PostId | number
  asMember: MemberId
  hide?: boolean // defaults to true
}

export class DeletePostsFixture extends StandardizedFixture {
  protected deletePostsParams: DeletePostParams[]

  public constructor(api: Api, query: QueryNodeApi, deletePostsParams: DeletePostParams[]) {
    super(api, query)
    this.deletePostsParams = deletePostsParams
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.deletePostsParams)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.deletePostsParams.map((params) =>
      this.api.tx.proposalsDiscussion.deletePost(
        params.asMember,
        params.postId,
        params.threadId,
        params.hide === undefined || params.hide
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveProposalsDiscussionEventDetails(result, 'PostDeleted')
  }

  protected assertQueriedPostsAreValid(
    qPosts: ProposalDiscussionPostFieldsFragment[],
    qEvents: ProposalDiscussionPostDeletedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const params = this.deletePostsParams[i]
      const qPost = qPosts.find((p) => p.id === params.postId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const expectedStatus =
        params.hide === undefined || params.hide
          ? 'ProposalDiscussionPostStatusRemoved'
          : 'ProposalDiscussionPostStatusLocked'
      Utils.assert(qPost, 'Query node: Post not found')
      Utils.assert(qPost.status.__typename === expectedStatus, `Invalid post status (${qPost.status.__typename})`)
      assert.equal(qPost.status.deletedInEvent?.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ProposalDiscussionPostDeletedEventFieldsFragment, i: number): void {
    const params = this.deletePostsParams[i]
    assert.equal(qEvent.post.id, params.postId.toString())
    assert.equal(qEvent.actor.id, params.asMember.toString())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalDiscussionPostDeletedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qPosts = await this.query.getProposalDiscussionPostsByIds(this.deletePostsParams.map((p) => p.postId))
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
