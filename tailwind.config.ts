import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';

const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}'
	],

	plugins: [require('flowbite/plugin')],

	darkMode: 'class',
	fontFamily: {
		sans: ['"Zed Sans"', ...defaultTheme.fontFamily.sans]
	},
	theme: {
		extend: {
			colors: {
				// slate
				primary: {
					'50': '#F8FAFC',
					'100': '#f1f5f9',
					'200': '#e2e8f0',
					'300': '#cbd5e1',
					'400': '#94a3b8',
					'500': '#64748b',
					'600': '#475569',
					'700': '#334155',
					'800': '#1e293b',
					'900': '#0f172a'
				}
			}
		}
	}
} satisfies Config;

module.exports = config;

export default config;
