import React, { useState, useRef, useEffect } from "react";
import { useCSS, TextFieldStyleProps } from "./TextField.style";
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { spacing } from "./../../theme";

type TextFieldProps = {
	label: string;
	helper?: string;
	value?: string;
	icon?: IconProp;
	onChange?: (e: React.ChangeEvent) => void;
} & TextFieldStyleProps;

export default function TextField({
	label,
	helper = "",
	value = "",
	icon = null,
	disabled = false,
	onChange,
	...styleProps
}: TextFieldProps) {
	const inputRef = useRef(null);
	const [isActive, setIsActive] = useState(!!value);
	const [inputTextValue, setInputTextValue] = useState(value);
	const styles = useCSS({ isActive, disabled, ...styleProps });

	useEffect(() => {
		if (isActive) {
			inputRef.current.focus();
		} else {
			inputRef.current.blur();
		}
	}, [isActive, inputRef]);

	function onTextFieldClick() {
		if (!disabled) setIsActive(true);
	}

	function onInputTextBlur() {
		setIsActive(false);
	}

	// FIXME: add correct typing to event, see here: https://github.com/DefinitelyTyped/DefinitelyTyped/pull/12239
	function onInputTextChange(event: any): void {
		if (!disabled) setInputTextValue(event.currentTarget.value);
	}

	return (
		<div css={styles.wrapper}>
			<div css={styles.container} onClick={onTextFieldClick}>
				<div css={styles.border}>
					<div
						css={styles.label}
						style={
							!inputTextValue && !isActive
								? {}
								: {
										position: "absolute",
										top: "-8px",
										left: "5px",
										fontSize: "0.7rem",
										padding: `0 ${spacing.xs}`,
								  }
						}
					>
						{label}
					</div>
					<input
						css={styles.input}
						style={{ display: !!inputTextValue || isActive ? "block" : "none" }}
						ref={inputRef}
						type="text"
						value={inputTextValue}
						onChange={disabled ? null : onInputTextChange}
						onBlur={onInputTextBlur}
					/>
					{!!icon && <FontAwesomeIcon icon={icon} css={styles.icon} />}
				</div>
			</div>
			{!!helper && <p css={styles.helper}>{helper}</p>}
		</div>
	);
}
