import chalk from 'chalk'
import ApiCommandBase from '../../base/ApiCommandBase'
import ExitCodes from '../../ExitCodes'

type ApiSetQueryNodeEndpointArgs = { endpoint: string }

export default class ApiSetQueryNodeEndpoint extends ApiCommandBase {
  static description = 'Set query node endpoint'
  static args = [
    {
      name: 'endpoint',
      required: false,
      description: 'Query node endpoint for the CLI to use',
    },
  ]

  async init() {
    await super.init()
  }

  async run() {
    const { endpoint }: ApiSetQueryNodeEndpointArgs = this.parse(ApiSetQueryNodeEndpoint)
      .args as ApiSetQueryNodeEndpointArgs
    let newEndpoint = ''
    if (endpoint) {
      if (this.isQueryNodeUriValid(endpoint)) {
        await this.setPreservedState({ queryNodeUri: endpoint })
        newEndpoint = endpoint
      } else {
        this.error('Provided endpoint seems to be incorrect!', { exit: ExitCodes.InvalidInput })
      }
    } else {
      newEndpoint = await this.promptForQueryNodeUri()
    }
    this.log(chalk.greenBright('Query node endpoint successfuly changed! New endpoint: ') + chalk.white(newEndpoint))
  }
}
