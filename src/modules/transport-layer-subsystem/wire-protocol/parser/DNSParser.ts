import punycode from '@dcoffey-zengenti/punynode';
import { DNSParseError } from '../../../../errors/result/DNSParseError.js';
import { Result, type TResult } from '../../../../result/Result.js';
import { DNS_RESPONSE_CODES } from '../DNS-core/constants/DNS_RESPONSE_CODES.js';
import { DNS_TYPES } from '../DNS-core/constants/DNS_TYPES.js';
import { DNSHeader } from '../DNS-core/DNSHeader.js';
import { DNSMessage } from '../DNS-core/DNSMessage.js';
import { DNSQuestion } from '../DNS-core/DNSQuestion.js';
import { DNSRecord } from '../DNS-core/resource-records/DNSRecord.js';
import { AData } from '../DNS-core/resource-records/RDATA/AData.js';
import { DomainName_Data } from '../DNS-core/resource-records/RDATA/DomainNameData.js';
import { MxData } from '../DNS-core/resource-records/RDATA/MxData.js';
import { OptData, type EDNSOption } from '../DNS-core/resource-records/RDATA/OptData.js';
import { RawData } from '../DNS-core/resource-records/RDATA/RawData.js';
import type { RecordData } from '../DNS-core/resource-records/RDATA/RecordData.js';
import { SOA_Data } from '../DNS-core/resource-records/RDATA/SOA_Data.js';
import { TxtData } from '../DNS-core/resource-records/RDATA/TxtData.js';
import { CursorBuffer } from './CursorBuffer.js';

export class DNSParser {
  public parse(message: Buffer): TResult<DNSMessage, DNSParseError> {
    const buffer = new CursorBuffer(message);

    /* --- Parse Header --- */

    /*
     * This is only the initial header as, at this point, we don't know if the message includes an EDNS OPT record that extends the RCODE field.
     * Once the message is fully parsed, a new header is instantiated that receives all values of the received header, plus the extended RCODE.
     */
    const initialHeaderResult = this.parseHeader(buffer);
    if (initialHeaderResult.isFailure()) {
      return initialHeaderResult;
    }

    const questionsResult = this.parseQuestions(buffer, initialHeaderResult.value.questionCount);
    if (questionsResult.isFailure()) {
      return questionsResult;
    }

    const answersResult = this.parseResourceRecord(buffer, initialHeaderResult.value.answerCount);
    if (answersResult.isFailure()) {
      return answersResult;
    }

    const authoritativeResult = this.parseResourceRecord(buffer, initialHeaderResult.value.authoritativeCount);
    if (authoritativeResult.isFailure()) {
      return authoritativeResult;
    }

    const additionalResult = this.parseResourceRecord(buffer, initialHeaderResult.value.additionalCount);
    if (additionalResult.isFailure()) {
      return additionalResult;
    }

    // Check if the received message supports EDNS -> if yet, use extended RCODE directly
    const optRecord = additionalResult.value.find((record) => record.type === DNS_TYPES.OPT);
    let finalHeader: DNSHeader;
    if (optRecord) {
      const header = initialHeaderResult.value;

      // This can safely disabled as `optRecord` is guaranteed to be of type OPT since it is the requirement specified in the predicate of the find function above.
      // eslint-disable-next-line @typescript-eslint/no-unsafe-type-assertion
      const optData = optRecord.data as OptData;
      const computedRcodeResult = this.computeExtendedRCode(header.responseCode, optData.extendedRcode);

      if (computedRcodeResult.isFailure()) {
        return computedRcodeResult;
      }

      finalHeader = new DNSHeader(
        header.id,
        header.isQuery,
        header.opCode,
        header.isAuthoritative,
        header.isTruncated,
        header.isRecursionDesired,
        header.isRecursionAvailable,
        header.z,
        computedRcodeResult.value,
        header.questionCount,
        header.answerCount,
        header.authoritativeCount,
        header.additionalCount
      );
    } else {
      finalHeader = initialHeaderResult.value;
    }

    const dnsMessage = new DNSMessage(
      finalHeader,
      questionsResult.value,
      answersResult.value,
      authoritativeResult.value,
      additionalResult.value
    );

    return Result.ok(dnsMessage);
  }

