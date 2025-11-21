export class DNSQuestion {
  /**
    A domain name represented as a sequence of labels, where
    each label consists of a length octet followed by that
    number of octets.  The domain name terminates with the
    zero length octet for the null label of the root.  Note
    that this field may be an odd number of octets; no
    padding is used.
   */
  qName: Uint8Array;
  /**
    A two octet code which specifies the type of the query.
    The values for this field include all codes valid for a
    TYPE field, together with some more general codes which
    can match more than one type of RR.
   */
  qType: Uint8Array;
  /**
    A two octet code that specifies the class of the query.
    For example, the QCLASS field is IN for the Internet.
   */
  qClass: Uint8Array;

  constructor(qName: Uint8Array, qType: Uint8Array, qClass: Uint8Array) {
    this.qName = qName;
    this.qType = qType;
    this.qClass = qClass;
  }
}
