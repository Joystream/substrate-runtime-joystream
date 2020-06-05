import { typography, colors, spacing } from "../../theme"
import { StyleFn, makeStyles } from "../../utils"

export type TabsStyleProps = {}

const container: StyleFn = () => ({
	fontFamily: typography.fonts.base,
	color: colors.white
})

const tabs: StyleFn = () => ({
	display: "flex"
})
const tab: StyleFn = () => ({
	flexBasis: "content",
	padding: `${spacing.m} ${spacing.l}`,
	cursor: "pointer",
	borderBottom: `3px solid ${colors.gray[900]}`,
	minWidth: "100px",
	colors: colors.gray[200],
	textAlign: "center"
})

const activeTab: StyleFn = () => ({
	...tab(),
	color: colors.white,
	backgroundColor: "transparent",
	borderBottom: `3px solid ${colors.blue[500]}`
})
export const useCSS = (props: TabsStyleProps) => ({
	container: makeStyles([container])(props),
	tabs: makeStyles([tabs])(props),
	tab: makeStyles([tab])(props),
	activeTab: makeStyles([activeTab])(props)
})
