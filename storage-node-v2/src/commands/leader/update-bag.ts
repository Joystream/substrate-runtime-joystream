import { flags } from '@oclif/command'
import { updateStorageBucketsForBag } from '../../services/runtime/extrinsics'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import { parseBagId } from '../../services/helpers/bagTypes'
import logger from '../../services/logger'

export default class LeaderUpdateBag extends ApiCommandBase {
  static description =
    'Add/remove a storage bucket from a bag (adds by default).'

  static flags = {
    bucket: flags.integer({
      char: 'b',
      required: true,
      description: 'Storage bucket ID',
    }),
    remove: flags.boolean({
      char: 'r',
      description: 'Remove a bucket from the bag',
    }),
    bagId: flags.string({
      char: 'i',
      required: true,
      description: `
      Bag ID. Format: {bag_type}:{sub_type}:{id}.
      - Bag types: 'static', 'dynamic'
      - Sub types: 'static:council', 'static:wg', 'dynamic:member', 'dynamic:channel'
      - Id: 
        - absent for 'static:council'
        - working group name for 'static:wg'
        - integer for 'dynamic:member' and 'dynamic:channel'
      Examples:
      - static:council
      - static:wg:storage
      - dynamic:member:4
      `,
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateBag)

    const bucket = flags.bucket ?? 0

    logger.info('Updating the bag...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()
    const bagId = parseBagId(api, flags.bagId)

    const success = await updateStorageBucketsForBag(
      api,
      bagId,
      account,
      bucket,
      flags.remove
    )

    this.exitAfterRuntimeCall(success)
  }
}
