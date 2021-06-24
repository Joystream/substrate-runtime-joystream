import AccountsCommandBase from '../../base/AccountsCommandBase'
import ExitCodes from '../../ExitCodes'
import { validateAddress } from '../../helpers/validation'
import { NameValueObj } from '../../Types'
import { displayHeader, displayNameValueTable } from '../../helpers/display'
import { formatBalance } from '@polkadot/util'
import moment from 'moment'

export default class AccountInfo extends AccountsCommandBase {
  static description = 'Display detailed information about specified account'
  static aliases = ['account:inspect']
  static args = [
    { name: 'address', required: false, description: 'An address to inspect (can also be provided interavtively)' },
  ]

  async run() {
    let { address } = this.parse(AccountInfo).args

    if (!address) {
      address = await this.promptForAnyAddress()
    } else if (validateAddress(address) !== true) {
      this.error('Invalid address', { exit: ExitCodes.InvalidInput })
    }

    const summary = await this.getApi().getAccountSummary(address)

    displayHeader('Account information')
    const accountRows: NameValueObj[] = [{ name: 'Address:', value: address }]
    if (this.isKeyAvailable(address)) {
      const pair = this.getPair(address)
      accountRows.push({ name: 'Account name', value: pair.meta.name })
      accountRows.push({ name: 'Type', value: pair.type })
      const creationDate = pair.meta.whenCreated
        ? moment(pair.meta.whenCreated as string | number).format('YYYY-MM-DD HH:mm:ss')
        : null
      if (creationDate) {
        accountRows.push({ name: 'Creation date', value: creationDate })
      }
    }
    displayNameValueTable(accountRows)

    displayHeader('Balances')
    const balances = summary.balances
    const balancesRows: NameValueObj[] = [
      { name: 'Total balance:', value: formatBalance(balances.votingBalance) },
      { name: 'Transferable balance:', value: formatBalance(balances.availableBalance) },
    ]
    if (balances.lockedBalance.gtn(0)) {
      balancesRows.push({ name: 'Locked balance:', value: formatBalance(balances.lockedBalance) })
    }
    if (balances.reservedBalance.gtn(0)) {
      balancesRows.push({ name: 'Reserved balance:', value: formatBalance(balances.reservedBalance) })
    }
    displayNameValueTable(balancesRows)
  }
}
