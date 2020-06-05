import React from "react"
import { mount } from "enzyme"
import VideoPlayer from "./../src/components/VideoPlayer"

describe("VideoPlayer component", () => {

  const component = mount(
    <VideoPlayer
      src={"https://www.computt.com/asset/v0/5EjgEKNpyDbNdjcJoZ8izWuzeDUtAcaUvwG8vUWZQZ256NLb"}
      poster={"https://ssl-static.libsyn.com/p/assets/a/4/8/f/a48f1a0697e958ce/Cover_2.png"}
    />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})