import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import { CategoryId } from '@joystream/types/forum'
import { WorkerId } from '@joystream/types/working-group'
import { Api } from '../../Api'
import { BaseQueryNodeFixture, FixtureRunner } from '../../Fixture'
import { QueryNodeApi } from '../../QueryNodeApi'
import { PostPath, ThreadPath } from '../../types'
import { Utils } from '../../utils'
import { BuyMembershipHappyCaseFixture } from '../membership'
import { HireWorkersFixture } from '../workingGroups/HireWorkersFixture'
import { AddPostsFixture, PostParams } from './AddPostsFixture'
import { CategoryParams, CreateCategoriesFixture } from './CreateCategoriesFixture'
import { CreateThreadsFixture, ThreadParams } from './CreateThreadsFixture'
import { CategoryModeratorStatusUpdate, UpdateCategoryModeratorsFixture } from './UpdateCategoryModeratorsFixture'

export type InitializeForumConfig = {
  numberOfForumMembers: number
  numberOfCategories: number
  threadsPerCategory?: number
  postsPerThread?: number
  moderatorsPerCategory?: number
}

export class InitializeForumFixture extends BaseQueryNodeFixture {
  protected createCategoriesRunner?: FixtureRunner
  protected createThreadsRunner?: FixtureRunner
  protected addPostsRunner?: FixtureRunner
  protected updateCategoryModeratorsRunner?: FixtureRunner

  protected config: InitializeForumConfig

  protected forumMemberIds: MemberId[] | undefined
  protected postIds: PostId[] | undefined
  protected threadIds: ThreadId[] | undefined
  protected categoryIds: CategoryId[] | undefined
  protected moderatorIds: WorkerId[] | undefined
  protected threadIdsByCategoryId: Map<number, ThreadId[]> = new Map()
  protected postIdsByThreadId: Map<number, PostId[]> = new Map()
  protected moderatorIdsByCategoryId: Map<number, WorkerId[]> = new Map()

  constructor(api: Api, query: QueryNodeApi, config: InitializeForumConfig) {
    super(api, query)
    this.config = config
  }

  public getCreatedPostsIds(): PostId[] {
    Utils.assert(this.postIds, 'Posts not yet created!')
    return this.postIds
  }

  public getCreatedPostsByThreadId(threadId: ThreadId): PostId[] {
    const postsIds = this.postIdsByThreadId.get(threadId.toNumber())
    Utils.assert(postsIds, `No posts found by threadId ${threadId}`)
    return postsIds
  }

  public getCreatedThreadIds(): ThreadId[] {
    Utils.assert(this.threadIds, 'Threads not yet created!')
    return this.threadIds
  }

  public getCreatedThreadsByCategoryId(categoryId: CategoryId): ThreadId[] {
    const threadIds = this.threadIdsByCategoryId.get(categoryId.toNumber())
    Utils.assert(threadIds, `No threads found by categoryId ${categoryId}`)
    return threadIds
  }

  public getCreatedCategoryIds(): CategoryId[] {
    Utils.assert(this.categoryIds, 'Categories not yet created!')
    return this.categoryIds
  }

  public getCreatedForumMemberIds(): CategoryId[] {
    Utils.assert(this.forumMemberIds, 'Forum members not yet created!')
    return this.forumMemberIds
  }

  public getCreatedForumModeratorIds(): WorkerId[] {
    Utils.assert(this.moderatorIds, 'Forum moderators not yet created!')
    return this.moderatorIds
  }

  public getCreatedForumModeratorsByCategoryId(categoryId: CategoryId): WorkerId[] {
    const moderatorIds = this.moderatorIdsByCategoryId.get(categoryId.toNumber())
    Utils.assert(moderatorIds, `No moderators found by categoryId ${categoryId}`)
    return moderatorIds
  }

  public getThreadPaths(): ThreadPath[] {
    Utils.assert(this.categoryIds, 'Threads not yet created')
    let paths: ThreadPath[] = []
    this.categoryIds.forEach((categoryId) => {
      const threadIds = this.getCreatedThreadsByCategoryId(categoryId)
      paths = paths.concat(threadIds.map((threadId) => ({ categoryId, threadId })))
    })

    return paths
  }

  public getPostsPaths(): PostPath[] {
    let paths: PostPath[] = []
    this.getThreadPaths().forEach(({ categoryId, threadId }) => {
      const postIds = this.getCreatedPostsByThreadId(threadId)
      paths = paths.concat(postIds.map((postId) => ({ categoryId, threadId, postId })))
    })

    return paths
  }

