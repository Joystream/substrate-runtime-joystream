import { typography, colors, spacing } from "../../theme"

export type TypographyStyleProps = {
  variant: "hero" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "subtitle1" | "subtitle2" | "body1" | "body2" | "caption" | "overhead"
}

export let makeStyles = ({
  variant = "body1"
}: TypographyStyleProps) => {

  const base = {
    fontFamily: typography.fonts.base,
    color: colors.white
  }

  let specific = {}

  switch(variant) {
    case "hero":
      specific = {
        fontSize: typography.sizes.hero,
        fontWeight: typography.weights.medium,
        margin: `${spacing.xxxl} 0`
      }
      break
    case "h1":
      specific = {
        fontSize: typography.sizes.h1,
        fontWeight: typography.weights.medium,
        margin: `${spacing.xxl} 0`
      }
      break
    case "h2":
      specific = {
        fontSize: typography.sizes.h2,
        fontWeight: typography.weights.medium,
        margin: `${spacing.xl} 0`
      }
      break
    case "h3":
      specific = {
        fontSize: typography.sizes.h3,
        fontWeight: typography.weights.medium,
        margin: `${spacing.l} 0`
      }
      break
    case "h4":
      specific = {
        fontSize: typography.sizes.h4,
        fontWeight: typography.weights.medium,
        margin: `${spacing.l} 0`
      }
      break
    case "h5":
      specific = {
        fontSize: typography.sizes.h5,
        fontWeight: typography.weights.medium,
        margin: `${spacing.m} 0`
      }
      break
    case "h6":
      specific = {
        fontSize: typography.sizes.h6,
        fontWeight: typography.weights.medium,
        margin: `${spacing.m} 0`
      }
      break
    case "subtitle1":
      specific = {
        fontSize: typography.sizes.subtitle1,
        fontWeight: typography.weights.light,
        margin: `${spacing.l} 0`
      }
      break
    case "subtitle2":
      specific = {
        fontSize: typography.sizes.subtitle2,
        fontWeight: typography.weights.regular,
        margin: `${spacing.m} 0`
      }
      break
    case "body1":
      specific = {
        fontSize: typography.sizes.body1,
        fontWeight: typography.weights.light,
        margin: `${spacing.s} 0`
      }
      break
    case "body2":
      specific = {
        fontSize: typography.sizes.body2,
        fontWeight: typography.weights.light,
        margin: `${spacing.xs} 0`
      }
      break
    case "caption":
      specific = {
        fontSize: typography.sizes.caption,
        fontWeight: typography.weights.light,
        margin: `${spacing.xs} 0`
      }
      break
    case "overhead":
      specific = {
        fontSize: typography.sizes.overhead,
        fontWeight: typography.weights.regular,
        margin: `${spacing.xs} 0`
      }
    break
  }

  return { ...base, ...specific }
}
