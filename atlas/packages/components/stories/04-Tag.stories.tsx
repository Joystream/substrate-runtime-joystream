import React from "react"
import { Tag } from "./../src"

export default {
  title: "Tag",
  component: Tag
}

export const Default = () => <Tag text="Finance" />

export const Multiple = () => (
  <div>
    <Tag text="Finance" />
    <Tag text="Sports" />
    <Tag text="Comedy" />
  </div>
)
