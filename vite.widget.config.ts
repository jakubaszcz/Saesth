import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
    root: "widget",
    base: "./",
    plugins: [react(), tailwindcss()],
    build: {
        outDir: "../src-tauri/wist-widget",
    },
});