// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

// import * as db from '$lib/server/database';

import { invoke } from "@tauri-apps/api/core";
/** @type {import('./$types').LayoutServerLoad} */
export async function load() {
    const feeds = await invoke("get_all_feeds");
    const categories = await invoke("get_all_categories");
    console.log('feeds',feeds);
    console.log('categories',categories);
    return {
        feeds: feeds,
        categories: categories
    };
}

// import { invoke } from "@tauri-apps/api/core";
// /** @type {import('./$types').LayoutServerLoad} */
// export async function load() {
// 	return {
// 		feeds: await invoke("get_feed")
// 	};
// }