import type { ConfigManager } from '../../config/ConfigManager.js';
import type { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import type { ResolutionSubsystem } from '../resolution-subsystem/ResolutionSubsystem.js';
import type { ReceivedData } from '../transport-layer-subsystem/ReceivedData.js';
import type { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import type { InflightQuery } from './InflightQuery.js';

export class RequestModule extends Module {
  private readonly resolutionSubsystem: ResolutionSubsystem;
  private readonly transportLayer: TransportLayerSubsystem;

  /**
   * @param K horizonID
   * @param V `InflightQuery`
   */
  private readonly inflightQueries: Map<number, InflightQuery>;
  private internalIdCounter: number;

  public constructor(
    resolutionSubsystem: ResolutionSubsystem,
    transportLayer: TransportLayerSubsystem,
    logger: Logger,
    config: ConfigManager
  ) {
    super(logger, config);

    this.resolutionSubsystem = resolutionSubsystem;
    this.transportLayer = transportLayer;

    this.inflightQueries = new Map();
    this.internalIdCounter = 1;
  }

  public start(): void {
    this.transportLayer.getDownstreamModule().subscribe((data) => {
      this.handleIncoming(data);
    });
    this.resolutionSubsystem.subscribe((data) => {
      this.handleReturning(data);
    });
  }

  public stop(): void {
    this.inflightQueries.clear();
  }

  /**
   * Handles incoming data. 'Incoming' means data that freshly enters Horizon's system i.e. a new query from a client.
   * @param data Incoming data.
   */
  private handleIncoming(data: ReceivedData): void {
    // Buffer is way too short to be a valid DNS message.
    // We check for length == 2 to be sure we can read the transaction ID safely.
    if (data.buf.length < 2) {
      return;
    }

    const queryTimeout = this.config.getConfig().resolverSubsystem.queryTimeout;

    const originalId = data.buf.readUInt16BE(0);
    const horizonId = this.generateId();

    const timeout = setTimeout(() => {
      this.logger.verbose(`Inflight query ${originalId.toString()} (Horizon ID: ${horizonId.toString()}) timed out.`);
      this.inflightQueries.delete(originalId);
    }, queryTimeout * 1000);

    const inflight: InflightQuery = {
      originalId,
      horizonId,
      clientIP: data.rinfo.address,
      clientPort: data.rinfo.port,
      receivedTimestamp: Date.now(),
      timeout,
    };

    this.inflightQueries.set(horizonId, inflight);

    // Write horizon's ID to the buffer before sending it to resolution subsystem
    data.buf.writeUint16BE(horizonId, 0);

    this.resolutionSubsystem.resolveQuery(data.buf);
  }

  /**
   * Handles returning data. 'Returning' means that data is returning to Horizon's system i.e. a response from a foreign server.
   * @param data Returning data.
   */
  private handleReturning(data: ReceivedData): void {
    this.logger.debug('Received returning data.');
    // Buffer is way too short to be a valid DNS message.
    // We check for length == 2 to be sure we can read the transaction ID safely.
    if (data.buf.length < 2) {
      this.logger.debug('Data buffer was shorter than 2. -> Returning...');
      return;
    }

    const horizonId = data.buf.readUint16BE(0);
    const inflight = this.inflightQueries.get(horizonId);

    // Check if the received data corresponds to an inflight query.
    if (!inflight) {
      this.logger.debug('No matching query in flight. -> Returning...');
      return;
    }

    // Land query
    this.logger.debug('Landing query...');
    clearTimeout(inflight.timeout);
    this.inflightQueries.delete(horizonId);

    // Write original transaction ID back to the buffer.
    this.logger.debug('Rewriting transaction ID to original ID...');
    data.buf.writeUInt16BE(inflight.originalId, 0);

    // Send answer to client
    this.logger.debug('Sending buffer to ', inflight.clientIP, ':', inflight.clientPort);
    this.transportLayer.getDownstreamModule().send(data.buf, inflight.clientIP, inflight.clientPort);
  }

  private generateId(): number {
    return (this.internalIdCounter++ % 65_535) + 1;
  }
}
