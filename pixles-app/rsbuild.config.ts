import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { TanStackRouterRspack } from '@tanstack/router-plugin/rspack';

const isDev = process.env.NODE_ENV === 'development';

export default defineConfig({
  plugins: [pluginReact()],
  // dev: {
  //   lazyCompilation: true, // Breaks UI
  // },
  server: {
    port: 5173,
  },
  performance: {
    buildCache: isDev,
    removeConsole: !isDev,
  },
  tools: {
    rspack: {
      plugins: [TanStackRouterRspack()],
      experiments: {
        incremental: isDev,
      }
    },
  },
});
