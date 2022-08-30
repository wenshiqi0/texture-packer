import { defineConfig } from 'vite';
import { resolve } from 'path';
import { viteSingleFile } from 'vite-plugin-singlefile';

export default defineConfig({
    build: {
        rollupOptions: {
            input: resolve(__dirname, 'web/index.html'),
        },
    },
    server: {
        port: 9008,
    },
    plugins: [
        viteSingleFile({
            removeViteModuleLoader: true,
        }),
    ],
    resolve: {
        alias: {
            "@common": resolve(__dirname, 'web/common/index.ts'),
            "@component": resolve(__dirname, 'web/component/index.ts'),
        }
    }
})
