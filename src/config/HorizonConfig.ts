import * as z from 'zod';

export const HorizonConfigSchema = z.object({
  configFileVersion: z.number(),
  resolverSubsystem: z.object({
    /**
     * How long a pending query should be considered alive. (in seconds)
     */
    queryTimeout: z.number(),
    authoritativeServerModule: z.object({}),
    recursiveResolverSubsystem: z.object({}),
    stubResolverModule: z.object({
      /**
       * The main foreign resolver to be used by Horizon when in stub resolver mode.
       */
      mainResolver: z.object({
        /**
         * The primary IPv4 address of the main resolver.
         */
        primaryIPv4: z.ipv4(),
        /**
         * The secondary IPv4 address of the main resolver. (optional)
         */
        secondaryIPv4: z.optional(z.ipv4()),
      }),
      /**
       * The fallback resolver to be used by Horizon when the main resolver is not reachable.
       */
      fallbackResolver: z.object({
        /**
         * The primary IPv4 address of the fallback resolver.
         */
        primaryIPv4: z.ipv4(),
        /**
         * The secondary IPv4 address of the fallback resolver. (optional)
         */
        secondaryIPv4: z.optional(z.ipv4()),
      }),
    }),
  }),
  transportLayerSubsystem: z.object({
    upstreamModule: z.object({
      /**
       * The IPv4 address to which the upstream module will bind to to perform DNS related work.
       */
      dnsIPv4Address: z.ipv4(),
      /**
       * The IPv4 port to which the upstream module will bind to to perform DNS related work.
       */
      dnsIPv4Port: z.number(),
    }),
    downstreamModule: z.object({
      /**
       * The IP address on which the DNS service will listen.
       */
      dnsIPv4Address: z.ipv4(),
    }),
  }),
});

export type HorizonConfig = z.infer<typeof HorizonConfigSchema>;
