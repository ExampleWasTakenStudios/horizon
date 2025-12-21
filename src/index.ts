import { Logger } from './logging/Logger.js';
import { ConsoleTransport } from './logging/transports/ConsoleTransport.js';
import { HeadModule } from './modules/head-module/HeadModule.js';

const mainLogger = new Logger('MAIN');
mainLogger.addTransport(new ConsoleTransport());

const _headModule = new HeadModule(mainLogger);
