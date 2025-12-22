import { createStream, type Options as RFSOptions, type RotatingFileStream } from 'rotating-file-stream';
import type { LogEntry } from '../LogEntry.js';
import type { LogLevel } from '../LogLevel.js';
import type { BaseTransport } from './BaseTransport.js';

/**
 * Helper to ensure Errors and BigInts are stringified correctly.
 * Standard JSON.stringify returns {} for Errors and throws on BigInt.
 */
const jsonReplacer = (_key: string, value: unknown): unknown => {
  if (value instanceof Error) {
    return {
      name: value.name,
      message: value.message,
      stack: value.stack,
      cause: value.cause,
    };
  }

  if (typeof value === 'bigint') {
    return value.toString();
  }

  return value;
};

export class RotatingFileTransport implements BaseTransport {
  private stream: RotatingFileStream;

  private maxLevel: LogLevel;

  constructor(filename: string, settings: RFSOptions, maxLevel: LogLevel) {
    this.maxLevel = maxLevel;

    this.stream = createStream(filename + '.json', settings);
  }

  log(logEntry: LogEntry): void {
    this.stream.write(JSON.stringify(logEntry, jsonReplacer) + '\n');
  }

  getMaxLevel(): Readonly<LogLevel> {
    return this.maxLevel;
  }

  setMaxLevel(level: LogLevel): void {
    this.maxLevel = level;
  }
}
