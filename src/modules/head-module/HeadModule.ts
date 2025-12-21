import { ConsoleTransport } from '../../logging/transports/ConsoleTransport.js';
import { Logger } from '../../logging/Logger.js';
import type { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule implements Module {
  private transportLayerSubsystem: TransportLayerSubsystem;
  private mainLogger: Logger;

  constructor() {
    this.mainLogger = new Logger('HEAD');
    this.mainLogger.addTransport(new ConsoleTransport());
    this.transportLayerSubsystem = new TransportLayerSubsystem(true);

    this.mainLogger.debug('I am a debug message');
    this.mainLogger.verbose('I am a verbose message');
    this.mainLogger.info('I am an info message');
    this.mainLogger.warn('I am a warn message');
    this.mainLogger.error('I am an error message');
    this.mainLogger.fatal('I am a fatal message');

    this.mainLogger.info('I am an info message with an attached object:\n', this.transportLayerSubsystem);
  }
}
