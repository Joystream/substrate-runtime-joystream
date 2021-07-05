import fs from 'fs'
import path from 'path'
import { CLIError } from '@oclif/errors'
import { Keyring } from '@polkadot/keyring'
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types'
import ExitCodes from '../../command-base/ExitCodes'

export function getAccountFromJsonFile(
  jsonBackupFilePath: string
): KeyringPair {
  if (!fs.existsSync(jsonBackupFilePath)) {
    throw new CLIError('Input file does not exist!', {
      exit: ExitCodes.FileError,
    })
  }
  if (path.extname(jsonBackupFilePath) !== '.json') {
    throw new CLIError('Invalid input file: File extension should be .json', {
      exit: ExitCodes.FileError,
    })
  }
  let accountJsonObj: KeyringPair$Json
  try {
    const accountJson = fs.readFileSync(jsonBackupFilePath)
    accountJsonObj = JSON.parse(accountJson.toString())
  } catch (e) {
    throw new CLIError(
      'Provided backup file is not valid or cannot be accessed',
      { exit: ExitCodes.FileError }
    )
  }
  if (typeof accountJsonObj !== 'object' || accountJsonObj === null) {
    throw new CLIError('Provided backup file is not valid', {
      exit: ExitCodes.FileError,
    })
  }

  const keyring = new Keyring()
  let account: KeyringPair
  try {
    // Try adding and retrieving the keys in order to validate that the backup file is correct
    keyring.addFromJson(accountJsonObj)
    account = keyring.getPair(accountJsonObj.address)
  } catch (e) {
    throw new CLIError('Provided backup file is not valid', {
      exit: ExitCodes.FileError,
    })
  }

  return account
}

// Returns 'Alice' keypair. Should be use in dev-mode only.
export function getAlicePair(): KeyringPair {
  const keyring = new Keyring({ type: 'sr25519' })
  return keyring.addFromUri('//Alice')
}
