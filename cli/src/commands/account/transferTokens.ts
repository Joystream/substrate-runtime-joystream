import { flags } from '@oclif/command'
import BN from 'bn.js'
import AccountsCommandBase from '../../base/AccountsCommandBase'
import ExitCodes from '../../ExitCodes'
import { checkBalance, isValidBalance, validateAddress } from '../../helpers/validation'

export default class AccountTransferTokens extends AccountsCommandBase {
  static description = 'Transfer tokens from any of the available accounts'

  static flags = {
    from: flags.string({
      required: false,
      description: 'Address of the sender (can also be provided interactively)',
    }),
    to: flags.string({
      required: false,
      description: 'Address of the recipient (can also be provided interactively)',
    }),
    amount: flags.string({
      required: true,
      description: 'Amount of tokens to transfer',
    }),
  }

  async run() {
    let { from, to, amount } = this.parse(AccountTransferTokens).flags

    if (!isValidBalance(amount)) {
      this.error('Invalid transfer amount', { exit: ExitCodes.InvalidInput })
    }

    // Initial validation
    if (!from) {
      from = await this.promptForAccount('Select sender account')
    } else if (!this.isKeyAvailable(from)) {
      this.error('Sender key not available', { exit: ExitCodes.InvalidInput })
    }

    if (!to) {
      to = await this.promptForAnyAddress('Select recipient')
    } else if (validateAddress(to) !== true) {
      this.error('Invalid recipient address', { exit: ExitCodes.InvalidInput })
    }

    const accBalances = (await this.getApi().getAccountsBalancesInfo([from]))[0]
    checkBalance(accBalances, new BN(amount))

    await this.sendAndFollowNamedTx(await this.getDecodedPair(from), 'balances', 'transferKeepAlive', [to, amount])
  }
}
