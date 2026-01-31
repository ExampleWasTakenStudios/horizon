import type { ConfigManager } from '../../config/ConfigManager.js';
import type { ResultError } from '../../errors/result/ResultError.js';
import type { Logger } from '../../logging/Logger.js';
import type { TResult } from '../../result/Result.js';
import { Subsystem } from '../Subsystem.js';
import type { TransportLayerSubsystem } from '../transport-layer-subsystem/TransportLayerSubsystem.js';
import { StubResolverModule } from './StubResolverModule.js';

export interface InflightQuery {
  timeout: NodeJS.Timeout;
  resolve(result: TResult<Buffer, ResultError>): void;
}

export class ResolutionSubsystem extends Subsystem {
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

  public async resolveQuery(query: Buffer): Promise<TResult<Buffer, ResultError>> {
    if (this.isAuthoritative) {
      // TODO: forward the query to the authoritative module
    }

    if (!this.isAuthoritative && this.isRecursive) {
      // TODO: forward the query to the recursive resolver module
    }

    // Stub Resolver Mode -> forward to stub resolver module
    this.logger.verbose('Authoritative and Recursive Resolvers are inactive -> falling back to Stub Resolver Mode.');

    return this.stubModule.resolveQuery(query);
  }
}
