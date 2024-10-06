import { defineConfig, type PluginOption } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";
import { visualizer } from "rollup-plugin-visualizer";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    visualizer({
      filename: "profiler/ui-bundle-stats.html"
    }) as PluginOption 
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        primary: resolve(__dirname, "screens/primary.html"),
        splashscreen: resolve(__dirname, "screens/splashscreen.html"),
        about: resolve(__dirname, "screens/about.html"),
        settings: resolve(__dirname, "screens/settings.html"),
      },
    },
  }
}));
