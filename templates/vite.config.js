import { defineConfig } from 'vite'
import { resolve } from 'path'

export default defineConfig({
  root: resolve(__dirname),
  build: {
    outDir: 'admin',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        admin: resolve(__dirname, 'admin/admin.js'),
      },
      output: {
        entryFileNames: '[name].min.js',
        chunkFileNames: '[name]-[hash].min.js',
        assetFileNames: '[name].[ext]',
      },
    },
    minify: 'esbuild',
  },
})
