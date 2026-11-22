import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Relative base so the built calculator works from any folder, for example GitHub Pages.
export default defineConfig({
  base: './',
  plugins: [react()],
});
