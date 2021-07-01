import electCouncil from '../flows/council/elect'
import proposalsDiscussion from '../flows/proposalsDiscussion'
import { scenario } from '../Scenario'

scenario(async ({ job, env }) => {
  const councilJob = job('electing council', electCouncil)
  job('proposal discussion', [proposalsDiscussion]).requires(councilJob)
})
