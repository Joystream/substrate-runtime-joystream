import Config from './Config'

export class State {
    private _lastProcessedBlock: number;

    constructor() {
        this._lastProcessedBlock = 0;
    }

    get lastProcessedBlock(): number {
         return this._lastProcessedBlock;
    }

    set lastProcessedBlock(blkNum: number) {
        this._lastProcessedBlock = blkNum;
    }
}

export class StateKeeper {
    constructor (config: Config) {

    }

    async state(): Promise<State> {
        return new State();
    }
}

module.exports = {
    State, StateKeeper
}