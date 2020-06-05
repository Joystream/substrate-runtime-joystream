import React from "react"
import { RouteComponentProps, useParams, navigate } from "@reach/router"
import { GenericSection, VideoPreview, Grid } from "components"
import ChannelHeader from "./../../components/ChannelHeader"

type ChannelProps = {
  name: string
  isPublic?: boolean
  isVerified?: boolean
  description?: string
  banner?: string
  videos?: any[]
  img: string
}

function ChannelComponent({
  name,
  isPublic = true,
  isVerified = false,
  description,
  banner,
  videos,
  img,
}: ChannelProps) {
  return (
    <>
      <ChannelHeader
        name={name}
        isPublic={isPublic}
        isVerified={isVerified}
        description={description}
        banner={banner}
        img={img}
      />
      <GenericSection title="Videos">
        <Grid
          minItemWidth="250"
          maxItemWidth="600"
          items={videos.map((video, idx) => (
            <VideoPreview
              onClick={() => navigate(`/videos/${idx}`)}
              onChannelClick={() => navigate(`/channels/${video.channel}`)}
              key={`title-${idx}`}
              title={video.title}
              poster={video.poster}
            />
          ))}
        />
      </GenericSection>
    </>
  )
}

type RouteProps = {
  videos: any
  channels: any
} & RouteComponentProps

export default function Channel({ videos, channels }: RouteProps) {
  let params = useParams()
  let channelVideos = videos[params.channelName]
  let channel = channels[params.channelName]
  return <ChannelComponent {...channel} videos={channelVideos} />
}
