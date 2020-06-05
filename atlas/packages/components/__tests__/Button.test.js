import React from "react"
import { mount } from "enzyme"
import Button from "./../src/components/Button"

describe("Button component", () => {

  it("Should render default button correctly", () => {
    expect(mount(<Button>Click me!</Button>)).toMatchSnapshot()
  })

  it("Should render custom button correctly", () => {
    expect(mount(<Button size="large" color="success">Hello Atlas</Button>)).toMatchSnapshot()
  })

})