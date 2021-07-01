import { Option } from '@polkadot/types/codec'
import { BlockNumber, Balance } from '@polkadot/types/interfaces'
import { u32, u64, u128, Null } from '@polkadot/types'
import { RegistryTypes } from '@polkadot/types/types'
import { JoyStructDecorated } from '../JoyStruct'
import { JoyEnum } from '../JoyEnum'
import { AccountId, MemberId, Hash } from '../common'
import { VotePower, CastVote } from '../referendum'

export type ICouncilStageAnnouncing = {
  candidatesCount: u64
}

export class CouncilStageAnnouncing
  extends JoyStructDecorated({
    candidatesCount: u64,
  })
  implements ICouncilStageAnnouncing {}

export type ICouncilStageElection = {
  candidatesCount: u64
}

export class CouncilStageElection
  extends JoyStructDecorated({
    candidatesCount: u64,
  })
  implements ICouncilStageElection {}

export class CouncilStage extends JoyEnum({
  Announcing: CouncilStageAnnouncing,
  Election: CouncilStageElection,
  Idle: Null,
} as const) {}

export type ICouncilStageUpdate = {
  stage: CouncilStage
  changed_at: BlockNumber
}

export class CouncilStageUpdate
  extends JoyStructDecorated({
    stage: CouncilStage,
    changed_at: u32,
  })
  implements ICouncilStageUpdate {}

export type ICandidate = {
  staking_account_id: AccountId
  reward_account_id: AccountId
  cycle_id: u64
  stake: Balance
  vote_power: VotePower // from referendum
  note_hash: Option<Hash>
}

export class Candidate
  extends JoyStructDecorated({
    staking_account_id: AccountId,
    reward_account_id: AccountId,
    cycle_id: u64,
    stake: u32,
    vote_power: VotePower, // from referendum
    note_hash: Option.with(Hash),
  })
  implements ICandidate {}

export type ICouncilMember = {
  staking_account_id: AccountId
  reward_account_id: AccountId
  membership_id: MemberId
  stake: Balance
  last_payment_block: BlockNumber
  unpaid_reward: Balance
}

export class CouncilMember
  extends JoyStructDecorated({
    staking_account_id: AccountId,
    reward_account_id: AccountId,
    membership_id: MemberId,
    stake: u128,
    last_payment_block: u32,
    unpaid_reward: u128,
  })
  implements ICouncilMember {}

export class CouncilMemberOf extends CouncilMember {}

export class CastVoteOf extends CastVote {}

export const councilTypes: RegistryTypes = {
  CouncilStageAnnouncing,
  CouncilStageElection,
  CouncilStageUpdate,
  CouncilStage,
  Candidate,
  CouncilMemberOf,
  CastVoteOf,
}

export default councilTypes
