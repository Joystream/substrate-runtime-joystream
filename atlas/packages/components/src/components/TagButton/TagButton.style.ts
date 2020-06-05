import { StyleFn, makeStyles } from "./../../utils/style-reducer";
import { typography, colors } from "../../theme";

export type TagButtonStyleProps = {
	selected?: boolean;
};

const baseStyles: StyleFn = () => ({
	border: `1px solid ${colors.blue[500]}`,
	color: colors.white,
	backgroundColor: colors.black,
	textAlign: "center",
	padding: "15px 20px",
	display: "inline-block",
	cursor: "default",
	fontFamily: typography.fonts.base,
	fontWeight: typography.weights.medium,
	fontSize: typography.sizes.button.large,
	margin: "0 15px 0 0",
	lineHeight: typography.sizes.button.large,

	span: {
		marginLeft: "20px",
		fontSize: typography.sizes.icon.xxlarge,
		fontWeight: typography.weights.regular,
		lineHeight: 0,
		verticalAlign: "sub",
	},

	"&::selection": {
		background: "transparent",
	},
});

const shadowFromProps: StyleFn = (styles, { selected = false }) => ({
	...styles,
	boxShadow: selected ? `3px 3px ${colors.blue[500]}` : "none",
});

export const useCSS = (props: TagButtonStyleProps) => makeStyles([baseStyles, shadowFromProps])(props);
