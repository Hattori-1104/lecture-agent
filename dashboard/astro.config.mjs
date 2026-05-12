// @ts-check
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';

// site と base は GitHub リポジトリ名に合わせて変更してください
// 例: site: 'https://yourname.github.io', base: '/lecture-agent'
export default defineConfig({
  site: 'https://Hattori-1104.github.io',
  base: '/lecture-agent',
  output: 'static',
  vite: {
    plugins: [tailwindcss()],
  },
});