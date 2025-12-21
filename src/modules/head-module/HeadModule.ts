import { LogManager } from '../../logging/LogManager.js';
import type { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule implements Module {
  private transportLayerSubsystem: TransportLayerSubsystem;
  private logManager: LogManager;

  constructor() {
    this.logManager = new LogManager();
    this.transportLayerSubsystem = new TransportLayerSubsystem(true);

    const logger = this.logManager.getLogger();

    /* logger.debug('I am a debug message');
    logger.verbose('I am a verbose message');
    logger.info('I am an info message');
    logger.warn('I am a warn message');
    logger.error('I am an error message'); */
    logger.fatal('I am a fatal message');

    // logger.info('I am an info message with an attached object', this.transportLayerSubsystem);
  }
}
