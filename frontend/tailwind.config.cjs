// Tailwind CSS config in CommonJS for troubleshooting
const forms = require('@tailwindcss/forms');
const typography = require('@tailwindcss/typography');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./src/lib/components/**/*.{svelte,ts}',
		'./src/routes/**/*.{svelte,ts}'
	],
	theme: {}, // minimal config, no custom colors
	plugins: [forms, typography]
};
