import { flags } from '@oclif/command'
import { setStorageOperatorMetadata } from '../../services/runtime/extrinsics'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class OperatorSetMetadata extends ApiCommandBase {
  static description = 'Accept pending storage bucket invitation.'

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
    metadata: flags.string({
      char: 'm',
      description: 'Storage bucket operator metadata',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(OperatorSetMetadata)

    const operator = flags.operatorId ?? 0
    const bucket = flags.bucketId ?? 0
    const metadata = flags.metadata ?? ''

    logger.info('Setting the storage operator metadata...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)

    const api = await this.getApi()
    const success = await setStorageOperatorMetadata(
      api,
      account,
      operator,
      bucket,
      metadata
    )

    this.exitAfterRuntimeCall(success)
  }
}
