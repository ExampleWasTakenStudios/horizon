import punycode from '@dcoffey-zengenti/punynode';
import type { ReturnResult } from '../../../../types/ReturnResult.js';
import { err, ok } from '../../../../utils/return-type-helpers.js';
import { DNS_CLASSES } from '../DNS-core/constants/DNS_CLASSES.js';
import { DNS_RESPONSE_CODES } from '../DNS-core/constants/DNS_RESPONSE_CODES.js';
import { DNS_TYPES } from '../DNS-core/constants/DNS_TYPES.js';
import { DNSHeader } from '../DNS-core/DNSHeader.js';
import { DNSPacket } from '../DNS-core/DNSPacket.js';
import { DNSQuestion } from '../DNS-core/DNSQuestion.js';
import { A_Record } from '../DNS-core/resource-records/A_Record.js';
import { CNAME_Record } from '../DNS-core/resource-records/CNAME_Record.js';
import { HINFO_Record } from '../DNS-core/resource-records/HINFO_Record.js';
import { MX_Record } from '../DNS-core/resource-records/MX_Record.js';
import { NS_Record } from '../DNS-core/resource-records/NS_Record.js';
import { OPT_Record } from '../DNS-core/resource-records/OPT_Record.js';
import { PTR_Record } from '../DNS-core/resource-records/PTR_Record.js';
import type { RDataMap } from '../DNS-core/resource-records/RDataMap.js';
import type { ResourceRecord } from '../DNS-core/resource-records/ResourceRecord.js';
import { SOA_Record } from '../DNS-core/resource-records/SOA_Record.js';
import { TXT_Record } from '../DNS-core/resource-records/TXT_Record.js';
import { Cursor } from './Cursor.js';
import { CursorBuffer } from './CursorBuffer.js';

/**
 * TODO: A note on logging
 * Any negative returns should probably be logged. However, currenty logging infrastructure doesn't
 * allow injecting a logger instance in this class because the super class centrally enabling injecting
 * a logger instance is the {@link Module} or {@link Subsystem}.
 * The fact that both contain indentical implementations, speaks to the fact that a refactor is required.
 */
export class DNSParser {
  parse(rawPacket: CursorBuffer): ReturnResult<DNSPacket> {
    const headerResult = this.parseHeader(rawPacket);
    if (!headerResult.success) {
      return err(headerResult.rCode);
    }

    const questionsResult = this.parseQuestions(rawPacket, headerResult.data);
    if (!questionsResult.success) {
      return err(questionsResult.rCode);
    }

    const answersResult = this.parseResourceRecord(rawPacket, headerResult.data.answerCount);
    if (!answersResult.success) {
      return err(answersResult.rCode);
    }

    const authoritativeResult = this.parseResourceRecord(rawPacket, headerResult.data.authoritativeCount);
    if (!authoritativeResult.success) {
      return err(authoritativeResult.rCode);
    }

    const additionalResult = this.parseResourceRecord(rawPacket, headerResult.data.additionalCount);
    if (!additionalResult.success) {
      return err(additionalResult.rCode);
    }

    return {
      success: true,
      data: new DNSPacket(
        headerResult.data,
        questionsResult.data,
        answersResult.data,
        authoritativeResult.data,
        additionalResult.data
      ),
    };
  }

  private parseHeader(rawPacket: CursorBuffer): ReturnResult<DNSHeader> {
    const id = rawPacket.readNextUint16();

    const flags = rawPacket.readNextUint16();

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

    const qdCount = rawPacket.readNextUint16();
    const anCount = rawPacket.readNextInt16();
    const nsCount = rawPacket.readNextInt16();
    const arCount = rawPacket.readNextInt16();

    return {
      success: true,
      data: new DNSHeader(id, !qr, opCode, !!aa, !!tc, !!rd, !!ra, z, rCode, qdCount, anCount, nsCount, arCount),
    };
  }

