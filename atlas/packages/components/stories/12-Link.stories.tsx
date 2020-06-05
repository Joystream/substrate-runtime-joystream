import React from "react"
import { Link } from "./../src"

export default {
  title: "Link",
  component: Link
}

export const Default = () =>
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Link to="path">Go to path</Link>
  </div>

export const Disabled = () =>
  <div style={{ backgroundColor: "black", padding: "50px 20px" }}>
    <Link to="www.google.com" disabled={true}>Go to Google</Link>
  </div>
