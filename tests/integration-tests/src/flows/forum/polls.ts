import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import {
  CategoryParams,
  CreateCategoriesFixture,
  VoteParams,
  CreateThreadsFixture,
  ThreadParams,
  VoteOnPollFixture,
} from '../../fixtures/forum'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'

export default async function polls({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:polls`)
  debug('Started')
  api.enableDebugTxLogs()

  // Create test member(s)
  const accounts = (await api.createKeyPairs(5)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, accounts)
  await new FixtureRunner(buyMembershipFixture).run()
  const memberIds = buyMembershipFixture.getCreatedMembers()

  // Create some test category first
  const categories: CategoryParams[] = [{ title: 'Polls', description: 'Testing the polls' }]
  const createCategoriesFixture = new CreateCategoriesFixture(api, query, categories)
  await new FixtureRunner(createCategoriesFixture).run()
  const [categoryId] = createCategoriesFixture.getCreatedCategoriesIds()

  // Create polls
  const pollThreads: ThreadParams[] = memberIds.map((memberId, i) => ({
    categoryId,
    asMember: memberId,
    title: `Poll ${i}`,
    text: `Poll ${i} desc`,
    poll: {
      description: `Poll ${i} question?`,
      alternatives: [`${i}:A`, `${i}:B`, `${i}:C`],
      endTime: new Date(Date.now() + (i + 1) * 60 * 60 * 1000), // +(i+1) hours
    },
  }))

  const createThreadsFixture = new CreateThreadsFixture(api, query, pollThreads)
  const createThreadsRunner = new FixtureRunner(createThreadsFixture)
  await createThreadsRunner.run()
  const pollThreadIds = createThreadsFixture.getCreatedThreadsIds()

  // Vote on polls
  const votes: VoteParams[] = pollThreadIds.reduce(
    (votesArray, threadId) =>
      votesArray.concat(
        memberIds.map((memberId, i) => {
          const index = i % 3
          return {
            threadId,
            categoryId,
            asMember: memberId,
            index,
          }
        })
      ),
    [] as VoteParams[]
  )
  const voteOnPollFixture = new VoteOnPollFixture(api, query, votes)
  const voteOnPollRunner = new FixtureRunner(voteOnPollFixture)
  await voteOnPollRunner.run()

  await Promise.all([createThreadsRunner.runQueryNodeChecks(), voteOnPollRunner.runQueryNodeChecks()])

  debug('Done')
}
