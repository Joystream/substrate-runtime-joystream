import React, { useState } from "react"
import { RadioButton } from "./../src"

export default {
  title: "RadioButton",
  component: RadioButton,
}

export const Primary = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton selected={isSelected} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const SelectedDisabled = () => {
  const [isSelected, setIsSelected] = useState(true)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton selected={isSelected} disabled={true} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const UnselectedDisabled = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton selected={isSelected} disabled={true} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const Error = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton selected={isSelected} error={true} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const WithLabel = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton label="Label" selected={isSelected} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const WithLabelStart = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton label="Label" position="start" selected={isSelected} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const WithLabelBottom = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton label="A longer label than normal" position="bottom" selected={isSelected} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}

export const WithLabelTop = () => {
  const [isSelected, setIsSelected] = useState(false)
  return (
    <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
      <RadioButton label="A longer label than normal" position="top" selected={isSelected} onClick={() => setIsSelected(!isSelected)} />
    </div>
  )
}
