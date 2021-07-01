import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import { CreateProposalsFixture, ExpireProposalsFixture } from '../../fixtures/proposals'
import {
  ChangeThreadsModeFixture,
  CreatePostsFixture,
  DeletePostParams,
  DeletePostsFixture,
  PostParams,
  PostUpdateParams,
  ThreadModeChangeParams,
  UpdatePostsFixture,
} from '../../fixtures/proposalsDiscussion'
import { Resource } from '../../Resources'
import { ThreadId } from '../../../../../types/common'
import { ALL_BYTES } from '../../consts'

export default async function proposalsDiscussion({ api, query, lock }: FlowProps): Promise<void> {
  const debug = Debugger('flow:proposals-discussion')
  debug('Started')
  api.enableDebugTxLogs()

  const threadsN = 3
  const accounts = (await api.createKeyPairs(threadsN)).map((kp) => kp.address)

  const buyMembershipsFixture = new BuyMembershipHappyCaseFixture(api, query, accounts)
  await new FixtureRunner(buyMembershipsFixture).run()
  const memberIds = buyMembershipsFixture.getCreatedMembers()

  const unlocks = await Promise.all(Array.from({ length: threadsN }, () => lock(Resource.Proposals)))
  const createProposalFixture = new CreateProposalsFixture(
    api,
    query,
    Array.from({ length: threadsN }, (v, i) => ({
      type: 'Signal',
      details: `Discussion test ${i}`,
      asMember: memberIds[i],
      title: `Discussion test proposal ${i}`,
      description: `Proposals discussion test proposal ${i}`,
    }))
  )
  await new FixtureRunner(createProposalFixture).run()
  const proposalsIds = createProposalFixture.getCreatedProposalsIds()
  const threadIds = await api.query.proposalsCodex.threadIdByProposalId.multi<ThreadId>(proposalsIds)

  const createPostsParams: PostParams[] = threadIds.reduce(
    (posts, threadId) =>
      posts.concat([
        // Standard case:
        {
          threadId,
          asMember: memberIds[0],
          metadata: { value: { text: 'Test' } },
          editable: true,
        },
        // Invalid repliesTo case:
        {
          threadId,
          asMember: memberIds[1],
          metadata: { value: { text: 'Test', repliesTo: 9999 }, expectReplyFailure: true },
          editable: true,
        },
        // ALL_BYTES metadata + non-editable case:
        {
          threadId,
          asMember: memberIds[2],
          metadata: { value: ALL_BYTES, expectFailure: true }, // expectFailure just means serialization failure, but the value will still be checked
          editable: false,
        },
      ]),
    [] as PostParams[]
  )
  const createPostsFixture = new CreatePostsFixture(api, query, createPostsParams)
  await new FixtureRunner(createPostsFixture).runWithQueryNodeChecks()
  const postIds = createPostsFixture.getCreatedPostsIds()

  const threadModeChangesParams: ThreadModeChangeParams[] = [
    { threadId: threadIds[0], asMember: memberIds[0], newMode: { Closed: memberIds } },
    { threadId: threadIds[1], asMember: memberIds[1], newMode: { Closed: [memberIds[0]] } },
    { threadId: threadIds[1], asMember: memberIds[1], newMode: 'Open' },
  ]
  const threadModeChanges = new ChangeThreadsModeFixture(api, query, threadModeChangesParams)
  const threadModeChangesRunner = new FixtureRunner(threadModeChanges)
  await threadModeChangesRunner.run()

  const createPostRepliesParams: PostParams[] = createPostsParams.map((params, i) => ({
    threadId: params.threadId,
    asMember: memberIds[i % memberIds.length],
    metadata: { value: { text: `Reply to post ${postIds[i].toString()}`, repliesTo: postIds[i].toNumber() } },
  }))
  const createRepliesFixture = new CreatePostsFixture(api, query, createPostRepliesParams)
  const createRepliesRunner = new FixtureRunner(createRepliesFixture)
  await createRepliesRunner.run()

  const updatePostsParams: PostUpdateParams[] = [
    { threadId: threadIds[0], postId: postIds[0], asMember: memberIds[0], newText: 'New text' },
    { threadId: threadIds[0], postId: postIds[1], asMember: memberIds[1], newText: ALL_BYTES },
  ]
  const updatePostsFixture = new UpdatePostsFixture(api, query, updatePostsParams)
  const updatePostsRunner = new FixtureRunner(updatePostsFixture)
  await updatePostsRunner.run()

  // TODO: Test anyone_can_delete_post (would require waiting PostLifetime)

  const deletePostsParams: DeletePostParams[] = postIds
    .map((postId, i) => ({ postId, ...createPostsParams[i] }))
    .filter((p) => p.editable !== false)
  const deletePostsFixture = new DeletePostsFixture(api, query, deletePostsParams)
  const deletePostsRunner = new FixtureRunner(deletePostsFixture)
  await deletePostsRunner.run()

  // Run compound query-node checks
  await Promise.all([
    createRepliesRunner.runQueryNodeChecks(),
    threadModeChangesRunner.runQueryNodeChecks(),
    updatePostsRunner.runQueryNodeChecks(),
    deletePostsRunner.runQueryNodeChecks(),
  ])

  // Wait until proposal expires and release locks
  await new FixtureRunner(new ExpireProposalsFixture(api, query, proposalsIds)).run()
  unlocks.forEach((unlock) => unlock())

  debug('Done')
}
