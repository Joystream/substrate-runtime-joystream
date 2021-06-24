import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { MetadataInput, PostAddedEventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumPostFieldsFragment, PostAddedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { CategoryId } from '@joystream/types/forum'
import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import { POST_DEPOSIT } from '../../consts'
import { ForumPostMetadata, IForumPostMetadata } from '@joystream/metadata-protobuf'

export type PostParams = {
  categoryId: CategoryId | number
  threadId: ThreadId | number
  asMember: MemberId
  editable?: boolean // defaults to true
  metadata: MetadataInput<IForumPostMetadata> & { expectReplyFailure?: boolean }
}

export class AddPostsFixture extends StandardizedFixture {
  protected events: PostAddedEventDetails[] = []

  protected postsParams: PostParams[]

  public constructor(api: Api, query: QueryNodeApi, postsParams: PostParams[]) {
    super(api, query)
    this.postsParams = postsParams
  }

  public getCreatedPostsIds(): PostId[] {
    if (!this.events.length) {
      throw new Error('Trying to get created posts ids before they were created!')
    }
    return this.events.map((e) => e.postId)
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return await Promise.all(
      this.postsParams.map(async ({ asMember }) =>
        (await this.api.query.members.membershipById(asMember)).controller_account.toString()
      )
    )
  }

  public async execute(): Promise<void> {
    const accounts = await this.getSignerAccountOrAccounts()
    // Send required funds to accounts (PostDeposit)
    await Promise.all(accounts.map((a) => this.api.treasuryTransferBalance(a, POST_DEPOSIT)))
    await super.execute()
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.postsParams.map((params) =>
      this.api.tx.forum.addPost(
        params.asMember,
        params.categoryId,
        params.threadId,
        Utils.getMetadataBytesFromInput(ForumPostMetadata, params.metadata),
        params.editable === undefined || params.editable
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<PostAddedEventDetails> {
    return this.api.retrievePostAddedEventDetails(result)
  }

  protected getPostExpectedText(postParams: PostParams): string {
    const expectedMetadata = Utils.getDeserializedMetadataFormInput(ForumPostMetadata, postParams.metadata)
    const metadataBytes = Utils.getMetadataBytesFromInput(ForumPostMetadata, postParams.metadata)
    return typeof expectedMetadata?.text === 'string' ? expectedMetadata.text : Utils.bytesToString(metadataBytes)
  }

  protected assertQueriedPostsAreValid(
    qPosts: ForumPostFieldsFragment[],
    qEvents: PostAddedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const qPost = qPosts.find((p) => p.id === e.postId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const postParams = this.postsParams[i]
      const expectedStatus =
        postParams.editable === undefined || postParams.editable ? 'PostStatusActive' : 'PostStatusLocked'
      const expectedMetadata = Utils.getDeserializedMetadataFormInput(ForumPostMetadata, postParams.metadata)
      Utils.assert(qPost, 'Query node: Post not found')
      assert.equal(qPost.thread.id, postParams.threadId.toString())
      assert.equal(qPost.author.id, postParams.asMember.toString())
      assert.equal(qPost.status.__typename, expectedStatus)
      assert.equal(qPost.text, this.getPostExpectedText(postParams))
      assert.equal(
        qPost.repliesTo?.id,
        postParams.metadata.expectReplyFailure ? undefined : expectedMetadata?.repliesTo?.toString()
      )
      Utils.assert(qPost.origin.__typename === 'PostOriginThreadReply', 'Query node: Invalid post origin')
      Utils.assert(qPost.origin.postAddedEvent, 'Query node: PostAddedEvent missing in post origin')
      assert.equal(qPost.origin.postAddedEvent.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: PostAddedEventFieldsFragment, i: number): void {
    const params = this.postsParams[i]
    assert.equal(qEvent.post.id, this.events[i].postId.toString())
    assert.equal(qEvent.text, this.getPostExpectedText(params))
    assert.equal(qEvent.isEditable, params.editable === undefined || params.editable)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getPostAddedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qPosts = await this.query.getPostsByIds(this.events.map((e) => e.postId))
    this.assertQueriedPostsAreValid(qPosts, qEvents)
  }
}
