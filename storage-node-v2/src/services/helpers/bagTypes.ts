import { BagId, DynamicBagType, Static } from '@joystream/types/storage'
import { ApiPromise } from '@polkadot/api'
import ExitCodes from '../../command-base/ExitCodes'
import { CLIError } from '@oclif/errors'

export function parseBagId(api: ApiPromise, bagId: string): BagId {
  const bagIdParts = bagId.toLowerCase().split(':')

  if (bagIdParts.length > 3 || bagIdParts.length < 2) {
    throw new CLIError(`Invalid bagId: ${bagId}`, {
      exit: ExitCodes.InvalidParameters,
    })
  }

  if (bagIdParts[0] === 'static') {
    return parseStaticBagId(api, bagId, bagIdParts)
  }

  if (bagIdParts[0] === 'dynamic') {
    return parseDynamicBagId()
  }

  throw new CLIError(`Invalid bagId: ${bagId}`, {
    exit: ExitCodes.InvalidParameters,
  })
}

function parseStaticBagId(
  api: ApiPromise,
  bagId: string,
  bagIdParts: string[]
): BagId {
  if (bagIdParts[1] === 'council') {
    if (bagIdParts.length === 2) {
      const staticBagId: Static = api.createType('Static', 'Council')
      const constructedBagId: BagId = api.createType('BagId', {
        'Static': staticBagId,
      })

      return constructedBagId
    }
  }

  throw new CLIError(`Invalid bagId: ${bagId}`, {
    exit: ExitCodes.InvalidParameters,
  })
}

function parseDynamicBagId(): BagId {
  throw new CLIError('Function not implemented.', {
    exit: ExitCodes.InvalidParameters,
  })
}

export function parseDynamicBagType(
  api: ApiPromise,
  bagType: 'Member' | 'Channel'
): DynamicBagType {
  return api.createType('DynamicBagType', bagType)
}
