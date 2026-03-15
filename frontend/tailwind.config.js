// Modular Tailwind theme setup (ESM Module)
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    './src/lib/components/**/*.{svelte,ts}',
  ],
  theme: {
    extend: {
      colors: {
        sidebar: 'rgb(24, 24, 36)',
        widget: 'rgb(36, 37, 59)',
        'sidebar-text': 'rgb(224, 231, 255)',
        primary: 'rgb(92, 155, 255)',
        background: 'rgb(14, 15, 24)',
      },
      borderRadius: {
        xl: '1rem',
      },
      spacing: {
        sidebar: '16rem',
      },
    },
  },
  plugins: [forms, typography],
};
