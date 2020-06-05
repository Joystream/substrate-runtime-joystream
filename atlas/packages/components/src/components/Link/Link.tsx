import React, { ReactNode, ReactChild } from "react";
import { Link } from "@reach/router";
import { CustomLinkStyleProps, useCSS } from "./Link.style";

type CustomLinkProps = {
	children: ReactChild;
	to: string;
	disabled?: boolean;
	className?: string;
	replace?: boolean;
	ref?: React.Ref<HTMLAnchorElement>;
	innerRef?: React.Ref<HTMLAnchorElement>;
	getProps?: any;
	state?: any;
	onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
} & CustomLinkStyleProps;

export default function CustomLink({
	children,
	to = "",
	disabled = false,
	className = "",
	replace = false,
	ref = () => {},
	innerRef = () => {},
	getProps = () => {},
	state = null,
	onClick,
	...props
}: CustomLinkProps) {
	let styles = useCSS(props);

	if (disabled) return <label css={styles.disabled}>{children}</label>;
	return (
		<Link
			to={to}
			css={styles.regular}
			className={className}
			replace={replace}
			ref={ref}
			innerRef={innerRef}
			getProps={getProps}
			state={state}
		>
			{children}
		</Link>
	);
}
