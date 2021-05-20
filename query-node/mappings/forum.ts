/*
eslint-disable @typescript-eslint/naming-convention
*/
import { SubstrateEvent, DatabaseManager } from '@dzlzv/hydra-common'
import { bytesToString, deserializeMetadata, genericEventFields, getWorker } from './common'
import {
  CategoryCreatedEvent,
  CategoryStatusActive,
  CategoryUpdatedEvent,
  ForumCategory,
  Worker,
  CategoryStatusArchived,
  CategoryDeletedEvent,
  CategoryStatusRemoved,
  ThreadCreatedEvent,
  ForumThread,
  Membership,
  ThreadStatusActive,
  ForumPoll,
  ForumPollAlternative,
  ThreadModeratedEvent,
  ThreadStatusModerated,
  ThreadTitleUpdatedEvent,
  ThreadDeletedEvent,
  ThreadStatusLocked,
  ThreadStatusRemoved,
  ThreadMovedEvent,
  ForumPost,
  PostStatusActive,
  PostOriginThreadInitial,
  VoteOnPollEvent,
  PostAddedEvent,
  PostStatusLocked,
  PostOriginThreadReply,
  CategoryStickyThreadUpdateEvent,
  CategoryMembershipOfModeratorUpdatedEvent,
  PostModeratedEvent,
  PostStatusModerated,
} from 'query-node/dist/model'
import { Forum } from './generated/types'
import { PrivilegedActor } from '@joystream/types/augment/all'
import { ForumPostMetadata } from '@joystream/metadata-protobuf'
import { Not } from 'typeorm'

async function getCategory(db: DatabaseManager, categoryId: string, relations?: string[]): Promise<ForumCategory> {
  const category = await db.get(ForumCategory, { where: { id: categoryId }, relations })
  if (!category) {
    throw new Error(`Forum category not found by id: ${categoryId}`)
  }

  return category
}

async function getThread(db: DatabaseManager, threadId: string): Promise<ForumThread> {
  const thread = await db.get(ForumThread, { where: { id: threadId } })
  if (!thread) {
    throw new Error(`Forum thread not found by id: ${threadId.toString()}`)
  }

  return thread
}

async function getPost(db: DatabaseManager, postId: string): Promise<ForumPost> {
  const post = await db.get(ForumPost, { where: { id: postId } })
  if (!post) {
    throw new Error(`Forum post not found by id: ${postId.toString()}`)
  }

  return post
}

async function getPollAlternative(db: DatabaseManager, threadId: string, index: number) {
  const poll = await db.get(ForumPoll, { where: { thread: { id: threadId } }, relations: ['pollAlternatives'] })
  if (!poll) {
    throw new Error(`Forum poll not found by threadId: ${threadId.toString()}`)
  }
  const pollAlternative = poll.pollAlternatives?.find((alt) => alt.index === index)
  if (!pollAlternative) {
    throw new Error(`Froum poll alternative not found by index ${index} in thread ${threadId.toString()}`)
  }

  return pollAlternative
}

async function getActorWorker(db: DatabaseManager, actor: PrivilegedActor): Promise<Worker> {
  const worker = await db.get(Worker, {
    where: {
      group: { id: 'forumWorkingGroup' },
      ...(actor.isLead ? { isLead: true } : { runtimeId: actor.asModerator.toNumber() }),
    },
    relations: ['group'],
  })

  if (!worker) {
    throw new Error(`Corresponding worker not found by forum PrivielagedActor: ${JSON.stringify(actor.toHuman())}`)
  }

  return worker
}

export async function forum_CategoryCreated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [categoryId, parentCategoryId, titleBytes, descriptionBytes] = new Forum.CategoryCreatedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)

  const category = new ForumCategory({
    id: categoryId.toString(),
    createdAt: eventTime,
    updatedAt: eventTime,
    title: bytesToString(titleBytes),
    description: bytesToString(descriptionBytes),
    status: new CategoryStatusActive(),
    parent: parentCategoryId.isSome ? new ForumCategory({ id: parentCategoryId.unwrap().toString() }) : undefined,
  })

  await db.save<ForumCategory>(category)

  const categoryCreatedEvent = new CategoryCreatedEvent({
    ...genericEventFields(event_),
    category,
  })
  await db.save<CategoryCreatedEvent>(categoryCreatedEvent)
}

