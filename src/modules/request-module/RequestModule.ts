import type { ConfigManager } from '../../config/ConfigManager.js';
import type { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import type { ResolutionSubsystem } from '../resolution-subsystem/ResolutionSubsystem.js';
import type { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import { QueryContainer } from './QueryContainer.js';

export class RequestModule extends Module {
  private readonly resolutionSubsystem: ResolutionSubsystem;
  private readonly transportLayer: TransportLayerSubsystem;

  public constructor(
    resolutionSubsystem: ResolutionSubsystem,
    transportLayer: TransportLayerSubsystem,
    logger: Logger,
    config: ConfigManager
  ) {
    super(logger, config);

    this.resolutionSubsystem = resolutionSubsystem;
    this.transportLayer = transportLayer;

    this.transportLayer
      .getDownstreamModule()
      .onReceiveUDP4((data, rinfo) => void this.handleQuery(data, rinfo.address, rinfo.port));
  }

  private async handleQuery(query: Buffer, sourceAddress: string, sourcePort: number): Promise<void> {
    const queryContainer = new QueryContainer(query, sourceAddress, sourcePort);

    const resolveResult = await this.resolutionSubsystem.resolveQuery(queryContainer.getQuery());
    if (resolveResult.isFailure()) {
      return;
    }

    this.transportLayer.getDownstreamModule().sendUDP4(resolveResult.value, sourceAddress, sourcePort);
  }
}
