import { typography, colors } from "../../theme"
import { StyleFn, makeStyles } from "../../utils"

export type NavButtonStyleProps = {
	type?: "primary" | "secondary"
}

const baseStyles: StyleFn = () => ({
	border: 0,
	color: colors.white,
	textAlign: "center",
	display: "inline-block",
	cursor: "default",
	fontFamily: typography.fonts.base,
	fontWeight: typography.weights.medium,
	fontSize: typography.sizes.subtitle1,
	margin: "1px",
	padding: 0,
	width: "50px",
	height: "50px",
	lineHeight: "50px",
	"&:hover": {
		borderColor: colors.blue[700]
	},
	"&:active": {
		borderColor: colors.blue[900]
	},
	"&::selection": {
		background: "transparent"
	}
})

const colorFromType: StyleFn = (styles, { type = "primary" }) => ({
	...styles,
	backgroundColor: type === "primary" ? colors.blue[700] : colors.black,
	"&:hover": {
		backgroundColor: type === "primary" ? colors.blue[700] : colors.black,
		color: type === "primary" ? colors.white : colors.blue[300]
	},
	"&:active": {
		backgroundColor: type === "primary" ? colors.blue[900] : colors.black,
		color: type === "primary" ? colors.white : colors.blue[700]
	}
})

export const useCSS = (props: NavButtonStyleProps) => makeStyles([baseStyles, colorFromType])(props)
