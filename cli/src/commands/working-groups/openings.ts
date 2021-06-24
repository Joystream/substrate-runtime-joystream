import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { displayTable } from '../../helpers/display'

export default class WorkingGroupsOpenings extends WorkingGroupsCommandBase {
  static description = 'Shows an overview of given working group openings'
  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run() {
    const openings = await this.getApi().openingsByGroup(this.group)

    const openingsRows = openings.map((o) => ({
      'Opening ID': o.openingId,
      Type: o.type.type,
      Applications: o.applications.length,
    }))
    displayTable(openingsRows, 5)
  }
}
