import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import chalk from 'chalk'
import { flags } from '@oclif/command'

export default class WorkingGroupsLeaveRole extends WorkingGroupsCommandBase {
  static description = 'Leave the worker or lead role associated with currently selected account.'
  static flags = {
    ...WorkingGroupsCommandBase.flags,
    rationale: flags.string({
      name: 'Optional rationale',
      required: false,
    }),
  }

  async run() {
    // Worker-only gate
    const worker = await this.getRequiredWorkerContext()

    const {
      flags: { rationale },
    } = this.parse(WorkingGroupsLeaveRole)

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(worker.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'leaveRole',
      [worker.workerId, rationale || null]
    )

    this.log(chalk.green(`Succesfully left the role! (worker id: ${chalk.white(worker.workerId.toString())})`))
  }
}
