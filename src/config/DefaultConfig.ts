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
    upstreamModule: {
      dnsIPv4Address: '',
      dnsIPv4Port: 1313,
    },
    downstreamModule: {
      dnsIPv4Address: '0.0.0.0',
    },
    wireProtocolModule: {},
  },
  analyticsModule: {},
};
