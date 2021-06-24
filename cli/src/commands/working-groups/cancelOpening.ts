import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import chalk from 'chalk'

export default class WorkingGroupsCancelOpening extends WorkingGroupsCommandBase {
  static description = 'Cancels (removes) an active opening'
  static args = [
    {
      name: 'openingId',
      required: true,
      description: 'Opening ID',
    },
  ]

  async run() {
    const { args } = this.parse(WorkingGroupsCancelOpening)

    // Lead-only gate
    const lead = await this.getRequiredLeadContext()

    const openingId = parseInt(args.openingId)
    await this.validateOpeningForLeadAction(openingId)

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'cancelOpening',
      [openingId]
    )

    this.log(chalk.green(`Opening ${chalk.white(openingId.toString())} has been cancelled!`))
  }
}
