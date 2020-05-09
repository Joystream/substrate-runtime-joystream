//import { BlockNumber } from '@polkadot/types/interfaces';
import QueryEvent from './QueryEvent';

export default class QueryEventBlock {

    readonly block_number: number;

    readonly query_events: QueryEvent[];

    constructor(block_number: number, query_events: QueryEvent[]) {
        this.block_number = block_number;
        this.query_events = query_events;
    }
}