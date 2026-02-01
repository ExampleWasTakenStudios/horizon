import { ConfigManager } from '@src/config/ConfigManager.js';
import type { EventListener, EventSource } from '@src/events/EventSource.js';
import { Logger } from '@src/logging/Logger.js';
import { Subsystem } from '../Subsystem.js';
import type { ReceivedData } from '../transport-layer-subsystem/ReceivedData.js';
import { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import type { Resolver } from './Resolver.js';
import { StubResolverModule } from './StubResolverModule.js';

export class ResolutionSubsystem extends Subsystem implements Resolver, EventSource<ReceivedData> {
  private readonly isAuthoritative: boolean;
  private readonly isRecursive: boolean;

  private readonly stubModule: StubResolverModule;

  private readonly transportLayer: TransportLayerSubsystem;

  public constructor(transportLayer: TransportLayerSubsystem, logger: Logger, config: ConfigManager) {
    super(logger, config);

    this.isAuthoritative = false;
    this.isRecursive = false;

    this.transportLayer = transportLayer;

    this.stubModule = new StubResolverModule(this.transportLayer, logger, config);
  }

  public resolveQuery(query: Buffer): void {
    if (this.isAuthoritative) {
      // TODO: forward query to authoritative module
    }

    if (!this.isAuthoritative && this.isRecursive) {
      // TODO: forward query to recursive module
    }

    // Fallback to stub resolver mode
    this.stubModule.resolveQuery(query);
  }

  public start(): void {
    this.stubModule.start();
  }

  public stop(): void {
    this.logger.verbose('Stopping...');
    this.stubModule.stop();
    this.logger.info('Stopped.');
  }

  public subscribe(listener: EventListener<ReceivedData>): void {
    // Proxy the subscriptions to the internal components
    this.stubModule.subscribe(listener);
  }
}
