import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { OpeningAddedEventDetails, WorkingGroupModuleName } from '../../types'
import { OpeningId } from '@joystream/types/working-group'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { OpeningAddedEventFieldsFragment, OpeningFieldsFragment } from '../../graphql/generated/queries'
import { WorkingGroupOpeningType } from '../../graphql/generated/schema'
import { assert } from 'chai'
import { MIN_APPLICATION_STAKE, MIN_UNSTANKING_PERIOD } from '../../consts'
import moment from 'moment'
import BN from 'bn.js'
import { IOpeningMetadata, OpeningMetadata } from '@joystream/metadata-protobuf'
import { createType } from '@joystream/types'
import { Bytes } from '@polkadot/types'
import { BaseWorkingGroupFixture } from './BaseWorkingGroupFixture'
import { assertQueriedOpeningMetadataIsValid } from './utils'

export type OpeningParams = {
  stake: BN
  unstakingPeriod: number
  reward: BN
  metadata: IOpeningMetadata | string
  expectMetadataFailure?: boolean
}

export const DEFAULT_OPENING_PARAMS: Omit<OpeningParams, 'metadata'> & { metadata: IOpeningMetadata } = {
  stake: MIN_APPLICATION_STAKE,
  unstakingPeriod: MIN_UNSTANKING_PERIOD,
  reward: new BN(10),
  metadata: {
    shortDescription: 'Test opening',
    description: '# Test opening',
    expectedEndingTimestamp: moment().unix() + 60,
    hiringLimit: 1,
    applicationDetails: '- This is automatically created opening, do not apply!',
    applicationFormQuestions: [
      { question: 'Question 1?', type: OpeningMetadata.ApplicationFormQuestion.InputType.TEXT },
      { question: 'Question 2?', type: OpeningMetadata.ApplicationFormQuestion.InputType.TEXTAREA },
    ],
  },
}

export class CreateOpeningsFixture extends BaseWorkingGroupFixture {
  protected asSudo: boolean
  protected events: OpeningAddedEventDetails[] = []

  protected openingsParams: OpeningParams[]

  public constructor(
    api: Api,
    query: QueryNodeApi,
    group: WorkingGroupModuleName,
    openingsParams?: OpeningParams[],
    asSudo = false
  ) {
    super(api, query, group)
    this.openingsParams = openingsParams || [DEFAULT_OPENING_PARAMS]
    this.asSudo = asSudo
  }

  public getCreatedOpeningIds(): OpeningId[] {
    if (!this.events.length) {
      throw new Error('Trying to get created opening ids before they were created!')
    }
    return this.events.map((e) => e.openingId)
  }

  protected async getSignerAccountOrAccounts(): Promise<string> {
    return this.asSudo ? (await this.api.query.sudo.key()).toString() : await this.api.getLeadRoleKey(this.group)
  }

  protected getOpeningMetadata(params: OpeningParams): IOpeningMetadata | null {
    if (typeof params.metadata === 'string') {
      try {
        return Utils.metadataFromBytes(OpeningMetadata, createType('Bytes', params.metadata))
      } catch (e) {
        if (!params.expectMetadataFailure) {
          throw e
        }
        return null
      }
    }

    return params.metadata
  }

  protected getOpeningMetadataBytes(params: { metadata: IOpeningMetadata | string }): Bytes {
    const { metadata } = params
    return typeof metadata === 'string'
      ? createType('Bytes', metadata)
      : Utils.metadataToBytes(OpeningMetadata, metadata)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    const extrinsics = this.openingsParams.map((params) =>
      this.api.tx[this.group].addOpening(
        this.getOpeningMetadataBytes(params),
        this.asSudo ? 'Leader' : 'Regular',
        { stake_amount: params.stake, leaving_unstaking_period: params.unstakingPeriod },
        params.reward
      )
    )

    return this.asSudo ? extrinsics.map((tx) => this.api.tx.sudo.sudo(tx)) : extrinsics
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<OpeningAddedEventDetails> {
    return this.api.retrieveOpeningAddedEventDetails(result, this.group)
  }

  protected assertQueriedOpeningsAreValid(
    qOpenings: OpeningFieldsFragment[],
    qEvents: OpeningAddedEventFieldsFragment[]
  ): void {
    this.events.map((e, i) => {
      const qOpening = qOpenings.find((o) => o.runtimeId === e.openingId.toNumber())
      const openingParams = this.openingsParams[i]
      const qEvent = this.findMatchingQueryNodeEvent(e, qEvents)
      Utils.assert(qOpening, 'Query node: Opening not found')
      assert.equal(qOpening.runtimeId, e.openingId.toNumber())
      assert.equal(qOpening.createdInEvent.id, qEvent.id)
      assert.equal(qOpening.group.name, this.group)
      assert.equal(qOpening.rewardPerBlock, openingParams.reward.toString())
      assert.equal(qOpening.type, this.asSudo ? WorkingGroupOpeningType.Leader : WorkingGroupOpeningType.Regular)
      assert.equal(qOpening.status.__typename, 'OpeningStatusOpen')
      assert.equal(qOpening.stakeAmount, openingParams.stake.toString())
      assert.equal(qOpening.unstakingPeriod, openingParams.unstakingPeriod)
      // Metadata
      assertQueriedOpeningMetadataIsValid(qOpening.metadata, this.getOpeningMetadata(openingParams))
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: OpeningAddedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.group.name, this.group)
    assert.equal(qEvent.opening.runtimeId, this.events[i].openingId.toNumber())
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query the events
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getOpeningAddedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the openings
    const qOpenings = await this.query.getOpeningsByIds(
      this.events.map((e) => e.openingId),
      this.group
    )
    this.assertQueriedOpeningsAreValid(qOpenings, qEvents)
  }
}
