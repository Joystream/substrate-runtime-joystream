import React from "react"
import { TagButton } from "../src/"

export default {
  title: "TagButton",
  component: TagButton,
}

export const Default = () => (
  <TagButton text="Finance" />
)

export const Selected = () => (
  <TagButton text="Finance" selected={true} />
)
