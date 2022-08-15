/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from 'vite'
import eslintPlugin from 'vite-plugin-eslint'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

export default defineConfig(async ({ command }) => ({
  base: command === 'serve' ? 'http://localhost:3000' : '/',
  root: resolve(__dirname, 'src'),
  build: {
    sourcemap: true,
    manifest: true,
    polyfillModulePreload: true,
    rollupOptions: {
      input: './src/app/index.tsx'
    },
    outDir: resolve(__dirname, '../public')
  },
  server: {
    port: 3000,
    strictPort: true,
    hmr: true,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true
      },
      '/static': {
        target: 'http://localhost:8080',
        changeOrigin: true
      }
    }
  },
  plugins: [
    react(),
    eslintPlugin({
      fix: true,
      emitError: true,
      include: ['src/**/*.{ts,tsx}']
    })
  ],
  resolve: {
    alias: {
      '@': '/app'
    }
  },
  test: {
    globals: true,
    environment: 'jsdom',
    coverage: {
      reporter: ['text', 'json', 'html']
    }
  }
}))
