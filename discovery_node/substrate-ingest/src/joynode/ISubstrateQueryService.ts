import { Hash, Header, BlockNumber, EventRecord, SignedBlock } from '@polkadot/types/interfaces';
import { Callback, Codec } from '@polkadot/types/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { ApiPromise} from '@polkadot/api';

/**
 * @description ...
 */
export default interface ISubstrateQueryService {

    getFinalizedHead(): Promise<Hash>;
    getHeader(hash?: Hash | Uint8Array | string): Promise<Header>;
    subscribeNewHeads(v: Callback<Header> ): UnsubscribePromise;
    subscribeFinalizedHeads(v: Callback<Header>): UnsubscribePromise;
    getBlockHash(blockNumber?: BlockNumber | Uint8Array | number | string): Promise<Hash>;
    getBlock(hash?: Hash | Uint8Array | string): Promise<SignedBlock>;
    // Cut down from at: (hash: Hash | Uint8Array | string, ...args: Parameters<F>) => PromiseOrObs<ApiType, ObsInnerType<ReturnType<F>>>;
    eventsAt(hash: Hash | Uint8Array | string): Promise<EventRecord[] & Codec>;
    substrateAPI(): ApiPromise;
    //eventsRange()
    //events()   
}

export function makeQueryService(api: ApiPromise): ISubstrateQueryService {

    return  { 
        getHeader: (hash?: Hash | Uint8Array | string) => { return api.rpc.chain.getHeader(hash)},
        getFinalizedHead: () => { return api.rpc.chain.getFinalizedHead();}, 
        subscribeNewHeads: (v: Callback<Header> ) => { return api.rpc.chain.subscribeNewHeads(v); },
        subscribeFinalizedHeads: (v: Callback<Header> ) => { return api.rpc.chain.subscribeFinalizedHeads(v); },
        getBlockHash: (blockNumber?: BlockNumber | Uint8Array | number | string) => { return api.rpc.chain.getBlockHash(blockNumber); },
        getBlock: (hash?: Hash | Uint8Array | string) => { return api.rpc.chain.getBlock(hash); },
        eventsAt: (hash: Hash | Uint8Array | string) => { return api.query.system.events.at(hash); },
        substrateAPI: () => { return api; }
    } as ISubstrateQueryService;
}