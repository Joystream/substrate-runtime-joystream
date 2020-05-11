import ISubstrateQueryService from './ISubstrateQueryService';
import { ApiPromise } from '@polkadot/api';

import { IProfile } from '@joystream/types/lib/members';
import { Hash } from '@polkadot/types/interfaces';
import { Codec } from '@polkadot/types/types';
import { EventEmitter } from 'events';
import Config from '../Config';
import { JoyStruct } from '@joystream/types/lib/JoyStruct';
const logger = require('log4js').getLogger('state-boot');

const DEFAULT_BOOTSTRAP_HEIGHT = 0;

export interface JoyEntity {
    entity: Object,
    type: string,
    blockNumber: number,
    id: any
}

export default class StateBootstrap extends EventEmitter {
    private _queryService: ISubstrateQueryService;
    private _api: ApiPromise;
    private _blockHeight: number
    
    constructor(queryService: ISubstrateQueryService, config: Config) {
        super();
        this._queryService = queryService;
        this._api = queryService.substrateAPI();
        this._blockHeight = config.get().bootstrap?.block || DEFAULT_BOOTSTRAP_HEIGHT;
        logger.debug(`Bootstrap from block at height ${this._blockHeight}`);
    }      

    public async * fetch(): AsyncGenerator<JoyEntity> {
        yield* this.members(this._blockHeight);
        // TODO: fetch other stuff like content etc
    }
    
    public async * members(height: number): AsyncGenerator<JoyEntity> {
        let blkHash: Hash = await this._queryService.getBlockHash(height);
        let ids: Codec = await this._api.query.members.membersCreated.at(blkHash);
        let num: number = parseInt(ids.toString())
        for (let i = 0; i < num; i++) {
            let profile = await this._api.query.members.memberProfile.at(blkHash, i) as JoyStruct<IProfile>;
            // TODO: this looks ugly, what is the proper way of type casting?
            let profileJSON = profile.toJSON() as Object;
            logger.trace(`BootstrapEnitity: ${JSON.stringify(profile, null, 2)}`);
            yield { 
                entity: profileJSON as IProfile, 
                type: "profile",
                blockNumber: height,
                id: (profileJSON as IProfile).handle
            };
        }
    }

    
}