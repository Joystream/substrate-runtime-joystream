import React from "react"
import { Header, Button } from "./../src"

export default {
  title: "Header",
  component: Header
}

export const Default = () => (
  <Header
    text="A user governed video platform"
    subtext="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore."
    background="https://raw.githubusercontent.com/Joystream/design/master/illustrations/SVG/mask.svg"
  >
    <Button text="Play" />
    <Button text="Share" type="secondary" />
  </Header>
)
