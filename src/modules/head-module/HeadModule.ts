import { Logger } from '../../logging/Logger.js';
import { ConfigModule } from '../config-module/ConfigModule.js';
import { Module } from '../Module.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule extends Module {
  private configModule: ConfigModule;
  private transportLayerSubsystem: TransportLayerSubsystem;

  constructor(logger: Logger) {
    super(logger);

    this.configModule = new ConfigModule(this.logger.spawnSubLogger('CONFIG MODULE'));
    this.transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.spawnSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      true
    );
  }
}
