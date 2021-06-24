import { WorkerId } from '@joystream/types/working-group'
import { StandardizedFixture } from '../../Fixture'

export abstract class WithForumWorkersFixture extends StandardizedFixture {
  protected forumLeadId?: WorkerId

  protected async getForumLeadId(): Promise<WorkerId> {
    if (!this.forumLeadId) {
      const optForumLeadId = await this.api.query.forumWorkingGroup.currentLead()
      if (optForumLeadId.isNone) {
        throw new Error('Forum working group lead not set!')
      }

      this.forumLeadId = optForumLeadId.unwrap()
    }

    return this.forumLeadId
  }

  protected async getSignersFromInput(input: { asWorker?: WorkerId }[]): Promise<string[]> {
    return Promise.all(
      input.map(async (r) => {
        const workerId = r.asWorker || (await this.getForumLeadId())
        return (await this.api.query.forumWorkingGroup.workerById(workerId)).role_account_id.toString()
      })
    )
  }
}
