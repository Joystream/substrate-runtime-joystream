import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { formatBalance } from '@polkadot/util'
import chalk from 'chalk'
import { flags } from '@oclif/command'
import { isValidBalance } from '../../helpers/validation'
import ExitCodes from '../../ExitCodes'
import BN from 'bn.js'

export default class WorkingGroupsSlashWorker extends WorkingGroupsCommandBase {
  static description = 'Slashes given worker stake. Requires lead access.'
  static args = [
    {
      name: 'workerId',
      required: true,
      description: 'Worker ID',
    },
    {
      name: 'amount',
      required: true,
      description: 'Slash amount',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
    rationale: flags.string({
      name: 'Optional rationale',
      required: false,
    }),
  }

  async run() {
    const {
      args: { amount, workerId },
      flags: { rationale },
    } = this.parse(WorkingGroupsSlashWorker)

    // Lead-only gate
    const lead = await this.getRequiredLeadContext()

    const groupMember = await this.getWorkerWithStakeForLeadAction(parseInt(workerId))

    this.log(chalk.white('Current worker stake: ', formatBalance(groupMember.stake)))

    if (!isValidBalance(amount) || groupMember.stake.lt(new BN(amount))) {
      this.error('Invalid slash amount', { exit: ExitCodes.InvalidInput })
    }

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'slashStake',
      [workerId, amount, rationale || null]
    )

    this.log(
      chalk.green(
        `${chalk.white(formatBalance(amount))} from worker ${chalk.white(
          workerId.toString()
        )} stake has been succesfully slashed!`
      )
    )
  }
}
