import type { LogEntry } from './LogEntry.js';

export interface BaseTransport {
  log(logEntry: LogEntry): void;
}