  private parseHeader(buffer: CursorBuffer): TResult<DNSHeader, DNSParseError> {
    const id = buffer.readNextUint16();
    const flags = buffer.readNextUint16();

    /**
      A one bit field that specifies whether this message is a query (0), or a response (1).
     */
    const qr = Boolean((flags & 0b1000000000000000) >> 15);

    /**
      A four bit field that specifies kind of query in this
      message.  This value is set by the originator of a query
      and copied into the response.  The values are:
      0               a standard query (QUERY)
      1               an inverse query (IQUERY)
      2               a server status request (STATUS)
      3-15            reserved for future use
     */
    const opCode = (flags & 0b0111100000000000) >> 11;
    /**
      Authoritative Answer - this bit is valid in responses,
      and specifies that the responding name server is an
      authority for the domain name in question section.

      Note that the contents of the answer section may have
      multiple owner names because of aliases.  The AA bit
      corresponds to the name which matches the query name, or
      the first owner name in the answer section.
     */
    const aa = Boolean((flags & 0b0000010000000000) >> 10);

    /**
      TrunCation - specifies that this message was truncated
      due to length greater than that permitted on the
      transmission channel.
     */
    const tc = Boolean((flags & 0b0000001000000000) >> 9);

    /**
      Recursion Desired - this bit may be set in a query and
      is copied into the response.  If RD is set, it directs
      the name server to pursue the query recursively.
      Recursive query support is optional.
     */
    const rd = Boolean((flags & 0b0000000100000000) >> 8);

    /*
      Recursion Available - this be is set or cleared in a
      response, and denotes whether recursive query support is
      available in the name server.
    */
    const ra = Boolean((flags & 0b0000000010000000) >> 7);

    /**
      Reserved for future use.  Must be zero in all queries
      and responses.
     */
    const z = (flags & 0b0000000001110000) >> 4;

    /**
      Response code - this 4 bit field is set as part of
      responses.  The values have the following
      interpretation:

      0               No error condition

      1               Format error - The name server was
                      unable to interpret the query.

      2               Server failure - The name server was
                      unable to process this query due to a
                      problem with the name server.

      3               Name Error - Meaningful only for
                      responses from an authoritative name
                      server, this code signifies that the
                      domain name referenced in the query does
                      not exist.

      4               Not Implemented - The name server does
                      not support the requested kind of query.

      5               Refused - The name server refuses to
                      perform the specified operation for
                      policy reasons.  For example, a name
                      server may not wish to provide the
                      information to the particular requester,
                      or a name server may not wish to perform
                      a particular operation (e.g., zone

     */
    const rCode = flags & 0b0000000000001111;

    const questionCount = buffer.readNextUint16();
    const answerCount = buffer.readNextUint16();
    const authoritativeCount = buffer.readNextUint16();
    const additionalCount = buffer.readNextUint16();

    return Result.ok(
      new DNSHeader(
        id,
        !qr,
        opCode,
        aa,
        tc,
        rd,
        ra,
        z,
        rCode,
        questionCount,
        answerCount,
        authoritativeCount,
        additionalCount
      )
    );
  }

  private parseQuestions(buffer: CursorBuffer, questionCount: number): TResult<DNSQuestion[], DNSParseError> {
    const questions: DNSQuestion[] = [];

    for (let i = 0; i < questionCount; i++) {
      const qNameResult = this.parseLabels(buffer);

      if (qNameResult.isFailure()) {
        return qNameResult;
      }

      const qType = buffer.readNextUint16();
      const qClass = buffer.readNextUint16();

      questions.push(new DNSQuestion(qNameResult.value, qType, qClass));
    }

    return Result.ok(questions);
  }

  private parseResourceRecord(buffer: CursorBuffer, resourceRecordCount: number): TResult<DNSRecord[], DNSParseError> {
    const resourceRecords: DNSRecord[] = [];

    for (let i = 0; i < resourceRecordCount; i++) {
      const nameResult = this.parseLabels(buffer);
      if (nameResult.isFailure()) {
        return nameResult;
      }

      const rrType = buffer.readNextUint16();
      const rrClass = buffer.readNextUint16();
      const ttl = buffer.readNextUint32();
      const rdLength = buffer.readNextUint16();

      let value: TResult<RecordData, DNSParseError>;
      if (rrType === (DNS_TYPES.OPT as number)) {
        value = this.parseOptData(buffer, rdLength, ttl, rrClass);
      } else {
        value = this.parseRData(buffer, rdLength, rrType);
      }

      if (value.isFailure()) {
        return value;
      }

      resourceRecords.push(new DNSRecord(nameResult.value, rrType, rrClass, ttl, rdLength, value.value));
    }

    return Result.ok(resourceRecords);
  }

