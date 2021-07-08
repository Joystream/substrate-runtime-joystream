import 'reflect-metadata';

import { SnakeNamingStrategy } from 'warthog';
import { snakeCase } from 'typeorm/util/StringUtils';

import { loadConfig } from '../src/config';
import { Logger } from '../src/logger';

import { buildServerSchema, getServer } from './server';
import { startPgSubsribers } from './pubsub';
import { queryTemplates } from './queryTemplates'


class CustomNamingStrategy extends SnakeNamingStrategy {
  constructor() {
    super();
  }
  tableName(className: string, customName?: string): string {
    return customName ? customName : `${snakeCase(className)}`;
  }
}

async function bootstrap() {
  await loadConfig();

  const server = getServer({ queryTemplates }, { namingStrategy: new CustomNamingStrategy() });

  // Create database tables. Warthog migrate command does not support CustomNamingStrategy thats why
  // we have this code
  const syncDatabase: string | undefined = process.env.SYNC;
  if (syncDatabase === 'true') {
    await server.establishDBConnection();
    process.exit(0);
  }
  await buildServerSchema(server);
  await startPgSubsribers();
  await server.start();
}

bootstrap().catch((error: Error) => {
  Logger.error(error);
  if (error.stack) {
    Logger.error(error.stack.split('\n'));
  }
  process.exit(1);
});
