import { FlowProps } from '../../Flow'
import {
  AddStakingAccountsHappyCaseFixture,
  BuyMembershipHappyCaseFixture,
  RemoveStakingAccountsHappyCaseFixture,
} from '../../fixtures/membership'

import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { assert } from 'chai'

export default async function managingStakingAccounts({ api, query, env }: FlowProps): Promise<void> {
  const debug = Debugger('flow:adding-staking-accounts')
  debug('Started')
  api.enableDebugTxLogs()

  const [account] = (await api.createKeyPairs(1)).map((key) => key.address)
  const buyMembershipHappyCaseFixture = new BuyMembershipHappyCaseFixture(api, query, [account])
  await new FixtureRunner(buyMembershipHappyCaseFixture).run()
  const [memberId] = buyMembershipHappyCaseFixture.getCreatedMembers()

  const N: number = +env.STAKING_ACCOUNTS_ADD_N!
  assert(N > 0)

  const stakingAccounts = (await api.createKeyPairs(N)).map((k) => k.address)
  const addStakingAccountsHappyCaseFixture = new AddStakingAccountsHappyCaseFixture(
    api,
    query,
    stakingAccounts.map((account) => ({ asMember: memberId, account }))
  )
  await new FixtureRunner(addStakingAccountsHappyCaseFixture).runWithQueryNodeChecks()

  const removeStakingAccountsHappyCaseFixture = new RemoveStakingAccountsHappyCaseFixture(
    api,
    query,
    { account, memberId },
    stakingAccounts
  )
  await new FixtureRunner(removeStakingAccountsHappyCaseFixture).runWithQueryNodeChecks()

  debug('Done')
}
