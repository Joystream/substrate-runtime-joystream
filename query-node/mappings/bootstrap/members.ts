// import { Connection } from 'typeorm'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { logger } from '../src/common'
import { MembershipEntryMethod, Membership } from 'query-node'

export interface IBootstrapMember {
  member_id: number
  root_account: string
  controller_account: string
  handle: string
  avatar_uri: string
  about: string
  registered_at_time: number
}

// export async function bootMembers(members: IBootstrapMember[], db: Connection): Promise<void> {
export async function bootMembers(db: DatabaseManager, members: IBootstrapMember[]): Promise<void> {
  for (const rawMember of members) {
    // create new membership
    const member = new Membership({
      // main data
      id: rawMember.member_id.toString(),
      rootAccount: rawMember.root_account,
      controllerAccount: rawMember.controller_account,
      handle: rawMember.handle,
      about: rawMember.about,
      avatarUri: rawMember.avatar_uri,
      createdInBlock: 0,
      entry: MembershipEntryMethod.GENESIS,

      // fill in auto-generated fields
      createdAt: new Date(rawMember.registered_at_time),
      updatedAt: new Date(rawMember.registered_at_time),
    })

    // save membership
    await db.save<Membership>(member)

    // emit log event
    logger.info('Member has been bootstrapped', { id: rawMember.member_id })
  }
}
