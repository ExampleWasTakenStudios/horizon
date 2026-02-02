/* eslint-disable no-console */
import dotenv from 'dotenv';
import esbuild from 'esbuild';
import esbuildPluginLicense from 'esbuild-plugin-license';
import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const str = JSON.stringify;

const envConfig = dotenv.config({ quiet: true, path: './prod.env' }).parsed;

if (!envConfig) {
  console.error(`
    ❌ .env file not found.
  `);
  process.exit(1);
}

const ENTRY_FILE = './out/compiled/index.js';
const COMMIT_HASH = execSync('git rev-parse --short HEAD').toString().trim();
const OUT_DIR = './out/';
const OUT_FILE = 'horizon.js';

const ESBUILD_PLUGIN_CONFIG = {
  thirdParty: {
    output: {
      file: 'out/third-party-licenses.txt',
      template(dependencies) {
        return dependencies
          .map((dependency) => {
            const { name, version, license } = dependency.packageJson;
            const licenseText = dependency.licenseText || 'License text not found';

            return `
-------------------------------------------------------------------------
Package:  ${name}
Version:  ${version}
License:  ${license}
-------------------------------------------------------------------------
${licenseText}
`;
          })
          .join('');
      },
    },
  },
};

if (!fs.existsSync(ENTRY_FILE)) {
  console.error(`
    ❌ Entry file ${ENTRY_FILE} not found.
  `);
  process.exit(1);
}

const define = {
  'process.env.NODE_ENV': str(envConfig.NODE_ENV),
  'process.env.HORIZON_VERSION': str(envConfig.HORIZON_VERSION),
  'process.env.HORIZON_COMMIT_HASH': str(COMMIT_HASH),
  'process.env.HORIZON_CONSOLE_LOG_LEVEL': str(envConfig.HORIZON_CONSOLE_LOG_LEVEL),
  'process.env.HORIZON_FILE_LOG_LEVEL': str(envConfig.HORIZON_FILE_LOG_LEVEL),
};

const _start = performance.mark('start_build');
await esbuild.build({
  entryPoints: [ENTRY_FILE],
  bundle: true,
  minify: true,
  platform: 'node',
  target: 'node25',
  outfile: path.join(OUT_DIR, OUT_FILE),
  format: 'esm',
  define: define,
  legalComments: 'inline',
  plugins: [esbuildPluginLicense(ESBUILD_PLUGIN_CONFIG)],
  banner: {
    js: "import { createRequire } from 'module'; const require = createRequire(import.meta.url);",
  },
});
const _end = performance.mark('end_build');
const measure = performance.measure('bundle', { start: 'start_build', end: 'end_build' });

console.log(`✅ Bundle complete in ${Math.round(measure.duration)} ms`);
