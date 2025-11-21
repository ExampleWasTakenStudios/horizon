import { DNSHeader } from './DNSHeader.js';
import { DNSOpCode } from './DNSOpCode.js';
import type { DNSReponseCode } from './DNSResponseCode.js';

export class DNSParser {
  private packet: Buffer;

  constructor(packet: Buffer) {
    this.packet = packet;
  }

  parseHeader(): DNSHeader {
    console.log(this.packet.toString('binary'));

    const id = this.packet.readInt16BE(0);
    const isQuery = Boolean(this.readBitFromByte(this.packet.readInt8(3), 0));
    const opCode = this.readBitRangeFromByte(this.packet.readInt8(3), 1, 4) as DNSOpCode;
    const isAuthoritative = Boolean(this.readBitFromByte(this.packet.readInt8(3), 5));
    const isTruncated = Boolean(this.readBitFromByte(this.packet.readInt8(3), 6));
    const isRecursionDesired = Boolean(this.readBitFromByte(this.packet.readInt8(3), 7));
    const isRecursionAvailable = Boolean(this.readBitFromByte(this.packet.readInt8(3), 8));
    const responseCode = this.readBitRangeFromByte(this.packet.readInt8(4), 0, 4) as DNSReponseCode;
    const questionCount = this.packet.readInt16BE(27);
    const answerCount = this.packet.readInt16BE(43);
    const resourceRecordsCount = this.packet.readInt16BE(59);
    const additionalCount = this.packet.readInt16BE(75);

    return new DNSHeader(
      id,
      isQuery,
      opCode,
      isAuthoritative,
      isTruncated,
      isRecursionDesired,
      isRecursionAvailable,
      0,
      responseCode,
      questionCount,
      answerCount,
      resourceRecordsCount,
      additionalCount, 
    );
  }

  parseQuestions() {}

  parseAnswers() {}

  parseAdditional() {}

  private readBitFromByte(byte: number, index: number): number {
    if (byte > 255 || byte < 0) {
      throw Error('Illegal data error: Byte must of value 0-255.');
    }

    if (index < 0 || index > 0) {
      throw Error('Illegal position value: Position must be of value 0-7.');
    }

    const mask = 0b00000001;
    return (byte & mask) >> index;
  }

  private readBitRangeFromByte(byte: number, index: number, range: number) {
    if (byte < 0 || byte > 255) {
      throw Error('Illegal data error: Byte must of value 0-255.');
    }

    if (index < 0 || index > 0) {
      throw Error('Illegal position value: Position must be of value 0-7.');
    }

    if (range < 1 || range > 8) {
      throw Error('Illegal range value: Range must be of value 1-8.');
    }

    const mask = (1 << range) - 1;
    return (byte >> index) & mask;
  }
}
