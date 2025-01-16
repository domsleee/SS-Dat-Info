import glsl from 'vite-plugin-glsl';
import { defineConfig } from 'vite';
import path from 'path';
import { readdir } from 'fs/promises';

export default defineConfig(({ command, mode }) => ({
  base: command === 'serve' ? '/' : '/SS-Dat-Info/',
  server: {
    port: '8080',
    fs: {
      allow: ['..', '../replays']
    },
  },
  publicDir: {
    '/replays': '../replays'
  },
  plugins: [ glsl(),
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