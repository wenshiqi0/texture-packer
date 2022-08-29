import { defineConfig } from 'vite';
import { resolve } from 'path';
import { viteSingleFile } from 'vite-plugin-singlefile';

export default defineConfig({
    build: {
        rollupOptions: {
            input: resolve(__dirname, 'web/index.html'),
        }
    },
    plugins: [
        viteSingleFile({
            removeViteModuleLoader: true
        }),
    ]
})
