import React from "react"
import { RouteComponentProps, navigate } from "@reach/router"
import { GenericSection, VideoPreview, ChannelSummary, Grid } from "components"

type ExploreViewProps = {
  videos: any
  channels: any
} & RouteComponentProps

export default function ExploreView({
  channels,
  videos
}: ExploreViewProps) {

  let allVideos: any[] = Object.values(videos).flat()
  let allChannels: any[] = Object.values(channels)

  return (
    <>
      <GenericSection topDivider title="Latest Videos" linkText="All Videos" onLinkClick={() => {}}>
        <Grid
          minItemWidth="250"
          items={allVideos.map((video, idx) => {
            let { img: channelImg } = channels[video.channel] || ""
            return (
              <VideoPreview
                key={`${video.title}-${idx}`}
                channelImg={channelImg}
                channel={video.channel}
                title={video.title}
                poster={video.poster}
                showChannel
                onClick={() => navigate(`videos/${idx}`)}
                onChannelClick={() => navigate(`channels/${video.channel}`)}
              />
            )
          })}
        />
      </GenericSection>
      <GenericSection topDivider title="Latest video channels" linkText="All Channels" onLinkClick={() => {}}>
        <div className="channel-gallery">
          {allChannels.map((channel, idx) => (
            <ChannelSummary
              key={`${channel.name}-${idx}`}
              img={channel.img}
              size="default"
              name={channel.name}
              isPublic={channel.isPublic}
              isVerified={channel.isVerified}
              onClick={() => navigate(`channels/${channel.name}`)}
            />
          ))}
        </div>
      </GenericSection>
    </>
  )
}
