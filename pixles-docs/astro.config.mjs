// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightLinksValidator from 'starlight-links-validator';
import tailwindcss from "@tailwindcss/vite";
import starlightVersions from 'starlight-versions'

// https://astro.build/config
export default defineConfig({
	site: "https://pixles.justinchung.net", // TODO: Get domain later
	integrations: [
		starlight({
			title: 'Pixles',
			description: 'Photo sharing for all!',
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/justin13888/Pixles' },
			],
			editLink: {
				baseUrl: 'https://github.com/justin13888/Pixles/tree/master/pixles-docs',
			},
			sidebar: [
				{
					label: 'Guides',
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: 'Getting Started', slug: 'guides/getting-started' },
						{ slug: 'guides/self-hosting' },
					],
				},
				{
					label: 'Development',
					autogenerate: { directory: 'development' },
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
			customCss: [
				'./src/styles/global.css',
			],
			// TODO: Add internationalization down the line: https://starlight.astro.build/guides/i18n/
			plugins: [
				starlightLinksValidator(),
				// starlightVersions({
				// 	// current: {
				// 	// 	label: 'master',
				// 	// },
				// 	versions: [
				// 		{ slug: 'Latest' }
				// 	],
				// }), // TODO: Add versions later
			],
		}),
	],
	vite: {
		plugins: [tailwindcss()],
	},
});
