import examples from "./index";

const scriptArg = process.argv[2];
const script = Object.keys(examples).includes(scriptArg) ? examples[scriptArg as keyof typeof examples] : null;

if (!scriptArg || !script) {
	console.error("Please specify valid example name.");
	console.error("Available examples:", Object.keys(examples));
	process.exit();
} else {
	script(...process.argv.slice(3))
		.then(() => process.exit())
		.catch(console.error);
}
