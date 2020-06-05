import React from "react"
import { Dropdown } from "./../src"

export default {
  title: "Dropdown",
  component: Dropdown,
}

const options = [
  {
    text: "Option 1",
    value: "1"
  },
  {
    text: "Option 2",
    value: "2"
  },
  {
    text: "Option 3",
    value: "3"
  }
]

const manyOptions = [
  {
    text: "Option 1",
    value: "1"
  },
  {
    text: "Option 2",
    value: "2"
  },
  {
    text: "Option 3",
    value: "3"
  },
  {
    text: "Option 4",
    value: "4"
  },
  {
    text: "Option 5",
    value: "5"
  },
  {
    text: "Option 6",
    value: "6"
  },
  {
    text: "Option 7",
    value: "7"
  },
  {
    text: "Option 8",
    value: "8"
  },
  {
    text: "Option 9",
    value: "9"
  },
  {
    text: "Option 10",
    value: "10"
  }
]

export const Default = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Dropdown label="Label" options={options} />
  </div>
)

export const DefaultWithValue = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Dropdown label="Label" options={options} value={options[1].value} />
  </div>
)

export const DefaultFocus = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Dropdown label="Label" options={options} focus={true} />
  </div>
)

export const DefaultError = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Dropdown label="Label" options={options} error={true} />
  </div>
)

export const DefaultWithManyOptions = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Dropdown label="Label" options={manyOptions} />
  </div>
)
