import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { displayCollapsedRow, displayHeader, memberHandle } from '../../helpers/display'

export default class WorkingGroupsApplication extends WorkingGroupsCommandBase {
  static description = 'Shows an overview of given application by Working Group Application ID'
  static args = [
    {
      name: 'wgApplicationId',
      required: true,
      description: 'Working Group Application ID',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    const { args } = this.parse(WorkingGroupsApplication)

    const application = await this.getApi().groupApplication(this.group, parseInt(args.wgApplicationId))

    displayHeader(`Details`)
    const applicationRow = {
      'Application ID': application.applicationId,
      'Member handle': memberHandle(application.member),
      'Role account': application.roleAccout.toString(),
      'Reward account': application.rewardAccount.toString(),
      'Staking account': application.stakingAccount.toString(),
      'Description': application.descriptionHash.toString(),
      'Opening ID': application.openingId.toString(),
    }
    displayCollapsedRow(applicationRow)
  }
}
