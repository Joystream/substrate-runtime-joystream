import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import {
  DeleteThreadsFixture,
  InitializeForumFixture,
  MoveThreadParams,
  MoveThreadsFixture,
  SetStickyThreadsFixture,
  StickyThreadsParams,
  ThreadRemovalInput,
  ThreadTitleUpdate,
  UpdateThreadTitlesFixture,
} from '../../fixtures/forum'
import { CategoryId } from '@joystream/types/forum'

export default async function threads({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:threads`)
  debug('Started')
  api.enableDebugTxLogs()

  // Initialize categories and threads
  const initializeForumFixture = new InitializeForumFixture(api, query, {
    numberOfForumMembers: 5,
    numberOfCategories: 3,
    threadsPerCategory: 3,
  })
  await new FixtureRunner(initializeForumFixture).runWithQueryNodeChecks()

  const categoryIds = initializeForumFixture.getCreatedCategoryIds()

  // Set threads as sticky (2 per category)
  let stickyThreadsParams: StickyThreadsParams[] = []
  categoryIds.forEach((categoryId) => {
    const threadIds = initializeForumFixture.getCreatedThreadsByCategoryId(categoryId)
    stickyThreadsParams = stickyThreadsParams.concat([
      { categoryId, stickyTreads: [threadIds[0], threadIds[1]] },
      { categoryId, stickyTreads: [threadIds[1], threadIds[2]] },
    ])
  })

  const setStickyThreadsFixture = new SetStickyThreadsFixture(api, query, stickyThreadsParams)
  const setStickyThreadsRunner = new FixtureRunner(setStickyThreadsFixture)
  await setStickyThreadsRunner.run()

  // Update titles
  let titleUpdates: ThreadTitleUpdate[] = []
  initializeForumFixture.getThreadPaths().forEach(
    (threadPath, i) =>
      (titleUpdates = titleUpdates.concat([
        { ...threadPath, newTitle: '' },
        { ...threadPath, newTitle: `Test updated title ${i}` },
      ]))
  )

  const updateThreadTitlesFixture = new UpdateThreadTitlesFixture(api, query, titleUpdates)
  const updateThreadTitlesRunner = new FixtureRunner(updateThreadTitlesFixture)
  await updateThreadTitlesRunner.run()

  // Run compound checks
  await Promise.all([setStickyThreadsRunner.runQueryNodeChecks(), updateThreadTitlesRunner.runQueryNodeChecks()])

  // Move threads to different categories
  const newThreadCategory = (oldCategory: CategoryId) =>
    categoryIds[(categoryIds.indexOf(oldCategory) + 1) % categoryIds.length]
  const threadCategoryUpdates: MoveThreadParams[] = initializeForumFixture.getThreadPaths().map((threadPath) => ({
    ...threadPath,
    newCategoryId: newThreadCategory(threadPath.categoryId),
  }))

  const moveThreadsFixture = new MoveThreadsFixture(api, query, threadCategoryUpdates)
  const moveThreadsRunner = new FixtureRunner(moveThreadsFixture)
  await moveThreadsRunner.run()

  // Remove threads
  // TODO: Should removing / moving threads also "unstick" them?
  const threadRemovals: ThreadRemovalInput[] = initializeForumFixture
    .getThreadPaths()
    .map(({ categoryId, threadId }, i) => ({
      threadId,
      categoryId: newThreadCategory(categoryId),
      hide: !!(i % 2), // Test both cases
    }))
  const removeThreadsFixture = new DeleteThreadsFixture(api, query, threadRemovals)
  const removeThreadsRunner = new FixtureRunner(removeThreadsFixture)
  await removeThreadsRunner.run()

  // Run compound query node checks
  await Promise.all([moveThreadsRunner.runQueryNodeChecks(), removeThreadsRunner.runQueryNodeChecks()])

  debug('Done')
}
