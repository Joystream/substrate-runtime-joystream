import { ApiPromise, WsProvider } from "@polkadot/api";
import MetadataVersioned from "@polkadot/types/Metadata/MetadataVersioned";

export async function createOldStateApi(
	newNodeUri: string,
	registerNewTypes: () => void,
	registerOldTypes: () => void,
	oldBlockNumber: number
): Promise<ApiPromise> {
	// Create initial api instance
	const initWsProvider = new WsProvider(newNodeUri);
	registerNewTypes();
	const initApi = await ApiPromise.create({ provider: initWsProvider });
	// Get required data in order to initialize the final api
	const oldBlockHash = await initApi.rpc.chain.getBlockHash(oldBlockNumber);
	const [genesisHash, oldMetadata, runtimeVersion] = await Promise.all([
		initApi.rpc.chain.getBlockHash(0),
		initApi.rpc.state.getMetadata(oldBlockHash),
		initApi.rpc.state.getRuntimeVersion(),
	]);
	// Create metadata key required by @polkadot/api and use it to create "metadataArg"
	const metadataKey = `${genesisHash}-${runtimeVersion.specVersion}`;
	const metadataArg = { [metadataKey]: new MetadataVersioned(oldMetadata.toJSON()).toHex() };
	initApi.disconnect();
	// Create final api (current node, old metadata and types)
	const wsProvider = new WsProvider(newNodeUri);
	registerOldTypes();
	return await ApiPromise.create({ provider: wsProvider, metadata: metadataArg });
}
