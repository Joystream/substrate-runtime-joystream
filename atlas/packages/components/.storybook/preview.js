import { addDecorator } from "@storybook/react";
import { withKnobs } from "@storybook/addon-knobs";
import { jsxDecorator } from "storybook-addon-jsx";

addDecorator(withKnobs);
addDecorator(jsxDecorator);
