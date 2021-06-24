import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { formatBalance } from '@polkadot/util'
import chalk from 'chalk'
import { isValidBalance } from '../../helpers/validation'
import ExitCodes from '../../ExitCodes'
import BN from 'bn.js'

export default class WorkingGroupsDecreaseWorkerStake extends WorkingGroupsCommandBase {
  static description =
    'Decreases given worker stake by an amount that will be returned to the worker role account. ' +
    'Requires lead access.'

  static args = [
    {
      name: 'workerId',
      required: true,
      description: 'Worker ID',
    },
    {
      name: 'amount',
      required: true,
      description: 'Amount of JOY to decrease the current worker stake by',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    const {
      args: { workerId, amount },
    } = this.parse(WorkingGroupsDecreaseWorkerStake)

    // Lead-only gate
    const lead = await this.getRequiredLeadContext()

    const groupMember = await this.getWorkerWithStakeForLeadAction(parseInt(workerId))

    this.log(chalk.white('Current worker stake: ', formatBalance(groupMember.stake)))

    if (!isValidBalance(amount) || groupMember.stake.lt(new BN(amount))) {
      this.error('Invalid amount', { exit: ExitCodes.InvalidInput })
    }

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'decreaseStake',
      [workerId, amount]
    )

    this.log(
      chalk.green(
        `${chalk.white(formatBalance(amount))} from worker ${chalk.white(workerId)} stake ` +
          `has been returned to worker's role account (${chalk.white(groupMember.roleAccount.toString())})!`
      )
    )
  }
}
