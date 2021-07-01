import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, WorkingGroupModuleName } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { StatusTextChangedEventFieldsFragment, UpcomingOpeningFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import {
  IUpcomingOpeningMetadata,
  UpcomingOpeningMetadata,
  WorkingGroupMetadataAction,
} from '@joystream/metadata-protobuf'
import Long from 'long'
import { Bytes } from '@polkadot/types'
import moment from 'moment'
import { DEFAULT_OPENING_PARAMS } from './CreateOpeningsFixture'
import { createType } from '@joystream/types'
import { assertQueriedOpeningMetadataIsValid } from './utils'
import { BaseWorkingGroupFixture } from './BaseWorkingGroupFixture'

export const DEFAULT_UPCOMING_OPENING_META: IUpcomingOpeningMetadata = {
  minApplicationStake: Long.fromString(DEFAULT_OPENING_PARAMS.stake.toString()),
  rewardPerBlock: Long.fromString(DEFAULT_OPENING_PARAMS.reward.toString()),
  expectedStart: moment().unix() + 3600,
  metadata: DEFAULT_OPENING_PARAMS.metadata,
}

export type UpcomingOpeningParams = {
  meta: IUpcomingOpeningMetadata | string
  expectMetadataFailure?: boolean
}

export class CreateUpcomingOpeningsFixture extends BaseWorkingGroupFixture {
  protected upcomingOpeningsParams: UpcomingOpeningParams[]
  protected createdUpcomingOpeningIds: string[] = []

  public constructor(api: Api, query: QueryNodeApi, group: WorkingGroupModuleName, params?: UpcomingOpeningParams[]) {
    super(api, query, group)
    this.upcomingOpeningsParams = params || [{ meta: DEFAULT_UPCOMING_OPENING_META }]
  }

  protected async getSignerAccountOrAccounts(): Promise<string> {
    return this.api.getLeadRoleKey(this.group)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.upcomingOpeningsParams.map((params) =>
      this.api.tx[this.group].setStatusText(this.getActionMetadataBytes(params))
    )
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveWorkingGroupsEventDetails(result, this.group, 'StatusTextChanged')
  }

  public getCreatedUpcomingOpeningIds(): string[] {
    if (!this.createdUpcomingOpeningIds.length) {
      throw new Error('Trying to get created UpcomingOpening ids before they are known')
    }
    return this.createdUpcomingOpeningIds
  }

  protected getUpcomingOpeningMeta(params: UpcomingOpeningParams): IUpcomingOpeningMetadata | null {
    if (typeof params.meta === 'string') {
      try {
        return Utils.metadataFromBytes(UpcomingOpeningMetadata, createType('Bytes', params.meta))
      } catch (e) {
        if (!params.expectMetadataFailure) {
          throw e
        }
        return null
      }
    }
    return params.meta
  }

  protected getActionMetadataBytes(params: UpcomingOpeningParams): Bytes {
    const upcomingOpeningMeta = this.getUpcomingOpeningMeta(params)
    if (!upcomingOpeningMeta) {
      return createType('Bytes', params.meta)
    }
    return Utils.metadataToBytes(WorkingGroupMetadataAction, {
      addUpcomingOpening: {
        metadata: upcomingOpeningMeta,
      },
    })
  }

  protected assertQueriedUpcomingOpeningsAreValid(
    qUpcomingOpenings: UpcomingOpeningFieldsFragment[],
    qEvents: StatusTextChangedEventFieldsFragment[]
  ): void {
    this.events.forEach((e, i) => {
      const expectedMeta = this.getUpcomingOpeningMeta(this.upcomingOpeningsParams[i])
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      const qUpcomingOpening = qUpcomingOpenings.find((o) => o.createdInEvent.id === qEvent.id)
      if (expectedMeta) {
        Utils.assert(qUpcomingOpening)
        assert.equal(
          qUpcomingOpening.expectedStart
            ? new Date(qUpcomingOpening.expectedStart).getTime()
            : qUpcomingOpening.expectedStart,
          expectedMeta.expectedStart || null
        )
        assert.equal(qUpcomingOpening.group.name, this.group)
        assert.equal(
          qUpcomingOpening.rewardPerBlock,
          expectedMeta.rewardPerBlock && expectedMeta.rewardPerBlock.toNumber()
            ? expectedMeta.rewardPerBlock.toString()
            : null
        )
        assert.equal(
          qUpcomingOpening.stakeAmount,
          expectedMeta.minApplicationStake && expectedMeta.minApplicationStake.toNumber()
            ? expectedMeta.minApplicationStake.toString()
            : null
        )
        Utils.assert(qEvent.result.__typename === 'UpcomingOpeningAdded')
        assert.equal(qEvent.result.upcomingOpeningId, qUpcomingOpening.id)
        assertQueriedOpeningMetadataIsValid(qUpcomingOpening.metadata, expectedMeta.metadata)
      } else {
        assert.isUndefined(qUpcomingOpening)
      }
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: StatusTextChangedEventFieldsFragment, i: number): void {
    const params = this.upcomingOpeningsParams[i]
    assert.equal(qEvent.group.name, this.group)
    assert.equal(qEvent.metadata, this.getActionMetadataBytes(params).toString())
    assert.equal(
      qEvent.result.__typename,
      params.expectMetadataFailure ? 'InvalidActionMetadata' : 'UpcomingOpeningAdded'
    )
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the event
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getStatusTextChangedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )
    // Query the opening
    const qUpcomingOpenings = await this.query.getUpcomingOpeningsByCreatedInEventIds(qEvents.map((e) => e.id))
    this.assertQueriedUpcomingOpeningsAreValid(qUpcomingOpenings, qEvents)

    this.createdUpcomingOpeningIds = qUpcomingOpenings.map((o) => o.id)
  }
}
