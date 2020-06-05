import { createStore } from "redux"
import rootReducer from "./reducers"

const store = createStore(
  rootReducer,
  (<any>window).__REDUX_DEVTOOLS_EXTENSION__ && (<any>window).__REDUX_DEVTOOLS_EXTENSION__()
)

export default store
