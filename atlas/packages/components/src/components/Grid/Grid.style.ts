import { StyleFn, makeStyles } from "../../utils";

export type GridStyleProps = {
	minItemWidth?: string | number;
	maxItemWidth?: string | number;
};
const container: StyleFn = (_, { minItemWidth = "300", maxItemWidth }) => ({
	display: "grid",
	gridTemplateColumns: `repeat(auto-fit, minmax(${minItemWidth}px, ${maxItemWidth ? `${maxItemWidth}px` : "1fr"}))`,
	gap: "30px",
});

const item: StyleFn = () => ({
	cursor: "pointer",
	width: "100%",
});

export const useCSS = (props: GridStyleProps) => ({
	container: makeStyles([container])(props),
	item: makeStyles([item])(props),
});
