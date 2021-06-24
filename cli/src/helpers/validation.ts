import BN from 'bn.js'
import ExitCodes from '../ExitCodes'
import { decodeAddress } from '@polkadot/util-crypto'
import { DeriveBalancesAll } from '@polkadot/api-derive/types'
import { CLIError } from '@oclif/errors'

export function validateAddress(address: string, errorMessage = 'Invalid address'): string | true {
  try {
    decodeAddress(address)
  } catch (e) {
    return errorMessage
  }

  return true
}

export function checkBalance(accBalances: DeriveBalancesAll, requiredBalance: BN): void {
  if (requiredBalance.gt(accBalances.availableBalance)) {
    throw new CLIError('Not enough balance available', { exit: ExitCodes.InvalidInput })
  }
}

// We assume balance can be bigger than JavaScript integer
export function isValidBalance(balance: string): boolean {
  return /^[1-9][0-9]{0,38}$/.test(balance)
}
