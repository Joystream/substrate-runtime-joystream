import { css } from "@emotion/core";
import { breakpoints } from "../../theme";

export type VideoPlayerStyleProps = {
  width?: string | number;
  height?: string | number;
  responsive?: boolean;
  ratio?: string;
};

export let makeStyles = ({
  width = "100%",
  height = "100%",
  responsive = true,
  ratio = "16:9",
}: VideoPlayerStyleProps) => {
  let ratioPerc = ratio
    .split(":")
    .map(x => Number(x))
    .reduce((x, y) => (y / x) * 100);

  return {
    containerStyles: css`
      max-width: ${breakpoints.medium};
      & .video-player {
      }
    `,
    playerStyles: css`
      width: ${width};
      height: ${height};
    `,
  };
};
