import path from 'path';
import { type Options as RFSOptions } from 'rotating-file-stream';
import { Logger } from './logging/Logger.js';
import { LogLevel } from './logging/LogLevel.js';
import { ConsoleTransport } from './logging/transports/ConsoleTransport.js';
import { RotatingFileTransport } from './logging/transports/RotatingFileTransport.js';
import { HeadModule } from './modules/head-module/HeadModule.js';

const ROTATING_STREAM_SETTINGS: RFSOptions = {
  interval: '1d',
  size: `${100 * 1024 * 1024}B`, // 100 Mib
  maxFiles: 14,
  intervalUTC: true,
  immutable: true,
  history: 'rotation-history.txt',
  path: path.join(process.cwd(), 'log'), // TODO: this should eventually be configurable
  compress: false, // TODO: this should eventually be changed to check if the current env is dev or prod -> if prod compress
  omitExtension: false,
};

const consoleTransport = new ConsoleTransport(LogLevel.DEBUG);
const rotatingFileTransport = new RotatingFileTransport('log', ROTATING_STREAM_SETTINGS, LogLevel.DEBUG);

const mainLogger = new Logger('MAIN');
mainLogger.addTransport(consoleTransport);
mainLogger.addTransport(rotatingFileTransport);

const _headModule = new HeadModule(mainLogger.getSubLogger('HEAD MODULE'));
