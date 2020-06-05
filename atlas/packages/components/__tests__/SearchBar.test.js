import React from "react"
import { mount } from "enzyme"
import SearchBar from "./../src/components/SearchBar"

describe("SearchBar component", () => {

  const component = mount(<SearchBar placeholder="Type here..." value="test" />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})