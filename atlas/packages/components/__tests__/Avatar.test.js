import React from "react"
import { mount } from "enzyme"
import Avatar from "./../src/components/Avatar"

describe("Avatar component", () => {

  const component = mount(<Avatar img="https://s3.amazonaws.com/keybase_processed_uploads/9003a57620356bd89d62bd34c7c0c305_360_360.jpg" />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})