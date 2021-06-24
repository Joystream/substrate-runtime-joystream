import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import chalk from 'chalk'

export default class AddCuratorGroupCommand extends ContentDirectoryCommandBase {
  static description = 'Create new Curator Group.'
  static aliases = ['addCuratorGroup']

  async run() {
    const account = await this.getRequiredSelectedAccount()
    await this.requireLead()

    await this.requestAccountDecoding(account)
    await this.buildAndSendExtrinsic(account, 'contentDirectory', 'addCuratorGroup')

    // TODO: Get id from event?
    console.log(chalk.green(`New group succesfully created!`))
  }
}
