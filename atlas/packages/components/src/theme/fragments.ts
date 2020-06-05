import { StyleFn } from "./../utils/style-reducer"
import { css } from "@emotion/core"
import spacing from "./spacing"
import typography from "./typography"
import colors from "./colors"
import { StyleObj, stripInline } from "../utils"

export function log(styles: StyleObj, props: any) {
	console.log("styles", styles)
	console.log("props", props)
	return styles
}

export function dimensionsFromProps(styles: StyleObj, { full }: { full: boolean }) {
	let display: string
	if (styles.display == null) {
		display = "block"
	}
	display = styles.display as string

	return {
		...styles,
		display: full && display.includes("inline") ? stripInline(display) : display,
		width: full ? "100%" : styles.width || ""
	}
}

export function disabled(styles: StyleObj, { disabled }: { disabled: boolean }): StyleObj {
	if (!disabled) {
		return styles
	}

	return {
		...unsetStyles(styles),
		backgroundColor: colors.gray[100],
		color: colors.white
	}
}

function unsetStyles(styles: StyleObj): StyleObj {
	// Filter and unset all properties that give color, on all states.
	// Need to add more?
	const colorProperties = ["color", "backgroundColor", "borderColor", "boxShadow", "fill", "stroke"]

	const filteredEntries = Object.entries(styles).map(([key, value]) => {
		// If it has a psuedo selector, we're going to disable color from that as well.
		if (key.includes("&:hover") || key.includes("&:active") || key.includes("&:focus")) {
			return unsetStyles(value as StyleObj)
		} else if (colorProperties.includes(key)) {
			return [key, "unset"]
		}
		return [key, value]
	})
	return Object.fromEntries(filteredEntries as any)
}
