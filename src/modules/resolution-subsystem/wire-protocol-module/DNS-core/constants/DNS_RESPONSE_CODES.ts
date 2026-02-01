/**
 * **NOTE:** This enum only holds RCODEs supported by Horizon.
 */
export enum DNS_RESPONSE_CODES {
  NOERROR = 0,
  FORMERR = 1,
  SERVFAIL = 2,
  NXDOMAIN = 3,
  NOTIMP = 4,
  REFUSED = 5,

  // EDNS(0)
  BADVERS = 16,
}
