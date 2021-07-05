import { Command, flags } from '@oclif/command'
import { createApi } from '../services/runtime/api'
import {
  getAccountFromJsonFile,
  getAlicePair,
} from '../services/runtime/accounts'
import { KeyringPair } from '@polkadot/keyring/types'
import { ApiPromise } from '@polkadot/api'
import logger from '../services/logger'
import ExitCodes from './ExitCodes'
import { CLIError } from '@oclif/errors'
import { Input } from '@oclif/parser'

export default abstract class ApiCommandBase extends Command {
  private api: ApiPromise | null = null

  static flags = {
    help: flags.help({ char: 'h' }),
    dev: flags.boolean({ char: 'm', description: 'Use development mode' }),
    apiUrl: flags.string({
      char: 'u',
      description:
        'Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944',
    }),
    keyfile: flags.string({
      char: 'k',
      description:
        'Key file for the account. Mandatory in non-dev environment.',
    }),
    password: flags.string({
      char: 'p',
      description: 'Key file password (optional).',
    }),
  }

  async finally(err: Error | undefined): Promise<void> {
    // called after run and catch regardless of whether or not the command errored
    // We'll force exit here, in case there is no error, to prevent console.log from hanging the process
    if (!err) this.exit(0)
    super.finally(err)
  }

  protected async getApi(): Promise<ApiPromise> {
    if (this.api === null) {
      throw new CLIError('Runtime Api is uninitialized.', {
        exit: ExitCodes.ApiError,
      })
    }

    return this.api
  }

  async init(): Promise<void> {
    // Oclif hack: https://github.com/oclif/oclif/issues/225#issuecomment-490555119
    /* eslint-disable @typescript-eslint/no-explicit-any */
    const { flags } = this.parse(<Input<any>>this.constructor)

    const apiUrl = flags.apiUrl ?? 'ws://localhost:9944'

    this.api = await createApi(apiUrl)

    await this.getApi()
  }

  async ensureDevelopmentChain(): Promise<void> {
    const api = await this.getApi()

    const developmentChainName = 'Development'
    const runningChainName = await api.rpc.system.chain()

    if (runningChainName.toString() !== developmentChainName) {
      throw new CLIError(
        'This command should only be run on a Development chain.',
        { exit: ExitCodes.DevelopmentModeOnly }
      )
    }

    logger.info('Development mode is ON.')
  }

  getAccount(flags: {
    dev?: boolean
    keyfile?: string
    password?: string
  }): KeyringPair {
    const keyfile = flags.keyfile ?? ''
    const password = flags.password

    let account: KeyringPair
    if (flags.dev) {
      account = getAlicePair()
    } else {
      if (keyfile === '') {
        this.error('Keyfile must be set.')
      }

      account = getAccountFromJsonFile(keyfile)
      account.unlock(password)
    }

    return account
  }

  exitAfterRuntimeCall(success: boolean): never {
    let exitCode = ExitCodes.OK
    if (!success) {
      exitCode = ExitCodes.UnsuccessfulRuntimeCall
    }

    this.exit(exitCode)
  }
}
