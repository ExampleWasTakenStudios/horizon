import tsConfigPaths from 'vite-plugin-tsconfig-paths';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['./src/**/*.ts'],
      exclude: ['src/**/constants/**', 'src/**/*.d.ts', 'src/**/index.ts'],
    },
    include: ['./src/**/*.test.ts', './test/**/*.test.ts'],
  },
  plugins: [tsConfigPaths.default()],
});