export async function forum_CategoryUpdated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [categoryId, newArchivalStatus, privilegedActor] = new Forum.CategoryUpdatedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const category = await getCategory(db, categoryId.toString())
  const actorWorker = await getActorWorker(db, privilegedActor)

  const categoryUpdatedEvent = new CategoryUpdatedEvent({
    ...genericEventFields(event_),
    category,
    newArchivalStatus: newArchivalStatus.valueOf(),
    actor: actorWorker,
  })
  await db.save<CategoryUpdatedEvent>(categoryUpdatedEvent)

  if (newArchivalStatus.valueOf()) {
    const status = new CategoryStatusArchived()
    status.categoryUpdatedEventId = categoryUpdatedEvent.id
    category.status = status
  } else {
    category.status = new CategoryStatusActive()
  }
  category.updatedAt = eventTime
  await db.save<ForumCategory>(category)
}

export async function forum_CategoryDeleted(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [categoryId, privilegedActor] = new Forum.CategoryDeletedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const category = await getCategory(db, categoryId.toString())
  const actorWorker = await getActorWorker(db, privilegedActor)

  const categoryDeletedEvent = new CategoryDeletedEvent({
    ...genericEventFields(event_),
    category,
    actor: actorWorker,
  })
  await db.save<CategoryDeletedEvent>(categoryDeletedEvent)

  const newStatus = new CategoryStatusRemoved()
  newStatus.categoryDeletedEventId = categoryDeletedEvent.id

  category.updatedAt = eventTime
  category.status = newStatus
  await db.save<ForumCategory>(category)
}

export async function forum_ThreadCreated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const { forumUserId, categoryId, title, text, poll } = new Forum.CreateThreadCall(event_).args
  const [threadId] = new Forum.ThreadCreatedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const author = new Membership({ id: forumUserId.toString() })

  const thread = new ForumThread({
    createdAt: eventTime,
    updatedAt: eventTime,
    id: threadId.toString(),
    author,
    category: new ForumCategory({ id: categoryId.toString() }),
    title: bytesToString(title),
    isSticky: false,
    status: new ThreadStatusActive(),
  })
  await db.save<ForumThread>(thread)

  if (poll.isSome) {
    const threadPoll = new ForumPoll({
      createdAt: eventTime,
      updatedAt: eventTime,
      description: bytesToString(poll.unwrap().description_hash), // FIXME: This should be raw description!
      endTime: new Date(poll.unwrap().end_time.toNumber()),
      thread,
    })
    await db.save<ForumPoll>(threadPoll)
    await Promise.all(
      poll.unwrap().poll_alternatives.map(async (alt, index) => {
        const alternative = new ForumPollAlternative({
          createdAt: eventTime,
          updatedAt: eventTime,
          poll: threadPoll,
          text: bytesToString(alt.alternative_text_hash), // FIXME: This should be raw text!
          index,
        })

        await db.save<ForumPollAlternative>(alternative)
      })
    )
  }

  const threadCreatedEvent = new ThreadCreatedEvent({
    ...genericEventFields(event_),
    thread,
    title: bytesToString(title),
    text: bytesToString(text),
  })
  await db.save<ThreadCreatedEvent>(threadCreatedEvent)

  const postOrigin = new PostOriginThreadInitial()
  postOrigin.threadCreatedEventId = threadCreatedEvent.id

  const initialPost = new ForumPost({
    // FIXME: The postId is unknown
    createdAt: eventTime,
    updatedAt: eventTime,
    author,
    thread,
    text: bytesToString(text),
    status: new PostStatusActive(),
    origin: postOrigin,
  })
  await db.save<ForumPost>(initialPost)
}

