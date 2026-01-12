import { ConfigManager } from '../../config/ConfigManager.js';
import { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule extends Module {
  private readonly transportLayerSubsystem: TransportLayerSubsystem;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);
    this.transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.spawnSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      config
    );
  }
}
