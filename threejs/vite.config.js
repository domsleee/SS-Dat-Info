import { defineConfig } from 'vite';
import path from 'path';
import { readdir } from 'fs/promises';

export default defineConfig(({ command }) => ({
  base: command === 'serve' ? '/' : '/SS-Dat-Info/',
  appType: 'mpa',
  server: {
    port: '8080',
    fs: {
      allow: ['..', '../replays']
    },
  },
  experimental: {
    enableNativePlugin: 'resolver'
  },
  build: {
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'index.html'),  // eslint-disable-line no-undef
        circleTool: path.resolve(__dirname, 'circleTool/index.html')  // eslint-disable-line no-undef
      }
    }
  },
  publicDir: {
    '/replays': '../replays'
  },
  plugins: [
    {
      name: 'list-public-dir',
      configureServer(server) {
        server.middlewares.use('/api/files', async (req, res) => {
          const dir = path.resolve(__dirname, 'public');  // eslint-disable-line no-undef
          const files = await readdir(dir, { recursive: true });
          res.setHeader('Content-Type', 'application/json');
          res.end(JSON.stringify(files));
        });
      },
    },
  ],
  define: {
    APP_VERSION: JSON.stringify(process.env.npm_package_version),  // eslint-disable-line no-undef
  },
  resolve: {
    alias: {
      buffer: 'buffer/'
    }
  },
  optimizeDeps: {
    include: ['buffer']
  }
}));