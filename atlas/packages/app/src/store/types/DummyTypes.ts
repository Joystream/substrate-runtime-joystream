// This is just a placeholder. It should be used as a guideline and then deleted.

export const ADD_DUMMY_ACTION = "ADD_DUMMY_ACTION"
export const REMOVE_DUMMY_ACTION = "REMOVE_DUMMY_ACTION"

export interface Dummy {
  id: string
  name: string
}

interface AddDummyAction {
  type: typeof ADD_DUMMY_ACTION
  dummy: Dummy
}

interface RemoveDummyAction {
  type: typeof REMOVE_DUMMY_ACTION
  id: string
}

export type DummyActionTypes = AddDummyAction | RemoveDummyAction
