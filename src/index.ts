import dotenv from 'dotenv';
import { HeadModule } from './modules/head-module/HeadModule.js';
/* --- THIS MUST BE THE FIRST THING TO BE EXECUTED */
const env = dotenv.config({ quiet: true });

if (env.error) {
  // As we are only starting here, throwing is actually required to prevent Horizon from loading without valid env vars set.
  // eslint-disable-next-line no-restricted-syntax
  throw env.error;
}

import path from 'path';
import { ConfigManager } from './config/ConfigManager.js';
import { Logger } from './logging/Logger.js';
import { ConsoleTransport } from './logging/transports/ConsoleTransport.js';
import {
  type RotatingFileTransportSettings,
  RotatingFileTransport,
} from './logging/transports/RotatingFileTransport.js';

const ROTATING_STREAM_SETTINGS: RotatingFileTransportSettings = {
  interval: '1d',
  // This format is required by the library
  // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  size: `${100 * 1024 * 1024}B`, // 100 Mib
  maxFiles: 14,
  intervalUTC: true,
  history: 'rotation-history.txt',
  path: path.join(process.cwd(), 'log'), // TODO: this should eventually be configurable
  compress: false, // TODO: this should eventually be changed to check if the current env is dev or prod -> if prod compress
  omitExtension: false,
};

const consoleTransport = new ConsoleTransport(process.env.HORIZON_CONSOLE_LOG_LEVEL);
const rotatingFileTransport = new RotatingFileTransport(
  'log',
  ROTATING_STREAM_SETTINGS,
  process.env.HORIZON_FILE_LOG_LEVEL
);

const logger = new Logger('MAIN');
logger.addTransport(consoleTransport);
logger.addTransport(rotatingFileTransport);

logger.info(`Bootstrapping Horizon ${process.env.HORIZON_VERSION} (${process.env.HORIZON_COMMIT_HASH})`);

process.on('uncaughtException', (error, origin) => {
  logger.fatal(
    'Uncaught Exception - Attempting cleanup and graceful exit...',
    '\nError: ',
    error,
    '\nOrigin: ',
    origin
  );

  // TODO:
  logger.info('Stopping head module.');
  headModule.stop();

  logger.warn('Exiting.');
  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  logger.error('Unhandled Promise Rejection!', '\nReason: ', reason, '\n Promise: ', promise);
});

process.on('warning', (warning) => {
  logger.warn('Node warning: ', warning);
});

process.on('exit', (code) => {
  logger.info('Exiting with exit code ', code);
});

process.on('SIGINT', () => {
  logger.info('Received SIGINT. Shutting down gracefully...');

  headModule.stop();

  process.exit(0);
});

const configManager = new ConfigManager(logger.spawnSubLogger('CONFIG MANAGER'));

const headModule = new HeadModule(logger.spawnSubLogger('HEAD MODULE'), configManager);
headModule.start();
