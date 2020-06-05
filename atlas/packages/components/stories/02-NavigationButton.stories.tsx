import React from "react"
import { NavButton } from "../src/"

export default {
  title: "NavButton",
  component: NavButton,
}

export const PrimaryRight = () => (
  <NavButton />
)

export const PrimaryLeft = () => (
  <NavButton direction="left" />
)

export const SecondaryRight = () => (
  <NavButton type="secondary" />
)

export const SecondaryLeft = () => (
  <NavButton type="secondary" direction="left" />
)

export const AppNavigation = () => (
  <div>
    <NavButton type="secondary" direction="left" />
    <NavButton type="secondary" direction="right" />
  </div>
)
