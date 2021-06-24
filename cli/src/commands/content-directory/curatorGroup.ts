import { WorkingGroups } from '../../Types'
import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import chalk from 'chalk'
import { displayCollapsedRow, displayHeader, memberHandle } from '../../helpers/display'

export default class CuratorGroupCommand extends ContentDirectoryCommandBase {
  static description = 'Show Curator Group details by ID.'
  static args = [
    {
      name: 'id',
      required: true,
      description: 'ID of the Curator Group',
    },
  ]

  async run() {
    const { id } = this.parse(CuratorGroupCommand).args
    const group = await this.getCuratorGroup(id)
    const classesMaintained = (await this.getApi().availableClasses()).filter(([, c]) =>
      c.class_permissions.maintainers.toArray().some((gId) => gId.toNumber() === parseInt(id))
    )
    const members = (await this.getApi().groupMembers(WorkingGroups.Curators)).filter((curator) =>
      group.curators.toArray().some((groupCurator) => groupCurator.eq(curator.workerId))
    )

    displayCollapsedRow({
      'ID': id,
      'Status': group.active.valueOf() ? 'Active' : 'Inactive',
    })
    displayHeader(`Classes maintained (${classesMaintained.length})`)
    this.log(classesMaintained.map(([, c]) => chalk.white(c.name.toString())).join(', '))
    displayHeader(`Group Members (${members.length})`)
    this.log(
      members
        .map((curator) => chalk.white(`${memberHandle(curator.profile)} (WorkerID: ${curator.workerId.toString()})`))
        .join(', ')
    )
  }
}
