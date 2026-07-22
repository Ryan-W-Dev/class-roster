import { defineConfig } from "vite";

export default defineConfig({
  root: "frontend",
  clearScreen: false,
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  server: {
    host: "localhost",
    port: 1420,
    strictPort: true,
  },
});
