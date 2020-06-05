import React, { useState, useRef, useEffect } from "react";
import { DropdownStyleProps, useCSS } from "./Dropdown.style";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChevronDown, faChevronUp } from "@fortawesome/free-solid-svg-icons";
import ChevronIconUp from "../../../assets/chevron-down-big.svg";
import ChevronIconDown from "../../../assets/chevron-up-big.svg";
import { spacing } from "./../../theme";

type DropdownOption = {
	text: string;
	value: string;
};

type DropdownProps = {
	label: string;
	helper?: string;
	value?: string;
	options: Array<DropdownOption>;
	onChange?: (option: DropdownOption) => void;
} & DropdownStyleProps;

export default function Dropdown({
	label,
	helper = "",
	value = "",
	options,
	disabled = false,
	onChange = () => {},
	...styleProps
}: DropdownProps) {
	const inputRef = useRef(null);
	const [isActive, setIsActive] = useState(!!value);
	const [inputTextValue, setInputTextValue] = useState(
		!!value ? options.find(option => option.value === value).text : ""
	);
	const [showOptions, setShowOptions] = useState(false);
	const styles = useCSS({ isActive, disabled, ...styleProps });

	function onToggleDropdown(): void {
		if (!disabled) {
			setShowOptions(!showOptions);
		}
	}

	function onOptionSelected(option: DropdownOption): void {
		setIsActive(false);
		setInputTextValue(option.text);
		onChange(option);
	}

	return (
		<div css={styles.wrapper}>
			<div css={styles.container} onClick={onToggleDropdown}>
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
						disabled={true}
						value={inputTextValue}
					/>
					{!showOptions && <ChevronIconUp css={styles.iconOpen} />}
					{!!showOptions && <ChevronIconDown css={styles.iconClose} />}
				</div>
				{showOptions && (
					<div css={styles.options}>
						{options.map((option, index) => (
							<div
								key={`${label}-${index}`}
								css={styles.option}
								defaultValue={option.value}
								onClick={() => onOptionSelected(option)}
							>
								{option.text}
							</div>
						))}
					</div>
				)}
			</div>
			{!!helper && <p css={styles.helper}>{helper}</p>}
		</div>
	);
}
