import type { ConfigManager } from '../../config/ConfigManager.js';
import { EventDispatcher } from '../../events/EventDispatcher.js';
import type { EventListener, EventSource } from '../../events/EventSource.js';
import type { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import type { ReceivedData } from '../transport-layer-subsystem/ReceivedData.js';
import type { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import type { Resolver } from './Resolver.js';

export class StubResolverModule extends Module implements Resolver, EventSource<ReceivedData> {
  private readonly transportLayer: TransportLayerSubsystem;
  private readonly dispatcher: EventDispatcher<ReceivedData>;

  public constructor(transportLayer: TransportLayerSubsystem, logger: Logger, config: ConfigManager) {
    super(logger, config);
    this.transportLayer = transportLayer;
    this.dispatcher = new EventDispatcher();
  }

  public resolveQuery(query: Buffer): void {
    const resolverAddress = this.config.getConfig().resolverSubsystem.stubResolverModule.mainResolver.primaryIPv4;

    this.transportLayer.getUpstreamModule().send(query, resolverAddress, 53);
  }

  public subscribe(listener: EventListener<ReceivedData>): void {
    this.dispatcher.subscribe(listener);
  }

  public start(): void {
    // Reverse Path: Listen to upstream and dispatch (forward) upstream's events to our subscribers.
    this.transportLayer.getUpstreamModule().subscribe((data) => {
      this.dispatcher.dispatch(data);
    });
  }

  public stop(): void {
    this.logger.info('Stopped.');
    return;
  }
}