export async function forum_ThreadModerated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [threadId, rationaleBytes, privilegedActor] = new Forum.ThreadModeratedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const actorWorker = await getActorWorker(db, privilegedActor)
  const thread = await getThread(db, threadId.toString())

  const threadModeratedEvent = new ThreadModeratedEvent({
    ...genericEventFields(event_),
    actor: actorWorker,
    thread,
    rationale: bytesToString(rationaleBytes),
  })

  await db.save<ThreadModeratedEvent>(threadModeratedEvent)

  const newStatus = new ThreadStatusModerated()
  newStatus.threadModeratedEventId = threadModeratedEvent.id

  thread.updatedAt = eventTime
  thread.status = newStatus
  await db.save<ForumThread>(thread)
}

export async function forum_ThreadTitleUpdated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [threadId, , , newTitleBytes] = new Forum.ThreadTitleUpdatedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const thread = await getThread(db, threadId.toString())

  const threadTitleUpdatedEvent = new ThreadTitleUpdatedEvent({
    ...genericEventFields(event_),
    thread,
    newTitle: bytesToString(newTitleBytes),
  })

  await db.save<ThreadTitleUpdatedEvent>(threadTitleUpdatedEvent)

  thread.updatedAt = eventTime
  thread.title = bytesToString(newTitleBytes)
  await db.save<ForumThread>(thread)
}

export async function forum_ThreadDeleted(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [threadId, , , hide] = new Forum.ThreadDeletedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const thread = await getThread(db, threadId.toString())

  const threadDeletedEvent = new ThreadDeletedEvent({
    ...genericEventFields(event_),
    thread,
  })

  await db.save<ThreadDeletedEvent>(threadDeletedEvent)

  const status = hide.valueOf() ? new ThreadStatusRemoved() : new ThreadStatusLocked()
  status.threadDeletedEventId = threadDeletedEvent.id
  thread.status = status
  thread.updatedAt = eventTime
  await db.save<ForumThread>(thread)
}

export async function forum_ThreadMoved(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [threadId, newCategoryId, privilegedActor, oldCategoryId] = new Forum.ThreadMovedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const thread = await getThread(db, threadId.toString())
  const actorWorker = await getActorWorker(db, privilegedActor)

  const threadMovedEvent = new ThreadMovedEvent({
    ...genericEventFields(event_),
    thread,
    oldCategory: new ForumCategory({ id: oldCategoryId.toString() }),
    newCategory: new ForumCategory({ id: newCategoryId.toString() }),
    actor: actorWorker,
  })

  await db.save<ThreadMovedEvent>(threadMovedEvent)

  thread.updatedAt = eventTime
  thread.category = new ForumCategory({ id: newCategoryId.toString() })
  await db.save<ForumThread>(thread)
}

export async function forum_VoteOnPoll(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [threadId, alternativeIndex, forumUserId] = new Forum.VoteOnPollEvent(event_).params
  const pollAlternative = await getPollAlternative(db, threadId.toString(), alternativeIndex.toNumber())
  const votingMember = new Membership({ id: forumUserId.toString() })

  const voteOnPollEvent = new VoteOnPollEvent({
    ...genericEventFields(event_),
    pollAlternative,
    votingMember,
  })

  await db.save<VoteOnPollEvent>(voteOnPollEvent)
}

export async function forum_PostAdded(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [postId, forumUserId, , threadId, metadataBytes, isEditable] = new Forum.PostAddedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)

  const metadata = deserializeMetadata(ForumPostMetadata, metadataBytes)
  const postText = metadata ? metadata.text || '' : bytesToString(metadataBytes)
  const repliesToPost =
    typeof metadata?.repliesTo === 'number' &&
    (await db.get(ForumPost, { where: { id: metadata.repliesTo.toString() } }))

  const postStatus = isEditable.valueOf() ? new PostStatusActive() : new PostStatusLocked()
  const postOrigin = new PostOriginThreadReply()

  const post = new ForumPost({
    id: postId.toString(),
    createdAt: eventTime,
    updatedAt: eventTime,
    text: postText,
    thread: new ForumThread({ id: threadId.toString() }),
    status: postStatus,
    author: new Membership({ id: forumUserId.toString() }),
    origin: postOrigin,
    repliesTo: repliesToPost || undefined,
  })
  await db.save<ForumPost>(post)

  const postAddedEvent = new PostAddedEvent({
    ...genericEventFields(event_),
    post,
    isEditable: isEditable.valueOf(),
    text: postText,
  })

  await db.save<PostAddedEvent>(postAddedEvent)
  // Update the other side of cross-relationship
  postOrigin.postAddedEventId = postAddedEvent.id
  await db.save<ForumPost>(post)
}

