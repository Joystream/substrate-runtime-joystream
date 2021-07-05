import { updateDataSizeFee } from '../../services/runtime/extrinsics'
import { flags } from '@oclif/command'
import ApiCommandBase from '../../command-base/ApiCommandBase'
import logger from '../../services/logger'

export default class LeaderUpdateDataFee extends ApiCommandBase {
  static description = `Update data size fee. Requires storage working group leader permissions.`

  static flags = {
    fee: flags.integer({
      char: 'f',
      required: true,
      description: 'New data size fee',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateDataFee)

    const fee = flags.fee

    logger.info('Updating data size fee...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    const success = await updateDataSizeFee(api, account, fee)

    this.exitAfterRuntimeCall(success)
  }
}
