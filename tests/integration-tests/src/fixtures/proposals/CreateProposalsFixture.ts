import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { ProposalCreatedEventDetails, ProposalDetailsJsonByType, ProposalType } from '../../types'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { Utils } from '../../utils'
import { ISubmittableResult } from '@polkadot/types/types/'
import { ProposalCreatedEventFieldsFragment, ProposalFieldsFragment } from '../../graphql/generated/queries'
import { assert } from 'chai'
import { ProposalId, ProposalParameters } from '@joystream/types/proposals'
import { MemberId } from '@joystream/types/common'
import { FixtureRunner, StandardizedFixture } from '../../Fixture'
import { AddStakingAccountsHappyCaseFixture } from '../membership'
import { getWorkingGroupModuleName } from '../../consts'
import { assertQueriedOpeningMetadataIsValid } from '../workingGroups/utils'
import { OpeningMetadata } from '@joystream/metadata-protobuf'
import { blake2AsHex } from '@polkadot/util-crypto'

export type ProposalCreationParams<T extends ProposalType = ProposalType> = {
  asMember: MemberId
  title: string
  description: string
  exactExecutionBlock?: number
  type: T
  details: ProposalDetailsJsonByType<T>
}

export class CreateProposalsFixture extends StandardizedFixture {
  protected events: ProposalCreatedEventDetails[] = []

  protected proposalsParams: ProposalCreationParams[]
  protected stakingAccounts: string[] = []

  public constructor(api: Api, query: QueryNodeApi, proposalsParams: ProposalCreationParams[]) {
    super(api, query)
    this.proposalsParams = proposalsParams
  }

  public getCreatedProposalsIds(): ProposalId[] {
    if (!this.events.length) {
      throw new Error('Trying to get created opening ids before they were created!')
    }
    return this.events.map((e) => e.proposalId)
  }

  protected proposalParams(i: number): ProposalParameters {
    const proposalType = this.proposalsParams[i].type
    return this.api.proposalParametersByType(proposalType)
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.api.getMemberSigners(this.proposalsParams)
  }

  protected async initStakingAccounts(): Promise<void> {
    const { api, query } = this
    const stakingAccounts = (await this.api.createKeyPairs(this.proposalsParams.length)).map((kp) => kp.address)
    const addStakingAccountsFixture = new AddStakingAccountsHappyCaseFixture(
      api,
      query,
      this.proposalsParams.map(({ asMember }, i) => ({
        asMember,
        account: stakingAccounts[i],
        stakeAmount: this.proposalParams(i).requiredStake.unwrapOr(undefined),
      }))
    )
    await new FixtureRunner(addStakingAccountsFixture).run()

    this.stakingAccounts = stakingAccounts
  }

