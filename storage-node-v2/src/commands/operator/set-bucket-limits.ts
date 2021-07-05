import ApiCommandBase from '../../command-base/ApiCommandBase'
import { setStorageBucketVoucherLimits } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import logger from '../../services/logger'

export default class LeaderSetBucketLimits extends ApiCommandBase {
  static description =
    'Set VoucherObjectsSizeLimit and VoucherObjectsNumberLimit for the storage bucket.'

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
    objects: flags.integer({
      char: 'o',
      required: true,
      description: `New 'voucher object number limit' value`,
    }),
    size: flags.integer({
      char: 's',
      required: true,
      description: `New 'voucher object size limit' value`,
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderSetBucketLimits)

    logger.info('Setting storage bucket limits....')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const worker = flags.workerId ?? 0
    const bucket = flags.bucketId ?? 0
    const objectsLimit = flags.objects ?? 0
    const sizeLimit = flags.size ?? 0

    const api = await this.getApi()
    const success = await setStorageBucketVoucherLimits(
      api,
      account,
      worker,
      bucket,
      sizeLimit,
      objectsLimit
    )

    this.exitAfterRuntimeCall(success)
  }
}
