import { assert } from 'chai'
import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails, WorkingGroupModuleName } from '../../types'
import { BaseWorkingGroupFixture } from './BaseWorkingGroupFixture'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { Utils } from '../../utils'
import { IWorkingGroupMetadata, WorkingGroupMetadataAction } from '@joystream/metadata-protobuf'
import {
  StatusTextChangedEventFieldsFragment,
  WorkingGroupFieldsFragment,
  WorkingGroupMetadataFieldsFragment,
} from '../../graphql/generated/queries'
import _ from 'lodash'
import { Bytes } from '@polkadot/types'

export class UpdateGroupStatusFixture extends BaseWorkingGroupFixture {
  protected updates: IWorkingGroupMetadata[]

  public constructor(api: Api, query: QueryNodeApi, group: WorkingGroupModuleName, updates: IWorkingGroupMetadata[]) {
    super(api, query, group)
    this.updates = updates
  }

  protected async getSignerAccountOrAccounts(): Promise<string> {
    return this.api.getLeadRoleKey(this.group)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[][]> {
    return this.updates.map((update) => {
      return [this.api.tx[this.group].setStatusText(this.getActionMetadataBytes(update))]
    })
  }

  protected async getEventFromResult(r: ISubmittableResult): Promise<EventDetails> {
    return this.api.retrieveWorkingGroupsEventDetails(r, this.group, 'StatusTextChanged')
  }

  protected getActionMetadataBytes(update: IWorkingGroupMetadata): Bytes {
    return Utils.metadataToBytes(WorkingGroupMetadataAction, {
      setGroupMetadata: {
        newMetadata: update,
      },
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: StatusTextChangedEventFieldsFragment, i: number): void {
    assert.equal(qEvent.group.name, this.group)
    assert.equal(qEvent.metadata, this.getActionMetadataBytes(this.updates[i]).toString())
    assert.equal(qEvent.result.__typename, 'WorkingGroupMetadataSet')
  }

  protected assertQueriedGroupIsValid(
    qGroup: WorkingGroupFieldsFragment,
    qMeta: WorkingGroupMetadataFieldsFragment
  ): void {
    if (!qGroup.metadata) {
      throw new Error(`Query node: Group metadata is empty!`)
    }
    assert.equal(qGroup.metadata.id, qMeta.id)
  }

  protected assertQueriedMetadataSnapshotsAreValid(
    eventDetails: EventDetails,
    preUpdateSnapshot: WorkingGroupMetadataFieldsFragment | null,
    postUpdateSnapshot: WorkingGroupMetadataFieldsFragment | null,
    update: IWorkingGroupMetadata
  ): asserts postUpdateSnapshot is WorkingGroupMetadataFieldsFragment {
    if (!postUpdateSnapshot) {
      throw new Error('Query node: WorkingGroupMetadata snapshot not found!')
    }
    const expectedMeta = _.mergeWith(preUpdateSnapshot, update, (destValue, sourceValue) =>
      sourceValue === null || sourceValue === undefined ? destValue : sourceValue
    )
    assert.equal(postUpdateSnapshot.status, expectedMeta.status || null)
    assert.equal(postUpdateSnapshot.statusMessage, expectedMeta.statusMessage || null)
    assert.equal(postUpdateSnapshot.description, expectedMeta.description || null)
    assert.equal(postUpdateSnapshot.about, expectedMeta.about || null)
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    // Query & check the event
    const qEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getStatusTextChangedEvents(this.events),
      (qEvents) => this.assertQueryNodeEventsAreValid(qEvents)
    )

    // Query the group
    const qGroup = await this.query.getWorkingGroup(this.group)
    if (!qGroup) {
      throw new Error('Query node: Working group not found!')
    }

    // Query & check the metadata snapshots
    const snapshots = await this.query.getGroupMetaSnapshotsByTimeAsc(qGroup.id)
    let lastSnapshot: WorkingGroupMetadataFieldsFragment | null = null
    this.events.forEach((postUpdateEvent, i) => {
      const postUpdateSnapshotIndex = snapshots.findIndex(
        (s) =>
          s.setInEvent.id === this.query.getQueryNodeEventId(postUpdateEvent.blockNumber, postUpdateEvent.indexInBlock)
      )
      const postUpdateSnapshot = postUpdateSnapshotIndex > -1 ? snapshots[postUpdateSnapshotIndex] : null
      const preUpdateSnapshot = postUpdateSnapshotIndex > 0 ? snapshots[postUpdateSnapshotIndex - 1] : null
      this.assertQueriedMetadataSnapshotsAreValid(
        postUpdateEvent,
        preUpdateSnapshot,
        postUpdateSnapshot,
        this.updates[i]
      )
      const qEvent = qEvents[i]
      Utils.assert(
        qEvent.result.__typename === 'WorkingGroupMetadataSet',
        'Invalid StatusTextChanged event result type'
      )
      Utils.assert(qEvent.result.metadata, 'Query node: Missing metadata relation')
      assert(qEvent.result.metadata.id, postUpdateSnapshot.id)
      lastSnapshot = postUpdateSnapshot
    })

    // Check the group
    if (lastSnapshot) {
      this.assertQueriedGroupIsValid(qGroup, lastSnapshot)
    }
  }
}
