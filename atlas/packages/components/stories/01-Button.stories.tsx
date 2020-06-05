import React from "react";
import { Button } from "../src";
import { faBan } from "@fortawesome/free-solid-svg-icons";

export default {
	title: "Button",
	component: Button,
};

export const Primary = () => (
	<>
		<Button onClick={() => console.log("Button clicked!")}>Regular</Button>
		<Button size="small" onClick={() => console.log("Button clicked!")}>
			Small
		</Button>
		<Button size="smaller" onClick={() => console.log("Button clicked!")}>
			Smaller
		</Button>
	</>
);

export const Secondary = () => (
	<>
		<Button type="secondary">Regular</Button>
		<Button type="secondary" size="small">
			Small
		</Button>
		<Button type="secondary" size="smaller">
			Smaller
		</Button>
	</>
);

export const PrimaryFullSize = () => <Button full>Primary Full Size</Button>;

export const SecondaryFullSize = () => (
	<Button full type="secondary">
		Secondary Full Size
	</Button>
);

export const PrimaryWithIcon = () => (
	<>
		<Button icon>Regular</Button>
		<Button icon size="small">
			Small
		</Button>
		<Button icon size="smaller">
			Smaller
		</Button>
	</>
);

export const SecondaryWithIcon = () => (
	<>
		<Button type="secondary" icon>
			Regular
		</Button>
		<Button type="secondary" icon size="small">
			Small
		</Button>
		<Button type="secondary" icon size="smaller">
			Smaller
		</Button>
	</>
);

export const PrimaryWithoutText = () => (
	<>
		<Button icon />
		<Button icon size="small" />
		<Button icon size="smaller" />
	</>
);

export const SecondaryWithoutText = () => (
	<>
		<Button type="secondary" icon />
		<Button type="secondary" icon size="small" />
		<Button type="secondary" icon size="smaller" />
	</>
);

export const Disabled = () => (
	<>
		<Button disabled={true}>Disabled</Button>
		<Button disabled={true} icon={true}>
			Disabled with icon
		</Button>
		<Button disabled={true} icon />
	</>
);
