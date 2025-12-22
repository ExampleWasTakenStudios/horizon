import type { LogEntry } from '../LogEntry.js';
import type { LogLevel } from '../LogLevel.js';

export interface BaseTransport {
  log(logEntry: LogEntry): void;
  getMaxLevel(): Readonly<LogLevel>;
  setMaxLevel(level: LogLevel): void;
}
