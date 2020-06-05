import React from "react"
import { TextField } from "./../src"
import { faBan } from "@fortawesome/free-solid-svg-icons"

export default {
  title: "TextField",
  component: TextField,
}

export const Primary = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" />
  </div>
)

export const Focus = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" focus={true} />
  </div>
)

export const Error = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" error={true} />
  </div>
)

export const Disabled = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" disabled={true} />
  </div>
)

export const DisabledWithValue = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" value="Disabled" disabled={true} />
  </div>
)

export const PrimaryWithIconRight = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" icon={faBan} iconPosition="right" />
  </div>
)

export const PrimaryWithIconLeft = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" icon={faBan} iconPosition="left" />
  </div>
)

export const PrimaryWithHelperText = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" helper="Helper text" />
  </div>
)

export const FocusWithHelperText = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" focus={true} helper="Helper text" />
  </div>
)

export const ErrorWithHelperText = () => (
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <TextField label="Label" error={true} helper="Helper text" />
  </div>
)
