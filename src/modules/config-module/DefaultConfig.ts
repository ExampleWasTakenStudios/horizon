import type { HorizonConfig } from './HorizonConfig.js';

export const DefaultConfig: HorizonConfig = {
  configFileVersion: 1,
  headModule: {},
  configModule: {},
  resolverSubsystem: {
    authoritativeServerModule: {},
    recursiveResolverSubsystem: {},
  },
  zoneSubsystem: {
    zoneLoaderModule: {},
    zoneAuthorityModule: {},
  },
  transportLayerSubsystem: {
    upstreamModule: {},
    downstreamModule: {},
    wireProtocolModule: {},
  },
  analyticsModule: {},
};
