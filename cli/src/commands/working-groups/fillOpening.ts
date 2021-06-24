import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import chalk from 'chalk'
import { JoyBTreeSet } from '@joystream/types/common'
import { ApplicationId } from '@joystream/types/working-group'

export default class WorkingGroupsFillOpening extends WorkingGroupsCommandBase {
  static description = "Allows filling working group opening that's currently in review. Requires lead access."
  static args = [
    {
      name: 'wgOpeningId',
      required: true,
      description: 'Working Group Opening ID',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    const { args } = this.parse(WorkingGroupsFillOpening)

    // Lead-only gate
    const lead = await this.getRequiredLeadContext()

    const openingId = parseInt(args.wgOpeningId)
    const opening = await this.getOpeningForLeadAction(openingId)

    const applicationIds = await this.promptForApplicationsToAccept(opening)

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount.toString()),
      apiModuleByGroup[this.group],
      'fillOpening',
      [openingId, new (JoyBTreeSet(ApplicationId))(this.getTypesRegistry(), applicationIds)]
    )

    this.log(chalk.green(`Opening ${chalk.white(openingId.toString())} succesfully filled!`))
    this.log(
      chalk.green('Accepted working group application IDs: ') +
        chalk.white(applicationIds.length ? applicationIds.join(chalk.green(', ')) : 'NONE')
    )
  }
}
