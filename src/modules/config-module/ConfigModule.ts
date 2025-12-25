import JSON5 from 'json5';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import * as z from 'zod';
import type { Logger } from '../../logging/Logger.js';
import { Module } from '../Module.js';
import { HorizonConfigSchema, type HorizonConfig } from './HorizonConfig.js';

export class ConfigModule extends Module {
  private readonly path: string;
  private readonly filename: string;

  private config: HorizonConfig;

  constructor(logger: Logger) {
    super(logger);

    this.filename = 'config.json5';
    this.path = path.join(os.homedir(), '.config', 'Horizon', this.filename);

    this.initialize();
  }

  private initialize(): void {
    this.logger.info('Starting initialization.');

    const fileContents = fs.readFileSync(this.path, { encoding: 'utf-8' });

    const parsedContents = this.parse(fileContents);

    if (!parsedContents.success) {
      this.logger.fatal('Zod Error occured while trying to parse config. ', parsedContents.error);
      throw parsedContents.error;
    } else {
      this.config = parsedContents.data; // FIXME: why does this error? the type of 'data' is compliant with HorizonConfig as is evident from the error message when trying to cast 'data' to 'HorizonConfig'.
    }
  }

  private stringify(config: HorizonConfig): string {
    return JSON5.stringify(config, { space: 2, quote: '"' });
  }

  private parse(str: string): z.ZodSafeParseResult<HorizonConfig> {
    const jsonResult: unknown = JSON5.parse(str);

    return HorizonConfigSchema.safeParse(jsonResult);
  }
}
