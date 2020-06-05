import React from "react"
import { mount } from "enzyme"
import Grid from "./../src/components/Grid"

describe("Grid component", () => {

  const Item = ({ text }) => <div>{text}</div>

  const component = mount(<Grid
      minItemWidth="250"
      maxItemWidth="600"
      items={[...Array(10).keys()].map((i, index) => <Item key={index} text={`test-${i}`} />)}
    />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})