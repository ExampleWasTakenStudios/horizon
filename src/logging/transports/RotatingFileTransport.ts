import { createStream, type Options as RFSOptions, type RotatingFileStream } from 'rotating-file-stream';
import type { LogEntry } from '../LogEntry.js';
import type { LogLevel } from '../LogLevel.js';
import type { BaseTransport } from './BaseTransport.js';

export class RotatingFileTransport implements BaseTransport {
  private stream: RotatingFileStream;

  private maxLevel: LogLevel;

  constructor(filename: string, settings: RFSOptions, maxLevel: LogLevel) {
    this.maxLevel = maxLevel;
    this.stream = createStream(filename + '.json', settings);
  }

  log(logEntry: LogEntry): void {
    this.stream.write(JSON.stringify(logEntry) + '\n');
  }

  getMaxLevel(): Readonly<LogLevel> {
    return this.maxLevel;
  }

  setMaxLevel(level: LogLevel): void {
    this.maxLevel = level;
  }

  private generateFilename(filename: string): string {
    const ts = new Date();
    const year = ts.getUTCFullYear().toString().padStart(4);
    const month = (ts.getUTCMonth() + 1).toString().padStart(2);
    const date = ts.getUTCDate().toString().padStart(2);
    const hour = ts.getUTCHours().toString().padStart(2);
    const minute = ts.getUTCMinutes().toString().padStart(2);
    const second = ts.getUTCSeconds().toString().padStart(2);

    return `${filename}-${year}-${month}-${date}-${hour}:${minute}:${second}.json`;
  }
}
