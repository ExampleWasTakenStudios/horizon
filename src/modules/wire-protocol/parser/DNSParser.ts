import { decode as decodePuny } from 'punycode';
import { IllegalCharStringError } from '../../../errors/IllegalCharStringError.js';
import { PointerLoopError } from '../../../errors/PointerLoopError.js';
import { DNS_CLASSES } from '../DNS-core/constants/DNS_CLASSES.js';
import { DNS_TYPES } from '../DNS-core/constants/DNS_TYPES.js';
import { DNSHeader } from '../DNS-core/DNSHeader.js';
import { DNSPacket } from '../DNS-core/DNSPacket.js';
import { DNSQuestion } from '../DNS-core/DNSQuestion.js';
import { A_Record } from '../DNS-core/resource-records/A_Record.js';
import { CNAME_Record } from '../DNS-core/resource-records/CNAME_Record.js';
import { HINFO_Record } from '../DNS-core/resource-records/HINFO_Record.js';
import { MX_Record } from '../DNS-core/resource-records/MX_Record.js';
import { NS_Record } from '../DNS-core/resource-records/NS_Record.js';
import { PTR_Record } from '../DNS-core/resource-records/PTR_Record.js';
import type { RDataMap } from '../DNS-core/resource-records/RDataMap.js';
import type { ResourceRecord } from '../DNS-core/resource-records/ResourceRecord.js';
import { SOA_Record } from '../DNS-core/resource-records/SOA_Record.js';
import { TXT_Record } from '../DNS-core/resource-records/TXT_Record.js';
import { Cursor } from './Cursor.js';
import { CursorBuffer } from './CursorBuffer.js';

export class DNSParser {
  parse(rawPacket: CursorBuffer): DNSPacket {
    const header = this.parseHeader(rawPacket);
    const questions = this.parseQuestions(rawPacket, header);
    const answers = this.parseResourceRecord(rawPacket, header.answerCount);
    const authoritative = this.parseResourceRecord(rawPacket, header.authoritativeCount);
    const additional = this.parseResourceRecord(rawPacket, header.additionalCount);

    return new DNSPacket(header, questions, answers, authoritative, additional);
  }

  private parseHeader(rawPacket: CursorBuffer): DNSHeader {
    const id = rawPacket.readUint16();

    const flags = rawPacket.readUint16();

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

    const qdCount = rawPacket.readUint16();
    const anCount = rawPacket.readInt16();
    const nsCount = rawPacket.readInt16();
    const arCount = rawPacket.readInt16();

    return new DNSHeader(id, !qr, opCode, !!aa, !!tc, !!rd, !!ra, z, rCode, qdCount, anCount, nsCount, arCount);
  }

  private parseQuestions(rawPacket: CursorBuffer, header: DNSHeader): DNSQuestion[] {
    const questions: DNSQuestion[] = [];

    for (let i = 0; i < header.questionCount; i++) {
      const qNameLabels = this.parseDomainName(rawPacket);
      const qType = rawPacket.readUint16();
      const qClass = rawPacket.readInt16();

      questions.push(new DNSQuestion(qNameLabels, qType, qClass));
    }

    return questions;
  }

  private parseResourceRecord(rawPacket: CursorBuffer, rrCount: number): ResourceRecord<RDataMap[DNS_TYPES]>[] {
    const resourceRecords: ResourceRecord<RDataMap[DNS_TYPES]>[] = [];

    for (let i = 0; i < rrCount; i++) {
      const name = this.parseDomainName(rawPacket);
      const type: DNS_TYPES = rawPacket.readUint16();
      const RR_class: DNS_CLASSES = rawPacket.readInt16();
      const ttl = rawPacket.readUint32();
      const rdLength = rawPacket.readUint16();
      const rawRData = rawPacket.subarray(rdLength);

      switch (type) {
        case DNS_TYPES.A: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new A_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.CNAME: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new CNAME_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.HINFO: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new HINFO_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.MX: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new MX_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.NS: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new NS_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.PTR: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new PTR_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.SOA: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new SOA_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
        case DNS_TYPES.TXT: {
          const rData = this.parseRData<typeof type>(rawPacket, type, rdLength, rawRData);
          resourceRecords.push(new TXT_Record(name, type, RR_class, ttl, rdLength, rData));
          break;
        }
      }
    }

    return resourceRecords;
  }

