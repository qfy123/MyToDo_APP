import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
    strictPort: true
  },
  build: {
    outDir: '../dist'
  },
  css: {
    postcss: './postcss.config.js'
  }
})