  public async execute(): Promise<void> {
    const { api, query } = this
    const {
      numberOfForumMembers,
      numberOfCategories,
      threadsPerCategory,
      postsPerThread,
      moderatorsPerCategory,
    } = this.config
    // Create forum members
    const accounts = (await api.createKeyPairs(numberOfForumMembers)).map((kp) => kp.address)
    const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, accounts)
    await new FixtureRunner(buyMembershipFixture).run()
    const forumMemberIds = buyMembershipFixture.getCreatedMembers()
    this.forumMemberIds = forumMemberIds

    // Create categories
    const categories: CategoryParams[] = Array.from({ length: numberOfCategories }, (v, i) => ({
      title: `Category ${i}`,
      description: `Initialize forum test category ${i}`,
    }))
    const createCategoriesFixture = new CreateCategoriesFixture(api, query, categories)
    this.createCategoriesRunner = new FixtureRunner(createCategoriesFixture)
    await this.createCategoriesRunner.run()
    this.categoryIds = createCategoriesFixture.getCreatedCategoriesIds()

    // Create and assign moderators
    if (moderatorsPerCategory) {
      const createModeratorsFixture = new HireWorkersFixture(
        api,
        query,
        'forumWorkingGroup',
        moderatorsPerCategory * numberOfCategories
      )
      await new FixtureRunner(createModeratorsFixture).run()
      const moderatorIds = createModeratorsFixture.getCreatedWorkerIds()
      this.moderatorIds = moderatorIds

      let moderatorUpdates: CategoryModeratorStatusUpdate[] = []
      this.categoryIds.forEach(
        (categoryId, i) =>
          (moderatorUpdates = moderatorUpdates.concat(
            Array.from({ length: moderatorsPerCategory }, (v, j) => ({
              canModerate: true,
              categoryId,
              moderatorId: moderatorIds[i * moderatorsPerCategory + j],
            }))
          ))
      )
      const updateCategoryModeratorsFixture = new UpdateCategoryModeratorsFixture(api, query, moderatorUpdates)
      this.updateCategoryModeratorsRunner = new FixtureRunner(updateCategoryModeratorsFixture)
      await this.updateCategoryModeratorsRunner.run()
      this.moderatorIds.forEach((moderatorId, i) => {
        const categoryId = moderatorUpdates[i].categoryId.toNumber()
        this.moderatorIdsByCategoryId.set(categoryId, [
          ...(this.moderatorIdsByCategoryId.get(categoryId) || []),
          moderatorId,
        ])
      })
    }

    // Create threads
    if (threadsPerCategory) {
      let threads: ThreadParams[] = []
      this.categoryIds.forEach(
        (categoryId) =>
          (threads = threads.concat(
            Array.from({ length: threadsPerCategory }, (v, i) => ({
              categoryId,
              asMember: forumMemberIds[i % forumMemberIds.length],
              title: `Thread ${i} in category ${categoryId.toString()}`,
              text: `Initialize forum test thread ${i} in category ${categoryId.toString()}`,
            }))
          ))
      )

      const createThreadsFixture = new CreateThreadsFixture(api, query, threads)
      this.createThreadsRunner = new FixtureRunner(createThreadsFixture)
      await this.createThreadsRunner.run()
      this.threadIds = createThreadsFixture.getCreatedThreadsIds()
      this.threadIds.forEach((threadId, i) => {
        const categoryId = threads[i].categoryId.toNumber()
        this.threadIdsByCategoryId.set(categoryId, [...(this.threadIdsByCategoryId.get(categoryId) || []), threadId])
      })
    }

    // Create posts
    if (postsPerThread) {
      let posts: PostParams[] = []
      this.getThreadPaths().forEach(
        (threadPath) =>
          (posts = posts.concat(
            Array.from({ length: postsPerThread || 0 }, (v, i) => ({
              ...threadPath,
              asMember: forumMemberIds[i % forumMemberIds.length],
              metadata: {
                value: { text: `Initialize forum test post ${i} in thread ${threadPath.threadId.toString()}` },
              },
              editable: true,
            }))
          ))
      )

      const addPostsFixture = new AddPostsFixture(api, query, posts)
      this.addPostsRunner = new FixtureRunner(addPostsFixture)
      await this.addPostsRunner.run()
      this.postIds = addPostsFixture.getCreatedPostsIds()
      this.postIds.forEach((postId, i) => {
        const post = posts[i]
        const threadId = typeof post.threadId === 'number' ? post.threadId : post.threadId.toNumber()
        this.postIdsByThreadId.set(threadId, [...(this.postIdsByThreadId.get(threadId) || []), postId])
      })
    }
  }

  public async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    await Promise.all([
      this.createCategoriesRunner?.runQueryNodeChecks(),
      this.createThreadsRunner?.runQueryNodeChecks(),
      this.addPostsRunner?.runQueryNodeChecks(),
      this.updateCategoryModeratorsRunner?.runQueryNodeChecks(),
    ])
  }
}
