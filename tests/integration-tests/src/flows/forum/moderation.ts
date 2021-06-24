import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import {
  InitializeForumFixture,
  ModerateThreadsFixture,
  ThreadModerationInput,
  PostModerationInput,
  ModeratePostsFixture,
} from '../../fixtures/forum'

export default async function threads({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:threads`)
  debug('Started')
  api.enableDebugTxLogs()

  // Initialize categories, threads and posts
  const MODERATORS_PER_CATEGORY = 3
  const initializeForumFixture = new InitializeForumFixture(api, query, {
    numberOfForumMembers: 5,
    numberOfCategories: 3,
    threadsPerCategory: MODERATORS_PER_CATEGORY + 1, // 1 thread per moderator + 1 for the lead
    postsPerThread: 3,
    moderatorsPerCategory: MODERATORS_PER_CATEGORY,
  })
  await new FixtureRunner(initializeForumFixture).runWithQueryNodeChecks()

  // Generate input (moderate posts and threads different moderators / lead)
  const threadModerations: ThreadModerationInput[] = []
  let postModerations: PostModerationInput[] = []
  initializeForumFixture.getCreatedCategoryIds().forEach((categoryId) => {
    const categoryModerators = initializeForumFixture.getCreatedForumModeratorsByCategoryId(categoryId)
    const categoryThreads = initializeForumFixture.getCreatedThreadsByCategoryId(categoryId)
    let i: number
    for (i = 0; i < MODERATORS_PER_CATEGORY; ++i) {
      const threadId = categoryThreads[i]
      const genericModerationInput = { categoryId, threadId, asWorker: categoryModerators[i] }
      threadModerations.push({
        ...genericModerationInput,
        rationale: `Moderate thread ${i} in category ${categoryId.toString()} rationale`,
      })
      postModerations = postModerations.concat(
        initializeForumFixture.getCreatedPostsByThreadId(threadId).map((postId, j) => ({
          ...genericModerationInput,
          postId,
          rationale: `Moderate post ${j} in thread ${i} in category ${categoryId.toString()} rationale`,
        }))
      )
    }
    // Moderate as lead
    const threadId = categoryThreads[i]
    threadModerations.push({ categoryId, threadId })
    postModerations = postModerations.concat(
      initializeForumFixture.getCreatedPostsByThreadId(threadId).map((postId) => ({ categoryId, threadId, postId }))
    )
  })

  // Run fixtures
  const moderateThreadsFixture = new ModerateThreadsFixture(api, query, threadModerations)
  const moderateThreadsRunner = new FixtureRunner(moderateThreadsFixture)
  await moderateThreadsRunner.run()

  const moderatePostsFixture = new ModeratePostsFixture(api, query, postModerations)
  const moderatePostsRunner = new FixtureRunner(moderatePostsFixture)
  await moderatePostsRunner.run()

  // Run query-node checks
  await Promise.all([moderateThreadsFixture.runQueryNodeChecks(), moderatePostsFixture.runQueryNodeChecks()])

  debug('Done')
}
