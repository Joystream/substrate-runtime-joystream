import * as CSS from "csstype"
import { css } from "@emotion/core"

//FIXME: This is not correctly typed, since it doesn't
export type StyleObj = { [k in keyof CSS.Properties]: number | string | StyleObj }
export type StyleFn = (style?: StyleObj, x?: any) => StyleObj

export type Modifiers = { [k: string]: StyleFn }

// TODO: Properly type this
type StyleMonad = (
	run: StyleFn
) => {
	run: StyleFn
	map: (f: (args: any) => any) => StyleMonad
	contramap: (f: (args: any) => any) => StyleMonad
	concat: (other: StyleMonad) => StyleMonad
}

export const Reducer = (run: StyleFn) => ({
	run,
	concat: (other: any) => Reducer((styles: StyleObj, props: any) => other.run(run(styles, props), props)),
	map: (f: (x: any) => any) => Reducer((styles: StyleObj, props: any) => f(run(styles, props))),
	contramap: (f: (x: any) => any) => Reducer((styles: StyleObj, props: any) => run(styles, f(props)))
})

export function combineReducers(...reducers: StyleFn[]) {
	return reducers.reduce(
		(acc, reducer) => acc.concat(Reducer(reducer)),
		Reducer(() => ({}))
	)
}

export function makeStyles(reducers: StyleFn[]) {
	const reducer = combineReducers(...reducers)
	return function(props: any) {
		const styles: any = reducer.run({}, props)
		return css(styles)
	}
}
