import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import { validateAddress } from '../../helpers/validation'
import chalk from 'chalk'
import ExitCodes from '../../ExitCodes'

export default class WorkingGroupsUpdateRoleAccount extends WorkingGroupsCommandBase {
  static description = 'Updates the worker/lead role account. Requires member controller account to be selected'
  static args = [
    {
      name: 'address',
      required: false,
      description: 'New role account address (if omitted, can be provided interactively)',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    let { address } = this.parse(WorkingGroupsUpdateRoleAccount).args

    const worker = await this.getRequiredWorkerContext('MemberController')

    if (!address) {
      address = await this.promptForAnyAddress('Select new role account')
    } else if (validateAddress(address) !== true) {
      this.error('Invalid address', { exit: ExitCodes.InvalidInput })
    }

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(worker.profile.membership.controller_account.toString()),
      apiModuleByGroup[this.group],
      'updateRoleAccount',
      [worker.workerId, address]
    )

    this.log(chalk.green(`Succesfully updated the role account to: ${chalk.white(address)})`))
  }
}
