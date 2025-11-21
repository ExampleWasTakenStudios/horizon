export class ResourceRecord {
  /**
   * A domain name to which this resource record pertains.
   */
  name: string;
  /**
    Two octets which specify the class of the data in the
    RDATA field.
   */
  type: Uint8Array;
  /**
    A 32 bit unsigned integer that specifies the time
    interval (in seconds) that the resource record may be
    cached before it should be discarded.  Zero values are
    interpreted to mean that the RR can only be used for the
    transaction in progress, and should not be cached.
   */
  ttl: Uint32Array;
  /**
    An unsigned 16 bit integer that specifies the length in
    octets of the RDATA field.
   */
  rdLength: Uint16Array;
  /**
    A variable length string of octets that describes the
    resource.  The format of this information varies
    according to the TYPE and CLASS of the resource record.
    For example, the if the TYPE is A and the CLASS is IN,
    the RDATA field is a 4 octet ARPA Internet address.
   */
  rdData: Uint8Array;

  constructor(name: string, type: Uint8Array, ttl: Uint32Array, rdLength: Uint16Array, rdData: Uint8Array) {
    this.name = name;
    this.type = type;
    this.ttl = ttl;
    this.rdLength = rdLength;
    this.rdData = rdData;
  }
}
