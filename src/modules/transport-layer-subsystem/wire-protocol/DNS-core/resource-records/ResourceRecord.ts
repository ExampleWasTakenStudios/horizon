import type { DNS_CLASSES } from '../constants/DNS_CLASSES.js';
import type { DNS_TYPES } from '../constants/DNS_TYPES.js';
import type { RDataMap } from './RDataMap.js';

export interface ResourceRecord<T> {
  /**
   * A domain name to which this resource record pertains.
   */
  readonly name: string;
  /**
    Two octets which specify the class of the data in the
    RDATA field.
   */
  readonly type: DNS_TYPES;
  /**
   * Two octets which specify the class of the data in the RDATA field.
   */
  readonly RR_class: DNS_CLASSES;
  /**
    A 32 bit unsigned integer that specifies the time
    interval (in seconds) that the resource record may be
    cached before it should be discarded.  Zero values are
    interpreted to mean that the RR can only be used for the
    transaction in progress, and should not be cached.
   */
  readonly ttl: number;
  /**
    An unsigned 16 bit integer that specifies the length in
    octets of the RDATA field.
   */
  readonly rdLength: number;
  /**
    A variable length string of octets that describes the
    resource.  The format of this information varies
    according to the TYPE and CLASS of the resource record.
    For example, the if the TYPE is A and the CLASS is IN,
    the RDATA field is a 4 octet ARPA Internet address.
   */
  readonly rData: T;
}

/**
 * Interface defining and OPT RR.
 */
export interface I_OPT_Record extends ResourceRecord<RDataMap[DNS_TYPES.OPT]> {
  RR_class: number;
  extendedRCode: number;
  ednsVersion: number;
}
