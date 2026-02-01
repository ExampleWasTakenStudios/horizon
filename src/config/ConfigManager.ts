import type { Logger } from '@src/logging/Logger.js';
import fs from 'fs';
import JSON5 from 'json5';
import path from 'path';
import type z from 'zod';
import { DefaultConfig } from './DefaultConfig.js';
import { type HorizonConfig, HorizonConfigSchema } from './HorizonConfig.js';

export class ConfigManager {
  private readonly logger: Logger;

  private readonly path: string;
  private readonly filename: string;

  private readonly config: HorizonConfig;

  public constructor(logger: Logger) {
    this.logger = logger;

    this.filename = 'config.json5';
    this.path = this.constructConfigPath();

    this.config = this.initialize();
  }

  public getConfig(): HorizonConfig {
    return this.config;
  }

  private initialize(): HorizonConfig {
    this.logger.info('Starting initialization. Using config path ', this.path);

    let fileContent: string;
    if (!fs.existsSync(this.path)) {
      this.logger.info('No config file found at ', this.path);
      fileContent = this.createConfig();
    } else {
      this.logger.info('Found config file at ', this.path);
      fileContent = fs.readFileSync(this.path, { encoding: 'utf-8' });
    }

    const parsedContents = this.parse(fileContent);

    if (!parsedContents.success) {
      this.logger.fatal('Zod Error occurred while trying to parse config. ', parsedContents.error);

      // This is initialization code that is explicitly supposed to crash upon invalid states to prevent UB.
      // eslint-disable-next-line no-restricted-syntax
      throw parsedContents.error;
    }

    this.logger.info('Initialization complete.');
    return parsedContents.data;
  }

  private createConfig(): string {
    this.logger.info('Creating config file at ', this.path);

    const fileContent = this.stringify(DefaultConfig);

    const dirPath = this.path
      .split('/')
      .filter((current) => current !== this.filename)
      .join('/');
    fs.mkdirSync(dirPath, { recursive: true });

    fs.writeFileSync(this.path, fileContent);

    return fileContent;
  }

  private stringify(config: HorizonConfig): string {
    return JSON5.stringify(config, { space: 4, quote: '"' });
  }

  private parse(str: string): z.ZodSafeParseResult<HorizonConfig> {
    this.logger.verbose('Parsing config file...');
    const jsonResult: unknown = JSON5.parse(str);

    return HorizonConfigSchema.safeParse(jsonResult);
  }

  private constructConfigPath(): string {
    const devPath = path.join(process.cwd(), '.config', 'Horizon', 'config.json5');
    const prodPath = path.join('/', 'etc', 'opt', 'Horizon', this.filename);
    return process.env.NODE_ENV === 'dev' ? devPath : prodPath;
  }
}
