import { DatabaseManager } from '@joystream/hydra-common'
import { BaseModel } from '@joystream/warthog'
import { WorkingGroup } from '@joystream/types/augment/all'

type EntityClass<T extends BaseModel> = {
  new (): T
  name: string
}

export async function getById<T extends BaseModel>(
  store: DatabaseManager,
  entityClass: EntityClass<T>,
  id: string,
  relations?: Exclude<
    keyof T & string,
    { [K in keyof T]: T[K] extends BaseModel | undefined ? '' : T[K] extends BaseModel[] | undefined ? '' : K }[keyof T]
  >[]
): Promise<T> {
  const result = await store.get(entityClass, { where: { id }, relations })
  if (!result) {
    throw new Error(`Expected ${entityClass.name} not found by ID: ${id}`)
  }

  return result
}

export type WorkingGroupModuleName = 'storageWorkingGroup' | 'contentDirectoryWorkingGroup'

export function getWorkingGroupModuleName(group: WorkingGroup): WorkingGroupModuleName {
  if (group.isContent) {
    return 'contentDirectoryWorkingGroup'
  } else if (group.isStorage) {
    return 'storageWorkingGroup'
  }

  throw new Error(`Unsupported working group encountered: ${group.type}`)
}
