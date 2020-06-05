import React from "react"

import {
  VideoPlayer,
  GenericSection,
  ChannelSummary,
  DetailsTable,
} from "components"
import { useParams, RouteComponentProps, navigate } from "@reach/router"

type VideoViewProps = {
  video: any
  channel: any
}
function VideoViewComponent({ video, channel }: VideoViewProps) {
  return (
    <>
      <GenericSection>
        <VideoPlayer src={video.src} poster={video.poster} />
      </GenericSection>
      <GenericSection
        topDivider
        title={video.title}
        className="video-details"
      >
        <ChannelSummary
          isPublic={channel.isPublic}
          img={channel.img}
          name={channel.name}
          isVerified={channel.isVerified}
          description={video.description}
          size="default"
          onClick={() => navigate(`/channels/${channel.name}`)}
        />
      </GenericSection>
      <GenericSection
        topDivider
        title="Video details"
        className="video-details-table"
      >
        <DetailsTable details={video.details} />
      </GenericSection>
    </>
  )
}

type RouteProps = {
  videos: any
  channels: any
} & RouteComponentProps

export default function VideoView({ videos, channels }: RouteProps) {
  let params = useParams()
  let video: any = Object.values(videos).flat()[params.idx]
  let channel = channels[video.channel]
  return <VideoViewComponent video={video} channel={channel} />
}
