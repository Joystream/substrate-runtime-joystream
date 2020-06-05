import React, { useState } from "react";
import { Checkbox } from "./../src";

export default {
	title: "Checkbox",
	component: Checkbox,
};

export const Primary = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ padding: "50px 20px", background: "black" }}>
			<Checkbox
				selected={selected}
				onChange={() => {
					setSelected(!selected);
				}}
			/>
		</div>
	);
};

export const PrimaryWithDash = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ padding: "50px 20px", background: "black" }}>
			<Checkbox
				icon="dash"
				selected={selected}
				onChange={() => {
					setSelected(!selected);
				}}
			/>
		</div>
	);
};

export const Error = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ padding: "50px 20px", background: "black" }}>
			<Checkbox
				selected={selected}
				error
				onChange={() => {
					setSelected(!selected);
				}}
			/>
		</div>
	);
};

export const ErrorWithDash = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ padding: "50px 20px", background: "black" }}>
			<Checkbox
				selected={selected}
				error
				icon="dash"
				onChange={() => {
					setSelected(!selected);
				}}
			/>
		</div>
	);
};

export const Disabled = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ display: "flex", padding: "50px 20px", background: "black" }}>
			<Checkbox
				disabled
				onChange={() => {
					setSelected(!selected);
				}}
				selected={selected}
			/>
			<Checkbox disabled icon="check" selected />
			<Checkbox disabled icon="dash" selected />
		</div>
	);
};

export const WithLabel = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ display: "flex", padding: "50px 20px", background: "black" }}>
			<Checkbox
				onChange={() => {
					setSelected(!selected);
				}}
				selected={selected}
				label="quite a long label"
			/>
		</div>
	);
};

export const WithLabelStart = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ display: "flex", padding: "50px 20px", background: "black" }}>
			<Checkbox
				onChange={() => {
					setSelected(!selected);
				}}
				selected={selected}
				label="quite a long label"
				labelPosition="start"
			/>
		</div>
	);
};

export const WithLabelTop = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ display: "flex", padding: "50px 20px", background: "black" }}>
			<Checkbox
				onChange={() => {
					setSelected(!selected);
				}}
				selected={selected}
				label="quite a long label"
				labelPosition="top"
			/>
		</div>
	);
};

export const WithLabelBottom = () => {
	const [selected, setSelected] = useState(false);
	return (
		<div style={{ display: "flex", padding: "50px 20px", background: "black" }}>
			<Checkbox
				onChange={() => {
					setSelected(!selected);
				}}
				selected={selected}
				label="quite a long label"
				labelPosition="bottom"
			/>
		</div>
	);
};
