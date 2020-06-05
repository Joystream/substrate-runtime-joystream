import React from "react"
import { shallow } from "enzyme"
import App from "./../src/App"

describe("App component", () => {

  const component = shallow(<App />)

  it("Should render.", () => {
    expect(component).toBeDefined()
  })

})