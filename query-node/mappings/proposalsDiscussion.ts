/*
eslint-disable @typescript-eslint/naming-convention
*/
import { EventContext, StoreContext, DatabaseManager } from '@dzlzv/hydra-common'
import {
  Membership,
  ProposalDiscussionPostStatusActive,
  ProposalDiscussionPostStatusLocked,
  ProposalDiscussionPost,
  ProposalDiscussionThread,
  ProposalDiscussionPostCreatedEvent,
  ProposalDiscussionPostUpdatedEvent,
  ProposalDiscussionThreadModeClosed,
  ProposalDiscussionWhitelist,
  ProposalDiscussionThreadModeOpen,
  ProposalDiscussionThreadModeChangedEvent,
  ProposalDiscussionPostDeletedEvent,
  ProposalDiscussionPostStatusRemoved,
} from 'query-node/dist/model'
import { bytesToString, deserializeMetadata, genericEventFields, MemoryCache } from './common'
import { ProposalsDiscussion } from './generated/types'
import { ProposalsDiscussionPostMetadata } from '@joystream/metadata-protobuf'
import { In } from 'typeorm'

async function getPost(store: DatabaseManager, id: string) {
  const post = await store.get(ProposalDiscussionPost, { where: { id } })
  if (!post) {
    throw new Error(`Proposal discussion post not found by id: ${id}`)
  }

  return post
}

async function getThread(store: DatabaseManager, id: string) {
  const thread = await store.get(ProposalDiscussionThread, { where: { id } })
  if (!thread) {
    throw new Error(`Proposal discussion thread not found by id: ${id}`)
  }

  return thread
}

export async function proposalsDiscussion_ThreadCreated({ event }: EventContext & StoreContext): Promise<void> {
  const [threadId] = new ProposalsDiscussion.ThreadCreatedEvent(event).params
  MemoryCache.lastCreatedProposalThreadId = threadId
}

export async function proposalsDiscussion_PostCreated({ event, store }: EventContext & StoreContext): Promise<void> {
  const { editable } = new ProposalsDiscussion.AddPostCall(event).args // FIXME: batch and sudo extrinsics handling
  const [postId, memberId, threadId, metadataBytes] = new ProposalsDiscussion.PostCreatedEvent(event).params
  const eventTime = new Date(event.blockTimestamp)

  const metadata = deserializeMetadata(ProposalsDiscussionPostMetadata, metadataBytes)

  const repliesTo =
    typeof metadata?.repliesTo === 'number'
      ? await store.get(ProposalDiscussionPost, { where: { id: metadata.repliesTo.toString() } })
      : undefined

  const text = typeof metadata?.text === 'string' ? metadata.text : bytesToString(metadataBytes)

  const post = new ProposalDiscussionPost({
    id: postId.toString(),
    createdAt: eventTime,
    updatedAt: eventTime,
    author: new Membership({ id: memberId.toString() }),
    status: editable.isTrue ? new ProposalDiscussionPostStatusActive() : new ProposalDiscussionPostStatusLocked(),
    text,
    repliesTo,
    thread: new ProposalDiscussionThread({ id: threadId.toString() }),
  })
  await store.save<ProposalDiscussionPost>(post)

  const postCreatedEvent = new ProposalDiscussionPostCreatedEvent({
    ...genericEventFields(event),
    post: post,
    text,
  })
  await store.save<ProposalDiscussionPostCreatedEvent>(postCreatedEvent)
}

export async function proposalsDiscussion_PostUpdated({ event, store }: EventContext & StoreContext): Promise<void> {
  const [postId, , , newTextBytes] = new ProposalsDiscussion.PostUpdatedEvent(event).params

  const post = await getPost(store, postId.toString())
  const newText = bytesToString(newTextBytes)

  post.text = newText
  post.updatedAt = new Date(event.blockTimestamp)
  await store.save<ProposalDiscussionPost>(post)

  const postUpdatedEvent = new ProposalDiscussionPostUpdatedEvent({
    ...genericEventFields(event),
    post,
    text: newText,
  })
  await store.save<ProposalDiscussionPostUpdatedEvent>(postUpdatedEvent)
}

export async function proposalsDiscussion_ThreadModeChanged({
  event,
  store,
}: EventContext & StoreContext): Promise<void> {
  const [threadId, threadMode, memberId] = new ProposalsDiscussion.ThreadModeChangedEvent(event).params
  const eventTime = new Date(event.blockTimestamp)

  const thread = await getThread(store, threadId.toString())

  if (threadMode.isClosed) {
    const newMode = new ProposalDiscussionThreadModeClosed()
    const whitelistMemberIds = threadMode.asClosed
    const members = await store.getMany(Membership, {
      where: { id: In(whitelistMemberIds.map((id) => id.toString())) },
    })
    const whitelist = new ProposalDiscussionWhitelist({
      createdAt: eventTime,
      updatedAt: eventTime,
      members,
    })
    await store.save<ProposalDiscussionWhitelist>(whitelist)
    newMode.whitelistId = whitelist.id
    thread.mode = newMode
  } else if (threadMode.isOpen) {
    const newMode = new ProposalDiscussionThreadModeOpen()
    thread.mode = newMode
  } else {
    throw new Error(`Unrecognized proposal thread mode: ${threadMode.type}`)
  }

  thread.updatedAt = eventTime
  await store.save<ProposalDiscussionThread>(thread)

  const threadModeChangedEvent = new ProposalDiscussionThreadModeChangedEvent({
    ...genericEventFields(event),
    actor: new Membership({ id: memberId.toString() }),
    newMode: thread.mode,
    thread: thread,
  })
  await store.save<ProposalDiscussionThreadModeChangedEvent>(threadModeChangedEvent)
}

export async function proposalsDiscussion_PostDeleted({ event, store }: EventContext & StoreContext): Promise<void> {
  const [memberId, , postId, hide] = new ProposalsDiscussion.PostDeletedEvent(event).params
  const eventTime = new Date(event.blockTimestamp)
  const post = await getPost(store, postId.toString())

  const postDeletedEvent = new ProposalDiscussionPostDeletedEvent({
    ...genericEventFields(event),
    post,
    actor: new Membership({ id: memberId.toString() }),
  })
  await store.save<ProposalDiscussionPostDeletedEvent>(postDeletedEvent)

  const newStatus = hide.isTrue ? new ProposalDiscussionPostStatusRemoved() : new ProposalDiscussionPostStatusLocked()
  newStatus.deletedInEventId = postDeletedEvent.id
  post.status = newStatus
  post.updatedAt = eventTime
  await store.save<ProposalDiscussionPost>(post)
}
