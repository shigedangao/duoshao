import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    port: 3000,
    strictPort: true,
  },
  // env variables
  envPrefix: ['VITE_', 'TAURI_'],
  plugins: [vue()],
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome97', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG && 'esbuild',
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  }
})
