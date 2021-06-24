import { ApiPromise, WsProvider } from '@polkadot/api'
import { types } from '@joystream/types'
import { createDBConnection } from '@dzlzv/hydra-processor'
import { makeDatabaseManager } from '@dzlzv/hydra-processor/lib/executor/TransactionalExecutor'
import path from 'path'

// A script to initialize processor database with some initial values that cannot be fetched from events / extrinics
async function init() {
  const provider = new WsProvider(process.env.WS_PROVIDER_ENDPOINT_URI)
  const api = await ApiPromise.create({ provider, types })
  // Will be resolved relatively to mappings/lib
  const entitiesPath = path.resolve(__dirname, '../../generated/graphql-server/dist/src/modules/**/*.model.js')
  // We need to create db connection (and configure env) before importing any warthog models
  const dbConnection = await createDBConnection([entitiesPath])
  const db = makeDatabaseManager(dbConnection.createEntityManager())
  // Only now we can import the initialization scripts (which include warthog models imports)
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const initializeDb = require('./initializeDb').default

  await initializeDb(api, db)
}

init()
  .then(() => {
    console.log('Processor database initialized')
    process.exit()
  })
  .catch((e) => {
    console.error(e)
    process.exit(-1)
  })
