import type { DNS_OP_CODES } from './constants/DNS_OP_CODES.js';
import type { DNS_RESPONSE_CODES } from './constants/DNS_REPONSE_CODES.js';

export class DNSHeader {
  /**
    A 16 bit identifier assigned by the program that
    generates any kind of query.  This identifier is copied
    the corresponding reply and can be used by the requester
    to match up replies to outstanding queries.
   */
  id: number;
  /**
    A one bit field that specifies whether this message is a
    query (0), or a response (1).
   */
  isQuery: boolean;
  /**
    A four bit field that specifies kind of query in this
    message.  This value is set by the originator of a query
    and copied into the response.  The values are:

    0       a standard query (QUERY)
    1       an inverse query (IQUERY)
    2       a server status request (STATUS)
    3-15    reserved for future use
   */
  opCode: typeof DNS_OP_CODES;
  /**
    Authoritative Answer - this bit is valid in responses,
    and specifies that the responding name server is an
    authority for the domain name in question section.

    Note that the contents of the answer section may have
    multiple owner names because of aliases.  The AA bit
    corresponds to the name which matches the query name, or
    the first owner name in the answer section.
   */
  isAuthoritative: boolean;
  /**
    TrunCation - specifies that this message was truncated
    due to length greater than that permitted on the
    transmission channel.
   */
  isTruncated: boolean;
  /**
    Recursion Desired - this bit may be set in a query and
    is copied into the response.  If RD is set, it directs
    the name server to pursue the query recursively.
    Recursive query support is optional.
   */
  isRecursionDesired: boolean;
  /**
    Recursion Available - this be is set or cleared in a
    response, and denotes whether recursive query support is
    available in the name server.
   */
  isRecursionAvailable: boolean;
  /**
    Reserved for future use.  Must be zero in all queries
    and responses.
   */
  z: number;
  /**
    Response code - this 4 bit field is set as part of
    responses.  The values have the following
    interpretation:
    0    No error condition
    1    Format error - The name server was
         unable to interpret the query.
    2    Server failure - The name server was
         unable to process this query due to a
         problem with the name server.
    3    Name Error - Meaningful only for
         responses from an authoritative name
         server, this code signifies that the
         domain name referenced in the query does
         not exist.
    4    Not Implemented - The name server does
         not support the requested kind of query.
    5    Refused - The name server refuses to
         perform the specified operation for
         policy reasons.  For example, a name
         server may not wish to provide the
         information to the particular requester,
         or a name server may not wish to perform
         a particular operation (e.g., zone
         transfer) for particular data.
    6-15 Reserved for future use.
   */
  responseCode: typeof DNS_RESPONSE_CODES;
  /**
    An unsigned 16 bit integer specifying the number of
    entries in the question section.
   */
  questionCount: number;
  /**
    An unsigned 16 bit integer specifying the number of
    entries in the answer section.
   */
  answerCount: number;
  /**
    An unsigned 16 bit integer specifying the number of name
    server resource records in the authority records
    section.
   */
  resourceRecordsCount: number;
  /**
    An unsigned 16 bit integer specifying the number of
    resource records in the additional records section.
   */
  additionalCount: number;

  constructor(
    id: number,
    isQuery: boolean,
    opCode: typeof DNS_OP_CODES,
    isAuthoritative: boolean,
    isTruncated: boolean,
    isRecursionDesired: boolean,
    isRecursionAvailable: boolean,
    z: number,
    responseCode: typeof DNS_RESPONSE_CODES,
    questionCount: number,
    answerCount: number,
    resourceRecordsCount: number,
    additionalCount: number
  ) {
    this.id = id;
    this.isQuery = isQuery;
    this.opCode = opCode;
    this.isAuthoritative = isAuthoritative;
    this.isTruncated = isTruncated;
    this.isRecursionDesired = isRecursionDesired;
    this.isRecursionAvailable = isRecursionAvailable;
    this.z = z;
    this.responseCode = responseCode;
    this.questionCount = questionCount;
    this.answerCount = answerCount;
    this.resourceRecordsCount = resourceRecordsCount;
    this.additionalCount = additionalCount;
  }
}
