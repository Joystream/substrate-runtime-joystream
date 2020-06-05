import React from "react"
import { mount } from "enzyme"
import Banner from "./../src/components/Banner"

describe("Banner component", () => {

  const component = mount(<Banner src="http://static.libsyn.com/p/assets/2/c/2/5/2c25acab892a768e/Twitter_Cover.png" />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})