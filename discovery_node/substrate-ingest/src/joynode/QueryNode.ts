// @ts-check
import { ApiPromise, WsProvider } from '@polkadot/api';
import { registerJoystreamTypes } from '@joystream/types';
import Config from '../Config'
import { State } from '../StateKeeper'
import ISubstrateQueryService, { makeQueryService } from './ISubstrateQueryService';
import QueryBlockProducer from './QueryBlockProducer';
import QueryEventBlock from './QueryEventBlock';

const logger = require('log4js').getLogger('query_node');

export enum QueryNodeState {
    NOT_STARTED,
    STARTING,
    STARTED,
    STOPPING,
    STOPPED,
  }

export default class QueryNode {
    private _state: QueryNodeState;

    private _websocketProvider?: WsProvider;
    
    // API instance for talking to Substrate full node.
    private _queryService?: ISubstrateQueryService;
    private _queryBlockProducer?: QueryBlockProducer;

    // Query index building node.
    //private _indexBuilder: IndexBuilder;

    constructor() {
        this._state = QueryNodeState.NOT_STARTED;
    }

    async build(config: Config) {
        let providerUrl = config.get()?.joystream?.ws_provider || process.env.JOYSTREAM_PROVIDER;
        if (!providerUrl) {
            throw Error('Invalid config: joystream.ws_provider is not set');
        }

        this._websocketProvider = new WsProvider(providerUrl);
        registerJoystreamTypes()

        const api  = await ApiPromise.create({ provider: this._websocketProvider });
        this._queryService = makeQueryService(api);
        this._queryBlockProducer = new QueryBlockProducer(this._queryService, config);
    }

    async run(state: State) {
        if (!this._queryBlockProducer) {
            throw Error('Query block producer is not initialized');
        }

        this._queryBlockProducer.on('QueryEventBlock', (query_event_block: QueryEventBlock): void => {
            this._onQueryEventBlock(query_event_block);
        });

        this._state = QueryNodeState.STARTING;
        await this._queryBlockProducer.start(state.lastProcessedBlock);
        this._state = QueryNodeState.STARTED;

        //while (this._state == QueryNodeState.STARTED) {
        //
        //}
    }

    _onQueryEventBlock(query_event_block: QueryEventBlock): void {
    
        query_event_block.query_events.forEach((query_event, index) => {
            logger.debug(`Processing event: ${query_event.event_name}`);    
        });
      }
    
    get state() {
        return this._state;
    }g

    async stop() {
        this._state = QueryNodeState.STOPPING;
        await this._queryBlockProducer?.stop();
        this._state = QueryNodeState.STOPPED;
    }
}