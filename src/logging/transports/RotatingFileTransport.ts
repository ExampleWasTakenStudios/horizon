import fs from 'node:fs';
import path from 'node:path';
import { createStream, type Options as RFSOptions, type RotatingFileStream } from 'rotating-file-stream';
import type { LogEntry } from '../LogEntry.js';
import { LogLevel } from '../LogLevel.js';
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

export interface RotatingFileTransportSettings extends RFSOptions {
  interval: Exclude<RFSOptions['interval'], undefined>;
  size: Exclude<RFSOptions['size'], undefined>;
  maxFiles: Exclude<RFSOptions['maxFiles'], undefined>;
  path: Exclude<RFSOptions['path'], undefined>;
}

export class RotatingFileTransport implements BaseTransport {
  private stream: RotatingFileStream;

  private maxLevel: LogLevel;

  constructor(
    filename: string,
    settings: RotatingFileTransportSettings,
    maxLevel: LogLevel,
    clearLogsOnStartup?: boolean
  ) {
    this.maxLevel = maxLevel;

    if (clearLogsOnStartup) {
      const contents = fs.readdirSync(settings.path);

      for (const current of contents) {
        fs.rmSync(path.join(settings.path, current));
      }
    }

    this.stream = createStream(filename + '.json', { ...settings });
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
