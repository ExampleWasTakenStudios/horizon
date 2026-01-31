import type { HorizonConfig } from './HorizonConfig.js';

export const DefaultConfig: HorizonConfig = {
  configFileVersion: 1,
  resolverSubsystem: {
    queryTimeout: 1,
    authoritativeServerModule: {},
    recursiveResolverSubsystem: {},
    stubResolverModule: {
      mainResolver: {
        primaryIPv4: '1.1.1.1',
        secondaryIPv4: '1.0.0.1',
      },
      fallbackResolver: {
        primaryIPv4: '8.8.8.8',
        secondaryIPv4: '8.8.4.4',
      },
    },
  },
  transportLayerSubsystem: {
    upstreamModule: {
      dnsIPv4Address: '0.0.0.0',
      dnsIPv4Port: 1313,
    },
    downstreamModule: {
      dnsIPv4Address: '0.0.0.0',
    },
  },
};
