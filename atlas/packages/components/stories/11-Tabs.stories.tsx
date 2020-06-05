import React from "react"
import { Tabs } from "./../src"
import { Tab } from "./../src"

export default {
  title: "Tabs",
  component: Tabs,
}

export const Default = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Tabs onChange={tab => console.log(`Active tab: ${tab}`)}>
      <Tab label="Tab 1">tab 1</Tab>
      <Tab label="Tab 2">tab 2</Tab>
      <Tab label="Tab 3">tab 3</Tab>
    </Tabs>
  </div>
)
