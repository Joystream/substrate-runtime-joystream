import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { WorkerId } from '@joystream/types/working-group'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ForumThreadWithPostsFieldsFragment, ThreadModeratedEventFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { CategoryId } from '@joystream/types/forum'
import { WithForumWorkersFixture } from './WithForumWorkersFixture'
import { ThreadId } from '@joystream/types/common'

export type ThreadModerationInput = {
  categoryId: CategoryId
  threadId: ThreadId
  rationale?: string
  asWorker?: WorkerId
}

export const DEFAULT_RATIONALE = 'Bad thread'

export class ModerateThreadsFixture extends WithForumWorkersFixture {
  protected moderations: ThreadModerationInput[]

  public constructor(api: Api, query: QueryNodeApi, moderations: ThreadModerationInput[]) {
    super(api, query)
    this.moderations = moderations
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.getSignersFromInput(this.moderations)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.moderations.map((u) =>
      this.api.tx.forum.moderateThread(
        u.asWorker ? { Moderator: u.asWorker } : { Lead: null },
        u.categoryId,
        u.threadId,
        u.rationale || DEFAULT_RATIONALE
      )
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveForumEventDetails(result, 'ThreadModerated')
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ForumThreadWithPostsFieldsFragment[],
    qEvents: ThreadModeratedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const moderation = this.moderations[i]
      const qThread = qThreads.find((t) => t.id === moderation.threadId.toString())
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qThread, 'Query node: Thread not found')
      Utils.assert(qThread.status.__typename === 'ThreadStatusModerated', 'Invalid thread status')
      Utils.assert(qThread.status.threadModeratedEvent, 'Query node: Missing ThreadModeratedEvent ref')
      assert.equal(qThread.status.threadModeratedEvent.id, qEvent.id)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ThreadModeratedEventFieldsFragment, i: number): void {
    const { threadId, asWorker, rationale } = this.moderations[i]
    assert.equal(qEvent.thread.id, threadId.toString())
    assert.equal(qEvent.actor.id, `forumWorkingGroup-${asWorker ? asWorker.toString() : this.forumLeadId!.toString()}`)
    assert.equal(qEvent.rationale, rationale || DEFAULT_RATIONALE)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getThreadModeratedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the threads
    const qThreads = await this.query.getThreadsWithPostsByIds(this.moderations.map((m) => m.threadId))
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
