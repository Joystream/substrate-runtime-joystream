import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { formatBalance } from '@polkadot/util'
import chalk from 'chalk'
import { flags } from '@oclif/command'
import { isValidBalance } from '../../helpers/validation'
import ExitCodes from '../../ExitCodes'
import BN from 'bn.js'

export default class WorkingGroupsEvictWorker extends WorkingGroupsCommandBase {
  static description = 'Evicts given worker. Requires lead access.'
  static args = [
    {
      name: 'workerId',
      required: true,
      description: 'Worker ID',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
    penalty: flags.string({
      description: 'Optional penalty in JOY',
      required: false,
    }),
    rationale: flags.string({
      description: 'Optional rationale',
      required: false,
    }),
  }

  async run() {
    const {
      args,
      flags: { penalty, rationale },
    } = this.parse(WorkingGroupsEvictWorker)

    const lead = await this.getRequiredLeadContext()

    const workerId = parseInt(args.workerId)
    // This will also make sure the worker is valid
    const groupMember = await this.getWorkerForLeadAction(workerId)

    if (penalty && !isValidBalance(penalty)) {
      this.error('Invalid penalty amount', { exit: ExitCodes.InvalidInput })
    }

    if (penalty && (!groupMember.stake || groupMember.stake.lt(new BN(penalty)))) {
      this.error('Penalty cannot exceed worker stake', { exit: ExitCodes.InvalidInput })
    }

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'terminateRole',
      [workerId, penalty || null, rationale || null]
    )

    this.log(chalk.green(`Worker ${chalk.white(workerId.toString())} has been evicted!`))
    if (penalty) {
      this.log(chalk.green(`${chalk.white(formatBalance(penalty))} of worker's stake has been slashed!`))
    }
  }
}
