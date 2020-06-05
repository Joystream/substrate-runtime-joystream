import React, { useState } from "react";
import { useCSS, CheckboxStyleProps } from "./Checkbox.style";
import CheckIcon from "../../../assets/check.svg";
import DashIcon from "../../../assets/dash.svg";

type CheckboxProps = {
	label?: string;
	icon?: "check" | "dash";
	onChange?: (e: React.ChangeEvent) => void;
} & CheckboxStyleProps;

export default function Checkbox({
	label = "",
	disabled = false,
	error = false,
	selected = false,
	icon = "check",
	labelPosition = "end",
	onChange = () => {},
	...styleProps
}: CheckboxProps) {
	const styles = useCSS({ ...styleProps, selected, error, disabled });
	return (
		<div css={styles.checkbox}>
			{(labelPosition === "start" || labelPosition === "top") && <label css={styles.label}>{label}</label>}
			<div css={styles.outerContainer}>
				<div css={styles.innerContainer}>
					<input
						css={styles.input}
						type="checkbox"
						checked={selected}
						disabled={disabled}
						onChange={onChange}
					/>
					{selected && (icon === "check" ? <CheckIcon css={styles.icon} /> : <DashIcon css={styles.icon} />)}
				</div>
			</div>
			{(labelPosition === "end" || labelPosition === "bottom") && <label css={styles.label}>{label}</label>}
		</div>
	);
}
