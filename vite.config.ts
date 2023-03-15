import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { internalIpV4 } from 'internal-ip';

export default defineConfig(async () => {
  const host = await internalIpV4();

  const config = {
    plugins: [sveltekit()],

    test: {
      include: ['src/**/*.{test,spec}.{js,ts}']
    },

    server: {
      host: '0.0.0.0',
      port: 5173,
      strictPort: true,
      hmr: {
        host,
        protocol: 'ws',
        port: 5183,
      }
    }
  };

  return config;
});
