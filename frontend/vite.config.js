import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
export default defineConfig({
    plugins: [react()],
    server: {
        host: "127.0.0.1",
        port: 5174,
        proxy: {
            "/api": "http://127.0.0.1:8897",
            "/health": "http://127.0.0.1:8897",
        },
    },
    build: {
        sourcemap: true,
        rollupOptions: {
            output: {
                manualChunks: function (id) {
                    if (id.indexOf("node_modules/react") !== -1 || id.indexOf("node_modules/react-dom") !== -1) {
                        return "vendor";
                    }
                },
            },
        },
    },
});
