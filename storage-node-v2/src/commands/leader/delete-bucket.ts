import { deleteStorageBucket } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderDeleteBucket extends ApiCommandBase {
  static description = `Delete a storage bucket. Requires storage working group leader permissions.`

  static flags = {
    bucketId: flags.integer({
      char: 'i',
      required: true,
      description: 'Storage bucket ID',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderDeleteBucket)

    const storageBucketId = flags.bucketId

    logger.info('Deleting storage bucket...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await deleteStorageBucket(api, account, storageBucketId)

    this.exitAfterRuntimeCall(success)
  }
}
