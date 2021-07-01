import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import { ElectCouncilFixture } from '../../fixtures/council/ElectCouncilFixture'

// Currently only used by proposals flow

export default async function electCouncil({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger('flow:elect-council')
  debug('Started')
  api.enableDebugTxLogs()

  const electCouncilFixture = new ElectCouncilFixture(api, query)
  await new FixtureRunner(electCouncilFixture).run()

  debug('Done')
}
