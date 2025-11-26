import { UnknownRRTypeError } from '../../../errors/UnknownRRTypeError.js';
import { DNS_CLASSES, type DNS_QCLASSES } from '../DNS-core/constants/DNS_CLASSES.js';
import { DNS_TYPES } from '../DNS-core/constants/DNS_TYPES.js';
import { DNSHeader } from '../DNS-core/DNSHeader.js';
import { DNSPacket } from '../DNS-core/DNSPacket.js';
import { DNSQuestion } from '../DNS-core/DNSQuestion.js';
import { A_Record } from '../DNS-core/resource-records/A_Record.js';
import { CNAME_Record } from '../DNS-core/resource-records/CNAME_Record.js';
import { NS_Record } from '../DNS-core/resource-records/NS_Record.js';
import type { RDataMap } from '../DNS-core/resource-records/RDataMap.js';
import type { ResourceRecord } from '../DNS-core/resource-records/ResourceRecord.js';
import { SOA_Record } from '../DNS-core/resource-records/SOA_Record.js';
import { Cursor } from './Cursor.js';

export interface DNSNameResult {
  labels: string[];
  bytesConsumed: number;
}

export interface DNSParseResult {
  /**
   * The parsed data.
   */
  data: DNSQuestion[] | ResourceRecord[];
  /**
   * The advanced cursor object.
   */
  cursor: Cursor;
}

/**
 * TODO: refactor cursor logic see https://github.com/ExampleWasTakenStudios/horizon/issues/16
 */
export class DNSParser {
  parse(rawPacket: Buffer): DNSPacket {
    const cursor = new Cursor();

    const header = this.parseHeader(cursor, rawPacket);
    const questions = this.parseQuestions(cursor, rawPacket, header);
    const answers = this.parseResourceRecordSection(cursor, rawPacket, header);
    const authority = this.parseResourceRecordSection(cursor, rawPacket, header);
    const additional = this.parseResourceRecordSection(cursor, rawPacket, header);

    return new DNSPacket(
      header,
      questions.data as DNSQuestion[],
      answers.data as ResourceRecord[],
      authority.data as ResourceRecord[],
      additional.data as ResourceRecord[]
    );
  }

  private parseHeader(cursor: Cursor, rawPacket: Buffer): DNSHeader {
    const id = rawPacket.readUint16BE(0);

    const flags = rawPacket.readUint16BE(2);

    // Extract flag bits
    // prettier-ignore
    const qr =     (flags & 0b1000000000000000) >> 15;
    // prettier-ignore
    const opCode = (flags & 0b0111100000000000) >> 11;
    // prettier-ignore
    const aa =     (flags & 0b0000010000000000) >> 10;
    // prettier-ignore
    const tc =     (flags & 0b0000001000000000) >> 9;
    // prettier-ignore
    const rd =     (flags & 0b0000000100000000) >> 8;
    // prettier-ignore
    const ra =     (flags & 0b0000000010000000) >> 7;
    // prettier-ignore
    const z =      (flags & 0b0000000001110000) >> 4;
    // prettier-ignore
    const rCode =  (flags & 0b0000000000001111);

    const qdCount = rawPacket.readUInt16BE(4);
    const anCount = rawPacket.readUInt16BE(6);
    const nsCount = rawPacket.readUInt16BE(8);
    const arCount = rawPacket.readUInt16BE(10);

    cursor.advance(12);

    return new DNSHeader(id, !qr, opCode, !!aa, !!tc, !!rd, !!ra, z, rCode, qdCount, anCount, nsCount, arCount);
  }

  private parseQuestions(cursor: Cursor, rawPacket: Buffer, header: DNSHeader): DNSQuestion[] {
    const questions: DNSQuestion[] = [];

    for (let i = 0; i < header.questionCount; i++) {
      const qNameLabels = this.parseNameLabels(cursor, rawPacket);
      const qType = this.parseQType(cursor, rawPacket);
      const qClass = this.parseQClass(cursor, rawPacket);

      questions.push(new DNSQuestion(qNameLabels, qType, qClass));
    }

    return questions;
  }

  /**
   * Parse **A SINGLE** resource record section.
   * @param startOffset The start offset in bytes at which to start parsing the RR section.
   * @returns A {@link DNSParseResult} that includes the parsed data as well as the number of bytes this section consumed.
   */
  private parseResourceRecordSection(
    cursor: Cursor,
    rawPacket: Buffer,
    header: DNSHeader
  ): ResourceRecord<RDataMap[DNS_TYPES]>[] {
    const answers: ResourceRecord<RDataMap[DNS_TYPES]>[] = [];

    for (let i = 0; i < this.header.answerCount; i++) {
      const nameLabels = this.parseNameLabels(cursor, rawPacket);

      const type: DNS_TYPES = rawPacket.readUInt16BE(cursor.getPosition());
      cursor.advance(2);

      const RR_class: DNS_CLASSES = rawPacket.readUint16BE(cursor.getPosition());
      cursor.advance(2);

      const ttl = rawPacket.readUInt32BE(cursor.getPosition());
      cursor.advance(4);

      const rdLength = rawPacket.readUint16BE(cursor.getPosition());
      cursor.advance(2);

      const rDataStart = cursor.getPosition();
      const rDataEnd = rDataStart + rdLength;
      const rData = rawPacket.subarray(rDataStart, rDataEnd);
      cursor.advance(rDataEnd - rDataStart);

      this.resourceRecordFactory(nameLabels.join('.'), type, RR_class, ttl, rdLength, rData);
    }

    return answers;
  }

