import { removeStorageBucketOperator } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderRemoveOperator extends ApiCommandBase {
  static description = `Remove a storage bucket operator. Requires storage working group leader permissions.`

  static flags = {
    bucketId: flags.integer({
      char: 'i',
      required: true,
      description: 'Storage bucket ID',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderRemoveOperator)

    const storageBucketId = flags.bucketId

    logger.info('Removing storage bucket operator...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await removeStorageBucketOperator(
      api,
      account,
      storageBucketId
    )

    this.exitAfterRuntimeCall(success)
  }
}
