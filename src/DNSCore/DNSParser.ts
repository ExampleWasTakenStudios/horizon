import { DNSHeader } from './DNSHeader.js';
import { DNSPacket } from './DNSPacket.js';
import { DNSQuestion } from './DNSQuestion.js';
import { ResourceRecord } from './ResourceRecord.js';

export interface DNSQNameResult {
  labels: string[];
  bytesConsumed: number;
}

export class DNSParser {
  private packet: Buffer;
  private header: DNSHeader;
  private questions: DNSQuestion[];
  private answers: ResourceRecord[];
  private authority: ResourceRecord[];
  private additional: ResourceRecord[];

  private dnsPacket: DNSPacket;

  constructor(packet: Buffer) {
    this.packet = packet;

    this.header = this.parseHeader();
    this.questions = this.parseQuestions();
    this.answers = this.parseAnswers();
    this.authority = this.parseAuthoritiy();
    this.additional = this.parseAdditional();

    this.dnsPacket = new DNSPacket(
      this.header,
      this.questions,
      this.answers,
      this.authority,
      this.additional
    );
  }

  getDNSPacket(): DNSPacket {
    return this.dnsPacket;
  }

  private parseHeader(): DNSHeader {
    const id = this.packet.readUint16BE(0);
    const flags = this.packet.readUint16BE(2);

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

    const qdCount = this.packet.readUInt16BE(4);
    const anCount = this.packet.readUInt16BE(6);
    const nsCount = this.packet.readUInt16BE(8);
    const arCount = this.packet.readUInt16BE(10);

    return new DNSHeader(
      id,
      !!!qr,
      opCode,
      !!aa,
      !!tc,
      !!rd,
      !!ra,
      z,
      rCode,
      qdCount,
      anCount,
      nsCount,
      arCount
    );
  }

  private parseQuestions(): DNSQuestion[] {
    const questions: DNSQuestion[] = [];

    let questionOffset = 12; // the offset in the DNS message at which a new question starts. this is initially 12 as DNS headers are 12 bytes long.
    for (let i = 0; i < this.header.questionCount; i++) {
      const qNameLabels = this.parseQNameLabels(questionOffset);
      const qType = this.parseQType(questionOffset + qNameLabels.bytesConsumed);
      const qClass = this.parseQClass(
        questionOffset + qNameLabels.bytesConsumed + 2
      );

      questions.push(new DNSQuestion(qNameLabels.labels, qType, qClass));
      questionOffset += qNameLabels.bytesConsumed + 4; // question offset: start of the question; qNameLabels.bytesConsumed: number of bytes used for the QNAME section; QTYPE + QCLASS 2 bytes each.
    }

    return questions;
  }

  private parseAnswers(): ResourceRecord[] {
    return [];
  }

  private parseAuthoritiy(): ResourceRecord[] {
    return [];
  }

  private parseAdditional(): ResourceRecord[] {
    return [];
  }

  /**
   * Parse a QNAME section of a DNS message. This method supports compression pointers by recusively walking the QNAME section.
   * @param startOffset The offset of the DNS message at which to start parsing.
   * @param visitedPointers A set with already visited pointers. This defaults to an empty set and should only be used by recursive calls to prevent pointer loops.
   * @returns The parsed labels and how many bytes of the DNS message were consumed.
   */
  private parseQNameLabels(
    startOffset: number,
    visitedPointers: Set<number> = new Set<number>()
  ): DNSQNameResult {
    const labels: string[] = [];
    let offset = startOffset;

    while (true) {
      // Current byte will usually be the length label. However, because it can also be a pointer, it is named "currentByte" to reflect that possibility.
      let currentByte = this.packet.readUint8(offset);

      // Check if current byte is a pointer
      if ((currentByte & 0xc0) == 0xc0) {
        const pointer = this.decodePointer(offset);

        if (visitedPointers.has(pointer)) {
          throw new Error(`Pointer loop detected at ${pointer}`);
        }

        visitedPointers.add(pointer);
        labels.push(...this.parseQNameLabels(pointer, visitedPointers).labels);

        offset += 2;
        break;
      }

      // Check if current byte is a zero terminator
      if (currentByte === 0) {
        offset++;
        break;
      }

      // Current byte is a length label
      const length = currentByte;
      offset++;
      const label = this.packet
        .subarray(offset, offset + length)
        .toString('ascii');

      labels.push(label);
      offset += length;
    }

    return {
      labels: labels,
      bytesConsumed: offset - startOffset,
    };
  }

  private parseQType(startOffset: number): number {
    return this.packet.readUint16BE(startOffset);
  }

  private parseQClass(startOffset: number): number {
    return this.packet.readUint16BE(startOffset);
  }

  /**
   * Decode a 14-bit pointer in a DNS message.
   * @param offset The offset at which the pointer is located in the DNS message
   * @returns An unsigned 16-bit integer containing the pointer value.
   */
  private decodePointer(offset: number): number {
    const high = (this.packet.readUint8(offset) & 0x3f) << 8;
    const low = this.packet.readUint8(offset + 1);

    return high | low;
  }
}
