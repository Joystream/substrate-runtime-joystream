import { BaseQueryNodeFixture, FixtureRunner } from '../../Fixture'
import { AddStakingAccountsHappyCaseFixture, BuyMembershipHappyCaseFixture } from '../membership'
import { blake2AsHex } from '@polkadot/util-crypto'
import { MINIMUM_STAKING_ACCOUNT_BALANCE } from '../../consts'
import { assert } from 'chai'

export class ElectCouncilFixture extends BaseQueryNodeFixture {
  public async execute(): Promise<void> {
    const { api, query } = this
    const { councilSize, minNumberOfExtraCandidates } = api.consts.council
    const numberOfCandidates = councilSize.add(minNumberOfExtraCandidates).toNumber()
    const numberOfVoters = numberOfCandidates

    // Prepare memberships
    const candidatesMemberAccounts = (await this.api.createKeyPairs(numberOfCandidates)).map((kp) => kp.address)
    const buyMembershipsFixture = new BuyMembershipHappyCaseFixture(api, query, candidatesMemberAccounts)
    await new FixtureRunner(buyMembershipsFixture).run()
    const candidatesMemberIds = buyMembershipsFixture.getCreatedMembers()

    // Prepare staking accounts
    const councilCandidateStake = api.consts.council.minCandidateStake
    const voteStake = api.consts.referendum.minimumStake

    const candidatesStakingAccounts = (await this.api.createKeyPairs(numberOfCandidates)).map((kp) => kp.address)
    const addStakingAccountsFixture = new AddStakingAccountsHappyCaseFixture(
      api,
      query,
      candidatesStakingAccounts.map((account, i) => ({
        asMember: candidatesMemberIds[i],
        account,
        stakeAmount: councilCandidateStake,
      }))
    )
    await new FixtureRunner(addStakingAccountsFixture).run()

    const votersStakingAccounts = (await this.api.createKeyPairs(numberOfVoters)).map((kp) => kp.address)
    await api.treasuryTransferBalanceToAccounts(votersStakingAccounts, voteStake.addn(MINIMUM_STAKING_ACCOUNT_BALANCE))

    // Announcing stage
    await this.api.untilCouncilStage('Announcing')

    const applyForCouncilTxs = candidatesMemberIds.map((memberId, i) =>
      api.tx.council.announceCandidacy(
        memberId,
        candidatesStakingAccounts[i],
        candidatesMemberAccounts[i],
        councilCandidateStake
      )
    )
    await api.prepareAccountsForFeeExpenses(candidatesMemberAccounts, applyForCouncilTxs)
    await api.sendExtrinsicsAndGetResults(applyForCouncilTxs, candidatesMemberAccounts)

    // Voting stage
    await this.api.untilCouncilStage('Voting')

    const cycleId = (await this.api.query.referendum.stage()).asType('Voting').current_cycle_id
    const votingTxs = votersStakingAccounts.map((account, i) => {
      const accountId = api.createType('AccountId', account)
      const optionId = candidatesMemberIds[i % numberOfCandidates]
      const salt = api.createType('Bytes', `salt${i}`)

      const payload = Buffer.concat([accountId.toU8a(), optionId.toU8a(), salt.toU8a(), cycleId.toU8a()])
      const commitment = blake2AsHex(payload)
      return api.tx.referendum.vote(commitment, voteStake)
    })
    await api.prepareAccountsForFeeExpenses(votersStakingAccounts, votingTxs)
    await api.sendExtrinsicsAndGetResults(votingTxs, votersStakingAccounts)

    // Revealing stage
    await this.api.untilCouncilStage('Revealing')

    const revealingTxs = votersStakingAccounts.map((account, i) => {
      const optionId = candidatesMemberIds[i % numberOfCandidates]
      return api.tx.referendum.revealVote(`salt${i}`, optionId)
    })
    await api.prepareAccountsForFeeExpenses(votersStakingAccounts, votingTxs)
    await api.sendExtrinsicsAndGetResults(revealingTxs, votersStakingAccounts)

    await this.api.untilCouncilStage('Idle')

    const councilMembers = await api.query.council.councilMembers()
    assert.sameMembers(
      councilMembers.map((m) => m.membership_id.toString()),
      candidatesMemberIds.slice(0, councilSize.toNumber()).map((id) => id.toString())
    )
  }
}
