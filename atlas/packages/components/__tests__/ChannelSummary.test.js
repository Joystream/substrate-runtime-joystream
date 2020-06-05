import React from "react"
import { mount } from "enzyme"
import ChannelSummary from "./../src/components/ChannelSummary"

describe("ChannelSummary component", () => {

  const component = mount(
    <ChannelSummary
      img={"https://s3.amazonaws.com/keybase_processed_uploads/9003a57620356bd89d62bd34c7c0c305_360_360.jpg"}
      size="default"
      name={"Test channel"}
      isPublic={true}
      isVerified={true}
    />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})