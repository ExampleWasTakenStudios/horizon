import dgram from 'node:dgram';
import { DNSParser } from './DNSCore/DNSParser.js';
import { DNS_QCLASSES } from './DNSCore/constants/DNS_CLASSES.js';
import { DNS_QTYPES } from './DNSCore/constants/DNS_TYPES.js';

const server = dgram.createSocket('udp4');

server.on('error', (err) => {
  console.error(`server error:\n${err.stack}`);
  server.close();
});

server.on('message', (buffer, rinfo) => {
  const parser = new DNSParser(buffer);

  if (parser.getDNSPacket().getQuestions().length < 1) {
    console.log('DNS Packet contains no questions.');
    return;
  }

  const stats = {
    header: parser.getDNSPacket().getHeader(),
    questions: parser.getDNSPacket().getQuestions(),
  }

  console.log(JSON.stringify(stats, null, 2));
});

server.on('listening', () => {
  const address = server.address();
  console.log(`server listening ${address.address}:${address.port}`);
});

server.bind({ address: '127.0.0.1', port: 53 });
