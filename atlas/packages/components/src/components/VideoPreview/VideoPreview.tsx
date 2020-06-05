import React from "react"

import { makeStyles, VideoPreviewStyleProps } from "./VideoPreview.styles"
import Avatar from "./../Avatar"

type VideoPreviewProps = {
  title: string
  channel?: string
  channelImg?: string
  showChannel?: boolean
  poster?: string
  onClick?: any
  onChannelClick?: any
} & VideoPreviewStyleProps

export default function VideoPreview({
  title,
  channel,
  channelImg,
  showChannel = false,
  poster,
  onClick,
  onChannelClick,
  ...styleProps
}: VideoPreviewProps) {
  let styles = makeStyles({ showChannel, ...styleProps })
  return (
    <div css={styles.container} onClick={onClick}>
      <div css={styles.coverContainer}>
        <img css={styles.cover} src={poster} onClick={event => { event.stopPropagation(); onClick() }} />
      </div>
      <div css={styles.infoContainer}>
        {showChannel && (
          <div css={styles.avatar} onClick={event => { event.stopPropagation(); onChannelClick() }}>
            <Avatar size="small" img={channelImg} />
          </div>
        )}
        <div css={styles.textContainer}>
          <h3 css={styles.title} onClick={event => { event.stopPropagation(); onClick() }}>{title}</h3>
          {showChannel && (
            <h3 css={styles.channel} onClick={event => { event.stopPropagation(); onChannelClick() }}>{channel}</h3>
          )}
        </div>
      </div>
    </div>
  )
}
