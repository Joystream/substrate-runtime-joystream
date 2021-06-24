import categories from '../flows/forum/categories'
import polls from '../flows/forum/polls'
import threads from '../flows/forum/threads'
import posts from '../flows/forum/posts'
import moderation from '../flows/forum/moderation'
import leadOpening from '../flows/working-groups/leadOpening'
import { scenario } from '../Scenario'

scenario(async ({ job }) => {
  const sudoHireLead = job('hiring working group leads', leadOpening)
  job('forum categories', categories).requires(sudoHireLead)
  job('forum threads', threads).requires(sudoHireLead)
  job('forum polls', polls).requires(sudoHireLead)
  job('forum posts', posts).requires(sudoHireLead)
  job('forum moderation', moderation).requires(sudoHireLead)
})
