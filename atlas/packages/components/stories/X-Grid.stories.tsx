import React from "react"
import { Grid } from "./../src"

export default {
  title: "Grid",
  component: Grid,
}

function Item() {
  return (
    <div>
      <img src="https://27pc93zx53q14ywwgt4yq513-wpengine.netdna-ssl.com/wp-content/uploads/2016/08/video-placeholder-brain-bites.png" style={{ width: '100%' }} />
      <p>Item title</p>
    </div>
  )
}

export const Default = () => <Grid items={([...Array(12).keys()].map(() => <Item />))} />

export const WithMinItemWidth300 = () => <Grid minItemWidth="300" items={([...Array(12).keys()].map(() => <Item />))} />

export const WithClassName = () => <Grid className="customGrid" items={([...Array(12).keys()].map(() => <Item />))} />
