import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
//    build:{
//        target:"esnext"
//    },
export default defineConfig({
    build: { target: "esnext", minify: false },
    optimizeDeps: {
        esbuildOptions: { target: "esnext" },
    },
    plugins: [react()],

})
