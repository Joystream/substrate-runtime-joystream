/*
eslint-disable @typescript-eslint/naming-convention
*/
import { EventContext, StoreContext, DatabaseManager, SubstrateEvent } from '@dzlzv/hydra-common'

import { StorageWorkingGroup as WorkingGroups } from './generated/types'
import {
  ApplicationMetadata,
  IAddUpcomingOpening,
  IOpeningMetadata,
  IRemoveUpcomingOpening,
  ISetGroupMetadata,
  IWorkingGroupMetadata,
  IWorkingGroupMetadataAction,
  OpeningMetadata,
  WorkingGroupMetadataAction,
} from '@joystream/metadata-protobuf'
import { Bytes } from '@polkadot/types'
import { deserializeMetadata, bytesToString, genericEventFields } from './common'
import BN from 'bn.js'
import {
  WorkingGroupOpening,
  OpeningAddedEvent,
  WorkingGroup,
  WorkingGroupOpeningMetadata,
  ApplicationFormQuestion,
  ApplicationFormQuestionType,
  OpeningStatusOpen,
  WorkingGroupOpeningType,
  WorkingGroupApplication,
  ApplicationFormQuestionAnswer,
  AppliedOnOpeningEvent,
  Membership,
  ApplicationStatusPending,
  ApplicationStatusAccepted,
  ApplicationStatusRejected,
  Worker,
  WorkerStatusActive,
  OpeningFilledEvent,
  OpeningStatusFilled,
  // LeaderSetEvent,
  OpeningCanceledEvent,
  OpeningStatusCancelled,
  ApplicationStatusCancelled,
  ApplicationWithdrawnEvent,
  ApplicationStatusWithdrawn,
  UpcomingWorkingGroupOpening,
  StatusTextChangedEvent,
  WorkingGroupMetadata,
  WorkingGroupMetadataSet,
  UpcomingOpeningRemoved,
  InvalidActionMetadata,
  WorkingGroupMetadataActionResult,
  UpcomingOpeningAdded,
  WorkerRoleAccountUpdatedEvent,
  WorkerRewardAccountUpdatedEvent,
  StakeIncreasedEvent,
  RewardPaidEvent,
  RewardPaymentType,
  NewMissedRewardLevelReachedEvent,
  WorkerExitedEvent,
  WorkerStatusLeft,
  WorkerStatusTerminated,
  TerminatedWorkerEvent,
  LeaderUnsetEvent,
  TerminatedLeaderEvent,
  WorkerRewardAmountUpdatedEvent,
  StakeSlashedEvent,
  StakeDecreasedEvent,
  WorkerStartedLeavingEvent,
  BudgetSetEvent,
  BudgetSpendingEvent,
  LeaderSetEvent,
  WorkerStatusLeaving,
} from 'query-node/dist/model'
import { createType } from '@joystream/types'

// Reusable functions
async function getWorkingGroup(
  store: DatabaseManager,
  event: SubstrateEvent,
  relations: string[] = []
): Promise<WorkingGroup> {
  const [groupName] = event.name.split('.')
  const group = await store.get(WorkingGroup, { where: { name: groupName }, relations })
  if (!group) {
    throw new Error(`Working group ${groupName} not found!`)
  }

  return group
}

async function getOpening(
  store: DatabaseManager,
  openingstoreId: string,
  relations: string[] = []
): Promise<WorkingGroupOpening> {
  const opening = await store.get(WorkingGroupOpening, { where: { id: openingstoreId }, relations })
  if (!opening) {
    throw new Error(`Opening not found by id ${openingstoreId}`)
  }

  return opening
}

async function getApplication(store: DatabaseManager, applicationstoreId: string): Promise<WorkingGroupApplication> {
  const application = await store.get(WorkingGroupApplication, { where: { id: applicationstoreId } })
  if (!application) {
    throw new Error(`Application not found by id ${applicationstoreId}`)
  }

  return application
}

async function getWorker(store: DatabaseManager, workerstoreId: string): Promise<Worker> {
  const worker = await store.get(Worker, { where: { id: workerstoreId } })
  if (!worker) {
    throw new Error(`Worker not found by id ${workerstoreId}`)
  }

  return worker
}