  private parseQuestions(rawPacket: CursorBuffer, header: DNSHeader): ReturnResult<DNSQuestion[]> {
    const questions: DNSQuestion[] = [];

    for (let i = 0; i < header.questionCount; i++) {
      const qNameLabels = this.parseDomainName(rawPacket);
      if (!qNameLabels.success) {
        return {
          success: false,
          rCode: qNameLabels.rCode,
        };
      }

      const qType = rawPacket.readNextUint16();
      const qClass = rawPacket.readNextUint16();

      questions.push(new DNSQuestion(qNameLabels.data, qType, qClass));
    }

    return ok(questions);
  }

  private parseResourceRecord(
    rawPacket: CursorBuffer,
    rrCount: number
  ): ReturnResult<ResourceRecord<RDataMap[DNS_TYPES]>[]> {
    const resourceRecords: ResourceRecord<RDataMap[DNS_TYPES]>[] = [];

    for (let i = 0; i < rrCount; i++) {
      const name = this.parseDomainName(rawPacket);
      if (!name.success) {
        return err(name.rCode);
      }

      const type: DNS_TYPES = rawPacket.readNextUint16();
      const RR_class: DNS_CLASSES = rawPacket.readNextUint16();
      const ttl = rawPacket.readNextUint32();
      const rdLength = rawPacket.readNextUint16();
      const rawRData = rawPacket.nextSubarray(rdLength);

      switch (type) {
        case DNS_TYPES.A: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new A_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.CNAME: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new CNAME_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.HINFO: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new HINFO_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.MX: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new MX_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.NS: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new NS_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.PTR: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new PTR_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.SOA: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new SOA_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.TXT: {
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }
          resourceRecords.push(new TXT_Record(name.data, type, RR_class, ttl, rdLength, rData.data));
          break;
        }
        case DNS_TYPES.OPT: {
          // At the moment, this call is actually unecessary, since 'parseRData()' just returns 'rawRData' but to
          // keep the data flow identical between all types of RRs we pass it to the method anyway.
          const rData = this.parseRData(rawPacket, type, rdLength, rawRData);
          if (!rData.success) {
            return err(rData.rCode);
          }

          const extendedRCode = (ttl >>> 24) & 0xff;
          const ednsVersion = (ttl >>> 16) & 0x00ff;

          if (ednsVersion !== 0) {
            return err(DNS_RESPONSE_CODES.BAD_VERSION);
          }

          resourceRecords.push(
            new OPT_Record(name.data, type, ttl, RR_class, rdLength, rData.data, extendedRCode, ednsVersion)
          );
          break;
        }
        default: {
          return err(DNS_RESPONSE_CODES.NOT_IMPLEMENTED);
        }
      }
    }

