import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, MemberContext } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumThreadWithPostsFieldsFragment, ThreadDeletedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { CategoryId } from '@joystream/types/forum'
import { ThreadId } from '@joystream/types/common'

export type ThreadRemovalInput = {
  threadId: ThreadId
  categoryId: CategoryId
  hide?: boolean // defaults to "true"
}

export class DeleteThreadsFixture extends StandardizedFixture {
  protected removals: ThreadRemovalInput[]
  protected threadAuthors: MemberContext[] = []

  public constructor(api: Api, query: QueryNodeApi, removals: ThreadRemovalInput[]) {
    super(api, query)
    this.removals = removals
  }

  protected async loadAuthors(): Promise<void> {
    this.threadAuthors = await Promise.all(
      this.removals.map(async (r) => {
        const thread = await this.api.query.forum.threadById(r.categoryId, r.threadId)
        const member = await this.api.query.members.membershipById(thread.author_id)
        return { account: member.controller_account.toString(), memberId: thread.author_id }
      })
    )
  }

  public async execute(): Promise<void> {
    await this.loadAuthors()
    await super.execute()
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.threadAuthors.map((a) => a.account)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.removals.map((r, i) =>
      this.api.tx.forum.deleteThread(
        this.threadAuthors[i].memberId,
        r.categoryId,
        r.threadId,
        r.hide === undefined ? true : r.hide
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'ThreadDeleted')
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ForumThreadWithPostsFieldsFragment[],
    qEvents: ThreadDeletedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const removal = this.removals[i]
      const hidden = removal.hide === undefined ? true : removal.hide
      const expectedStatus = hidden ? 'ThreadStatusRemoved' : 'ThreadStatusLocked'
      const qThread = qThreads.find((t) => t.id === removal.threadId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qThread, 'Query node: Thread not found')
      Utils.assert(qThread.status.__typename === expectedStatus, `Invalid thread status. Expected: ${expectedStatus}`)
      Utils.assert(qThread.status.threadDeletedEvent, 'Query node: Missing ThreadDeletedEvent ref')
      assert.equal(qThread.status.threadDeletedEvent.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ThreadDeletedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.thread.id, this.removals[i].threadId.toString())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getThreadDeletedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.removals.map((r) => r.threadId))
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
