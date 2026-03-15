// Modular Tailwind theme setup (ESM Module)
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';

/** @type {import('tailwindcss').Config} */
export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./src/lib/components/**/*.{svelte,ts}',
		'./src/routes/**/*.{svelte,ts}'
	],
	theme: {
		extend: {
			colors: {
				background: '#f8fafc', // pale gray, Tailwind gray-50
				widget: '#ffffff', // white for card backgrounds
				sidebar: '#1e293b', // navy blue (Tailwind gray-800) or set to preferred hex
				'sidebar-text': '#f1f5f9', // very light text for contrast
				'widget-border': '#e5e7eb', // light gray border, matches Tailwind gray-200
				primary: 'rgb(92, 155, 255)'
			},
			borderRadius: {
				xl: '1rem'
			},
			spacing: {
				sidebar: '16rem'
			}
		}
	},
	plugins: [forms, typography]
};
