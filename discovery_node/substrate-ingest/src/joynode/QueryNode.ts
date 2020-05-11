// @ts-check
import { ApiPromise, WsProvider } from '@polkadot/api';
import { registerJoystreamTypes } from '@joystream/types';
import Config from '../Config'
import { State } from '../state/StateKeeper'
import ISubstrateQueryService, { makeQueryService } from './ISubstrateQueryService';
import QueryBlockProducer from './QueryBlockProducer';
import QueryEventBlock from './QueryEventBlock';
import { EventEmitter } from 'events';
import StateBootstrap from './StateBootstrap';

const logger = require('log4js').getLogger('query_node');

export enum QueryNodeState {
    NOT_STARTED,
    STARTING,
    STARTED,
    STOPPING,
    STOPPED,
  }

 /**
  * This class encapsulates the data ingestion from the JoyStream node.
  * Currently it has 
  *  - a QueryBlockProducer which reads off all the events contained in the 
  * blocks through the Substrate API, 
  *  - StateBootstrap which is responsible for reading off blockchain state at a given height. 
  */ 
export default class QueryNode extends EventEmitter {
    private _state: QueryNodeState;

    private _websocketProvider?: WsProvider;
    
    // API instance for talking to Substrate full node.
    private _queryService?: ISubstrateQueryService;
    private _queryBlockProducer?: QueryBlockProducer;
    private _stateBootstrap?: StateBootstrap;
    private _api?: ApiPromise;

    // Query index building node.
    //private _indexBuilder: IndexBuilder;

    constructor() {
        super();
        this._state = QueryNodeState.NOT_STARTED;
    }

    async build(config: Config) {
        let providerUrl = config.get()?.joystream?.ws_provider || process.env.JOYSTREAM_PROVIDER;
        if (!providerUrl) {
            throw Error('Invalid config: joystream.ws_provider is not set');
        }

        this._websocketProvider = new WsProvider(providerUrl);
        registerJoystreamTypes();
        
        this._api  = await ApiPromise.create({ provider: this._websocketProvider });
        this._queryService = makeQueryService(this._api);
        this._queryBlockProducer = new QueryBlockProducer(this._queryService, config);
        this._stateBootstrap = new StateBootstrap(this._queryService, config);
    }

    async start(state: State) {
        if (!this._queryBlockProducer) {
            throw Error('Query block producer is not initialized');
        }

        //this._queryBlockProducer.on('QueryEventBlock', (query_event_block: QueryEventBlock): void => {
        //    this._onQueryEventBlock(query_event_block);
        //});

        this._state = QueryNodeState.STARTED;
        logger.debug(`Starting at state ${state}`);
        await this._queryBlockProducer.start(state.inBlock + 1);
        
    }

    get producer(): QueryBlockProducer {
        if (this._queryBlockProducer == undefined) {
            throw new Error("QueryBlockProducer is not initialized");
        }
        return this._queryBlockProducer;
    }

    get bootstrapper(): StateBootstrap {
        if (this._stateBootstrap == undefined) {
            throw new Error("Bootstrapper is not initialized");
        }
        return this._stateBootstrap;
    }
    
    get state() {
        return this._state;
    }

    async stop() {
        logger.info("Stopping the query node");
        this._state = QueryNodeState.STOPPING;
        await this._queryBlockProducer?.stop();
        this._state = QueryNodeState.STOPPED;
    }
}