import chalk from 'chalk'
import ApiCommandBase from '../../base/ApiCommandBase'
import ExitCodes from '../../ExitCodes'

type ApiSetUriArgs = { uri: string }

export default class ApiSetUri extends ApiCommandBase {
  static description = 'Set api WS provider uri'
  static args = [
    {
      name: 'uri',
      required: false,
      description: 'Uri of the node api WS provider (if skipped, a prompt will be displayed)',
    },
  ]

  async init() {
    await super.init()
  }

  async run() {
    const args: ApiSetUriArgs = this.parse(ApiSetUri).args as ApiSetUriArgs
    let newUri = ''
    if (args.uri) {
      if (this.isApiUriValid(args.uri)) {
        await this.setPreservedState({ apiUri: args.uri })
        newUri = args.uri
      } else {
        this.error('Provided uri seems to be incorrect!', { exit: ExitCodes.InvalidInput })
      }
    } else {
      newUri = await this.promptForApiUri()
    }
    this.log(chalk.greenBright('Api uri successfuly changed! New uri: ') + chalk.white(newUri))
  }
}
