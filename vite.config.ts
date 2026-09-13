import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  server: {
    port: 5173,
    watch: {
      ignored: ['**/src-tauri/target/**'],
    },
  },
  optimizeDeps: {
    include: ['@tauri-apps/api'],
  },
});

