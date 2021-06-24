import BN from 'bn.js'
import { WorkingGroupModuleName } from './types'

export const MINIMUM_STAKING_ACCOUNT_BALANCE = 200
export const MIN_APPLICATION_STAKE = new BN(2000)
export const MIN_UNSTANKING_PERIOD = 43201
export const LEADER_OPENING_STAKE = new BN(2000)
export const THREAD_DEPOSIT = new BN(30)
export const POST_DEPOSIT = new BN(10)

export const lockIdByWorkingGroup: { [K in WorkingGroupModuleName]: string } = {
  storageWorkingGroup: '0x0606060606060606',
  contentDirectoryWorkingGroup: '0x0707070707070707',
  forumWorkingGroup: '0x0808080808080808',
  membershipWorkingGroup: '0x0909090909090909',
}

export const workingGroups: WorkingGroupModuleName[] = [
  'storageWorkingGroup',
  'contentDirectoryWorkingGroup',
  'forumWorkingGroup',
  'membershipWorkingGroup',
]
