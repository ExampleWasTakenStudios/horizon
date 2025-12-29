import JSON5 from 'json5';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import * as z from 'zod';
import type { Logger } from '../logging/Logger.js';
import { DefaultConfig } from './DefaultConfig.js';
import { HorizonConfigSchema, type HorizonConfig } from './HorizonConfig.js';

export class ConfigManager {
  private logger: Logger;

  private readonly path: string;
  private readonly filename: string;

  private config: HorizonConfig;

  constructor(logger: Logger) {
    this.logger = logger;

    this.filename = 'config.json5';
    this.path = this.constructConfigPath();

    this.config = this.initialize();
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
      this.logger.fatal('Zod Error occured while trying to parse config. ', parsedContents.error);
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
    const prodPath = path.join(os.homedir(), '.config', 'Horizon', this.filename);
    return process.env.NODE_ENV === 'dev' ? devPath : prodPath;
  }

  getConfig(): HorizonConfig {
    return this.config;
  }
}
