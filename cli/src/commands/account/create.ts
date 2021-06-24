import AccountsCommandBase, { DEFAULT_ACCOUNT_TYPE } from '../../base/AccountsCommandBase'
import { KeypairType } from '@polkadot/util-crypto/types'
import { flags } from '@oclif/command'

export default class AccountCreate extends AccountsCommandBase {
  static description = 'Create a new account'

  static flags = {
    name: flags.string({
      required: false,
      description: 'Account name',
    }),
    type: flags.enum<KeypairType>({
      required: false,
      description: `Account type (defaults to ${DEFAULT_ACCOUNT_TYPE})`,
      options: ['sr25519', 'ed25519'],
    }),
  }

  async run() {
    const { name, type } = this.parse(AccountCreate).flags
    await this.createAccount(name, undefined, undefined, type)
  }
}
