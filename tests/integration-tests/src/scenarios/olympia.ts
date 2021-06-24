import creatingMemberships from '../flows/membership/creatingMemberships'
import updatingMemberProfile from '../flows/membership/updatingProfile'
import updatingMemberAccounts from '../flows/membership/updatingAccounts'
import invitingMebers from '../flows/membership/invitingMembers'
import transferringInvites from '../flows/membership/transferringInvites'
import managingStakingAccounts from '../flows/membership/managingStakingAccounts'
import { scenario } from '../Scenario'

scenario(async ({ job }) => {
  job('creating members', creatingMemberships)
  job('updating member profile', updatingMemberProfile)
  job('updating member accounts', updatingMemberAccounts)
  job('inviting members', invitingMebers)
  job('transferring invites', transferringInvites)
  job('managing staking accounts', managingStakingAccounts)
})
