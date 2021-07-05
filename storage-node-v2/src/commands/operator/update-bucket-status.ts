import { flags } from '@oclif/command'
import { updateStorageBucketStatus } from '../../services/runtime/extrinsics'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class OperatorUpdateStorageBucketStatus extends ApiCommandBase {
  static description = 'Update storage bucket status (accepting new bags).'

  static flags = {
    workerId: flags.integer({
      char: 'w',
      required: true,
      description: 'Storage operator worker ID',
    }),
    bucketId: flags.integer({
      char: 'i',
      required: true,
      description: 'Storage bucket ID',
    }),
    enable: flags.boolean({
      char: 'e',
      description: 'Enables accepting new bags (default).',
    }),
    disable: flags.boolean({
      char: 'd',
      description: 'Disables accepting new bags.',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(OperatorUpdateStorageBucketStatus)

    const worker = flags.workerId ?? 0
    const bucket = flags.bucketId ?? 0
    const disable = flags.disable
    const newStatus = !disable

    logger.info('Updating the storage bucket status...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)

    const api = await this.getApi()
    const success = await updateStorageBucketStatus(
      api,
      account,
      worker,
      bucket,
      newStatus
    )

    this.exitAfterRuntimeCall(success)
  }
}
