// This is just a placeholder. It should be used as a guideline and then deleted.

import { Dummy, DummyActionTypes, ADD_DUMMY_ACTION, REMOVE_DUMMY_ACTION } from "./../types/DummyTypes"

const initialState: Dummy[] = []

const DummyReducer = (state = initialState, action: DummyActionTypes): Dummy[] => {
  switch (action.type) {
    case ADD_DUMMY_ACTION:
      return [
        ...state,
        action.dummy
      ]
    case REMOVE_DUMMY_ACTION:
      return state.filter(dummy => dummy.id !== action.id)
    default:
      return state
  }
}

export default DummyReducer
