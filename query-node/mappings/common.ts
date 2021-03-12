import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'

import { Block } from 'query-node/dist/src/modules/block/block.model'
import { Network } from 'query-node/src/modules/enums/enums'

const currentNetwork = Network.BABYLON

// prepare block record
export async function prepareBlock(db: DatabaseManager, event: SubstrateEvent): Promise<Block> {
  let block = await db.get(Block, { where: { block: event.blockNumber } })

  if (block) {
      return block
  }

  return new Block({
    block: event.blockNumber,
    executedAt: new Date(event.blockTimestamp.toNumber()),
    network: currentNetwork,
  })
}