import { typography, colors } from "../../theme"
import { makeStyles, StyleFn } from "../../utils"
import { disabled, dimensionsFromProps, log } from "../../theme/fragments"

export type ButtonStyleProps = {
	text?: string
	type?: "primary" | "secondary"
	full?: boolean
	size?: "regular" | "small" | "smaller"
	children?: React.ReactNode
	disabled?: boolean
}

const baseStyles: StyleFn = () => ({
	borderWidth: "1px",
	borderStyle: "solid",
	fontFamily: typography.fonts.base,
	fontWeight: typography.weights.medium,
	display: "inline-flex",
	justifyContent: "center",
	alignItems: "center",
	color: colors.white,
	"&::selected": {
		background: "transparent"
	}
})
const colorFromType: StyleFn = (styles, { type }: ButtonStyleProps) => {
	switch (type) {
		case "primary":
			return {
				...styles,
				backgroundColor: colors.blue[500],
				borderColor: colors.blue[500],

				"&:hover": {
					backgroundColor: colors.blue[700],
					borderColor: colors.blue[700],
					color: colors.white
				},
				"&:active": {
					backgroundColor: colors.blue[900],
					borderColor: colors.blue[900],
					color: colors.white
				}
			}

		case "secondary":
			return {
				...styles,
				color: colors.blue[500],
				backgroundColor: colors.black,
				borderColor: colors.blue[500],

				"&:hover": {
					borderColor: colors.blue[700],
					color: colors.blue[300]
				},

				"&:active": {
					borderColor: colors.blue[700],
					color: colors.blue[700]
				}
			}
	}
}
const paddingFromType: StyleFn = (
	styles,
	{
		size = "regular",
		children,
		full
	}: { size: "regular" | "small" | "smaller"; children?: React.ReactNode; full: boolean }
) => {
	const buttonHeight = size === "regular" ? "20px" : size === "small" ? "15px" : "10px"
	return {
		...styles,
		margin: `0 ${full ? "0" : "15px"} 0 0`,
		padding:
			size === "regular"
				? !!children
					? "14px 17px"
					: "14px"
				: size === "small"
				? !!children
					? "12px 14px"
					: "12px"
				: "10px",
		fontSize:
			size === "regular"
				? typography.sizes.button.large
				: size === "small"
				? typography.sizes.button.medium
				: typography.sizes.button.small,

		height: buttonHeight,
		maxHeight: buttonHeight
	}
}

const iconStyles: StyleFn = (styles, { children, size }) => {
	return {
		...styles,
		marginRight: children != null ? "10px" : "0",
		fontSize:
			size === "regular"
				? typography.sizes.icon.large
				: size === "small"
				? typography.sizes.icon.medium
				: typography.sizes.icon.small,

		flexShrink: 0,
		"& > *": {
			stroke: "currentColor"
		}
	}
}

export const useCSS = (props: ButtonStyleProps) => ({
	container: makeStyles([baseStyles, colorFromType, dimensionsFromProps, paddingFromType, disabled])(props),
	icon: makeStyles([iconStyles])(props)
})

// 	text,
// 	type = "primary",
// 	width = "normal",
// 	size = "regular",
// 	disabled = false,
// }: ButtonStyleProps) => {
//

// 	const primaryButton = {
// 		container: css`
// 			border: 1px solid ${colors.blue[500]};
// 			color: ${colors.white};
// 			background-color: ${colors.blue[500]};

// 			padding: ${size === "regular"
// 				? !!text
// 					? "14px 17px"
// 					: "14px"
// 				: size === "small"
// 				? !!text
// 					? "12px 14px"
// 					: "12px"
// 				: "10px"};
// 			font-size: ${size === "regular"
// 				? typography.sizes.button.large
// 				: size === "small"
// 				? typography.sizes.button.medium
// 				: typography.sizes.button.small};
//
//
// 		`,
// 	};

// 	const secondaryButton = {
// 		container: css`
// 			border: 1px solid ${colors.blue[500]};

// 			background-color: ${colors.black};
// 			justify-content: center;
// 			padding: ${size === "regular"
// 				? !!text
// 					? "14px 17px"
// 					: "14px"
// 				: size === "small"
// 				? !!text
// 					? "12px 14px"
// 					: "12px"
// 				: "10px"};
// 			font-size: ${size === "regular"
// 				? typography.sizes.button.large
// 				: size === "small"
// 				? typography.sizes.button.medium
// 				: typography.sizes.button.small};
// 			margin: 0 ${width === "normal" ? "15px" : "0"} 0 0;
// 			height: ${buttonHeight};
// 			max-height: ${buttonHeight};

// 			&:hover {
// 				background-color: ${colors.black};
// 				border-color: ${colors.blue[700]};
// 				color: ${colors.blue[300]};
// 			}

// 			&:active {
// 				background-color: ${colors.black};
// 				border-color: ${colors.blue[700]};
// 				color: ${colors.blue[700]};
// 			}
// 		`,
// 	};

// 	const disabledButton = {
// 		container: css`
// 			border: 1px solid ${colors.white};
// 			color: ${colors.white};
// 			background-color: ${colors.gray[100]};
// 			justify-content: center;
// 			padding: ${size === "regular"
// 				? !!text
// 					? "14px 17px"
// 					: "14px"
// 				: size === "small"
// 				? !!text
// 					? "12px 14px"
// 					: "12px"
// 				: "10px"};
// 			display: ${width === "normal" ? "inline-flex" : "flex"};
// 			align-items: center;
// 			cursor: ${disabled ? "not-allowed" : "default"};
// 			font-family: ${typography.fonts.base};
// 			font-weight: ${typography.weights.medium};
// 			font-size: ${size === "regular"
// 				? typography.sizes.button.large
// 				: size === "small"
// 				? typography.sizes.button.medium
// 				: typography.sizes.button.small};
// 			margin: 0 ${width === "normal" ? "15px" : "0"} 0 0;
// 			height: ${buttonHeight};
// 			max-height: ${buttonHeight};

// 			&:hover {
// 				background-color: ${colors.gray[100]};
// 				border-color: ${colors.white};
// 				color: ${colors.white};
// 			}

// 			&:active {
// 				background-color: ${colors.gray[100]};
// 				border-color: ${colors.white};
// 				color: ${colors.white};
// 			}

// 			&::selection {
// 				background: transparent;
// 			}
// 		`,
// 	};

// 	const icon = css`
// 		margin-right: ${!!text ? "10px" : "0"};
// 		font-size: ${size === "regular"
// 			? typography.sizes.icon.large
// 			: size === "small"
// 			? typography.sizes.icon.medium
// 			: typography.sizes.icon.small};

// 		& > path:nth-of-type(1) {
// 			color: inherit;
// 			flex-shrink: 0;
// 		}
// 	`;

// 	const result = disabled ? disabledButton : type === "primary" ? primaryButton : secondaryButton;
// 	return { icon, ...result };
// };
