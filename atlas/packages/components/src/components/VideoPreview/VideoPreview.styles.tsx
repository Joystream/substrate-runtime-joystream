import { css } from "@emotion/core"
import { typography, colors } from "./../../theme"

export type VideoPreviewStyleProps = {
  showChannel?: boolean
}

export let makeStyles = ({ showChannel = false }: VideoPreviewStyleProps) => {
  return {
    container: css``,
    link: css`
      text-decoration: none;
    `,
    coverContainer: css`
      width: 100%;
      background-color: black;
    `,
    cover: css`
      display: block;
      width: 100%;
      height: auto;
    `,
    infoContainer: css`
      display: grid;
      grid-template: auto / ${showChannel ? "45px auto" : "auto"};
      margin: 10px 0 0;
    `,
    avatar: css`
      grid-column: 1 / 1;
    `,
    textContainer: css`
      grid-column: ${showChannel ? "2 / 2" : "1 / 1"};
    `,
    title: css`
      margin: 0;
      font-weight: ${typography.weights.bold};
      text-transform: capitalize;
      color: ${colors.black.base};
      font-size: ${typography.sizes.small};
    `,
    channel: css`
      margin: 5px 0 0;
      font-size: ${typography.sizes.xsmall};
      color: ${colors.grey.darker};
    `
  }
}
