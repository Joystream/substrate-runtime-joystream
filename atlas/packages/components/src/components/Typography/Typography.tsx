import React from "react"
import { makeStyles, TypographyStyleProps } from "./Typography.style"

type TypographyProps = {
  children: React.ReactNode
  onClick?: (e: React.MouseEvent<any>) => void
} & TypographyStyleProps

export default function Typography({
  children,
  onClick,
  ...styleProps
}: TypographyProps) {
  let styles = makeStyles(styleProps)
  return (
    <div css={styles} onClick={onClick}>
      {children}
    </div>
  )
}
