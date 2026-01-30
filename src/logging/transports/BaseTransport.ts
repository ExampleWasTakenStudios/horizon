import type { LogEntry } from '../LogEntry.js';
import type { LOG_LEVEL } from '../LogLevel.js';

export interface BaseTransport {
  log(logEntry: LogEntry): void;
  getMaxLevel(): Readonly<LOG_LEVEL>;
  setMaxLevel(level: LOG_LEVEL): void;
}
