import React from "react";
import { GridStyleProps, useCSS } from "./Grid.style";

type SectionProps = {
	items?: React.ReactNode[];
	className?: string;
} & GridStyleProps;

export default function Grid({ items = [], className = "", ...styleProps }: SectionProps) {
	let styles = useCSS(styleProps);
	return (
		<div css={styles.container} className={className}>
			{items.map((item, index) => (
				<div key={`grid-item-${index}`} css={styles.item}>
					{item}
				</div>
			))}
		</div>
	);
}
