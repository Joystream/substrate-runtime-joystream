import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import {
  ProposalDiscussionThreadFieldsFragment,
  ProposalDiscussionThreadModeChangedEventFieldsFragment,
} from '../../graphql/generated/queries'
import { assert } from 'chai'
import { StandardizedFixture } from '../../Fixture'
import { MemberId, ThreadId } from '@joystream/types/common'
import { CreateInterface } from '@joystream/types'
import { ThreadMode } from '@joystream/types/proposals'
import _ from 'lodash'

export type ThreadModeChangeParams = {
  threadId: ThreadId | number
  newMode: CreateInterface<ThreadMode>
  asMember: MemberId
}

export class ChangeThreadsModeFixture extends StandardizedFixture {
  protected threadsModeChangeParams: ThreadModeChangeParams[]

  public constructor(api: Api, query: QueryNodeApi, threadsModeChangeParams: ThreadModeChangeParams[]) {
    super(api, query)
    this.threadsModeChangeParams = threadsModeChangeParams
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.threadsModeChangeParams)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.threadsModeChangeParams.map((params) =>
      this.api.tx.proposalsDiscussion.changeThreadMode(params.asMember, params.threadId, params.newMode)
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveProposalsDiscussionEventDetails(result, 'ThreadModeChanged')
  }

  protected assertQueriedThreadsAreValid(
    qThreads: ProposalDiscussionThreadFieldsFragment[],
    qEvents: ProposalDiscussionThreadModeChangedEventFieldsFragment[]
  ): void {
    for (const [threadId, changes] of _.entries(
      _.groupBy(this.threadsModeChangeParams, (p) => p.threadId.toString())
    )) {
      const finalUpdate = _.last(changes)
      const qThread = qThreads.find((t) => t.id === threadId.toString())
      Utils.assert(qThread, 'Query node: Thread not found!')
      assert.includeDeepMembers(
        qThread.modeChanges.map((e) => e.id),
        qEvents.filter((e) => e.thread.id === qThread.id).map((e) => e.id)
      )
      Utils.assert(finalUpdate)
      const newMode = this.api.createType('ThreadMode', finalUpdate.newMode)
      if (newMode.isOfType('Closed')) {
        Utils.assert(
          qThread.mode.__typename === 'ProposalDiscussionThreadModeClosed',
          `Invalid thread status ${qThread.mode.__typename}`
        )
        Utils.assert(qThread.mode.whitelist, 'Query node: Missing thread.mode.whitelist')
        assert.sameDeepMembers(
          qThread.mode.whitelist.members.map((m) => m.id),
          newMode.asType('Closed').map((memberId) => memberId.toString())
        )
      } else if (newMode.isOfType('Open')) {
        assert.equal(qThread.mode.__typename, 'ProposalDiscussionThreadModeOpen')
      } else {
        throw new Error(`Unknown thread mode: ${newMode.type}`)
      }
    }
  }

  protected assertQueryNodeEventIsValid(
    qEvent: ProposalDiscussionThreadModeChangedEventFieldsFragment,
    i: number
  ): void {
    const params = this.threadsModeChangeParams[i]
    assert.equal(qEvent.thread.id, params.threadId.toString())
    assert.equal(qEvent.actor.id, params.asMember.toString())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getProposalDiscussionThreadModeChangedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the posts
    const qThreads = await this.query.getProposalDiscussionThreadsByIds(
      this.threadsModeChangeParams.map((p) => p.threadId)
    )
    this.assertQueriedThreadsAreValid(qThreads, qEvents)
  }
}
