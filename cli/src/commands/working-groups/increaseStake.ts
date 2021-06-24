import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { formatBalance } from '@polkadot/util'
import chalk from 'chalk'
import ExitCodes from '../../ExitCodes'
import { isValidBalance } from '../../helpers/validation'

export default class WorkingGroupsIncreaseStake extends WorkingGroupsCommandBase {
  static description = 'Increases current role (lead/worker) stake. Requires active role account to be selected.'
  static args = [
    {
      name: 'amount',
      required: true,
      description: 'Amount of JOY to increase the current stake by',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    // Worker-only gate
    const worker = await this.getRequiredWorkerContext()

    const {
      args: { amount },
    } = this.parse(WorkingGroupsIncreaseStake)

    if (!isValidBalance(amount)) {
      this.error('Invalid stake amount!', { exit: ExitCodes.InvalidInput })
    }

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(worker.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'increaseStake',
      [worker.workerId, amount]
    )

    this.log(
      chalk.green(
        `Worker ${chalk.white(worker.workerId.toString())} stake has been increased by ${chalk.white(
          formatBalance(amount)
        )}`
      )
    )
  }
}