async function getApplicationFormQuestions(
  store: DatabaseManager,
  openingstoreId: string
): Promise<ApplicationFormQuestion[]> {
  const openingWithQuestions = await getOpening(store, openingstoreId, [
    'metadata',
    'metadata.applicationFormQuestions',
  ])

  if (!openingWithQuestions) {
    throw new Error(`Opening not found by id: ${openingstoreId}`)
  }
  if (!openingWithQuestions.metadata.applicationFormQuestions) {
    throw new Error(`Application form questions not found for opening: ${openingstoreId}`)
  }
  return openingWithQuestions.metadata.applicationFormQuestions
}

const InputTypeToApplicationFormQuestionType = {
  [OpeningMetadata.ApplicationFormQuestion.InputType.TEXT]: ApplicationFormQuestionType.TEXT,
  [OpeningMetadata.ApplicationFormQuestion.InputType.TEXTAREA]: ApplicationFormQuestionType.TEXTAREA,
}

function parseQuestionInputType(
  type?: OpeningMetadata.ApplicationFormQuestion.InputType | null
): ApplicationFormQuestionType {
  const validType: OpeningMetadata.ApplicationFormQuestion.InputType = type || 0
  return InputTypeToApplicationFormQuestionType[validType]
}

export async function createWorkingGroupOpeningMetadata(
  store: DatabaseManager,
  eventTime: Date,
  originalMeta: Bytes | IOpeningMetadata
): Promise<WorkingGroupOpeningMetadata> {
  let originallyValid: boolean
  let metadata: IOpeningMetadata
  if (originalMeta instanceof Bytes) {
    const deserializedMetadata = await deserializeMetadata(OpeningMetadata, originalMeta)
    metadata = deserializedMetadata || {}
    originallyValid = !!deserializedMetadata
  } else {
    metadata = originalMeta
    originallyValid = true
  }

  const {
    applicationFormQuestions,
    applicationDetails,
    description,
    expectedEndingTimestamp,
    hiringLimit,
    shortDescription,
  } = metadata

  const openingMetadata = new WorkingGroupOpeningMetadata({
    createdAt: eventTime,
    updatedAt: eventTime,
    originallyValid,
    applicationDetails: applicationDetails || undefined,
    description: description || undefined,
    shortDescription: shortDescription || undefined,
    hiringLimit: hiringLimit || undefined,
    expectedEnding: expectedEndingTimestamp ? new Date(expectedEndingTimestamp) : undefined,
    applicationFormQuestions: [],
  })

  await store.save<WorkingGroupOpeningMetadata>(openingMetadata)

  await Promise.all(
    (applicationFormQuestions || []).map(async ({ question, type }, index) => {
      const applicationFormQuestion = new ApplicationFormQuestion({
        createdAt: eventTime,
        updatedAt: eventTime,
        question: question || undefined,
        type: parseQuestionInputType(type),
        index,
        openingMetadata,
      })
      await store.save<ApplicationFormQuestion>(applicationFormQuestion)
      return applicationFormQuestion
    })
  )

  return openingMetadata
}

async function createApplicationQuestionAnswers(
  store: DatabaseManager,
  application: WorkingGroupApplication,
  metadataBytes: Bytes
) {
  const metadata = deserializeMetadata(ApplicationMetadata, metadataBytes)
  if (!metadata) {
    return
  }
  const questions = await getApplicationFormQuestions(store, application.opening.id)
  const { answers } = metadata
  await Promise.all(
    (answers || []).slice(0, questions.length).map(async (answer, index) => {
      const applicationFormQuestionAnswer = new ApplicationFormQuestionAnswer({
        createdAt: application.createdAt,
        updatedAt: application.updatedAt,
        application,
        question: questions[index],
        answer,
      })

      await store.save<ApplicationFormQuestionAnswer>(applicationFormQuestionAnswer)
      return applicationFormQuestionAnswer
    })
  )
}

