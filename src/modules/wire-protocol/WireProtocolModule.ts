import type { Module } from '../Module.js';
import { DNSPacket } from './DNS-core/DNSPacket.js';
import { DNSParser } from './DNS-core/DNSParser.js';

export class WireProtocolModule implements Module {
  decode(buffer: Buffer): DNSPacket {
    const parser = new DNSParser(buffer);

    return parser.parse();
  }

  encode(dnsPacket: DNSPacket): Buffer {
    // TODO: implement DNS packet encoder
    return Buffer.from('');
  }
}
