// This is just a placeholder. It should be used as a guideline and then deleted.

import { ADD_DUMMY_ACTION, REMOVE_DUMMY_ACTION, Dummy, DummyActionTypes } from "./../types/DummyTypes"

export const AddDummy = (dummy: Dummy): DummyActionTypes => ({
  type: ADD_DUMMY_ACTION,
  dummy
})

export const RemoveDummy = (id: string): DummyActionTypes => ({
  type: REMOVE_DUMMY_ACTION,
  id
})
