import React, { useState } from "react";
import { useCSS, RadioButtonStyleProps } from "./RadioButton.style";

type RadioButtonProps = {
	label?: string;
	onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
} & RadioButtonStyleProps;

export default function RadioButton({
	label = "",
	position = "end",
	disabled = false,
	onClick,
	...styleProps
}: RadioButtonProps) {
	const styles = useCSS({ disabled, position, ...styleProps });

	return (
		<div css={styles.container} onClick={disabled ? null : onClick}>
			{(position === "start" || position === "top") && <label css={styles.label}>{label}</label>}
			<div css={styles.outterDot}>
				<div css={styles.dot}></div>
			</div>
			{(position === "end" || position === "bottom") && <label css={styles.label}>{label}</label>}
		</div>
	);
}
