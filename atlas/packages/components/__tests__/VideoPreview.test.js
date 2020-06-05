import React from "react"
import { mount } from "enzyme"
import VideoPreview from "./../src/components/VideoPreview"

describe("VideoPreview component", () => {

  const component = mount(
    <VideoPreview
      title="Test"
      poster="https://ssl-static.libsyn.com/p/assets/a/4/8/f/a48f1a0697e958ce/Cover_2.png"
    />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})