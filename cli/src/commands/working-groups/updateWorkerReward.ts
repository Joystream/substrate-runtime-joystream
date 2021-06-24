import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { formatBalance } from '@polkadot/util'
import chalk from 'chalk'
import { Reward } from '../../Types'
import ExitCodes from '../../ExitCodes'
import { isValidBalance } from '../../helpers/validation'

export default class WorkingGroupsUpdateWorkerReward extends WorkingGroupsCommandBase {
  static description = "Change given worker's reward (amount only). Requires lead access."
  static args = [
    {
      name: 'workerId',
      required: true,
      description: 'Worker ID',
    },
    {
      name: 'newReward',
      required: true,
      description: 'New reward',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  formatReward(reward?: Reward): string {
    return reward ? formatBalance(reward.valuePerBlock) + ' / block' : 'NONE'
  }

  async run() {
    const {
      args: { workerId, newReward },
    } = this.parse(WorkingGroupsUpdateWorkerReward)

    if (!isValidBalance(newReward)) {
      this.error('Invalid reward', { exit: ExitCodes.InvalidInput })
    }

    const lead = await this.getRequiredLeadContext()

    // This will also make sure the worker is valid
    const groupMember = await this.getWorkerForLeadAction(workerId)

    const { reward } = groupMember

    this.log(chalk.white(`Current worker reward: ${this.formatReward(reward)}`))

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'updateRewardAmount',
      [workerId, newReward]
    )

    const updatedGroupMember = await this.getApi().groupMember(this.group, workerId)
    this.log(chalk.green(`Worker ${chalk.white(workerId.toString())} reward has been updated!`))
    this.log(chalk.green(`New worker reward: ${chalk.white(this.formatReward(updatedGroupMember.reward))}`))
  }
}