  /**
   * Parse a NAME section of a DNS message. This method supports compression pointers by recusively walking the NAME section.
   * @param cursor - The cursor indicating the offset of the DNS message at which to start parsing.
   * @param rawPacket - The buffer containing the DNS message.
   * @param visitedPointers-  A set with already visited pointers. This defaults to an empty set and should only be used by recursive calls to prevent pointer loops.
   * @returns The parsed labels and how many bytes of the DNS message were consumed.
   */
  private parseNameLabels(
    cursor: Cursor,
    rawPacket: Buffer,
    visitedPointers: Set<number> = new Set<number>()
  ): string[] {
    const labels: string[] = [];

    while (true) {
      // Current byte will usually be the length label. However, because it can also be a pointer, it is named "currentByte" to reflect that possibility.
      const currentByte = rawPacket.readUint8(cursor.getPosition());

      // Check if current byte is a pointer
      if ((currentByte & 0xc0) == 0xc0) {
        const pointer = this.decodePointer(cursor, rawPacket);

        if (visitedPointers.has(pointer.getPosition())) {
          throw new Error(`Pointer loop detected at ${pointer}`);
        }

        visitedPointers.add(pointer.getPosition());
        labels.push(...this.parseNameLabels(pointer, rawPacket, visitedPointers));

        cursor.advance(2);
        break;
      }

      // Check if current byte is a zero terminator
      if (currentByte === 0) {
        cursor.advance(1);
        break;
      }

      // Current byte is a length label
      const length = currentByte;
      cursor.advance(1);
      const label = rawPacket.subarray(cursor.getPosition(), cursor.getPosition() + length).toString('ascii');

      labels.push(label);
      cursor.advance(length);
    }

    return labels;
  }

  private parseQType(cursor: Cursor, rawPacket: Buffer): number {
    const qType = rawPacket.readUint16BE(cursor.getPosition());
    cursor.advance(2);

    return qType;
  }

  private parseQClass(cursor: Cursor, rawPacket: Buffer): DNS_QCLASSES {
    const qClass = rawPacket.readUint16BE(cursor.getPosition());
    cursor.advance(2);

    return qClass;
  }

  /**
   * Decode a 14-bit pointer in a DNS message.
   * @param cursor - Cursor The position of the pointer in the DNS message.
   * @param rawPacket - The buffer containing the DNS message.
   * @returns - A new cursor object representing the target of the pointer.
   */
  private decodePointer(cursor: Cursor, rawPacket: Buffer): Cursor {
    const high = (rawPacket.readUint8(cursor.getPosition()) & 0x3f) << 8;
    const low = rawPacket.readUint8(cursor.getPosition() + 1);

    return new Cursor(high | low);
  }

  private resourceRecordFactory<ReturnType extends keyof RDataMap[DNS_TYPES]>(
    cursor: Cursor,
    rawPacket: Buffer,
    ResourceRecord: ResourceRecord<Buffer>
  ): ResourceRecord<ReturnType> {
    switch (ResourceRecord.type) {
      case DNS_TYPES.A: {
        const octets: string[] = [];

        for (let i = 0; i <= 4; i++) {
          octets.push(ResourceRecord.rData.readUint8(i).toString());
        }

        return new A_Record(
          ResourceRecord.name,
          ResourceRecord.type,
          ResourceRecord.RR_class,
          ResourceRecord.ttl,
          ResourceRecord.rdLength,
          octets.join('.')
        );
      }
      case DNS_TYPES.NS: {
        const domains: string[] = this.parseNameLabels(cursor, rawPacket);

        return new NS_Record(
          ResourceRecord.name,
          ResourceRecord.type,
          ResourceRecord.RR_class,
          ResourceRecord.ttl,
          ResourceRecord.rdLength,
          domains.join('.')
        );
      }
      case DNS_TYPES.CNAME: {
        const domains: string[] = this.parseNameLabels(cursor, rawPacket);

        return new CNAME_Record(
          ResourceRecord.name,
          ResourceRecord.type,
          ResourceRecord.RR_class,
          ResourceRecord.ttl,
          ResourceRecord.rdLength,
          domains.join('.')
        ) as ResourceRecord<ReturnType>;
      }
      case DNS_TYPES.SOA: {
        const mName = this.parseNameLabels(cursor, rawPacket);
        const rName = this.parseNameLabels(cursor, rawPacket);
        const serial = rawPacket.readUint32BE(cursor.getPosition());
        cursor.advance(4);
        const refresh = rawPacket.readUint32BE(cursor.getPosition());
        cursor.advance(4);
        const retry = rawPacket.readUint32BE(cursor.getPosition());
        cursor.advance(4);
        const expire = rawPacket.readUint32BE(cursor.getPosition());
        cursor.advance(4);
        const minimum = rawPacket.readUint32BE(cursor.getPosition());
        cursor.advance(4);

        return new SOA_Record(
          ResourceRecord.name,
          ResourceRecord.type,
          ResourceRecord.RR_class,
          ResourceRecord.ttl,
          ResourceRecord.rdLength,
          {
            mName: mName.join('.'),
            rName: rName.join('.'),
            serial,
            refresh,
            retry,
            expire,
            minimum,
          }
        );
      }
      default: {
        throw new UnknownRRTypeError(`Unknown TYPE ${ResourceRecord.type} in ${JSON.stringify(ResourceRecord)}.`);
      }
    }
  }
}
