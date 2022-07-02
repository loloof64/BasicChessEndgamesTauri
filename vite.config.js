// vite.config.js
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

import path from "path";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      vue: 'vue/dist/vue.esm-bundler.js',
      'balm-ui-plus': 'balm-ui/dist/balm-ui-plus.esm.js',
      'balm-ui-css': 'balm-ui/dist/balm-ui.css',
      '@': path.resolve(__dirname, './src'),
    }
  }
});