async function handleAddUpcomingOpeningAction(
  store: DatabaseManager,
  event: SubstrateEvent,
  statusChangedEvent: StatusTextChangedEvent,
  action: IAddUpcomingOpening
): Promise<UpcomingOpeningAdded | InvalidActionMetadata> {
  const upcomingOpeningMeta = action.metadata || {}
  const group = await getWorkingGroup(store, event)
  const eventTime = new Date(event.blockTimestamp)
  const openingMeta = await await createWorkingGroupOpeningMetadata(
    store,
    eventTime,
    upcomingOpeningMeta.metadata || {}
  )
  const { rewardPerBlock, expectedStart, minApplicationStake } = upcomingOpeningMeta
  const upcomingOpening = new UpcomingWorkingGroupOpening({
    createdAt: eventTime,
    updatedAt: eventTime,
    metadata: openingMeta,
    group,
    rewardPerBlock: rewardPerBlock?.toNumber() ? new BN(rewardPerBlock.toString()) : undefined,
    expectedStart: expectedStart ? new Date(expectedStart) : undefined,
    stakeAmount: minApplicationStake?.toNumber() ? new BN(minApplicationStake.toString()) : undefined,
    createdInEvent: statusChangedEvent,
  })
  await store.save<UpcomingWorkingGroupOpening>(upcomingOpening)

  const result = new UpcomingOpeningAdded()
  result.upcomingOpeningId = upcomingOpening.id

  return result
}

async function handleRemoveUpcomingOpeningAction(
  store: DatabaseManager,
  action: IRemoveUpcomingOpening
): Promise<UpcomingOpeningRemoved | InvalidActionMetadata> {
  const { id } = action
  const upcomingOpening = await store.get(UpcomingWorkingGroupOpening, { where: { id } })
  let result: UpcomingOpeningRemoved | InvalidActionMetadata
  if (upcomingOpening) {
    result = new UpcomingOpeningRemoved()
    result.upcomingOpeningId = upcomingOpening.id
    await store.remove<UpcomingWorkingGroupOpening>(upcomingOpening)
  } else {
    const error = `Cannot remove upcoming opening: Entity by id ${id} not found!`
    console.error(error)
    result = new InvalidActionMetadata()
    result.reason = error
  }
  return result
}

async function handleSetWorkingGroupMetadataAction(
  store: DatabaseManager,
  event: SubstrateEvent,
  statusChangedEvent: StatusTextChangedEvent,
  action: ISetGroupMetadata
): Promise<WorkingGroupMetadataSet> {
  const { newMetadata } = action
  const group = await getWorkingGroup(store, event, ['metadata'])
  const oldMetadata = group.metadata
  const eventTime = new Date(event.blockTimestamp)
  const setNewOptionalString = (field: keyof IWorkingGroupMetadata) =>
    typeof newMetadata?.[field] === 'string' ? newMetadata[field] || undefined : oldMetadata?.[field]

  const newGroupMetadata = new WorkingGroupMetadata({
    createdAt: eventTime,
    updatedAt: eventTime,
    setInEvent: statusChangedEvent,
    group,
    status: setNewOptionalString('status'),
    statusMessage: setNewOptionalString('statusMessage'),
    about: setNewOptionalString('about'),
    description: setNewOptionalString('description'),
  })
  await store.save<WorkingGroupMetadata>(newGroupMetadata)

  group.metadata = newGroupMetadata
  group.updatedAt = eventTime
  await store.save<WorkingGroup>(group)

  const result = new WorkingGroupMetadataSet()
  result.metadataId = newGroupMetadata.id

  return result
}

async function handleWorkingGroupMetadataAction(
  store: DatabaseManager,
  event: SubstrateEvent,
  statusChangedEvent: StatusTextChangedEvent,
  action: IWorkingGroupMetadataAction
): Promise<typeof WorkingGroupMetadataActionResult> {
  if (action.addUpcomingOpening) {
    return handleAddUpcomingOpeningAction(store, event, statusChangedEvent, action.addUpcomingOpening)
  } else if (action.removeUpcomingOpening) {
    return handleRemoveUpcomingOpeningAction(store, action.removeUpcomingOpening)
  } else if (action.setGroupMetadata) {
    return handleSetWorkingGroupMetadataAction(store, event, statusChangedEvent, action.setGroupMetadata)
  } else {
    const result = new InvalidActionMetadata()
    result.reason = 'No known action was provided'
    return result
  }
}

