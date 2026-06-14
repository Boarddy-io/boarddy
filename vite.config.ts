import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;
const isWebBuild = process.env.VITE_WEB_BUILD === "true";
console.log("--- Vite Config Env Info ---");
console.log("process.env.VITE_WEB_BUILD =", process.env.VITE_WEB_BUILD);
console.log("isWebBuild =", isWebBuild);
console.log("----------------------------");

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],
  base: isWebBuild ? "/boarddy/" : "./",
  resolve: isWebBuild ? {
    alias: {
      "@tauri-apps/api/core": path.resolve(__dirname, "./src/tauri-mock.ts"),
      "@tauri-apps/api/webviewWindow": path.resolve(__dirname, "./src/tauri-mock.ts"),
      "@tauri-apps/api/event": path.resolve(__dirname, "./src/tauri-mock.ts"),
    }
  } : undefined,

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
