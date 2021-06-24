import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { displayHeader, displayNameValueTable, displayTable, memberHandle, shortAddress } from '../../helpers/display'
import { formatBalance } from '@polkadot/util'

import chalk from 'chalk'

export default class WorkingGroupsOverview extends WorkingGroupsCommandBase {
  static description = 'Shows an overview of given working group (current lead and workers)'
  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    const lead = await this.getApi().groupLead(this.group)
    const members = await this.getApi().groupMembers(this.group)

    displayHeader('Group lead')
    if (lead) {
      displayNameValueTable([
        { name: 'Member id:', value: lead.memberId.toString() },
        { name: 'Member handle:', value: memberHandle(lead.profile) },
        { name: 'Role account:', value: lead.roleAccount.toString() },
      ])
    } else {
      this.log(chalk.yellow('No lead assigned!'))
    }

    const pairs = this.getPairs()

    displayHeader('Members')
    const membersRows = members.map((m) => ({
      'Worker id': m.workerId.toString(),
      'Member id': m.memberId.toString(),
      'Member handle': memberHandle(m.profile),
      Stake: formatBalance(m.stake),
      'Reward': formatBalance(m.reward?.valuePerBlock),
      'Missed reward': formatBalance(m.reward?.totalMissed),
      'Role account': shortAddress(m.roleAccount),
      '':
        (lead?.workerId.eq(m.workerId) ? '\u{2B50}' : '  ') +
        ' ' +
        (pairs.some((p) => p.address === m.roleAccount.toString()) ? '\u{1F511}' : '  '),
    }))
    displayTable(membersRows, 5)

    displayHeader('Legend')
    this.log('\u{2B50} - Leader')
    this.log('\u{1F511} - Role key available in CLI')
  }
}