async function handleTerminatedWorker({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, optPenalty, optRationale] = new WorkingGroups.TerminatedWorkerEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const EventConstructor = worker.isLead ? TerminatedLeaderEvent : TerminatedWorkerEvent

  const terminatedEvent = new EventConstructor({
    ...genericEventFields(event),
    group,
    worker,
    penalty: optPenalty.unwrapOr(undefined),
    rationale: optRationale.isSome ? bytesToString(optRationale.unwrap()) : undefined,
  })

  await store.save(terminatedEvent)

  const status = new WorkerStatusTerminated()
  status.terminatedWorkerEventId = terminatedEvent.id
  worker.status = status
  worker.stake = new BN(0)
  worker.rewardPerBlock = new BN(0)
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function findLeaderSetEventByTxHash(store: DatabaseManager, txHash?: string): Promise<LeaderSetEvent> {
  const leaderSetEvent = await store.get(LeaderSetEvent, { where: { inExtrinsic: txHash } })

  if (!leaderSetEvent) {
    throw new Error(`LeaderSet event not found by tx hash: ${txHash}`)
  }

  return leaderSetEvent
}

// Mapping functions
export async function workingGroups_OpeningAdded({ store, event }: EventContext & StoreContext): Promise<void> {
  const [
    openingRuntimeId,
    metadataBytes,
    openingType,
    stakePolicy,
    optRewardPerBlock,
  ] = new WorkingGroups.OpeningAddedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const eventTime = new Date(event.blockTimestamp)

  const opening = new WorkingGroupOpening({
    createdAt: eventTime,
    updatedAt: eventTime,
    id: `${group.name}-${openingRuntimeId.toString()}`,
    runtimeId: openingRuntimeId.toNumber(),
    applications: [],
    group,
    rewardPerBlock: optRewardPerBlock.unwrapOr(new BN(0)),
    stakeAmount: stakePolicy.stake_amount,
    unstakingPeriod: stakePolicy.leaving_unstaking_period.toNumber(),
    status: new OpeningStatusOpen(),
    type: openingType.isLeader ? WorkingGroupOpeningType.LEADER : WorkingGroupOpeningType.REGULAR,
  })

  const metadata = await createWorkingGroupOpeningMetadata(store, eventTime, metadataBytes)
  opening.metadata = metadata

  await store.save<WorkingGroupOpening>(opening)

  const openingAddedEvent = new OpeningAddedEvent({
    ...genericEventFields(event),
    group,
    opening,
  })

  await store.save<OpeningAddedEvent>(openingAddedEvent)
}

export async function workingGroups_AppliedOnOpening({ store, event }: EventContext & StoreContext): Promise<void> {
  const eventTime = new Date(event.blockTimestamp)

  const [
    {
      opening_id: openingRuntimeId,
      description: metadataBytes,
      member_id: memberId,
      reward_account_id: rewardAccount,
      role_account_id: roleAccout,
      stake_parameters: { stake, staking_account_id: stakingAccount },
    },
    applicationRuntimeId,
  ] = new WorkingGroups.AppliedOnOpeningEvent(event).params

  const group = await getWorkingGroup(store, event)
  const openingstoreId = `${group.name}-${openingRuntimeId.toString()}`

  const application = new WorkingGroupApplication({
    createdAt: eventTime,
    updatedAt: eventTime,
    id: `${group.name}-${applicationRuntimeId.toString()}`,
    runtimeId: applicationRuntimeId.toNumber(),
    opening: new WorkingGroupOpening({ id: openingstoreId }),
    applicant: new Membership({ id: memberId.toString() }),
    rewardAccount: rewardAccount.toString(),
    roleAccount: roleAccout.toString(),
    stakingAccount: stakingAccount.toString(),
    status: new ApplicationStatusPending(),
    answers: [],
    stake,
  })

  await store.save<WorkingGroupApplication>(application)
  await createApplicationQuestionAnswers(store, application, metadataBytes)

  const appliedOnOpeningEvent = new AppliedOnOpeningEvent({
    ...genericEventFields(event),
    group,
    opening: new WorkingGroupOpening({ id: openingstoreId }),
    application,
  })

  await store.save<AppliedOnOpeningEvent>(appliedOnOpeningEvent)
}

export async function workingGroups_LeaderSet({ store, event }: EventContext & StoreContext): Promise<void> {
  const group = await getWorkingGroup(store, event)

  const leaderSetEvent = new LeaderSetEvent({
    ...genericEventFields(event),
    group,
  })

  await store.save<LeaderSetEvent>(leaderSetEvent)
}

export async function workingGroups_OpeningFilled({ store, event }: EventContext & StoreContext): Promise<void> {
  const eventTime = new Date(event.blockTimestamp)

  const [openingRuntimeId, applicationIdToWorkerIdMap, applicationIdsSet] = new WorkingGroups.OpeningFilledEvent(
    event
  ).params

  const group = await getWorkingGroup(store, event)
  const opening = await getOpening(store, `${group.name}-${openingRuntimeId.toString()}`, [
    'applications',
    'applications.applicant',
  ])
  const acceptedApplicationIds = createType('Vec<ApplicationId>', applicationIdsSet.toHex() as any)

  // Save the event
  const openingFilledEvent = new OpeningFilledEvent({
    ...genericEventFields(event),
    group,
    opening,
  })

  await store.save<OpeningFilledEvent>(openingFilledEvent)

  // Update applications and create new workers
  const hiredWorkers = (
    await Promise.all(
      (opening.applications || [])
        // Skip withdrawn applications
        .filter((application) => application.status.isTypeOf !== 'ApplicationStatusWithdrawn')
        .map(async (application) => {
          const isAccepted = acceptedApplicationIds.some((runtimeId) => runtimeId.toNumber() === application.runtimeId)
          const applicationStatus = isAccepted ? new ApplicationStatusAccepted() : new ApplicationStatusRejected()
          applicationStatus.openingFilledEventId = openingFilledEvent.id
          application.status = applicationStatus
          application.updatedAt = eventTime
          await store.save<WorkingGroupApplication>(application)
          if (isAccepted) {
            // Cannot use "applicationIdToWorkerIdMap.get" here,
            // it only works if the passed instance is identical to BTreeMap key instance (=== instead of .eq)
            const [, workerRuntimeId] =
              Array.from(applicationIdToWorkerIdMap.entries()).find(
                ([applicationRuntimeId]) => applicationRuntimeId.toNumber() === application.runtimeId
              ) || []
            if (!workerRuntimeId) {
              throw new Error(
                `Fatal: No worker id found by accepted application id ${application.id} when handling OpeningFilled event!`
              )
            }
            const worker = new Worker({
              createdAt: eventTime,
              updatedAt: eventTime,
              id: `${group.name}-${workerRuntimeId.toString()}`,
              runtimeId: workerRuntimeId.toNumber(),
              application,
              group,
              isLead: opening.type === WorkingGroupOpeningType.LEADER,
              membership: application.applicant,
              stake: application.stake,
              roleAccount: application.roleAccount,
              rewardAccount: application.rewardAccount,
              stakeAccount: application.stakingAccount,
              payouts: [],
              status: new WorkerStatusActive(),
              entry: openingFilledEvent,
              rewardPerBlock: opening.rewardPerBlock,
            })
            await store.save<Worker>(worker)
            return worker
          }
        })
    )
  ).filter((w) => w !== undefined) as Worker[]

  // Set opening status
  const openingFilled = new OpeningStatusFilled()
  openingFilled.openingFilledEventId = openingFilledEvent.id
  opening.status = openingFilled
  opening.updatedAt = eventTime
  await store.save<WorkingGroupOpening>(opening)

  // Update working group and LeaderSetEvent if necessary
  if (opening.type === WorkingGroupOpeningType.LEADER && hiredWorkers.length) {
    group.leader = hiredWorkers[0]
    group.updatedAt = eventTime
    await store.save<WorkingGroup>(group)

    const leaderSetEvent = await findLeaderSetEventByTxHash(store, openingFilledEvent.inExtrinsic)
    leaderSetEvent.worker = hiredWorkers[0]
    leaderSetEvent.updatedAt = eventTime
    await store.save<LeaderSetEvent>(leaderSetEvent)
  }
}

export async function workingGroups_OpeningCanceled({ store, event }: EventContext & StoreContext): Promise<void> {
  const [openingRuntimeId] = new WorkingGroups.OpeningCanceledEvent(event).params

  const group = await getWorkingGroup(store, event)
  const opening = await getOpening(store, `${group.name}-${openingRuntimeId.toString()}`, ['applications'])
  const eventTime = new Date(event.blockTimestamp)

  // Create and save event
  const openingCanceledEvent = new OpeningCanceledEvent({
    ...genericEventFields(event),
    group,
    opening,
  })

  await store.save<OpeningCanceledEvent>(openingCanceledEvent)

  // Set opening status
  const openingCancelled = new OpeningStatusCancelled()
  openingCancelled.openingCanceledEventId = openingCanceledEvent.id
  opening.status = openingCancelled
  opening.updatedAt = eventTime

  await store.save<WorkingGroupOpening>(opening)

  // Set applications status
  const applicationCancelled = new ApplicationStatusCancelled()
  applicationCancelled.openingCanceledEventId = openingCanceledEvent.id
  await Promise.all(
    (opening.applications || [])
      // Skip withdrawn applications
      .filter((application) => application.status.isTypeOf !== 'ApplicationStatusWithdrawn')
      .map(async (application) => {
        application.status = applicationCancelled
        application.updatedAt = eventTime
        await store.save<WorkingGroupApplication>(application)
      })
  )
}

export async function workingGroups_ApplicationWithdrawn({ store, event }: EventContext & StoreContext): Promise<void> {
  const [applicationRuntimeId] = new WorkingGroups.ApplicationWithdrawnEvent(event).params

  const group = await getWorkingGroup(store, event)
  const application = await getApplication(store, `${group.name}-${applicationRuntimeId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  // Create and save event
  const applicationWithdrawnEvent = new ApplicationWithdrawnEvent({
    ...genericEventFields(event),
    group,
    application,
  })

  await store.save<ApplicationWithdrawnEvent>(applicationWithdrawnEvent)

  // Set application status
  const statusWithdrawn = new ApplicationStatusWithdrawn()
  statusWithdrawn.applicationWithdrawnEventId = applicationWithdrawnEvent.id
  application.status = statusWithdrawn
  application.updatedAt = eventTime

  await store.save<WorkingGroupApplication>(application)
}

export async function workingGroups_StatusTextChanged({ store, event }: EventContext & StoreContext): Promise<void> {
  const [, optBytes] = new WorkingGroups.StatusTextChangedEvent(event).params
  const group = await getWorkingGroup(store, event)

  // Since result cannot be empty at this point, but we already need to have an existing StatusTextChangedEvent
  // in order to be able to create UpcomingOpening.createdInEvent relation, we use a temporary "mock" result
  const mockResult = new InvalidActionMetadata()
  mockResult.reason = 'Metadata not yet processed'
  const statusTextChangedEvent = new StatusTextChangedEvent({
    ...genericEventFields(event),
    group,
    metadata: optBytes.isSome ? optBytes.unwrap().toString() : undefined,
    result: mockResult,
  })

  await store.save<StatusTextChangedEvent>(statusTextChangedEvent)

  let result: typeof WorkingGroupMetadataActionResult

  if (optBytes.isSome) {
    const metadata = deserializeMetadata(WorkingGroupMetadataAction, optBytes.unwrap())
    if (metadata) {
      result = await handleWorkingGroupMetadataAction(store, event, statusTextChangedEvent, metadata)
    } else {
      result = new InvalidActionMetadata()
      result.reason = 'Invalid metadata: Cannot deserialize metadata binary'
    }
  } else {
    const error = 'No encoded metadata was provided'
    console.error(`StatusTextChanged event: ${error}`)
    result = new InvalidActionMetadata()
    result.reason = error
  }

  // Now we can set the "real" result
  statusTextChangedEvent.result = result
  await store.save<StatusTextChangedEvent>(statusTextChangedEvent)
}

export async function workingGroups_WorkerRoleAccountUpdated({
  store,
  event,
}: EventContext & StoreContext): Promise<void> {
  const [workerId, accountId] = new WorkingGroups.WorkerRoleAccountUpdatedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerRoleAccountUpdatedEvent = new WorkerRoleAccountUpdatedEvent({
    ...genericEventFields(event),
    group,
    worker,
    newRoleAccount: accountId.toString(),
  })

  await store.save<WorkerRoleAccountUpdatedEvent>(workerRoleAccountUpdatedEvent)

  worker.roleAccount = accountId.toString()
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_WorkerRewardAccountUpdated({
  store,
  event,
}: EventContext & StoreContext): Promise<void> {
  const [workerId, accountId] = new WorkingGroups.WorkerRewardAccountUpdatedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerRewardAccountUpdatedEvent = new WorkerRewardAccountUpdatedEvent({
    ...genericEventFields(event),
    group,
    worker,
    newRewardAccount: accountId.toString(),
  })

  await store.save<WorkerRoleAccountUpdatedEvent>(workerRewardAccountUpdatedEvent)

  worker.rewardAccount = accountId.toString()
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_StakeIncreased({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, increaseAmount] = new WorkingGroups.StakeIncreasedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const stakeIncreasedEvent = new StakeIncreasedEvent({
    ...genericEventFields(event),
    group,
    worker,
    amount: increaseAmount,
  })

  await store.save<StakeIncreasedEvent>(stakeIncreasedEvent)

  worker.stake = worker.stake.add(increaseAmount)
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_RewardPaid({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, rewardAccountId, amount, rewardPaymentType] = new WorkingGroups.RewardPaidEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const rewardPaidEvent = new RewardPaidEvent({
    ...genericEventFields(event),
    group,
    worker,
    amount,
    rewardAccount: rewardAccountId.toString(),
    paymentType: rewardPaymentType.isRegularReward ? RewardPaymentType.REGULAR : RewardPaymentType.MISSED,
  })

  await store.save<RewardPaidEvent>(rewardPaidEvent)

  // Update group budget
  group.budget = group.budget.sub(amount)
  group.updatedAt = eventTime

  await store.save<WorkingGroup>(group)
}

export async function workingGroups_NewMissedRewardLevelReached({
  store,
  event,
}: EventContext & StoreContext): Promise<void> {
  const [workerId, newMissedRewardAmountOpt] = new WorkingGroups.NewMissedRewardLevelReachedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const newMissedRewardLevelReachedEvent = new NewMissedRewardLevelReachedEvent({
    ...genericEventFields(event),
    group,
    worker,
    newMissedRewardAmount: newMissedRewardAmountOpt.unwrapOr(new BN(0)),
  })

  await store.save<NewMissedRewardLevelReachedEvent>(newMissedRewardLevelReachedEvent)

  // Update worker
  worker.missingRewardAmount = newMissedRewardAmountOpt.unwrapOr(undefined)
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_WorkerExited({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId] = new WorkingGroups.WorkerExitedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerExitedEvent = new WorkerExitedEvent({
    ...genericEventFields(event),
    group,
    worker,
  })

  await store.save<WorkerExitedEvent>(workerExitedEvent)

  const newStatus = new WorkerStatusLeft()
  newStatus.workerStartedLeavingEventId = (worker.status as WorkerStatusLeaving).workerStartedLeavingEventId
  newStatus.workerExitedEventId = workerExitedEvent.id

  worker.status = newStatus
  worker.stake = new BN(0)
  worker.rewardPerBlock = new BN(0)
  worker.missingRewardAmount = undefined
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_LeaderUnset({ store, event }: EventContext & StoreContext): Promise<void> {
  const group = await getWorkingGroup(store, event, ['leader'])
  const eventTime = new Date(event.blockTimestamp)

  const leaderUnsetEvent = new LeaderUnsetEvent({
    ...genericEventFields(event),
    group,
    leader: group.leader,
  })

  await store.save<LeaderUnsetEvent>(leaderUnsetEvent)

  group.leader = undefined
  group.updatedAt = eventTime

  await store.save<WorkingGroup>(group)
}

export async function workingGroups_TerminatedWorker(ctx: EventContext & StoreContext): Promise<void> {
  await handleTerminatedWorker(ctx)
}
export async function workingGroups_TerminatedLeader(ctx: EventContext & StoreContext): Promise<void> {
  await handleTerminatedWorker(ctx)
}

export async function workingGroups_WorkerRewardAmountUpdated({
  store,
  event,
}: EventContext & StoreContext): Promise<void> {
  const [workerId, newRewardPerBlockOpt] = new WorkingGroups.WorkerRewardAmountUpdatedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerRewardAmountUpdatedEvent = new WorkerRewardAmountUpdatedEvent({
    ...genericEventFields(event),
    group,
    worker,
    newRewardPerBlock: newRewardPerBlockOpt.unwrapOr(new BN(0)),
  })

  await store.save<WorkerRewardAmountUpdatedEvent>(workerRewardAmountUpdatedEvent)

  worker.rewardPerBlock = newRewardPerBlockOpt.unwrapOr(new BN(0))
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_StakeSlashed({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, slashedAmount, requestedAmount, optRationale] = new WorkingGroups.StakeSlashedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerStakeSlashedEvent = new StakeSlashedEvent({
    ...genericEventFields(event),
    group,
    worker,
    requestedAmount,
    slashedAmount,
    rationale: optRationale.isSome ? bytesToString(optRationale.unwrap()) : undefined,
  })

  await store.save<StakeSlashedEvent>(workerStakeSlashedEvent)

  worker.stake = worker.stake.sub(slashedAmount)
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_StakeDecreased({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, amount] = new WorkingGroups.StakeDecreasedEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerStakeDecreasedEvent = new StakeDecreasedEvent({
    ...genericEventFields(event),
    group,
    worker,
    amount,
  })

  await store.save<StakeDecreasedEvent>(workerStakeDecreasedEvent)

  worker.stake = worker.stake.sub(amount)
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_WorkerStartedLeaving({ store, event }: EventContext & StoreContext): Promise<void> {
  const [workerId, optRationale] = new WorkingGroups.WorkerStartedLeavingEvent(event).params
  const group = await getWorkingGroup(store, event)
  const worker = await getWorker(store, `${group.name}-${workerId.toString()}`)
  const eventTime = new Date(event.blockTimestamp)

  const workerStartedLeavingEvent = new WorkerStartedLeavingEvent({
    ...genericEventFields(event),
    group,
    worker,
    rationale: optRationale.isSome ? bytesToString(optRationale.unwrap()) : undefined,
  })

  await store.save<WorkerStartedLeavingEvent>(workerStartedLeavingEvent)

  const status = new WorkerStatusLeaving()
  status.workerStartedLeavingEventId = workerStartedLeavingEvent.id
  worker.status = status
  worker.updatedAt = eventTime

  await store.save<Worker>(worker)
}

export async function workingGroups_BudgetSet({ store, event }: EventContext & StoreContext): Promise<void> {
  const [newBudget] = new WorkingGroups.BudgetSetEvent(event).params
  const group = await getWorkingGroup(store, event)
  const eventTime = new Date(event.blockTimestamp)

  const budgetSetEvent = new BudgetSetEvent({
    ...genericEventFields(event),
    group,
    newBudget,
  })

  await store.save<BudgetSetEvent>(budgetSetEvent)

  group.budget = newBudget
  group.updatedAt = eventTime

  await store.save<WorkingGroup>(group)
}

export async function workingGroups_BudgetSpending({ store, event }: EventContext & StoreContext): Promise<void> {
  const [reciever, amount, optRationale] = new WorkingGroups.BudgetSpendingEvent(event).params
  const group = await getWorkingGroup(store, event)
  const eventTime = new Date(event.blockTimestamp)

  const budgetSpendingEvent = new BudgetSpendingEvent({
    ...genericEventFields(event),
    group,
    amount,
    reciever: reciever.toString(),
    rationale: optRationale.isSome ? bytesToString(optRationale.unwrap()) : undefined,
  })

  await store.save<BudgetSpendingEvent>(budgetSpendingEvent)

  group.budget = group.budget.sub(amount)
  group.updatedAt = eventTime

  await store.save<WorkingGroup>(group)
}
