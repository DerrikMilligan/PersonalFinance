import { PUBLIC_TAURI_DEV } from "$env/static/public";

console.log(PUBLIC_TAURI_DEV || 'Running in prod mode');

export const prerender = true;
export const ssr = PUBLIC_TAURI_DEV === "true";
