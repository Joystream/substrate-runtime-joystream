import React from "react"
import { Router } from "@reach/router"
import { Provider } from "react-redux"

import store from "./store"
import data from "../staticData"

import ChannelView from "./views/ChannelView"
import ExploreView from "./views/ExploreView"
import VideoView from "./views/VideoView"
import ScrollToTop from "./components/ScrollToTop"

let { channels, videos } = data

export default function App() {
  return (
    <main className="main-section">
      <Provider store={store}>
        <Router primary={false}>
          <ScrollToTop path="/">
            <ExploreView path="/" channels={channels} videos={videos} />
            <ChannelView
              path="/channels/:channelName"
              channels={channels}
              videos={videos}
            />
            <VideoView path="/videos/:idx" channels={channels} videos={videos} />
          </ScrollToTop>
        </Router>
      </Provider>
    </main>
  )
}