export async function forum_CategoryStickyThreadUpdate(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [categoryId, newStickyThreadsIdsVec, privilegedActor] = new Forum.CategoryStickyThreadUpdateEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const actorWorker = await getActorWorker(db, privilegedActor)
  const newStickyThreadsIds = newStickyThreadsIdsVec.map((id) => id.toString())
  const threadsToSetSticky = await db.getMany(ForumThread, {
    where: { category: { id: categoryId.toString() }, id: newStickyThreadsIds },
  })
  const threadsToUnsetSticky = await db.getMany(ForumThread, {
    where: { category: { id: categoryId.toString() }, isSticky: true, id: Not(newStickyThreadsIds) },
  })

  const setStickyUpdates = (threadsToSetSticky || []).map(async (t) => {
    t.updatedAt = eventTime
    t.isSticky = true
    await db.save<ForumThread>(t)
  })

  const unsetStickyUpdates = (threadsToUnsetSticky || []).map(async (t) => {
    t.updatedAt = eventTime
    t.isSticky = false
    await db.save<ForumThread>(t)
  })

  await Promise.all(setStickyUpdates.concat(unsetStickyUpdates))

  const categoryStickyThreadUpdateEvent = new CategoryStickyThreadUpdateEvent({
    ...genericEventFields(event_),
    actor: actorWorker,
    category: new ForumCategory({ id: categoryId.toString() }),
    newStickyThreads: threadsToSetSticky,
  })

  await db.save<CategoryStickyThreadUpdateEvent>(categoryStickyThreadUpdateEvent)
}

export async function forum_CategoryMembershipOfModeratorUpdated(
  db: DatabaseManager,
  event_: SubstrateEvent
): Promise<void> {
  const [moderatorId, categoryId, canModerate] = new Forum.CategoryMembershipOfModeratorUpdatedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const moderator = await getWorker(db, 'forumWorkingGroup', moderatorId.toNumber())
  const category = await getCategory(db, categoryId.toString(), ['moderators'])

  if (canModerate.valueOf()) {
    category.moderators.push(moderator)
    category.updatedAt = eventTime
    await db.save<ForumCategory>(category)
  } else {
    category.moderators.splice(category.moderators.map((m) => m.id).indexOf(moderator.id), 1)
    category.updatedAt = eventTime
    await db.save<ForumCategory>(category)
  }

  const categoryMembershipOfModeratorUpdatedEvent = new CategoryMembershipOfModeratorUpdatedEvent({
    ...genericEventFields(event_),
    category,
    moderator,
    newCanModerateValue: canModerate.valueOf(),
  })
  await db.save<CategoryMembershipOfModeratorUpdatedEvent>(categoryMembershipOfModeratorUpdatedEvent)
}

export async function forum_PostModerated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  const [postId, rationaleBytes, privilegedActor] = new Forum.PostModeratedEvent(event_).params
  const eventTime = new Date(event_.blockTimestamp)
  const actorWorker = await getActorWorker(db, privilegedActor)
  const post = await getPost(db, postId.toString())

  const postModeratedEvent = new PostModeratedEvent({
    ...genericEventFields(event_),
    actor: actorWorker,
    post,
    rationale: bytesToString(rationaleBytes),
  })

  await db.save<PostModeratedEvent>(postModeratedEvent)

  const newStatus = new PostStatusModerated()
  newStatus.postModeratedEventId = postModeratedEvent.id

  post.updatedAt = eventTime
  post.status = newStatus
  await db.save<ForumPost>(post)
}

export async function forum_PostDeleted(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  // TODO
}

export async function forum_PostTextUpdated(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  // TODO
}

export async function forum_PostReacted(db: DatabaseManager, event_: SubstrateEvent): Promise<void> {
  // TODO
}