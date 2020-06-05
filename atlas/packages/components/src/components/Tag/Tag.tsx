import React from "react";
import { TagStyleProps, useCSS } from "./Tag.style";

type TagProps = {
	text: string;
} & TagStyleProps;

export default function Tag({ text, ...styleProps }: TagProps) {
	let styles = useCSS(styleProps);
	return <div css={styles}>{text}</div>;
}
