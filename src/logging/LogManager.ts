import { type ILogObj } from 'tslog';
import { HorizonLogger } from './HorizonLogger.js';

export class LogManager {
  private readonly logger: HorizonLogger<ILogObj>;

  constructor() {
    this.logger = new HorizonLogger({
      name: 'MAIN',
      minLevel: +process.env['MIN_LOGGING_LVL']!,
      prettyLogTemplate: '[{{rawIsoStr}}] [{{name}}] [{{logLevelName}}]\t',
      stylePrettyLogs: true,
      prettyLogStyles: {
        logLevelName: {
          DEBUG: 'white',
          VERBOSE: 'cyan',
          INFO: 'green',
          WARN: 'yellow',
          ERROR: 'red',
          FATAL: ['bold', 'redBright'],
        },
        dateIsoStr: 'white',
        name: ['bold', 'white'],
      },
      prettyLogTimeZone: process.env['NODE_ENV']! === 'dev' ? 'local' : 'UTC',
    });
  }

  getLogger(): Readonly<HorizonLogger<ILogObj>> {
    return this.logger;
  }
}
