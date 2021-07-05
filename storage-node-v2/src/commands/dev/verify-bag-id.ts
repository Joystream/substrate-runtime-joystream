import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import { parseBagId } from '../../services/helpers/bagTypes'
import logger from '../../services/logger'

export default class DevVerifyBagId extends ApiCommandBase {
  static description =
    'The command verifies bag id supported by the storage node. Requires chain connection.'

  static flags = {
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
    const { flags } = this.parse(DevVerifyBagId)

    const api = await this.getApi()
    parseBagId(api, flags.bagId)

    logger.info(`Correct bag id: ${flags.bagId}`)
  }
}
