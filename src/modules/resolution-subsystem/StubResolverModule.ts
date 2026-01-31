import type { ConfigManager } from '../../config/ConfigManager.js';
import { ResultError } from '../../errors/result/ResultError.js';
import type { Logger } from '../../logging/Logger.js';
import { Result, type TResult } from '../../result/Result.js';
import { Module } from '../Module.js';
import type { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import type { InflightQuery } from './ResolutionSubsystem.js';
import type { Resolver } from './Resolver.js';

export class StubResolverModule extends Module implements Resolver {
  private readonly transportLayer: TransportLayerSubsystem;

  private readonly inflightQueries: Map<number, InflightQuery>;

  public constructor(transportLayer: TransportLayerSubsystem, logger: Logger, config: ConfigManager) {
    super(logger, config);
    this.transportLayer = transportLayer;

    this.inflightQueries = new Map();

    this.transportLayer.getUpstreamModule().onReceiveUDP4((payload: Buffer) => {
      this.onIncoming(payload);
    });
  }

  public resolve(query: Buffer): Promise<TResult<Buffer, ResultError>> {
    const queryTimeout = this.config.getConfig().resolverSubsystem.queryTimeout;
    const resolverAddress = this.config.getConfig().resolverSubsystem.stubResolverModule.mainResolver.primaryIPv4;

    if (query.length < 2) {
      return Promise.resolve(Result.fail(new ResultError('[STUB RESOLVER] Got invalid DNS query.')));
    }

    const queryId = query.readUint16BE(0);

    return new Promise((resolve) => {
      const timeout = setTimeout(() => {
        this.cleanupInflightQuery(queryId);
      }, queryTimeout * 1000);

      this.inflightQueries.set(queryId, { timeout: timeout, resolve: resolve });

      this.transportLayer.getUpstreamModule().sendUDP4(query, resolverAddress, 53);
    });
  }

  private onIncoming(data: Buffer): void {
    if (data.length < 2) {
      // Data too short to be a DNS message so we ignore it.
      return;
    }

    const queryId = data.readUint16BE(0);

    const inflightQuery = this.inflightQueries.get(queryId);

    if (!inflightQuery) {
      // We are not waiting on this query so we ignore it.
      return;
    }

    this.cleanupInflightQuery(queryId);

    inflightQuery.resolve(Result.ok(data));
  }

  private cleanupInflightQuery(id: number): void {
    const query = this.inflightQueries.get(id);
    if (!query) {
      return;
    }

    clearTimeout(query.timeout);
    this.inflightQueries.delete(id);
  }
}
