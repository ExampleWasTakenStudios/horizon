import chalk from 'chalk';
import { stderr, stdout } from 'process';
import type { LogEntry } from '../LogEntry.js';
import { LOG_LEVEL } from '../LogLevel.js';
import type { BaseTransport } from './BaseTransport.js';

export class ConsoleTransport implements BaseTransport {
  private maxLevel: LOG_LEVEL;

  public constructor(maxLevel: LOG_LEVEL) {
    this.maxLevel = maxLevel;
  }

  public log(logEntry: LogEntry): void {
    let out: typeof stdout | typeof stderr;
    if (logEntry.level <= LOG_LEVEL.ERROR) {
      out = stderr;
    } else {
      out = stdout;
    }

    const timestamp = chalk.whiteBright(`[${chalk.white(logEntry.timestamp.toISOString())}]`);
    const source = chalk.whiteBright(`[${logEntry.source}]`);
    const level = chalk.whiteBright(`[${this.colorize(logEntry.level, LOG_LEVEL[logEntry.level])}]`);
    const message = this.colorize(logEntry.level, logEntry.message);
    const data = logEntry.data ? this.colorize(logEntry.level, this.processData(...logEntry.data)) : null;

    out.write(`${timestamp} ${source} ${level} ${message}${data ?? ''}\n`);
  }

  public getMaxLevel(): Readonly<LOG_LEVEL> {
    return this.maxLevel;
  }

  public setMaxLevel(level: LOG_LEVEL): void {
    this.maxLevel = level;
  }

  private processData(...data: unknown[]): string {
    let str = '';

    for (const current of data) {
      if (current instanceof Error) {
        str += `\n${current.stack ?? current.message}`;
        continue;
      }

      switch (typeof current) {
        case 'string': {
          str += current;
          break;
        }
        case 'number':
        case 'boolean':
        case 'bigint': {
          str += current.toString();
          break;
        }
        default: {
          str += JSON.stringify(current, undefined, 2);
          break;
        }
      }
    }

    return str;
  }

  private colorize(level: LOG_LEVEL, input: string): string {
    switch (level) {
      case LOG_LEVEL.FATAL: {
        return chalk.red(input);
      }
      case LOG_LEVEL.ERROR: {
        return chalk.red(chalk.bold(input));
      }
      case LOG_LEVEL.WARN: {
        const orange = chalk.hex('#FFA500');
        return orange(input);
      }
      case LOG_LEVEL.INFO: {
        return chalk.greenBright(input);
      }
      case LOG_LEVEL.VERBOSE: {
        return chalk.cyan(input);
      }
      case LOG_LEVEL.DEBUG: {
        return chalk.white(input);
      }
    }
  }
}