  private parseLabels(buffer: CursorBuffer, visitedPointers = new Set<number>()): TResult<string[], DNSParseError> {
    const labels: string[] = [];

    if (buffer.getRemaining() < 1) {
      return Result.fail(new DNSParseError('Less than one byte remaining in buffer.', DNS_RESPONSE_CODES.FORMERR));
    }

    for (;;) {
      const currentByte = buffer.readNextUint8();

      // Check if current byte is a pointer
      if ((currentByte & 0xc0) === 0xc0) {
        const pointer = this.decodePointer(currentByte, buffer.readNextUint8());
        // Check for pointer loop
        if (visitedPointers.has(pointer)) {
          return Result.fail(new DNSParseError('Pointer loop detected.', DNS_RESPONSE_CODES.FORMERR));
        }

        visitedPointers.add(pointer);
        const labelResult = this.parseLabels(buffer.fork(pointer), visitedPointers);
        if (labelResult.isFailure()) {
          return labelResult;
        }

        labels.push(...labelResult.value);
        break;
      }

      // Check if currentByte is a zero terminator (0x00)
      if (currentByte === 0) {
        break;
      }

      // currentByte must be a length label
      const length = currentByte;
      if (length > buffer.getRemaining()) {
        return Result.fail(new DNSParseError('Name label length exceeds buffer length.', DNS_RESPONSE_CODES.FORMERR));
      }

      const label = punycode.toUnicode(buffer.nextSubarray(length).toString('ascii'));

      labels.push(label);
    }

    return Result.ok(labels);
  }

  private parseCharString(buffer: CursorBuffer): TResult<string, DNSParseError> {
    const length = buffer.readNextUint8();
    if (length > buffer.getRemaining()) {
      return Result.fail(new DNSParseError('Char string length exceeds buffer length.', DNS_RESPONSE_CODES.FORMERR));
    }

    const text = buffer.nextSubarray(length);

    return Result.ok(punycode.toUnicode(text.toString('ascii')));
  }

  private parseOptData(
    buffer: CursorBuffer,
    rdLength: number,
    ttl: number,
    rrClass: number
  ): TResult<OptData, DNSParseError> {
    const extendedRcode = (ttl >> 24) & 0xff;
    const version = (ttl >> 16) & 0xff;
    if (version !== 0) {
      return Result.fail(
        new DNSParseError(`Received unsupported EDNS version ${version.toString()}.`, DNS_RESPONSE_CODES.BADVERS)
      );
    }

    const doBit = !!((ttl >> 15) & 0xff);

    // Parse OPT-Data (RDATA)
    const options: EDNSOption[] = [];
    while (buffer.getCursorPosition() < rdLength) {
      const optionCode = buffer.readNextUint16();
      const optionLength = buffer.readNextUint16();
      if (optionLength > buffer.getRemaining()) {
        return Result.fail(new DNSParseError('EDNS Option Data exceeds buffer length.', DNS_RESPONSE_CODES.FORMERR));
      }
      const optionData = buffer.nextSubarray(optionLength);

      options.push({ code: optionCode, data: optionData });
    }

    return Result.ok(new OptData(rrClass, extendedRcode, version, doBit, options));
  }

