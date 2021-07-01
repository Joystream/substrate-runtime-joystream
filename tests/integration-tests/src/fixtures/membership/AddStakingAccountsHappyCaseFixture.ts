import { Api } from '../../Api'
import { assert } from 'chai'
import { QueryNodeApi } from '../../QueryNodeApi'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { StandardizedFixture } from '../../Fixture'
import { EventDetails } from '../../types'
import {
  MembershipFieldsFragment,
  StakingAccountAddedEventFieldsFragment,
  StakingAccountConfirmedEventFieldsFragment,
} from '../../graphql/generated/queries'
import { MINIMUM_STAKING_ACCOUNT_BALANCE } from '../../consts'
import { MemberId } from '@joystream/types/common'
import BN from 'bn.js'
import _ from 'lodash'
import { Utils } from '../../utils'
import { SubmittableResult } from '@polkadot/api'

type AddStakingAccountInput = {
  asMember: MemberId
  account: string
  stakeAmount?: BN
}

export class AddStakingAccountsHappyCaseFixture extends StandardizedFixture {
  protected inputs: AddStakingAccountInput[]

  public constructor(api: Api, query: QueryNodeApi, inputs: AddStakingAccountInput[]) {
    super(api, query)
    this.inputs = inputs
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    const addCandidateSigners = this.inputs.map((i) => i.account)
    return addCandidateSigners.concat(await this.api.getMemberSigners(this.inputs))
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[][]> {
    const addExtrinsics = this.inputs.map(({ asMember }) => this.api.tx.members.addStakingAccountCandidate(asMember))
    const confirmExtrinsics = this.inputs.map(({ asMember, account }) =>
      this.api.tx.members.confirmStakingAccount(asMember, account)
    )
    return [addExtrinsics, confirmExtrinsics]
  }

  protected async getEventFromResult(result: SubmittableResult): Promise<EventDetails> {
    let event
    // It's either of those, but since we can't be sure which one it is in this case, we use a try-catch workaround
    try {
      event = await this.api.retrieveMembershipEventDetails(result, 'StakingAccountAdded')
    } catch (e) {
      event = await this.api.retrieveMembershipEventDetails(result, 'StakingAccountConfirmed')
    }

    return event
  }

  protected assertQueryNodeEventIsValid(
    qEvent: StakingAccountAddedEventFieldsFragment | StakingAccountConfirmedEventFieldsFragment,
    i: number
  ): void {
    assert.equal(qEvent.member.id, this.inputs[i % this.inputs.length].asMember.toString())
    assert.equal(qEvent.account, this.inputs[i % this.inputs.length].account.toString())
  }

  protected assertQueriedMembersAreValid(qMembers: MembershipFieldsFragment[]): void {
    const inputsByMember = _.groupBy(this.inputs, (v) => v.asMember.toString())

    for (const [memberId, inputs] of Object.entries(inputsByMember)) {
      const stakingAccounts = inputs.map((i) => i.account)
      const qMember = qMembers.find((m) => m.id === memberId)
      Utils.assert(qMember, 'Query node: Member not found!')
      assert.includeMembers(qMember.boundAccounts, stakingAccounts)
    }
  }

  async execute(): Promise<void> {
    await Promise.all(
      this.inputs.map(({ account, stakeAmount }) =>
        this.api.treasuryTransferBalance(account, (stakeAmount || new BN(0)).addn(MINIMUM_STAKING_ACCOUNT_BALANCE))
      )
    )
    await super.execute()
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    const addedEvents = this.events.slice(0, this.inputs.length)
    const confirmedEvents = this.events.slice(this.inputs.length)
    // Query the events
    const qConfirmedEvents = await this.query.tryQueryWithTimeout(
      () => this.query.getStakingAccountConfirmedEvents(confirmedEvents),
      (qEvents) => assert.equal(qEvents.length, confirmedEvents.length)
    )
    const qAddedEvents = await this.query.getStakingAccountAddedEvents(addedEvents)
    this.assertQueryNodeEventsAreValid(qAddedEvents.concat(qConfirmedEvents))

    const qMembers = await this.query.getMembersByIds(this.inputs.map(({ asMember }) => asMember))
    await this.assertQueriedMembersAreValid(qMembers)
  }
}