  public async execute(): Promise<void> {
    await this.initStakingAccounts()
    await super.execute()
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.proposalsParams.map(({ asMember, description, title, exactExecutionBlock, details, type }, i) => {
      const proposalDetails = { [type]: details } as { [K in ProposalType]: ProposalDetailsJsonByType<K> }
      return this.api.tx.proposalsCodex.createProposal(
        {
          member_id: asMember,
          description: description,
          title: title,
          exact_execution_block: exactExecutionBlock,
          staking_account_id: this.stakingAccounts[i],
        },
        proposalDetails
      )
    })
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<ProposalCreatedEventDetails> {
    return this.api.retrieveProposalCreatedEventDetails(result)
  }

  protected assertProposalDetailsAreValid(
    params: ProposalCreationParams<ProposalType>,
    qProposal: ProposalFieldsFragment
  ): void {
    const proposalDetails = this.api.createType('ProposalDetails', { [params.type]: params.details })
    switch (params.type) {
      case 'AmendConstitution': {
        Utils.assert(qProposal.details.__typename === 'AmendConstitutionProposalDetails')
        const details = proposalDetails.asType('AmendConstitution')
        assert.equal(qProposal.details.text, details.toString())
        break
      }
      case 'CancelWorkingGroupLeadOpening': {
        Utils.assert(qProposal.details.__typename === 'CancelWorkingGroupLeadOpeningProposalDetails')
        const details = proposalDetails.asType('CancelWorkingGroupLeadOpening')
        const [openingId, workingGroup] = details
        const expectedId = `${getWorkingGroupModuleName(workingGroup)}-${openingId.toString()}`
        assert.equal(qProposal.details.opening?.id, expectedId)
        break
      }
      case 'CreateBlogPost': {
        Utils.assert(qProposal.details.__typename === 'CreateBlogPostProposalDetails')
        const details = proposalDetails.asType('CreateBlogPost')
        const [title, body] = details
        assert.equal(qProposal.details.title, title.toString())
        assert.equal(qProposal.details.body, body.toString())
        break
      }
      case 'CreateWorkingGroupLeadOpening': {
        Utils.assert(qProposal.details.__typename === 'CreateWorkingGroupLeadOpeningProposalDetails')
        const details = proposalDetails.asType('CreateWorkingGroupLeadOpening')
        assert.equal(qProposal.details.group?.id, getWorkingGroupModuleName(details.working_group))
        assert.equal(qProposal.details.rewardPerBlock, details.reward_per_block.toString())
        assert.equal(qProposal.details.stakeAmount, details.stake_policy.stake_amount.toString())
        assert.equal(qProposal.details.unstakingPeriod, details.stake_policy.leaving_unstaking_period.toNumber())
        Utils.assert(qProposal.details.metadata)
        assertQueriedOpeningMetadataIsValid(
          qProposal.details.metadata,
          Utils.metadataFromBytes(OpeningMetadata, details.description)
        )
        break
      }
      case 'DecreaseWorkingGroupLeadStake': {
        Utils.assert(qProposal.details.__typename === 'DecreaseWorkingGroupLeadStakeProposalDetails')
        const details = proposalDetails.asType('DecreaseWorkingGroupLeadStake')
        const [workerId, amount, group] = details
        const expectedId = `${getWorkingGroupModuleName(group)}-${workerId.toString()}`
        assert.equal(qProposal.details.amount, amount.toString())
        assert.equal(qProposal.details.lead?.id, expectedId)
        break
      }
      case 'EditBlogPost': {
        Utils.assert(qProposal.details.__typename === 'EditBlogPostProposalDetails')
        const details = proposalDetails.asType('EditBlogPost')
        const [postId, newTitle, newBody] = details
        assert.equal(qProposal.details.blogPost, postId.toString())
        assert.equal(qProposal.details.newTitle, newTitle.unwrapOr(undefined)?.toString())
        assert.equal(qProposal.details.newBody, newBody.unwrapOr(undefined)?.toString())
        break
      }
      case 'FillWorkingGroupLeadOpening': {
        Utils.assert(qProposal.details.__typename === 'FillWorkingGroupLeadOpeningProposalDetails')
        const details = proposalDetails.asType('FillWorkingGroupLeadOpening')
        const expectedOpeningId = `${getWorkingGroupModuleName(details.working_group)}-${details.opening_id.toString()}`
        const expectedApplicationId = `${getWorkingGroupModuleName(
          details.working_group
        )}-${details.successful_application_id.toString()}`
        assert.equal(qProposal.details.opening?.id, expectedOpeningId)
        assert.equal(qProposal.details.application?.id, expectedApplicationId)
        break
      }
      case 'FundingRequest': {
        Utils.assert(qProposal.details.__typename === 'FundingRequestProposalDetails')
        const details = proposalDetails.asType('FundingRequest')
        assert.sameDeepMembers(
          qProposal.details.destinationsList?.destinations.map(({ amount, account }) => ({ amount, account })) || [],
          details.map((d) => ({ amount: d.amount.toString(), account: d.account.toString() }))
        )
        break
      }
      case 'LockBlogPost': {
        Utils.assert(qProposal.details.__typename === 'LockBlogPostProposalDetails')
        const postId = proposalDetails.asType('LockBlogPost')
        assert.equal(qProposal.details.blogPost, postId.toString())
        break
      }
      case 'RuntimeUpgrade': {
        Utils.assert(qProposal.details.__typename === 'RuntimeUpgradeProposalDetails')
        const details = proposalDetails.asType('RuntimeUpgrade')
        Utils.assert(qProposal.details.newRuntimeBytecode, 'Missing newRuntimeBytecode relationship')
        assert.equal(qProposal.details.newRuntimeBytecode.id, blake2AsHex(details.toU8a(true)))
        const expectedBytecode = '0x' + Buffer.from(details.toU8a(true)).toString('hex')
        const actualBytecode = qProposal.details.newRuntimeBytecode.bytecode
        if (actualBytecode !== expectedBytecode) {
          const diffStartPos = expectedBytecode.split('').findIndex((c, i) => actualBytecode[i] !== c)
          const diffSubExpected = expectedBytecode.slice(diffStartPos, diffStartPos + 10)
          const diffSubActual = actualBytecode.slice(diffStartPos, diffStartPos + 10)
          throw new Error(
            `Runtime bytecode doesn't match the expected one! Diff starts at pos ${diffStartPos}. ` +
              `Expected: ${diffSubExpected}.., Actual: ${diffSubActual}...`
          )
        }
        break
      }
      case 'SetCouncilBudgetIncrement': {
        Utils.assert(qProposal.details.__typename === 'SetCouncilBudgetIncrementProposalDetails')
        const details = proposalDetails.asType('SetCouncilBudgetIncrement')
        assert.equal(qProposal.details.newAmount, details.toString())
        break
      }
      case 'SetCouncilorReward': {
        Utils.assert(qProposal.details.__typename === 'SetCouncilorRewardProposalDetails')
        const details = proposalDetails.asType('SetCouncilorReward')
        assert.equal(qProposal.details.newRewardPerBlock, details.toString())
        break
      }
      case 'SetInitialInvitationBalance': {
        Utils.assert(qProposal.details.__typename === 'SetInitialInvitationBalanceProposalDetails')
        const details = proposalDetails.asType('SetInitialInvitationBalance')
        assert.equal(qProposal.details.newInitialInvitationBalance, details.toString())
        break
      }
      case 'SetInitialInvitationCount': {
        Utils.assert(qProposal.details.__typename === 'SetInitialInvitationCountProposalDetails')
        const details = proposalDetails.asType('SetInitialInvitationCount')
        assert.equal(qProposal.details.newInitialInvitationsCount, details.toNumber())
        break
      }
      case 'SetMaxValidatorCount': {
        Utils.assert(qProposal.details.__typename === 'SetMaxValidatorCountProposalDetails')
        const details = proposalDetails.asType('SetMaxValidatorCount')
        assert.equal(qProposal.details.newMaxValidatorCount, details.toNumber())
        break
      }
      case 'SetMembershipLeadInvitationQuota': {
        Utils.assert(qProposal.details.__typename === 'SetMembershipLeadInvitationQuotaProposalDetails')
        const details = proposalDetails.asType('SetMembershipLeadInvitationQuota')
        assert.equal(qProposal.details.newLeadInvitationQuota, details.toNumber())
        break
      }
      case 'SetMembershipPrice': {
        Utils.assert(qProposal.details.__typename === 'SetMembershipPriceProposalDetails')
        const details = proposalDetails.asType('SetMembershipPrice')
        assert.equal(qProposal.details.newPrice, details.toString())
        break
      }
      case 'SetReferralCut': {
        Utils.assert(qProposal.details.__typename === 'SetReferralCutProposalDetails')
        const details = proposalDetails.asType('SetReferralCut')
        assert.equal(qProposal.details.newReferralCut, details.toNumber())
        break
      }
      case 'SetWorkingGroupLeadReward': {
        Utils.assert(qProposal.details.__typename === 'SetWorkingGroupLeadRewardProposalDetails')
        const details = proposalDetails.asType('SetWorkingGroupLeadReward')
        const [workerId, reward, group] = details
        const expectedId = `${getWorkingGroupModuleName(group)}-${workerId.toString()}`
        assert.equal(qProposal.details.newRewardPerBlock, reward.toString())
        assert.equal(qProposal.details.lead?.id, expectedId)
        break
      }
      case 'Signal': {
        Utils.assert(qProposal.details.__typename === 'SignalProposalDetails')
        const details = proposalDetails.asType('Signal')
        assert.equal(qProposal.details.text, details.toString())
        break
      }
      case 'SlashWorkingGroupLead': {
        Utils.assert(qProposal.details.__typename === 'SlashWorkingGroupLeadProposalDetails')
        const details = proposalDetails.asType('SlashWorkingGroupLead')
        const [workerId, amount, group] = details
        const expectedId = `${getWorkingGroupModuleName(group)}-${workerId.toString()}`
        assert.equal(qProposal.details.lead?.id, expectedId)
        assert.equal(qProposal.details.amount, amount.toString())
        break
      }
      case 'TerminateWorkingGroupLead': {
        Utils.assert(qProposal.details.__typename === 'TerminateWorkingGroupLeadProposalDetails')
        const details = proposalDetails.asType('TerminateWorkingGroupLead')
        const expectedId = `${getWorkingGroupModuleName(details.working_group)}-${details.worker_id.toString()}`
        assert.equal(qProposal.details.lead?.id, expectedId)
        assert.equal(qProposal.details.slashingAmount, details.slashing_amount.toString())
        break
      }
      case 'UnlockBlogPost': {
        Utils.assert(qProposal.details.__typename === 'UnlockBlogPostProposalDetails')
        const postId = proposalDetails.asType('UnlockBlogPost')
        assert.equal(qProposal.details.blogPost, postId.toString())
        break
      }
      case 'UpdateWorkingGroupBudget': {
        Utils.assert(qProposal.details.__typename === 'UpdateWorkingGroupBudgetProposalDetails')
        const details = proposalDetails.asType('UpdateWorkingGroupBudget')
        const [balance, group, balanceKind] = details
        assert.equal(qProposal.details.amount, (balanceKind.isOfType('Negative') ? '-' : '') + balance.toString())
        assert.equal(qProposal.details.group?.id, getWorkingGroupModuleName(group))
        break
      }
      case 'VetoProposal': {
        Utils.assert(qProposal.details.__typename === 'VetoProposalDetails')
        const details = proposalDetails.asType('VetoProposal')
        assert.equal(qProposal.details.proposal?.id, details.toString())
        break
      }
    }
  }

  protected assertQueriedProposalsAreValid(qProposals: ProposalFieldsFragment[]): void {
    this.events.map((e, i) => {
      const proposalParams = this.proposalsParams[i]
      const qProposal = qProposals.find((p) => p.id === e.proposalId.toString())
      Utils.assert(qProposal, 'Query node: Proposal not found')
      assert.equal(qProposal.councilApprovals, 0)
      assert.equal(qProposal.creator.id, proposalParams.asMember.toString())
      assert.equal(qProposal.description, proposalParams.description)
      assert.equal(qProposal.title, proposalParams.title)
      assert.equal(qProposal.stakingAccount, this.stakingAccounts[i].toString())
      assert.equal(qProposal.exactExecutionBlock, proposalParams.exactExecutionBlock)
      assert.equal(qProposal.status.__typename, 'ProposalStatusDeciding')
      assert.equal(qProposal.statusSetAtBlock, e.blockNumber)
      assert.equal(new Date(qProposal.statusSetAtTime).getTime(), e.blockTimestamp)
      assert.equal(qProposal.createdInEvent.inBlock, e.blockNumber)
      assert.equal(qProposal.createdInEvent.inExtrinsic, this.extrinsics[i].hash.toString())
      assert.equal(qProposal.discussionThread.mode.__typename, 'ProposalDiscussionThreadModeOpen')
      this.assertProposalDetailsAreValid(proposalParams, qProposal)
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: ProposalCreatedEventFieldsFragment, i: number): void {
    // TODO: https://github.com/Joystream/joystream/issues/2457
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()

    // Query the proposals
    await this.query.tryQueryWithTimeout(
      () => this.query.getProposalsByIds(this.events.map((e) => e.proposalId)),
      (result) => this.assertQueriedProposalsAreValid(result)
    )
  }
}
