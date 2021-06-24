import AccountsCommandBase, { DEFAULT_ACCOUNT_TYPE, KEYRING_OPTIONS } from '../../base/AccountsCommandBase'
import { flags } from '@oclif/command'
import Keyring from '@polkadot/keyring'
import { KeypairType } from '@polkadot/util-crypto/types'

export default class AccountImport extends AccountsCommandBase {
  static description = 'Import account using mnemonic phrase, seed, suri or json backup file'

  static flags = {
    name: flags.string({
      required: false,
      description: 'Account name',
    }),
    mnemonic: flags.string({
      required: false,
      description: 'Mnemonic phrase',
      exclusive: ['backupFilePath', 'seed', 'suri'],
    }),
    seed: flags.string({
      required: false,
      description: 'Secret seed',
      exclusive: ['backupFilePath', 'mnemonic', 'suri'],
    }),
    backupFilePath: flags.string({
      required: false,
      description: 'Path to account backup JSON file',
      exclusive: ['mnemonic', 'seed', 'suri'],
    }),
    suri: flags.string({
      required: false,
      description: 'Substrate uri',
      exclusive: ['mnemonic', 'seed', 'backupFilePath'],
    }),
    type: flags.enum<KeypairType>({
      required: false,
      description: `Account type (defaults to ${DEFAULT_ACCOUNT_TYPE})`,
      options: ['sr25519', 'ed25519'],
      exclusive: ['backupFilePath'],
    }),
  }

  async run() {
    const { name, mnemonic, seed, backupFilePath, suri, type } = this.parse(AccountImport).flags

    const keyring = new Keyring(KEYRING_OPTIONS)

    if (mnemonic) {
      keyring.addFromMnemonic(mnemonic, {}, type)
    } else if (seed) {
      keyring.addFromSeed(Buffer.from(seed), {}, type)
    } else if (suri) {
      keyring.addFromUri(suri, {}, type)
    } else if (backupFilePath) {
      const pair = this.fetchAccountFromJsonFile(backupFilePath)
      keyring.addPair(pair)
    } else {
      this._help()
      return
    }

    await this.createAccount(name, keyring.getPairs()[0])
  }
}