  private parseRData(
    buffer: CursorBuffer,
    rdLength: number,
    rrType: DNS_TYPES
  ): TResult<AData | DomainName_Data | MxData | RawData | SOA_Data | TxtData, DNSParseError> {
    // Check that rdLength is no longer than the buffer
    if (buffer.getRemaining() < rdLength) {
      return Result.fail(new DNSParseError('RDLENGTH exceeds buffer length.', DNS_RESPONSE_CODES.FORMERR));
    }

    switch (rrType) {
      case DNS_TYPES.A: {
        return this.handleAData(rdLength, buffer);
      }
      case DNS_TYPES.CNAME:
      case DNS_TYPES.NS: {
        return this.handleDomainNameData(rrType, buffer);
      }
      case DNS_TYPES.MX: {
        return this.handleMxData(buffer);
      }
      case DNS_TYPES.SOA: {
        return this.handleSoaData(buffer);
      }
      case DNS_TYPES.TXT: {
        return this.handleTxtData(rdLength, buffer);
      }
      case DNS_TYPES.OPT: // Technically redundant but added for clarity
      default: {
        return Result.ok(new RawData(rrType, buffer.nextSubarray(rdLength)));
      }
    }
  }

  private handleAData(rdLength: number, buffer: CursorBuffer): TResult<AData, DNSParseError> {
    if (rdLength !== 4) {
      return Result.fail(new DNSParseError("RDLENGTH of RRs of type 'A' must be 4.", DNS_RESPONSE_CODES.FORMERR));
    }

    const [b0 = undefined, b1 = undefined, b2 = undefined, b3 = undefined] = buffer.nextSubarray(4);
    if (b0 === undefined || b1 === undefined || b2 === undefined || b3 === undefined) {
      return Result.fail(
        new DNSParseError('One or more octets were `undefined` in RDATA of A RR.', DNS_RESPONSE_CODES.FORMERR)
      );
    }

    return Result.ok(new AData(`${b0.toString()}.${b1.toString()}.${b2.toString()}.${b3.toString()}`));
  }

  private handleDomainNameData(type: DNS_TYPES, buffer: CursorBuffer): TResult<DomainName_Data, DNSParseError> {
    const labelResult = this.parseLabels(buffer);
    if (labelResult.isFailure()) {
      return labelResult;
    }

    return Result.ok(new DomainName_Data(type, labelResult.value));
  }

  private handleMxData(buffer: CursorBuffer): TResult<MxData, DNSParseError> {
    const preference = buffer.readNextUint16();
    const exchangeResult = this.parseLabels(buffer);
    if (exchangeResult.isFailure()) {
      return exchangeResult;
    }

    return Result.ok(new MxData(preference, exchangeResult.value));
  }

  private handleSoaData(buffer: CursorBuffer): TResult<SOA_Data, DNSParseError> {
    const mNameResult = this.parseLabels(buffer);
    if (mNameResult.isFailure()) {
      return mNameResult;
    }

    const rNameResult = this.parseLabels(buffer);
    if (rNameResult.isFailure()) {
      return rNameResult;
    }

    const serial = buffer.readNextUint32();
    const refresh = buffer.readNextUint32();
    const retry = buffer.readNextUint32();
    const expire = buffer.readNextUint32();
    const minimum = buffer.readNextUint32();

    return Result.ok(new SOA_Data(mNameResult.value, rNameResult.value, serial, refresh, retry, expire, minimum));
  }

  private handleTxtData(rdLength: number, buffer: CursorBuffer): TResult<TxtData, DNSParseError> {
    // Declare start position of RDATA to ensure we don't read more than defined as RDLENGTH.
    const startCursorPosition = buffer.getCursorPosition();

    const text: string[] = [];
    do {
      const parsedTextResult = this.parseCharString(buffer);
      if (parsedTextResult.isFailure()) {
        return parsedTextResult;
      }

      text.push(parsedTextResult.value);
    } while (buffer.getCursorPosition() <= startCursorPosition + rdLength);

    return Result.ok(new TxtData(text));
  }

  private decodePointer(highByte: number, lowByte: number): number {
    return ((highByte & 0x3f) << 8) | lowByte;
  }

  private computeExtendedRCode(baseRCode: number, extendedRCode: number): TResult<DNS_RESPONSE_CODES, DNSParseError> {
    if (baseRCode < 0 || baseRCode > 15) {
      return Result.fail(new DNSParseError('Base RCODE must be between 0-15.', DNS_RESPONSE_CODES.FORMERR));
    }
    if (extendedRCode < 0 || extendedRCode > 255) {
      return Result.fail(new DNSParseError('Extended RCODE must be between 0-255.', DNS_RESPONSE_CODES.FORMERR));
    }

    return Result.ok((extendedRCode << 4) | baseRCode);
  }
}
