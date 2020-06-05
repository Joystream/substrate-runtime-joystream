import { StyleFn, makeStyles } from "../../utils"
import { typography, colors, spacing } from "../../theme"
import { CSSProperties } from "react"

export type RadioButtonStyleProps = {
	selected?: boolean
	disabled?: boolean
	error?: boolean
	position?: "end" | "start" | "top" | "bottom"
}
const container: StyleFn = (_, { position }) => ({
	fontFamily: typography.fonts.base,
	display: position === "bottom" || position === "bottom" ? "inline-block" : "inline-flex",
	alignItems: "center",
	"&:focus": {
		outline: "none"
	}
})

const outerDot: StyleFn = (_, { position, disabled }) => ({
	width: spacing.xxl,
	height: spacing.xxl,
	borderRadius: "50%",
	position: "relative",
	margin: position === "bottom" ? `0 auto ${spacing.xs}` : position === "top" ? `${spacing.xs} auto 0` : "",
	"&:hover": {
		backgroundColor: disabled ? "none" : colors.gray[50]
	},
	"&:active": {
		backgroundColor: disabled ? "none" : colors.gray[100]
	},
	"&:focus": {
		backgroundColor: disabled ? "none" : colors.blue[100],
		outline: "none"
	}
})

const dot: StyleFn = (_, { disabled, selected, error }) => {
	return {
		width: spacing.m,
		height: spacing.m,
		borderWidth: 1,
		borderColor: disabled
			? colors.gray[200]
			: error
			? colors.error
			: selected
			? colors.blue[500]
			: colors.gray[300],
		borderStyle: "solid",
		borderRadius: "50%",
		position: "absolute",
		top: "7px",
		left: "7px",
		"&:focus": {
			borderColor: disabled ? colors.gray[200] : colors.gray[700]
		},
		"&:active": {
			borderColor: disabled ? colors.gray[200] : colors.gray[700]
		}
	}
}

const BackgroundFromProps: StyleFn = (styles, { disabled, selected, error }) => {
	let key = selected ? `backgroundImage` : `backgroundColor`
	const SELECTED_ERROR = `repeating-radial-gradient(circle, ${colors.error} 0px, ${colors.error} 3px, transparent 3px, transparent 6px, ${colors.error} 6px, ${colors.error} 8px)`
	const SELECTED_DISABLED = `repeating-radial-gradient(circle, ${colors.gray[200]} 0px, ${colors.gray[200]} 3px, transparent 3px, transparent 6px, ${colors.gray[200]} 6px, ${colors.gray[200]} 8px)`
	const SELECTED_DEFAULT = `repeating-radial-gradient(circle, ${colors.blue[500]} 0px, ${colors.blue[500]} 3px, transparent 3px, transparent 6px, ${colors.blue[500]} 6px, ${colors.blue[500]} 8px)`
	const UNSELECTED_DISABLED = colors.gray[50]

	let value =
		selected && error
			? SELECTED_ERROR
			: selected && disabled
			? SELECTED_DISABLED
			: selected
			? SELECTED_DEFAULT
			: disabled
			? UNSELECTED_DISABLED
			: styles[key as keyof CSSProperties]
	return {
		...styles,
		[key]: value
	}
}

const label: StyleFn = (_, { position }) => {
	const key = position === "end" ? "margin-left" : position === "start" ? "margin-right" : "margin"
	const value =
		key === "margin-left" || key === "margin-right"
			? spacing.xs
			: position === "bottom"
			? `0 auto ${spacing.xs}`
			: `${spacing.xs} auto 0`

	return {
		color: colors.white,
		[key]: value
	}
}

export const useCSS = ({
	selected = false,
	disabled = false,
	error = false,
	position = "end"
}: RadioButtonStyleProps) => {
	const props = { selected, disabled, error, position }
	return {
		container: makeStyles([container])(props),
		outterDot: makeStyles([outerDot])(props),
		dot: makeStyles([dot, BackgroundFromProps])(props),
		label: makeStyles([label])(props)
	}
}
