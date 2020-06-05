import React, { Children } from "react"
import { makeStyles, HeaderStyleProps } from "./Header.style"

type HeaderProps = {
  text: string,
  subtext?: string,
  children?: React.ReactNode
} & HeaderStyleProps

export default function Header({
  text,
  subtext = "",
  children,
  ...styleProps
}: HeaderProps) {
  let styles = makeStyles(styleProps)
  return (
    <div css={styles}>
      <div id="content">
        <h1>
          {text}
        </h1>
        {!!subtext && 
          <p>
            {subtext}
          </p>
        }
        {children}
      </div>
    </div>
  )
}
