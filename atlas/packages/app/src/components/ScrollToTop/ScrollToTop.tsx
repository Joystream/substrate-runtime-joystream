import React, { useEffect } from "react"

type ScrollToTopProps = {
  path: string
  children?: any
  location?: any
}

const ScrollToTop = ({ children, location }: ScrollToTopProps) => {
  useEffect(() => window.scrollTo(0, 0), [location.pathname])
  return children
}

export default ScrollToTop
