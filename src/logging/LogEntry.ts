import type { LogLevel } from './LogLevel.js';

export interface LogEntry {
  timestamp: Date;
  level: LogLevel;
  source: string;
  message: string;
  data?: unknown;
}
