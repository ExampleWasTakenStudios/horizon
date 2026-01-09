import punycode from '@dcoffey-zengenti/punynode';
import type { ReturnResult } from '../../../../types/ReturnResult.js';
import { err, ok } from '../../../../utils/return-type-helpers.js';
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
  parse(message: Buffer): ReturnResult<DNSMessage> {
    const buffer = new CursorBuffer(message);

    // Parse Header

    /**
     * This is only the initial header as, at this point, we don't know if the message includes an EDNS OPT record that extends the RCODE field.
     * Once the message is fully parsed, a new header is instantiated that receives all values of the received header, plus the extended RCODE.
     */
    const initialHeaderResult = this.parseHeader(buffer);
    if (!initialHeaderResult.success) {
      return err(initialHeaderResult);
    }

    const questionsResult = this.parseQuestions(buffer, initialHeaderResult.data.questionCount);
    if (!questionsResult.success) {
      return err(questionsResult);
    }

    const answersResult = this.parseResourceRecord(buffer, initialHeaderResult.data.answerCount);
    if (!answersResult.success) {
      return err(answersResult);
    }

    const authoritativeResult = this.parseResourceRecord(buffer, initialHeaderResult.data.authoritativeCount);
    if (!authoritativeResult.success) {
      return err(authoritativeResult);
    }

    const additionalResult = this.parseResourceRecord(buffer, initialHeaderResult.data.additionalCount);
    if (!additionalResult.success) {
      return err(additionalResult);
    }

    // Check if the received message supports EDNS -> if yet, use extended RCODE directly
    const optRecord = additionalResult.data.find((record) => record.type === DNS_TYPES.OPT);
    let finalHeader: DNSHeader;
    if (optRecord) {
      const header = initialHeaderResult.data;
      const optData = optRecord.data as OptData;
      const computedRcode = this.computeExtendedRCode(header.responseCode, optData.extendedRcode);

      if (!computedRcode.success) {
        return err(computedRcode);
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
        computedRcode.data,
        header.questionCount,
        header.answerCount,
        header.authoritativeCount,
        header.additionalCount
      );
    } else {
      finalHeader = initialHeaderResult.data;
    }

    const dnsMessage = new DNSMessage(
      finalHeader,
      questionsResult.data,
      answersResult.data,
      authoritativeResult.data,
      additionalResult.data
    );

    return ok(dnsMessage);
  }

  private parseHeader(buffer: CursorBuffer): ReturnResult<DNSHeader> {
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

    return ok(
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

  private parseQuestions(buffer: CursorBuffer, questionCount: number): ReturnResult<DNSQuestion[]> {
    const questions: DNSQuestion[] = [];

    for (let i = 0; i < questionCount; i++) {
      const qName = this.parseLabels(buffer);

      if (!qName.success) {
        return err(qName);
      }

      const qType = buffer.readNextUint16();
      const qClass = buffer.readNextUint16();

      questions.push(new DNSQuestion(qName.data, qType, qClass));
    }

    return ok(questions);
  }

  private parseResourceRecord(buffer: CursorBuffer, resourceRecordCount: number): ReturnResult<DNSRecord[]> {
    const resourceRecords: DNSRecord[] = [];

    for (let i = 0; i < resourceRecordCount; i++) {
      const name = this.parseLabels(buffer);
      if (!name.success) {
        return err(name);
      }

      const rrType = buffer.readNextUint16();
      const rrClass = buffer.readNextUint16();
      const ttl = buffer.readNextUint32();
      const rdLength = buffer.readNextUint16();

      let data: ReturnResult<RecordData>;
      if (rrType === (DNS_TYPES.OPT as number)) {
        data = this.parseOptData(buffer, rdLength, ttl, rrClass);
      } else {
        data = this.parseRData(buffer, rdLength, rrType);
      }

      if (!data.success) {
        return err(data);
      }

      resourceRecords.push(new DNSRecord(name.data, rrType, rrClass, ttl, rdLength, data.data));
    }

    return ok(resourceRecords);
  }

  private parseLabels(buffer: CursorBuffer, visitedPointers: Set<number> = new Set()): ReturnResult<string[]> {
    const labels: string[] = [];

    if (buffer.getRemaining() < 1) {
      return err({
        rCode: DNS_RESPONSE_CODES.FORMERR,
        debugInfo: ['Less than one byte remaining in buffer. Buffer: ', buffer],
      });
    }

    while (true) {
      const currentByte = buffer.readNextUint8();

      // Check if current byte is a pointer
      if ((currentByte & 0xc0) === 0xc0) {
        const pointer = this.decodePointer(currentByte, buffer.readNextUint8());
        // Check for pointer loop
        if (visitedPointers.has(pointer)) {
          return err({
            rCode: DNS_RESPONSE_CODES.FORMERR,
            debugInfo: ['Pointer loop in buffer ', buffer, ' Pointer: ', pointer],
          });
        }

        visitedPointers.add(pointer);
        const labelResult = this.parseLabels(buffer.fork(pointer), visitedPointers);
        if (!labelResult.success) {
          return err(labelResult);
        }

        labels.push(...labelResult.data);
        break;
      }

      // Check if currentByte is a zero terminator (0x00)
      if (currentByte === 0) {
        break;
      }

      // currentByte must be a length label
      const length = currentByte;
      if (length > buffer.getRemaining()) {
        return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'Name label length exceeds buffer length' });
      }

      const label = punycode.toUnicode(buffer.nextSubarray(length).toString('ascii'));

      labels.push(label);
    }

    return ok(labels);
  }

  private parseCharString(buffer: CursorBuffer): ReturnResult<string> {
    const length = buffer.readNextUint8();
    if (length > buffer.getRemaining()) {
      return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'Char string length exceeds buffer length.' });
    }

    const text = buffer.nextSubarray(length);

    return ok(punycode.toUnicode(text.toString('ascii')));
  }

  private parseOptData(buffer: CursorBuffer, rdLength: number, ttl: number, rrClass: number): ReturnResult<OptData> {
    const extendedRcode = (ttl >> 24) & 0xff;
    const version = (ttl >> 16) & 0xff;
    if (version !== 0) {
      return err({ rCode: DNS_RESPONSE_CODES.BADVERS, debugInfo: ['Received EDNS version ', version] });
    }

    const doBit = !!((ttl >> 15) & 0xff);

    // Parse OPT-Data (RDATA)
    const options: EDNSOption[] = [];
    while (buffer.getCursorPosition() < rdLength) {
      const optionCode = buffer.readNextUint16();
      const optionLength = buffer.readNextUint16();
      if (optionLength > buffer.getRemaining()) {
        return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'EDNS Option Data exceeds buffer length.' });
      }
      const optionData = buffer.nextSubarray(optionLength);

      options.push({ code: optionCode, data: optionData });
    }

    return ok(new OptData(rrClass, extendedRcode, version, doBit, options));
  }

  private parseRData(
    buffer: CursorBuffer,
    rdLength: number,
    rrType: DNS_TYPES
  ): ReturnResult<AData | DomainName_Data | MxData | RawData | SOA_Data | TxtData> {
    // Declare start position of RDATA to ensure we don't read more than defined as RDLENGTH.
    const startCursorPosition = buffer.getCursorPosition();

    // Check that rdLength is no longer than the buffer
    if (buffer.getRemaining() < rdLength) {
      return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'RDLENGTH exceeds buffer length.' });
    }

    switch (rrType) {
      case DNS_TYPES.A: {
        if (rdLength !== 4) {
          return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: "RDLENGTH of RRs of type 'A' must be 4." });
        }

        const slice = buffer.nextSubarray(4);
        return ok(new AData(`${slice[0]}.${slice[1]}.${slice[2]}.${slice[3]}`));
      }
      case DNS_TYPES.CNAME:
      case DNS_TYPES.NS:
      case DNS_TYPES.MX: {
        const preference = buffer.readNextUint16();
        const exchange = this.parseLabels(buffer);
        if (!exchange.success) {
          return err(exchange);
        }

        return ok(new MxData(preference, exchange.data));
      }
      case DNS_TYPES.SOA: {
        const mName = this.parseLabels(buffer);
        if (!mName.success) {
          return err(mName);
        }

        const rName = this.parseLabels(buffer);
        if (!rName.success) {
          return err(rName);
        }

        const serial = buffer.readNextUint32();
        const refresh = buffer.readNextUint32();
        const retry = buffer.readNextUint32();
        const expire = buffer.readNextUint32();
        const minimum = buffer.readNextUint32();

        return ok(new SOA_Data(mName.data, rName.data, serial, refresh, retry, expire, minimum));
      }
      case DNS_TYPES.TXT: {
        const text: string[] = [];
        do {
          const parsedText = this.parseCharString(buffer);
          if (!parsedText.success) {
            return err(parsedText);
          }

          text.push(parsedText.data);
        } while (buffer.getCursorPosition() <= startCursorPosition + rdLength);

        return ok(new TxtData(text));
      }
      default: {
        return ok(new RawData(rrType, buffer.nextSubarray(rdLength)));
      }
    }
  }

  private decodePointer(highByte: number, lowByte: number): number {
    return ((highByte & 0x3f) << 8) | lowByte;
  }

  private computeExtendedRCode(baseRCode: number, extendedRCode: number): ReturnResult<DNS_RESPONSE_CODES> {
    if (baseRCode < 0 || baseRCode > 15) {
      return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'Base RCODE must be 0-15' });
    }
    if (extendedRCode < 0 || extendedRCode > 255) {
      return err({ rCode: DNS_RESPONSE_CODES.FORMERR, debugInfo: 'Exteded RCODE must be 0-255' });
    }

    return ok((extendedRCode << 4) | baseRCode);
  }
}
