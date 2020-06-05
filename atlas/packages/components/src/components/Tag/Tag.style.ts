import { StyleFn, makeStyles } from "./../../utils/style-reducer";
import { css } from "@emotion/core";
import { typography, colors } from "../../theme";

export type TagStyleProps = {};

const styles: StyleFn = () => ({
	border: `1px solid ${colors.blue[700]}`,
	color: colors.white,
	backgroundColor: colors.black,
	textAlign: "center",
	padding: "10px 15px",
	display: "inline-block",
	cursor: "default",
	fontFamily: typography.fonts.base,
	fontWeight: typography.weights.regular,
	fontSize: typography.sizes.button.medium,
	margin: "0 15px 0 0",

	"&::selection": {
		background: "transparent",
	},
});

export const useCSS = ({}: TagStyleProps) => makeStyles([styles])({});
