import * as z from 'zod';

export const HorizonConfigSchema = z.object({
  configFileVersion: z.number(),
  headModule: z.object({}),
  configModule: z.object({}),
  resolverSubsystem: z.object({
    authoritativeServerModule: z.object({}),
    recursiveResolverSubsystem: z.object({}),
  }),
  zoneSubsystem: z.object({
    zoneLoaderModule: z.object({}),
    zoneAuthorityModule: z.object({}),
  }),
  transportLayerSubsystem: z.object({
    upstreamModule: z.object({}),
    downstreamModule: z.object({
      /**
       * The IP address on which the DNS service will listen.
       */
      dnsIPAddress: z.ipv4(),
    }),
    wireProtocolModule: z.object({}),
  }),
  analyticsModule: z.object({}),
});

export type HorizonConfig = z.infer<typeof HorizonConfigSchema>;
