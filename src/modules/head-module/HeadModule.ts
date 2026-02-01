import { ConfigManager } from '../../config/ConfigManager.js';
import { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import { RequestModule } from '../request-module/RequestModule.js';
import { ResolutionSubsystem } from '../resolution-subsystem/ResolutionSubsystem.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule extends Module {
  private readonly requestModule: RequestModule;
  private readonly resolutionSubsystem: ResolutionSubsystem;
  private readonly transportLayerSubsystem: TransportLayerSubsystem;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.spawnSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      this.config
    );

    this.resolutionSubsystem = new ResolutionSubsystem(
      this.transportLayerSubsystem,
      this.logger.spawnSubLogger('RESOLUTION SUBSYSTEM'),
      this.config
    );

    this.requestModule = new RequestModule(
      this.resolutionSubsystem,
      this.transportLayerSubsystem,
      this.logger.spawnSubLogger('REQUEST MODULE'),
      this.config
    );
  }

  public start(): void {
    this.requestModule.start();
    this.resolutionSubsystem.start();
    this.transportLayerSubsystem.start();
  }

  public stop(): void {
    this.requestModule.stop();
    this.resolutionSubsystem.stop();
    this.transportLayerSubsystem.stop();
  }
}
