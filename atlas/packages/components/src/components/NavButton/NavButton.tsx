import React from "react";
import { useCSS, NavButtonStyleProps } from "./NavButton.style";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ChevronRightIcon from "../../../assets/chevron-right-big.svg";
import ChevronLeftIcon from "../../../assets/chevron-left-big.svg";
import { faChevronLeft, faChevronRight } from "@fortawesome/free-solid-svg-icons";

type NavButtonProps = {
	direction?: "right" | "left";
	onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
} & NavButtonStyleProps;

export default function NavButton({ direction = "right", onClick, ...styleProps }: NavButtonProps) {
	let styles = useCSS(styleProps);
	return (
		<div css={styles} onClick={onClick}>
			{direction === "right" ? <ChevronRightIcon /> : <ChevronLeftIcon />}
		</div>
	);
}
