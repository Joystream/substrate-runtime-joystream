import { registerJoystreamTypes as registerConstantinopoleTypes } from '@constantinopole/types';
import { registerJoystreamTypes as registerNicaeaTypes } from '@nicaea/types';
import { createOldStateApi } from '@joystream/js';

// Those can be overriden with args
const NODE_URI = 'wss://rome-rpc-endpoint.joystream.org:9944/';
const BLOCK_BEFORE_RUNTIME_UPGRADE = 1800000;

export default async function main(
	nodeUri = NODE_URI,
	atBlock = BLOCK_BEFORE_RUNTIME_UPGRADE
) {
	const api = await createOldStateApi(
		nodeUri,
		registerNicaeaTypes,
		registerConstantinopoleTypes,
		atBlock
	);

	// Querying state using api method that no longer exists in Nicaea
	const blockHash = await api.rpc.chain.getBlockHash(atBlock);
    const res = await api.query.dataDirectory.primaryLiaisonAccountId.at(blockHash);
    console.log(
		`api.query.dataDirectory.primaryLiaisonAccountId at block #${atBlock}:`,
		res.toJSON()
	);
}
