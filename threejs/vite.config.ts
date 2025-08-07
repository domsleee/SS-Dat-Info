import { defineConfig } from 'vite';
import path from 'path';
import { readdir } from 'fs/promises';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig(({ command }) => ({
  base: command === 'serve' ? '/' : '/SS-Dat-Info/',
  appType: 'mpa',
  server: {
    port: 8080,
    fs: {
      allow: ['..', '../replays']
    },
  },
  experimental: {
    enableNativePlugin: 'resolver'
  },
  build: {
    sourcemap: true,
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'index.html'),
        circleTool: path.resolve(__dirname, 'circleTool/index.html')
      }
    }
  },
  publicDir: {
    '/replays': '../replays'
  } as unknown as string,
  plugins: [
    tailwindcss(),
    {
      name: 'list-public-dir',
      configureServer(server) {
        server.middlewares.use('/api/files', async (req, res) => {
          const dir = path.resolve(__dirname, 'public');
          const files = await readdir(dir, { recursive: true });
          res.setHeader('Content-Type', 'application/json');
          res.end(JSON.stringify(files));
        });
      },
    },
  ],
  define: {
    APP_VERSION: JSON.stringify(process.env.npm_package_version),
  }
}));