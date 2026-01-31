import { ConfigManager } from '../../config/ConfigManager.js';
import { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import { RequestModule } from '../request-module/RequestModule.js';
import { ResolutionSubsystem } from '../resolution-subsystem/ResolutionSubsystem.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';

export class HeadModule extends Module {
  // @ts-expect-error Unused variable necessary for architecture.
  private readonly _requestModule: RequestModule;
  private readonly _resolutionSubsystem: ResolutionSubsystem;
  private readonly _transportLayerSubsystem: TransportLayerSubsystem;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this._transportLayerSubsystem = new TransportLayerSubsystem(
      this.logger.spawnSubLogger('TRANSPORT LAYER SUBSYSTEM'),
      this.config
    );

    this._resolutionSubsystem = new ResolutionSubsystem(
      this._transportLayerSubsystem,
      this.logger.spawnSubLogger('RESOLUTION SUBSYSTEM'),
      this.config
    );

    this._requestModule = new RequestModule(
      this._resolutionSubsystem,
      this._transportLayerSubsystem,
      this.logger.spawnSubLogger('REQUEST MODULE'),
      this.config
    );
  }
}