  private parseDomainName(rawPacket: CursorBuffer, visitedPointers: Set<number> = new Set<number>()): string {
    const nameLabels: string[] = [];

    while (true) {
      const currentByte = rawPacket.readUint8();

      // Check if currentByte is a pointer
      if ((currentByte & 0xc0) == 0xc0) {
        const pointer = this.decodePointer(rawPacket.cloneBuffer(), currentByte);

        // Check if the pointer has been visited before
        if (visitedPointers.has(pointer.getPosition())) {
          throw new PointerLoopError(`Pointer loop detected for pointer ${pointer.getPosition().toString(16)}.`);
        }

        // Because this is just a lookup, we clone the rawPacket and use it to resolve the pointer.
        // This way, we don't advance the pointer of rawPacket, that is used to parse the original DNS message.
        const lookupPacket = new CursorBuffer(Buffer.from(rawPacket.cloneBuffer()));

        visitedPointers.add(pointer.getPosition());
        nameLabels.push(...this.parseDomainName(lookupPacket, visitedPointers));

        break;
      }

      // Check if currentByte is a zero terminator (0x00)
      if (currentByte === 0) {
        break;
      }

      // currentByte is a length label
      const length = currentByte;
      nameLabels.push(decodePuny(rawPacket.subarray(length).toString('ascii')));
    }

    return nameLabels.join('.') + '.';
  }

  private parseCharString(rawPacket: CursorBuffer): string {
    const length = rawPacket.readUint8();

    if (length + 1 > 256) {
      throw new IllegalCharStringError(`Got ${length + 1} bytes. Must be <= 256.`);
    }

    return decodePuny(rawPacket.subarray(length).toString('ascii'));
  }

  private decodePointer(buffer: Buffer, position: number): Cursor {
    const high = (buffer.readUint8(position) & 0x3f) << 8;
    const low = buffer.readUint8(position + 1);

    return new Cursor(high | low);
  }

  private parseRData<RRType extends DNS_TYPES>(
    rawPacket: CursorBuffer,
    rrType: RRType,
    rdLength: number,
    rawRData: Buffer
  ): RDataMap[RRType] {
    const rDataCursorBuffer = new CursorBuffer(rawRData);
    const rawPacketClone = rawPacket.clone();

    switch (rrType) {
      case DNS_TYPES.A: {
        const octets: string[] = [];

        for (let i = 0; i < 4; i++) {
          octets.push(rawPacketClone.readUint8().toString());
        }

        return octets.join('.') as RDataMap[RRType];
      }
      case DNS_TYPES.CNAME: {
        return this.parseDomainName(rDataCursorBuffer) as RDataMap[RRType];
      }
      case DNS_TYPES.HINFO: {
        const cpu = this.parseCharString(rDataCursorBuffer);
        const os = this.parseCharString(rDataCursorBuffer);

        return { cpu, os } as RDataMap[RRType];
      }
      case DNS_TYPES.MX: {
        const preference = rDataCursorBuffer.readInt16();
        const exchange = this.parseDomainName(rawPacketClone);

        return {
          preference,
          exchange,
        } as RDataMap[RRType];
      }
      case DNS_TYPES.NS: {
        return this.parseDomainName(rawPacketClone) as RDataMap[RRType];
      }
      case DNS_TYPES.PTR: {
        return this.parseDomainName(rawPacketClone) as RDataMap[RRType];
      }
      case DNS_TYPES.SOA: {
        const mName = this.parseDomainName(rawPacketClone);
        const rName = this.parseDomainName(rawPacketClone);
        const serial = rawPacketClone.readUint32();
        const refresh = rawPacketClone.readInt32();
        const retry = rawPacketClone.readInt32();
        const expire = rawPacketClone.readInt32();
        const minimum = rawPacketClone.readUint32();

        return {
          mName,
          rName,
          serial,
          refresh,
          retry,
          expire,
          minimum,
        } as RDataMap[RRType];
      }
      case DNS_TYPES.TXT: {
        const charStrings: string[] = [];

        while (rDataCursorBuffer.getCursorPosition() < rdLength) {
          charStrings.push(this.parseCharString(rawPacketClone));
        }

        return charStrings as RDataMap[RRType];
      }
    }
  }
}
