import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { DeletePostsFixture, InitializeForumFixture, PostsRemovalInput } from '../../fixtures/forum'
import { POST_DEPOSIT } from '../../consts'
import { formatBalance } from '@polkadot/util'

export default async function threads({ api, query, env }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:multiple-post-deletions-bug`)
  debug('Started')
  api.enableDebugTxLogs()

  const initializeForumFixture = new InitializeForumFixture(api, query, {
    numberOfForumMembers: 1,
    numberOfCategories: 1,
    threadsPerCategory: 1,
    postsPerThread: 1,
  })
  await new FixtureRunner(initializeForumFixture).run()

  const [memberId] = initializeForumFixture.getCreatedForumMemberIds()
  const [postPath] = initializeForumFixture.getPostsPaths()

  const memberBalaceBefore = await api.getBalance(await api.getControllerAccountOfMember(memberId))

  const x = parseInt(env.POST_DELETIONS_COUNT || '3')
  debug(`Deleting same post ${x} times`)

  const postRemovals: PostsRemovalInput[] = [
    {
      posts: Array.from({ length: x }, () => ({
        ...postPath,
        hide: false,
      })),
      asMember: memberId,
      rationale: 'Getting some free tokens',
    },
  ]
  const deletePostsFixture = new DeletePostsFixture(api, query, postRemovals)
  const deletePostsRunner = new FixtureRunner(deletePostsFixture)
  await deletePostsRunner.run()

  const memberBalaceAfter = await api.getBalance(await api.getControllerAccountOfMember(memberId))

  debug('Post deposit:', formatBalance(POST_DEPOSIT))
  debug('Balance before:', formatBalance(memberBalaceBefore))
  debug('Balance after:', formatBalance(memberBalaceAfter))

  debug('Done')
}
