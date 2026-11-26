import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// Set by `tauri android dev` so a physical phone can reach the dev server over the LAN.
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [react()],
  // Keep Rust compile errors visible in the terminal.
  clearScreen: false,
  server: {
    // Must match devUrl in src-tauri/tauri.conf.json.
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  // shortcut: Spike 0 has no interface logic to test; drop this once P1 adds the first test.
  test: { passWithNoTests: true },
});
