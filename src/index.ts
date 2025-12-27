import 'dotenv/config';
import path from 'path';
import { ConfigManager } from './config/ConfigManager.js';
import { Logger } from './logging/Logger.js';
import { ConsoleTransport } from './logging/transports/ConsoleTransport.js';
import {
  RotatingFileTransport,
  type RotatingFileTransportSettings,
} from './logging/transports/RotatingFileTransport.js';
import { HeadModule } from './modules/head-module/HeadModule.js';

const ROTATING_STREAM_SETTINGS: RotatingFileTransportSettings = {
  interval: '1d',
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

const mainLogger = new Logger('MAIN');
mainLogger.addTransport(consoleTransport);
mainLogger.addTransport(rotatingFileTransport);

process.on('uncaughtException', (error, origin) => {
  mainLogger.fatal(
    'Uncaught Exception - Attempting cleanup and graceful exit...',
    '\nError: ',
    error,
    '\nOrigin: ',
    origin
  );

  // TODO: destroy all modules, end and file streams etc,

  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  mainLogger.error('Unhandled Promise Rejection!', '\nReason: ', reason, '\n Promise: ', promise);
});

process.on('warning', (warning) => mainLogger.warn('Node warning: ', warning));

process.on('exit', (code) => mainLogger.info('Exiting with exit code ', code));

const configManager = new ConfigManager(mainLogger.spawnSubLogger('CONFIG MANAGER'));
const _headModule = new HeadModule(mainLogger.spawnSubLogger('HEAD MODULE'), configManager);
