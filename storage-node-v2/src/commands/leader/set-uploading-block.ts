import { updateUploadingBlockedStatus } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderSetUploadingBlock extends ApiCommandBase {
  static description = `Set global uploading block. Requires storage working group leader permissions.`

  static flags = {
    enable: flags.boolean({
      char: 'e',
      description: 'Enables global uploading block (default).',
    }),
    disable: flags.boolean({
      char: 'd',
      description: 'Disables global uploading block.',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderSetUploadingBlock)

    const disable = flags.disable
    const newStatus = !disable

    logger.info('Setting global uploading block...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await updateUploadingBlockedStatus(api, account, newStatus)

    this.exitAfterRuntimeCall(success)
  }
}
