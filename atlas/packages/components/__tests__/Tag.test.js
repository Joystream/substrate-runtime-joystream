import React from "react"
import { mount } from "enzyme"
import Tag from "./../src/components/Tag"
import { faCheck } from "@fortawesome/free-solid-svg-icons"
import { colors } from "./../src/theme"

describe("Tag component", () => {

  const component = mount(<Tag icon={faCheck} text="test" color={colors.other.success} />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

  it("Should render icon.", () => {
    expect(component.find("svg"))
      .toHaveLength(1)
  })

  it("Should render text.", () => {
    expect(component.contains(<span>test</span>))
      .toBe(true)
  })

})