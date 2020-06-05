import React, { useState } from "react";
import { useCSS, TabsStyleProps } from "./Tabs.style";

type TabsProps = {
	children: Array<React.ReactNode>;
	onChange?: (tab: string) => void;
} & TabsStyleProps;

export default function Tabs({ children, onChange, ...styleProps }: TabsProps) {
	const [activeTab, setActiveTab] = useState(0);

	function onTabChange(tab: any): void {
		setActiveTab(tab);
		onChange(tab);
	}

	let styles = useCSS(styleProps);

	return (
		<div css={styles.container}>
			<div css={styles.tabs}>
				{children.map((tab: any, index: any) => (
					<div
						key={`tab-${index}`}
						css={index === activeTab ? styles.activeTab : styles.tab}
						onClick={() => onTabChange(index)}
					>
						{tab.props.label}
					</div>
				))}
			</div>
			<div>{children.filter((tab: any, index: any) => index === activeTab).map((tab: any) => tab)}</div>
		</div>
	);
}
