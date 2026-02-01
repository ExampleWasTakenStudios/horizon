import { ConfigManager } from '@src/config/ConfigManager.js';
import { Logger } from '@src/logging/Logger.js';
import { Subsystem } from '../Subsystem.js';
import { DownstreamModule } from './DownstreamModule.js';
import { UpstreamModule } from './UpstreamModule.js';

export class TransportLayerSubsystem extends Subsystem {
  private readonly upstreamModule: UpstreamModule;
  private readonly downstreamModule: DownstreamModule;

  public constructor(logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.upstreamModule = new UpstreamModule(logger.spawnSubLogger('UPSTREAM MODULE'), config);
    this.downstreamModule = new DownstreamModule(logger.spawnSubLogger('DOWNSTREAM MODULE'), config);
  }

  public start(): void {
    this.downstreamModule.start();
    this.upstreamModule.start();
  }

  public stop(): void {
    this.logger.verbose('Stopping...');

    this.downstreamModule.stop();
    this.upstreamModule.stop();

    this.logger.info('Stopped.');
  }

  public getDownstreamModule(): DownstreamModule {
    return this.downstreamModule;
  }

  public getUpstreamModule(): UpstreamModule {
    return this.upstreamModule;
  }
}
