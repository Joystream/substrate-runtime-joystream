import Config from '../Config';
import { State, StateKeeper } from '../state/StateKeeper';
import { Client } from '@elastic/elasticsearch'
import QueryEventBlock from '../joynode/QueryEventBlock';
import { JoyEntity } from '../joynode/StateBootstrap';

const logger = require('log4js').getLogger('es-uploader');

const ENTITY_TYPES_TO_INDEX: { [type: string]: string } = {
    "profile": "members"
}

/**
 * This class encapsulates all the logic related to reading/writing/creating 
 * ElasticSearch indices. First, it tries to recover the last state of the ETL process by
 * looking at the last event in `events`. If there's no such index, it goes to the bootstrap mode, 
 * creates all the necessary indices (currently with very basic schemas).
 * 
 * It has a `bootstrap()` handle to load the initial state (typically a snapshotted from the Jostream at given
 * height, e.g. genesis). 
 * 
 * In the normal operation is simply uploades all the Joystream events directly to `events`, but
 * in the future will also update the `members`, `content` and other indices consumed from the 
 * Transfomer
 * 
 *
 */
export default class ESUploader {
    private _client: Client;

    constructor(config: Config) {
        this._client = new Client({ node: config.get().elasticsearch.node })
        this._client.on('response', (err:any, result:any) => {
            if (err) {
              logger.error(`ES response ERROR: ${JSON.stringify(err, null, 2)}`)
            } 
            logger.trace(`ES response: ${JSON.stringify(result, null, 2)}`);
        })
        
        this._client.on('request', (err:any, result: any) => {
            if (err) {
                logger.error(`ES request ERROR: ${JSON.stringify(err, null, 2)}`)
            } 
            logger.trace(`ES request: ${JSON.stringify(result, null, 2)}`);
        })
    }

    async restore(): Promise<State> {

        await this.createIfNotExists("events");
        await this.createIfNotExists("members");
        
        const response = await this._client.search({
            index: 'events',
            size: 1,        
            body: {
                query: {
                    match_all: {}
                },
                sort: [
                     {
                         "blockNumber": {
                             "order":"desc" 
                         }
                     }
                ]
            }    
        })
        if (response.body.hits.total.value == 0) {
            return StateKeeper.nullState()
        }
        return {
            inBlock: response.body.hits.hits[0]._source.blockNumber,
            lastProcessedEventInBlock: response.body.hits.hits[0]._source.index
        }
    }

    async createIfNotExists(index: string) {
        const exists = await this._client.indices.exists({index});
        logger.trace(`Index ${index}: ${JSON.stringify(exists, null, 2)}`);
        if (!exists.body) {
            logger.debug(`Creating index ${index}`);
            // TODO: specify mappings and settings from the schema files
            const response = await this._client.indices.create({
                index,
                body: {
                    mappings: {
                        properties: {
                            "blockNumber": {"type": "integer"}
                        }
                    }
                }
            });
        }
    }

    async bootstrap(entities: AsyncGenerator<JoyEntity>):Promise<State> {
        let toLoad: JoyEntity[] = [];
        for await (const entity of entities) {
            toLoad.push(entity);
            if (toLoad.length >= 10) {
                await this.flush(toLoad);
                toLoad = [];
            }   
        }
        await this.flush(toLoad);
        return {
            inBlock: 0,
            lastProcessedEventInBlock: -1
        }
    }

    async * sync(blocks: AsyncGenerator<QueryEventBlock>):AsyncGenerator<State> {
        for await (const block of blocks) {
            await this.process(block);
            yield {
                inBlock: block.block_number,
                lastProcessedEventInBlock: block.query_events.length
            }
        }
    }

    async flush(toLoad: JoyEntity[]) {
        const body = toLoad.
            filter((x:JoyEntity) => x.type in ENTITY_TYPES_TO_INDEX).flatMap(
                (doc, i) => [
                    { index: { _index: ENTITY_TYPES_TO_INDEX[doc.type], _id: doc.id } },
                    doc
                ]
            );
        await this._client.bulk({ refresh: 'true', body });
    }

    async process(eventBlock: QueryEventBlock) {
        const body = eventBlock.query_events.flatMap(
            (doc, i) => [{ index: { _index: 'events' } }, 
            {   
                eventRecord: doc.toDTO(),
                blockNumber: eventBlock.block_number,
                index: i,
            }]
        );
        
        const { body: bulkResponse } = await this._client.bulk({ refresh: 'true', body });
        if (bulkResponse.errors) {
            const erroredDocuments: any[] = [];
            // The items array has the same order of the dataset we just indexed.
            // The presence of the `error` key indicates that the operation
            // that we did for the document has failed.
            bulkResponse.items.forEach((action:any, i:number) => {
              const operation = Object.keys(action)[0]
              if (action[operation].error) {
                erroredDocuments.push({
                  // If the status is 429 it means that you can retry the document,
                  // otherwise it's very likely a mapping error, and you should
                  // fix the document before to try it again.
                  status: action[operation].status,
                  error: action[operation].error,
                  operation: body[i * 2],
                  document: body[i * 2 + 1]
                })
              }
            })
            logger.error(erroredDocuments)
        }
    }
}