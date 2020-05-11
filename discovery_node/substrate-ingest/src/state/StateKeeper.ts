import Config from '../Config'

const logger = require('log4js').getLogger('state-keeper');

export interface State {
    inBlock: number;  
    lastProcessedEventInBlock: number;  
}

/**
 * This class is currently very thin. It's purpose is to keep the whole state of the ETL
 * process by taking the blocks from the producer, sending to the transformer (not yet implemented)
 * and finally to the the loader (ESUploader). All the intermediary steps are going to change the state,
 * which in turn will also be more fine-grained. This will enable fail-safe restart and observability using 
 * e.g. prometheus exporter listening to the state changes and the stats. 
 */
export class StateKeeper {
    private _config: Config;
    private _state: State;

    constructor (config: Config) {
        this._config = config;
        this._state = StateKeeper.nullState();
    }

    set state(s: State) {
        logger.info(`State updated: ${JSON.stringify(s, null, 2)}`);
        this._state = s;
    }

    public static nullState(): State {
        return {
            inBlock: -1,
            lastProcessedEventInBlock: 0
        }
    }

    public shouldBootstrap(): boolean {
        return (this._state.inBlock < 0)
    }
}

module.exports = {
    StateKeeper
}