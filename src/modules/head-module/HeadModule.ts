import { Logger } from '../../logging/Logger.js';
import { ConsoleTransport } from '../../logging/transports/ConsoleTransport.js';
import type { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule implements Module {
  private transportLayerSubsystem: TransportLayerSubsystem;
  private logger: Logger;

  constructor() {
    this.logger = new Logger('HEAD');
    this.logger.addTransport(new ConsoleTransport());
    this.transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.getSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      true
    );

    this.logger.debug('I am a debug message');
    this.logger.verbose('I am a verbose message');
    this.logger.info('I am an info message');
    this.logger.warn('I am a warn message');
    this.logger.info('I am an info message with an attached object:\n', this.transportLayerSubsystem);
    this.logger.error('I am an error message');
    this.logger.fatal('I am a fatal message');
  }
}
