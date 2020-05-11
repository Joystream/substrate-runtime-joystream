import ISubstrateQueryService from './ISubstrateQueryService';
import QueryEvent from './QueryEvent';
import QueryEventBlock from './QueryEventBlock';
import { EventRecord, SignedBlock, Hash } from '@polkadot/types/interfaces';
import { EventEmitter } from 'events';
import { Codec } from '@polkadot/types/types';

import Config from '../Config';
const logger = require('log4js').getLogger('producer');

const DEFAULT_NEW_BLOCK_POLL_INTERVAL_MS = 5000;

/**
 * QueryBlockProducer yields new blocks of the JoyStream blockchain though the 
 * `blocks()` async generator.  Each block is an instance of `QueryEventBlock` and 
 * represents all the mutations of the Joystream-state (including extrinsics) which allows
 * to replay them by the downstream consumers. The `block()` generator waits until
 *  - new `QueryEventBlock` is consumed by a downstream client
 *  - next block height is not greater than `_chainHeight` which follows 
 *  the last known finalized block height
 */
export default class QueryBlockProducer extends EventEmitter {
    private _started: boolean;
    private readonly _queryService: ISubstrateQueryService;
    private _newHeadsUnsubscriber: () => void;
    private _nextBlockHeight: number;
    private _chainHeight: number;
    private _pollIntervalMs: number;
    private _lastAckedBlockHeight: number; 
    private _maxUnackedBlocks: number;

    constructor(query_service: ISubstrateQueryService, config: Config) {
        super();

        this._started = false;
        this._queryService = query_service;
        this._pollIntervalMs = config.get().joysteam?.poll_interval || DEFAULT_NEW_BLOCK_POLL_INTERVAL_MS;
        logger.debug(`Polling ${this._pollIntervalMs} ms`);

        // TODO
        // need to set this up, when state is better, it
        // will be refactored
        this._newHeadsUnsubscriber = () => { };

        this._nextBlockHeight = 0;
        this._lastAckedBlockHeight = 0;
        this._maxUnackedBlocks = Number.MAX_SAFE_INTEGER; // todo: read from config
        this._chainHeight = 0;
    }

    // TODO: We cannot assume first block has events... we need more robust logic.
    async start(at_block?: number) {
        if (this._started) throw Error(`Cannot start when already started.`);

        logger.debug(`Starting at block ${at_block}`);
        // mark as started
        this._started = true;

        // Try to get initial header right away
        this._chainHeight = await this._queryService
            .getFinalizedHead()
            .then((hash) => {
                return this._queryService.getHeader(hash);
            })
            .then((header) => {
                return header.number.toNumber();
            });

        this._nextBlockHeight = at_block ? at_block : 0;
        this._lastAckedBlockHeight = 0; // TODO: listen for the global state keeper and update
        
        this._newHeadsUnsubscriber = await this._queryService.subscribeNewHeads((header) => {
            this._chainHeight = header.number.toNumber();
            logger.debug(`New block found at height #${this._chainHeight}`)
        });

    }

    async stop() {
        if (!this._started) throw new Error(`Cannot stop when not already started.`);

        // THIS IS VERY CRUDE, NEED TO MANAGE LOTS OF STUFF HERE!
        logger.debug("Stopping Query Block Producer");
        (await this._newHeadsUnsubscriber)();

        this._started = false;
    }

    async * blocks():AsyncGenerator<QueryEventBlock> { 
        while (this._started) {
            let height = this._nextBlockHeight;
            let block_hash_of_target = await this.getBlockHashOrWait(height, () => {
                return (this._chainHeight >= height) && 
                    (height - this._lastAckedBlockHeight <= this._maxUnackedBlocks)
            });
            let records = await this._queryService.eventsAt(block_hash_of_target);
            logger.debug(`\tRead ${records.length} events of the block at height ${height}.`);

            let signed_block = await this._queryService.getBlock(block_hash_of_target);
            logger.debug(`\tFetched full block at height ${height}.`);    
            
            this._nextBlockHeight++;
            yield this.emitBlockEvent(signed_block, records);
        }
    }

    private async getBlockHashOrWait(height: number, waitFor:()=>boolean): Promise<Hash> {
        return new Promise<Hash>((resolve, reject) => {
            let checkHeight = () => {
                if (!this._started) {
                    reject("The block producer is stopped")
                    return;
                }
                
                if (waitFor()) {
                    this._queryService.getBlockHash(height)
                        .then((h: Hash) => resolve(h))
                        .catch((e) => reject(e));
                } else {
                    logger.debug(`Current chain height: ${this._chainHeight}, waiting for: ${height}`);
                    setTimeout(checkHeight, this._pollIntervalMs);
                }    

            }
            checkHeight();
        });
    }

    private emitBlockEvent(signed_block: SignedBlock, records: EventRecord[] & Codec):QueryEventBlock {
        let extrinsics_array = signed_block.block.extrinsics.toArray();
        let query_events: QueryEvent[] = records.map(
                (record, index): QueryEvent => {
                    // Extract the phase, event
                    const { phase } = record;

                    // Try to recover extrinsic: only possible if its right phase, and extrinsics arra is non-empty, the last constraint
                    // is needed to avoid events from build config code in genesis, and possibly other cases.
                    let extrinsic =
                        phase.isApplyExtrinsic && extrinsics_array.length
                            ? extrinsics_array[phase.asApplyExtrinsic.toBn()]
                            : undefined;

                    let query_event = new QueryEvent(record, extrinsic);

                    // Logging
                    query_event.log(0, (x) => logger.trace(x));

                    return query_event;
                }
            );

            let query_block = new QueryEventBlock(this._nextBlockHeight, query_events);
            this.emit('QueryEventBlock', query_block);
            logger.debug(`\tEmitted query event block.`);
            return query_block;
    }


    
}