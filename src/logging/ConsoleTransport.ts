import chalk from 'chalk';
import { stderr, stdout } from 'process';
import type { BaseTransport } from './BaseTransport.js';
import type { LogEntry } from './LogEntry.js';
import { LogLevel } from './LogLevel.js';

export class ConsoleTransport implements BaseTransport {
  log(logEntry: LogEntry): void {
    let out: typeof stdout | typeof stderr;
    if (logEntry.level <= LogLevel.ERROR) {
      out = stderr;
    } else {
      out = stdout;
    }

    const timestamp = chalk.white(logEntry.timestamp.toISOString());
    const source = logEntry.source;
    const level = this.colorizeLevel(logEntry.level);
    const message = logEntry.message;
    const data = logEntry.data ? JSON.stringify(logEntry.data, undefined, 2) : null;

    out.write(`[${timestamp}] [${source}]\t[${level}]\t${message}${data ? `${data}` : ''}\n`);
  }

  private colorizeLevel(level: LogLevel): string {
    switch (level) {
      case LogLevel.FATAL: {
        return chalk.red(chalk.bold(LogLevel[level]));
      }
      case LogLevel.ERROR: {
        return chalk.red(LogLevel[level]);
      }
      case LogLevel.WARN: {
        return chalk.yellow(LogLevel[level]);
      }
      case LogLevel.INFO: {
        return chalk.green(LogLevel[level]);
      }
      case LogLevel.VERBOSE: {
        return chalk.cyan(LogLevel[level]);
      }
      case LogLevel.DEBUG: {
        return chalk.white(LogLevel[level]);
      }
    }
  }
}
