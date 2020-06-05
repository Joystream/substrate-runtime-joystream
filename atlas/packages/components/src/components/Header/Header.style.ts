import { css } from "@emotion/core"
import { typography, colors } from "../../theme"

export type HeaderStyleProps = {
  background?: string
}

export let makeStyles = ({
  background = ""
}: HeaderStyleProps) => {
  return css`
    background-color: ${colors.black};
    text-align: left;
    cursor: default;
    color: ${colors.white};
    font-family: ${typography.fonts.base};
    height: 600px;
    display: flex;
    align-content: center;
    align-items: center;
    background-image: url(${background});
    background-repeat: no-repeat;
    background-position: center right;
    background-size: contain;

    div#content {
      margin: 0 100px;
    }   
  `
}
