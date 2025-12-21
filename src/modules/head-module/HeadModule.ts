import { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule extends Module {
  private transportLayerSubsystem: TransportLayerSubsystem;

  constructor(logger: Logger) {
    super(logger);

    this.transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.getSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      true
    );
  }
}
