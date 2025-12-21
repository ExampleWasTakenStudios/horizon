import { BaseLogger, type ILogObjMeta, type ISettingsParam } from 'tslog';

export enum LOG_LEVELS {
  DEBUG = 0,
  VERBOSE = 1,
  INFO = 2,
  WARN = 3,
  ERROR = 4,
  FATAL = 5,
}

export class HorizonLogger<LogObj> extends BaseLogger<LogObj> {
  constructor(settings?: ISettingsParam<LogObj>, logObj?: LogObj) {
    super(settings, logObj, 5);
  }

  fatal(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.FATAL, 'FATAL', ...args);
  }

  error(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.ERROR, 'ERROR', ...args);
  }

  warn(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.WARN, 'WARN', ...args);
  }

  info(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.INFO, 'INFO', ...args);
  }

  verbose(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.VERBOSE, 'VERBOSE', ...args);
  }

  debug(...args: unknown[]): (LogObj & ILogObjMeta) | undefined {
    return super.log(LOG_LEVELS.DEBUG, 'DEBUG', ...args);
  }
}
