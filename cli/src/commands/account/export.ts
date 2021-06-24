import fs from 'fs'
import chalk from 'chalk'
import path from 'path'
import ExitCodes from '../../ExitCodes'
import AccountsCommandBase from '../../base/AccountsCommandBase'
import { flags } from '@oclif/command'

type AccountExportArgs = { destPath: string }

export default class AccountExport extends AccountsCommandBase {
  static description = 'Export account(s) to given location'
  static MULTI_EXPORT_FOLDER_NAME = 'exported_accounts'

  static args = [
    {
      name: 'destPath',
      required: true,
      description: 'Path where the exported files should be placed',
    },
  ]

  static flags = {
    name: flags.string({
      char: 'n',
      description: 'Name of the account to export',
      required: false,
      exclusive: ['all'],
    }),
    all: flags.boolean({
      char: 'a',
      description: `If provided, exports all existing accounts into "${AccountExport.MULTI_EXPORT_FOLDER_NAME}" folder inside given path`,
      required: false,
      exclusive: ['name'],
    }),
  }

  exportAccount(name: string, destPath: string): string {
    const sourceFilePath: string = this.getAccountFilePath(name)
    const destFilePath: string = path.join(destPath, this.getAccountFileName(name))
    try {
      fs.copyFileSync(sourceFilePath, destFilePath)
    } catch (e) {
      this.error(`Error while trying to copy into the export file: (${destFilePath}). Permissions issue?`, {
        exit: ExitCodes.FsOperationFailed,
      })
    }

    return destFilePath
  }

  async run() {
    const { destPath } = this.parse(AccountExport).args as AccountExportArgs
    let { name, all } = this.parse(AccountExport).flags
    const accounts = this.fetchAccounts()

    if (all) {
      const exportPath: string = path.join(destPath, AccountExport.MULTI_EXPORT_FOLDER_NAME)
      try {
        if (!fs.existsSync(exportPath)) fs.mkdirSync(exportPath)
      } catch (e) {
        this.error(`Failed to create the export folder (${exportPath})`, { exit: ExitCodes.FsOperationFailed })
      }
      for (const acc of accounts) {
        this.exportAccount(acc.meta.name, exportPath)
      }
      this.log(chalk.greenBright(`All accounts succesfully exported to: ${chalk.white(exportPath)}!`))
    } else {
      if (!name) {
        name = await this.promptForAccount()
      }
      const exportedFilePath: string = this.exportAccount(name, destPath)
      this.log(chalk.greenBright(`Account succesfully exported to: ${chalk.white(exportedFilePath)}`))
    }
  }
}
