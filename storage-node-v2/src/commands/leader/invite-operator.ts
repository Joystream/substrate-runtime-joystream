import { inviteStorageBucketOperator } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderInviteOperator extends ApiCommandBase {
  static description = `Invite a storage bucket operator. Requires storage working group leader permissions.`

  static flags = {
    bucketId: flags.integer({
      char: 'i',
      required: true,
      description: 'Storage bucket ID',
    }),
    operatorId: flags.integer({
      char: 'w',
      required: true,
      description: 'Storage bucket operator ID (storage group worker ID)',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderInviteOperator)

    const storageBucketId = flags.bucketId
    const operatorId = flags.operatorId

    logger.info('Inviting storage bucket operator...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await inviteStorageBucketOperator(
      api,
      account,
      storageBucketId,
      operatorId
    )

    this.exitAfterRuntimeCall(success)
  }
}
