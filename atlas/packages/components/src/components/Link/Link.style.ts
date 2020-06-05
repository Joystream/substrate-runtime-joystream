import { typography, colors } from "../../theme";
import { StyleFn, makeStyles } from "../../utils";

export type CustomLinkStyleProps = {};

const regular: StyleFn = () => ({
	fontFamily: typography.fonts.base,
	fontSize: typography.sizes.overhead,
	color: colors.blue[400],
	textDecoration: "none",
	cursor: "pointer",
});
const disabled: StyleFn = () => ({
	fontFamily: typography.fonts.base,
	fontSize: typography.sizes.overhead,
	color: colors.gray[200],
	textDecoration: "none",
	cursor: "not-allowed",
});
export const useCSS = (props: CustomLinkStyleProps) => ({
	regular: makeStyles([regular])(props),
	disabled: makeStyles([disabled])(props),
});
