import { hireStorageWorkingGroupLead } from '../../services/runtime/hireLead'
import ApiCommandBase from '../../command-base/ApiCommandBase'

export default class DevInit extends ApiCommandBase {
  static description =
    'Initialize development environment. Sets Alice as storage working group leader.'

  async run(): Promise<void> {
    await this.ensureDevelopmentChain()

    const api = await this.getApi()
    await hireStorageWorkingGroupLead(api)
  }
}
