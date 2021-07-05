import { cancelStorageBucketOperatorInvite } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderCancelInvite extends ApiCommandBase {
  static description = `Cancel a storage bucket operator invite. Requires storage working group leader permissions.`

  static flags = {
    bucketId: flags.integer({
      char: 'i',
      required: true,
      description: 'Storage bucket ID',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderCancelInvite)

    const storageBucketId = flags.bucketId

    logger.info('Canceling storage bucket operator invite...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await cancelStorageBucketOperatorInvite(
      api,
      account,
      storageBucketId
    )

    this.exitAfterRuntimeCall(success)
  }
}
