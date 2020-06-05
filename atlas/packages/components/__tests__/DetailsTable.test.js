import React from "react"
import { mount } from "enzyme"
import DetailsTable from "./../src/components/DetailsTable"

describe("DetailsTable component", () => {

  const component = mount(<DetailsTable details={{
    explicit: "yes",
    "first released": "2019-02-27",
    language: "English",
    category: "Science & Technology",
    license: "Original content",
    attribution: ""
  }} />)

  it("Should render correctly", () => {
    expect(component).toMatchSnapshot()
  })

})