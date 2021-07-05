import { flags } from '@oclif/command'
import { updateBlacklist } from '../../services/runtime/extrinsics'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderUpdateBlacklist extends ApiCommandBase {
  static description =
    'Add/remove a content ID from the blacklist (adds by default).'

  static flags = {
    cid: flags.string({
      char: 'c',
      required: true,
      description: 'Content ID',
    }),
    remove: flags.boolean({
      char: 'r',
      description: 'Remove a content ID from the blaclist',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateBlacklist)

    logger.info('Updating blacklist...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await updateBlacklist(
      api,
      account,
      flags.cid,
      flags.remove
    )

    this.exitAfterRuntimeCall(success)
  }
}