    return ok(resourceRecords);
  }

  private parseDomainName(
    rawPacket: CursorBuffer,
    visitedPointers: Set<number> = new Set<number>()
  ): ReturnResult<string> {
    const nameLabels: string[] = [];

    while (true) {
      const currentByte = rawPacket.readNextUint8();

      // Check if currentByte is a pointer
      if ((currentByte & 0xc0) == 0xc0) {
        const pointer = this.decodePointer(rawPacket.cloneBuffer(), currentByte);

        // Check if the pointer has been visited before
        if (visitedPointers.has(pointer.getPosition())) {
          return err(DNS_RESPONSE_CODES.FORMAT_ERROR);
        }

        // Because this is just a lookup, we clone the rawPacket and use it to resolve the pointer.
        // This way, we don't advance the pointer of rawPacket, that is used to parse the original DNS message.
        const lookupPacket = new CursorBuffer(rawPacket.cloneBuffer(), pointer.getPosition());

        visitedPointers.add(pointer.getPosition());
        const domainNameResult = this.parseDomainName(lookupPacket, visitedPointers);
        if (!domainNameResult.success) {
          return err(domainNameResult.rCode);
        }

        nameLabels.push(domainNameResult.data);

        break;
      }

      // Check if currentByte is a zero terminator (0x00)
      if (currentByte === 0) {
        break;
      }

      // currentByte is a length label
      const length = currentByte;
      const asciiString = rawPacket.nextSubarray(length).toString('ascii');

      const label = asciiString.startsWith('xn--') ? punycode.toUnicode(asciiString) : asciiString;

      nameLabels.push(label);
    }

    return ok(nameLabels.join('.'));
  }

  private parseCharString(rawPacket: CursorBuffer): ReturnResult<string> {
    const length = rawPacket.readNextUint8();

    if (length > 255) {
      return err(DNS_RESPONSE_CODES.FORMAT_ERROR);
    }

    return ok(rawPacket.nextSubarray(length).toString('ascii'));
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
  ): ReturnResult<RDataMap[RRType]> {
    const rDataCursorBuffer = new CursorBuffer(rawRData);
    const rawPacketClone = rawPacket.clone();

    switch (rrType) {
      case DNS_TYPES.A: {
        const octets: string[] = [];

        for (let i = 0; i < 4; i++) {
          octets.push(rDataCursorBuffer.readNextUint8().toString());
        }

        return ok(octets.join('.') as RDataMap[RRType]);
      }
      case DNS_TYPES.CNAME: {
        const nameResult = this.parseDomainName(rDataCursorBuffer);
        if (!nameResult.success) {
          return err(nameResult.rCode);
        }

        return ok(nameResult.data as RDataMap[RRType]);
      }
      case DNS_TYPES.HINFO: {
        const cpu = this.parseCharString(rDataCursorBuffer);
        if (!cpu.success) {
          return err(cpu.rCode);
        }

        const os = this.parseCharString(rDataCursorBuffer);
        if (!os.success) {
          return err(os.rCode);
        }

        return ok({ cpu, os } as RDataMap[RRType]);
      }
      case DNS_TYPES.MX: {
        const preference = rDataCursorBuffer.readNextInt16();
        const exchange = this.parseDomainName(rawPacketClone);
        if (!exchange.success) {
          return err(exchange.rCode);
        }

        return ok({
          preference,
          exchange,
        } as RDataMap[RRType]);
      }
      case DNS_TYPES.NS:
      case DNS_TYPES.PTR: {
        const name = this.parseDomainName(rawPacketClone);
        if (!name.success) {
          return err(name.rCode);
        }

        return ok(name.data as RDataMap[RRType]);
      }
      case DNS_TYPES.SOA: {
        const mName = this.parseDomainName(rawPacketClone);
        if (!mName.success) {
          return err(mName.rCode);
        }

        const rName = this.parseDomainName(rawPacketClone);
        if (!rName.success) {
          return err(rName.rCode);
        }

        const serial = rawPacketClone.readNextUint32();
        const refresh = rawPacketClone.readNextInt32();
        const retry = rawPacketClone.readNextInt32();
        const expire = rawPacketClone.readNextInt32();
        const minimum = rawPacketClone.readNextUint32();

        return ok({
          mName,
          rName,
          serial,
          refresh,
          retry,
          expire,
          minimum,
        } as RDataMap[RRType]);
      }
      case DNS_TYPES.TXT: {
        const charStrings: string[] = [];

        while (rDataCursorBuffer.getCursorPosition() < rdLength) {
          const charStringResult = this.parseCharString(rDataCursorBuffer);
          if (!charStringResult.success) {
            return err(charStringResult.rCode);
          }

          charStrings.push(charStringResult.data);
        }

        return ok(charStrings as RDataMap[RRType]);
      }
      case DNS_TYPES.OPT: {
        // We do not support any RDATA in the OPT RR at the moment so we just return it back, thereby effectively ignoring it.
        return ok(rawRData as RDataMap[RRType]);
      }
    }
  }
}
