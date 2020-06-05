export function stripInline(str: string) {
	return str.replace(/inline-?/gi, "") || "block";
}
