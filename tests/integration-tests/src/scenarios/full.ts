import categories from '../flows/forum/categories'
import polls from '../flows/forum/polls'
import threads from '../flows/forum/threads'
import posts from '../flows/forum/posts'
import moderation from '../flows/forum/moderation'
import leadOpening from '../flows/working-groups/leadOpening'
import creatingMemberships from '../flows/membership/creatingMemberships'
import updatingMemberProfile from '../flows/membership/updatingProfile'
import updatingMemberAccounts from '../flows/membership/updatingAccounts'
import invitingMebers from '../flows/membership/invitingMembers'
import transferringInvites from '../flows/membership/transferringInvites'
import managingStakingAccounts from '../flows/membership/managingStakingAccounts'
import membershipSystem from '../flows/membership/membershipSystem'
import openingsAndApplications from '../flows/working-groups/openingsAndApplications'
import upcomingOpenings from '../flows/working-groups/upcomingOpenings'
import groupStatus from '../flows/working-groups/groupStatus'
import workerActions from '../flows/working-groups/workerActions'
import groupBudget from '../flows/working-groups/groupBudget'
import { scenario } from '../Scenario'

scenario(async ({ job }) => {
  // Membership:
  const membershipSystemJob = job('membership system', membershipSystem)

  // All other jobs should be executed after membershipSystemJob,
  // otherwise changing membershipPrice etc. may break them
  job('creating members', creatingMemberships).after(membershipSystemJob)
  job('updating member profile', updatingMemberProfile).after(membershipSystemJob)
  job('updating member accounts', updatingMemberAccounts).after(membershipSystemJob)
  job('inviting members', invitingMebers).after(membershipSystemJob)
  job('transferring invites', transferringInvites).after(membershipSystemJob)
  job('managing staking accounts', managingStakingAccounts).after(membershipSystemJob)

  // Working groups:
  const sudoHireLead = job('sudo lead opening', leadOpening).after(membershipSystemJob)
  job('openings and applications', openingsAndApplications).requires(sudoHireLead)
  job('upcoming openings', upcomingOpenings).requires(sudoHireLead)
  job('group status', groupStatus).requires(sudoHireLead)
  job('worker actions', workerActions).requires(sudoHireLead)
  job('group budget', groupBudget).requires(sudoHireLead)
  // Forum:
  job('forum categories', categories).requires(sudoHireLead)
  job('forum threads', threads).requires(sudoHireLead)
  job('forum polls', polls).requires(sudoHireLead)
  job('forum posts', posts).requires(sudoHireLead)
  job('forum moderation', moderation).requires(sudoHireLead)
})
