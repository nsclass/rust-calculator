import { defineConfig } from 'vite'
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  base: '/rust-calculator/',
  plugins: [react()],
  server: {
    proxy: {
      '/calculate': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
})
