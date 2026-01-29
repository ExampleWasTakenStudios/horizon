import type { LOG_LEVEL } from './LogLevel.js';

export interface LogEntry {
  timestamp: Date;
  level: LOG_LEVEL;
  source: string;
  message: string;
  data?: unknown[];
}
