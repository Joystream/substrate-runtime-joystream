import React from "react";
import { makeStyles, AvatarStyleProps } from "./Avatar.style";

export type AvatarProps = {} & AvatarStyleProps;

export default function Avatar({ ...styleProps }: AvatarProps) {
  let styles = makeStyles(styleProps);
  return (
    <div css={styles} />
  );
}
