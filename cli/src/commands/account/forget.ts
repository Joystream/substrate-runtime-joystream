import fs from 'fs'
import chalk from 'chalk'
import ExitCodes from '../../ExitCodes'
import AccountsCommandBase from '../../base/AccountsCommandBase'

export default class AccountForget extends AccountsCommandBase {
  static description = 'Forget (remove) account from the list of available accounts'

  async run() {
    const selecteKey = await this.promptForAccount('Select an account to forget', false, false)
    await this.requireConfirmation('Are you sure you want to PERMANENTLY FORGET this account?')

    const accountFilePath = this.getAccountFilePath(this.getPair(selecteKey).meta.name)

    try {
      fs.unlinkSync(accountFilePath)
    } catch (e) {
      this.error(`Could not remove account file (${accountFilePath}). Permissions issue?`, {
        exit: ExitCodes.FsOperationFailed,
      })
    }

    this.log(chalk.greenBright(`\nAccount has been forgotten!`))
  }
}
