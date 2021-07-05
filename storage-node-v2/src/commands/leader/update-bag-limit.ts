import ApiCommandBase from '../../command-base/ApiCommandBase'
import { updateStorageBucketsPerBagLimit } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import logger from '../../services/logger'

export default class LeaderUpdateBagLimit extends ApiCommandBase {
  static description =
    'Update StorageBucketsPerBagLimit variable in the Joystream node storage.'

  static flags = {
    limit: flags.integer({
      char: 'l',
      required: true,
      description: 'New StorageBucketsPerBagLimit value',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateBagLimit)

    logger.info('Update "Storage buckets per bag" number limit....')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const limit = flags.limit ?? 0

    const api = await this.getApi()
    const success = await updateStorageBucketsPerBagLimit(api, account, limit)

    this.exitAfterRuntimeCall(success)
  }
}
