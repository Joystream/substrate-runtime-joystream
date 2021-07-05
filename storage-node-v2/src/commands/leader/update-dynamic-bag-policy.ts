import ApiCommandBase from '../../command-base/ApiCommandBase'
import { updateNumberOfStorageBucketsInDynamicBagCreationPolicy } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import logger from '../../services/logger'
import { parseDynamicBagType } from '../../services/helpers/bagTypes'

export default class LeaderUpdateDynamicBagPolicy extends ApiCommandBase {
  static description =
    'Update number of storage buckets used in the dynamic bag creation policy.'

  static flags = {
    number: flags.integer({
      char: 'n',
      required: true,
      description: 'New storage buckets number',
    }),
    member: flags.boolean({
      char: 'e',
      description: 'Member dynamic bag type (default)',
    }),
    channel: flags.boolean({
      char: 'c',
      description: 'Channel dynamic bag type',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateDynamicBagPolicy)

    logger.info('Update "Storage buckets per bag" number limit....')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const newNumber = flags.number ?? 0

    let dynamicBagTypeString: 'Member' | 'Channel' = 'Member' // Default
    if (flags.channel) {
      dynamicBagTypeString = 'Channel'
    }

    const api = await this.getApi()
    const dynamicBagType = parseDynamicBagType(api, dynamicBagTypeString)
    const success =
      await updateNumberOfStorageBucketsInDynamicBagCreationPolicy(
        api,
        account,
        dynamicBagType,
        newNumber
      )

    this.exitAfterRuntimeCall(success)
  }
}
