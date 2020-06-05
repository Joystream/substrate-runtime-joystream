import { StyleFn, makeStyles } from "./../../utils";
import { css } from "@emotion/core";
import { typography, colors, spacing } from "./../../theme";

export type DropdownStyleProps = {
	disabled?: boolean;
	focus?: boolean;
	error?: boolean;
	isActive?: boolean;
};

const wrapper: StyleFn = () => ({
	display: "block",
	maxWidth: "250px",
	fontFamily: typography.fonts.base,
});

const container: StyleFn = (_, { disabled }) => ({
	position: "relative",
	width: "100%",
	height: "48px",
	display: "inline-flex",
	cursor: disabled ? "not-allowed" : "default",
});

const borderBase: StyleFn = () => ({
	position: "absolute",
	top: 0,
	left: 0,
	right: 0,
	bottom: 0,
	borderWidth: "1px",
	borderStyle: "solid",
	display: "flex",
	alignItems: "center",
	justifyContent: "left",
});

const label: StyleFn = (_, { error }) => ({
	color: error ? colors.error : colors.gray[400],
	padding: `0 ${spacing.xxxxl} 0 ${spacing.s}`,
	backgroundColor: colors.black,
	fontSize: typography.sizes.body2,
	"&::selection": {
		backgroundColor: "transparent",
	},
});

const input: StyleFn = () => ({
	display: "none",
	widht: "100%",
	margin: `0 ${spacing.xxxxl} 0 ${spacing.s}`,
	background: "none",
	border: "none",
	color: colors.white,
	outline: "none",
	fontSize: typography.sizes.body2,
	padding: "5px 0",
});

const iconOpen: StyleFn = () => ({
	color: colors.gray[300],
	fontSize: typography.sizes.icon.medium,
	position: "absolute",
	top: spacing.m,
	right: spacing.s,
});

const iconClose: StyleFn = (_, { error }) => ({
	color: error ? colors.error : colors.blue[500],
	fontSize: typography.sizes.icon.medium,
	position: "absolute",
	top: spacing.m,
	right: spacing.s,
});

const helper: StyleFn = (_, { error }) => ({
	color: error ? colors.error : colors.gray[400],
	fontSize: typography.sizes.caption,
	margin: `${spacing.xxs} ${spacing.xs}`,
});

const options: StyleFn = () => ({
	backgroundColor: colors.gray[700],
	color: colors.white,
	display: "block",
	width: "100%",
	position: "absolute",
	top: "50px",
	maxHeight: "145px",
	overflowX: "none",
	overflowY: "auto",
});

const option: StyleFn = () => ({
	padding: spacing.s,
	fontSize: typography.sizes.body2,
	"&:hover": {
		backgroundColor: colors.gray[600],
	},
});

const borderColorFromProps: StyleFn = (styles, { error, disabled, isActive, focus }) => {
	const borderColor = disabled
		? colors.gray[200]
		: error
		? colors.error
		: focus
		? colors.blue[500]
		: isActive
		? colors.gray[200]
		: colors.gray[400];

	return {
		...styles,
		borderColor,
	};
};

export const useCSS = (props: DropdownStyleProps) => ({
	wrapper: makeStyles([wrapper])(props),
	container: makeStyles([container])(props),
	border: makeStyles([borderBase, borderColorFromProps])(props),
	label: makeStyles([label])(props),
	input: makeStyles([input])(props),
	iconOpen: makeStyles([iconOpen])(props),
	iconClose: makeStyles([iconClose])(props),
	helper: makeStyles([helper])(props),
	options: makeStyles([options])(props),
	option: makeStyles([option])(props),
});
