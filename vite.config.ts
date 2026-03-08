import { fileURLToPath, URL } from 'node:url'
import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";

import UnoCSS from 'unocss/vite'
import {celerixPreset} from '@celerix/spectrum/unoPreset'

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
    plugins: [
        UnoCSS({
            // @ts-ignore, during local development celerixPreset's path may throw off an error
            presets: [celerixPreset()],
        }),
        vue()
    ],

    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));
