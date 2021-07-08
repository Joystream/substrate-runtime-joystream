import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { FindConditions } from 'typeorm'
import { Bytes } from '@polkadot/types'
import { fixBlockTimestamp } from './eventFix'

import { inconsistentState, logger, createPredictableId } from './common'

import { Channel, Worker, WorkerType } from 'query-node'
import { GatewayWorkingGroup, StorageWorkingGroup } from '../../generated/types'
import { ApplicationId, ApplicationIdToWorkerIdMap, WorkerId } from '@joystream/types/augment'

/// ///////////////// Storage working group //////////////////////////////////////

export async function storageWorkingGroup_OpeningFilled(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { applicationIdToWorkerIdMap } = new StorageWorkingGroup.OpeningFilledEvent(event).data

  // call generic processing
  await workingGroup_OpeningFilled(db, WorkerType.STORAGE, applicationIdToWorkerIdMap, event)
}

export async function storageWorkingGroup_WorkerStorageUpdated(
  db: DatabaseManager,
  event: SubstrateEvent
): Promise<void> {
  // read event data
  const { workerId, bytes: newMetadata } = new StorageWorkingGroup.WorkerStorageUpdatedEvent(event).data

  // call generic processing
  await workingGroup_WorkerStorageUpdated(db, WorkerType.STORAGE, workerId, newMetadata)
}

export async function storageWorkingGroup_TerminatedWorker(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new StorageWorkingGroup.TerminatedWorkerEvent(event).data

  // call generic processing
  await workingGroup_TerminatedWorker(db, event, WorkerType.STORAGE, workerId)
}

export async function storageWorkingGroup_WorkerExited(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new StorageWorkingGroup.WorkerExitedEvent(event).data

  // call generic processing
  await workingGroup_WorkerExited(db, event, WorkerType.STORAGE, workerId)
}

export async function storageWorkingGroup_TerminatedLeader(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new StorageWorkingGroup.TerminatedLeaderEvent(event).data

  // call generic processing
  await workingGroup_TerminatedLeader(db, event, WorkerType.STORAGE, workerId)
}

/// ///////////////// Gateway working group //////////////////////////////////////

export async function gatewayWorkingGroup_OpeningFilled(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { applicationIdToWorkerIdMap } = new GatewayWorkingGroup.OpeningFilledEvent(event).data

  // call generic processing
  await workingGroup_OpeningFilled(db, WorkerType.GATEWAY, applicationIdToWorkerIdMap, event)
}

export async function gatewayWorkingGroup_WorkerStorageUpdated(
  db: DatabaseManager,
  event: SubstrateEvent
): Promise<void> {
  // read event data
  const { workerId, bytes: newMetadata } = new GatewayWorkingGroup.WorkerStorageUpdatedEvent(event).data

  // call generic processing
  await workingGroup_WorkerStorageUpdated(db, WorkerType.GATEWAY, workerId, newMetadata)
}

export async function gatewayWorkingGroup_TerminatedWorker(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new GatewayWorkingGroup.TerminatedWorkerEvent(event).data

  // call generic processing
  await workingGroup_TerminatedWorker(db, event, WorkerType.GATEWAY, workerId)
}

export async function gatewayWorkingGroup_WorkerExited(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new GatewayWorkingGroup.WorkerExitedEvent(event).data

  // call generic processing
  await workingGroup_WorkerExited(db, event, WorkerType.GATEWAY, workerId)
}

export async function gatewayWorkingGroup_TerminatedLeader(db: DatabaseManager, event: SubstrateEvent): Promise<void> {
  // read event data
  const { workerId } = new GatewayWorkingGroup.TerminatedLeaderEvent(event).data

  // call generic processing
  await workingGroup_TerminatedLeader(db, event, WorkerType.GATEWAY, workerId)
}

/// ///////////////// Generic working group processing ///////////////////////////

export async function workingGroup_OpeningFilled(
  db: DatabaseManager,
  workerType: WorkerType,
  applicationIdToWorkerIdMap: ApplicationIdToWorkerIdMap,
  event: SubstrateEvent
): Promise<void> {
  const workerIds = [...applicationIdToWorkerIdMap.values()]

  for (const workerId of workerIds) {
    await createWorker(db, workerId, workerType, event)
  }

  // emit log event
  logger.info('Workers have been created', { ids: workerIds.map((item) => item.toString()), workerType })
}

export async function workingGroup_WorkerStorageUpdated(
  db: DatabaseManager,
  workerType: WorkerType,
  workerId: WorkerId,
  newMetadata: Bytes
): Promise<void> {
  // load worker
  const worker = await db.get(Worker, {
    where: {
      workerId: workerId.toString(),
      type: workerType,
    } as FindConditions<Worker>,
  })

  // ensure worker exists
  if (!worker) {
    return inconsistentState('Non-existing worker update requested', workerId)
  }

  worker.metadata = newMetadata.toUtf8()

  await db.save<Worker>(worker)

  // emit log event
  logger.info('Worker has been updated', { workerId, workerType })
}

export async function workingGroup_TerminatedWorker(
  db: DatabaseManager,
  event: SubstrateEvent,
  workerType: WorkerType,
  workerId: WorkerId
): Promise<void> {
  // do removal logic
  await deactivateWorker(db, event, workerType, workerId)

  // emit log event
  logger.info('Worker has been removed (worker terminated)', { workerId, workerType })
}

export async function workingGroup_WorkerExited(
  db: DatabaseManager,
  event: SubstrateEvent,
  workerType: WorkerType,
  workerId: WorkerId
): Promise<void> {
  // do removal logic
  await deactivateWorker(db, event, workerType, workerId)

  // emit log event
  logger.info('Worker has been removed (worker exited)', { workerId, workerType })
}

export async function workingGroup_TerminatedLeader(
  db: DatabaseManager,
  event: SubstrateEvent,
  workerType: WorkerType,
  workerId: WorkerId
): Promise<void> {
  // do removal logic
  await deactivateWorker(db, event, workerType, workerId)

  // emit log event
  logger.info('Working group leader has been removed (worker exited)', { workerId, workerType })
}

/// ///////////////// Helpers ////////////////////////////////////////////////////

async function createWorker(
  db: DatabaseManager,
  workerId: WorkerId,
  workerType: WorkerType,
  event: SubstrateEvent
): Promise<void> {
  // create entity
  const newWorker = new Worker({
    id: await createPredictableId(db),
    workerId: workerId.toString(),
    type: workerType,
    isActive: true,

    createdAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),
    updatedAt: new Date(fixBlockTimestamp(event.blockTimestamp).toNumber()),
  })

  // save worker
  await db.save<Worker>(newWorker)
}

async function deactivateWorker(
  db: DatabaseManager,
  event: SubstrateEvent,
  workerType: WorkerType,
  workerId: WorkerId
) {
  // load worker
  const worker = await db.get(Worker, {
    where: {
      workerId: workerId.toString(),
      type: workerType,
    } as FindConditions<Worker>,
  })

  // ensure worker exists
  if (!worker) {
    return inconsistentState('Non-existing worker deletion requested', workerId)
  }

  // update worker
  worker.isActive = false

  // set last update time
  worker.updatedAt = new Date(fixBlockTimestamp(event.blockTimestamp).toNumber())

  // save worker
  await db.save<Worker>(worker)
}
