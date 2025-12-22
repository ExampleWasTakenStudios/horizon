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
}
