import type { RemoteInfo } from 'dgram';

export interface ReceivedData {
  buf: Buffer;
  rinfo: RemoteInfo;
}
