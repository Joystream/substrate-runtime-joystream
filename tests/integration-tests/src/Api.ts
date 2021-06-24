import { ApiPromise, WsProvider, Keyring } from '@polkadot/api'
import { u32, BTreeMap } from '@polkadot/types'
import { ISubmittableResult } from '@polkadot/types/types'
import { KeyringPair } from '@polkadot/keyring/types'
import { AccountId, MemberId, PostId, ThreadId } from '@joystream/types/common'

import { AccountInfo, Balance, EventRecord, BlockNumber, BlockHash } from '@polkadot/types/interfaces'
import BN from 'bn.js'
import { QueryableConsts, QueryableStorage, SubmittableExtrinsic, SubmittableExtrinsics } from '@polkadot/api/types'
import { Sender, LogLevel } from './sender'
import { Utils } from './utils'
import { types } from '@joystream/types'

import { v4 as uuid } from 'uuid'
import Debugger from 'debug'
import { DispatchError } from '@polkadot/types/interfaces/system'
import {
  EventDetails,
  MemberInvitedEventDetails,
  MembershipBoughtEventDetails,
  MembershipEventName,
  OpeningAddedEventDetails,
  WorkingGroupsEventName,
  WorkingGroupModuleName,
  AppliedOnOpeningEventDetails,
  OpeningFilledEventDetails,
  ForumEventName,
  CategoryCreatedEventDetails,
  PostAddedEventDetails,
  ThreadCreatedEventDetails,
} from './types'
import {
  ApplicationId,
  Opening,
  OpeningId,
  WorkerId,
  ApplyOnOpeningParameters,
  Worker,
} from '@joystream/types/working-group'
import { DeriveAllSections } from '@polkadot/api/util/decorate'
import { ExactDerive } from '@polkadot/api-derive'
import { CategoryId } from '@joystream/types/forum'

export enum WorkingGroups {
  StorageWorkingGroup = 'storageWorkingGroup',
  ContentDirectoryWorkingGroup = 'contentDirectoryWorkingGroup',
  MembershipWorkingGroup = 'membershipWorkingGroup',
  ForumWorkingGroup = 'forumWorkingGroup',
}

export class ApiFactory {
  private readonly api: ApiPromise
  private readonly keyring: Keyring
  // source of funds for all new accounts
  private readonly treasuryAccount: string

  public static async create(
    provider: WsProvider,
    treasuryAccountUri: string,
    sudoAccountUri: string
  ): Promise<ApiFactory> {
    const debug = Debugger('api-factory')
    let connectAttempts = 0
    while (true) {
      connectAttempts++
      debug(`Connecting to chain, attempt ${connectAttempts}..`)
      try {
        const api = await ApiPromise.create({ provider, types })

        // Wait for api to be connected and ready
        await api.isReady

        // If a node was just started up it might take a few seconds to start producing blocks
        // Give it a few seconds to be ready.
        await Utils.wait(5000)

        return new ApiFactory(api, treasuryAccountUri, sudoAccountUri)
      } catch (err) {
        if (connectAttempts === 3) {
          throw new Error('Unable to connect to chain')
        }
      }
      await Utils.wait(5000)
    }
  }

  constructor(api: ApiPromise, treasuryAccountUri: string, sudoAccountUri: string) {
    this.api = api
    this.keyring = new Keyring({ type: 'sr25519' })
    this.treasuryAccount = this.keyring.addFromUri(treasuryAccountUri).address
    this.keyring.addFromUri(sudoAccountUri)
  }

  public getApi(label: string): Api {
    return new Api(this.api, this.treasuryAccount, this.keyring, label)
  }

  public close(): void {
    this.api.disconnect()
  }
}

export class Api {
  private readonly api: ApiPromise
  private readonly sender: Sender
  private readonly keyring: Keyring
  // source of funds for all new accounts
  private readonly treasuryAccount: string

  constructor(api: ApiPromise, treasuryAccount: string, keyring: Keyring, label: string) {
    this.api = api
    this.keyring = keyring
    this.treasuryAccount = treasuryAccount
    this.sender = new Sender(api, keyring, label)
  }

  public get tx(): SubmittableExtrinsics<'promise'> {
    return this.api.tx
  }

  public get query(): QueryableStorage<'promise'> {
    return this.api.query
  }

  public get consts(): QueryableConsts<'promise'> {
    return this.api.consts
  }

  public get derive(): DeriveAllSections<'promise', ExactDerive> {
    return this.api.derive
  }

  public async signAndSend(
    tx: SubmittableExtrinsic<'promise'>,
    sender: AccountId | string
  ): Promise<ISubmittableResult> {
    return this.sender.signAndSend(tx, sender)
  }

  public async sendExtrinsicsAndGetResults(
    txs: SubmittableExtrinsic<'promise'>[],
    sender: AccountId | string | AccountId[] | string[],
    preserveOrder = false
  ): Promise<ISubmittableResult[]> {
    let results: ISubmittableResult[] = []
    if (preserveOrder) {
      for (const i in txs) {
        const tx = txs[i]
        const result = await this.sender.signAndSend(tx, Array.isArray(sender) ? sender[i] : sender)
        results.push(result)
      }
    } else {
      results = await Promise.all(
        txs.map((tx, i) => this.sender.signAndSend(tx, Array.isArray(sender) ? sender[i] : sender))
      )
    }
    return results
  }

  public async makeSudoCall(tx: SubmittableExtrinsic<'promise'>): Promise<ISubmittableResult> {
    const sudo = await this.api.query.sudo.key()
    return this.signAndSend(this.api.tx.sudo.sudo(tx), sudo)
  }

  public enableDebugTxLogs(): void {
    this.sender.setLogLevel(LogLevel.Debug)
  }

  public enableVerboseTxLogs(): void {
    this.sender.setLogLevel(LogLevel.Verbose)
  }

  // Create new keys and store them in the shared keyring
  public async createKeyPairs(n: number, withExistentialDeposit = true): Promise<KeyringPair[]> {
    const nKeyPairs: KeyringPair[] = []
    for (let i = 0; i < n; i++) {
      // What are risks of generating duplicate keys this way?
      // Why not use a deterministic /TestKeys/N and increment N
      nKeyPairs.push(this.keyring.addFromUri(i + uuid().substring(0, 8)))
    }
    if (withExistentialDeposit) {
      await Promise.all(
        nKeyPairs.map(({ address }) => this.treasuryTransferBalance(address, this.existentialDeposit()))
      )
    }
    return nKeyPairs
  }

  // Well known WorkingGroup enum defined in runtime
  public getWorkingGroupString(workingGroup: WorkingGroups): string {
    switch (workingGroup) {
      case WorkingGroups.StorageWorkingGroup:
        return 'Storage'
      case WorkingGroups.ContentDirectoryWorkingGroup:
        return 'Content'
      case WorkingGroups.ForumWorkingGroup:
        return 'Forum'
      case WorkingGroups.MembershipWorkingGroup:
        return 'Membership'
      default:
        throw new Error(`Invalid working group string representation: ${workingGroup}`)
    }
  }

  public getBlockDuration(): BN {
    return this.api.createType('Moment', this.api.consts.babe.expectedBlockTime)
  }

  public durationInMsFromBlocks(durationInBlocks: number): number {
    return this.getBlockDuration().muln(durationInBlocks).toNumber()
  }

  public getValidatorCount(): Promise<BN> {
    return this.api.query.staking.validatorCount<u32>()
  }

  public getBestBlock(): Promise<BN> {
    return this.api.derive.chain.bestNumber()
  }

  public async getBlockHash(blockNumber: number | BlockNumber): Promise<BlockHash> {
    return this.api.rpc.chain.getBlockHash(blockNumber)
  }

  public async getControllerAccountOfMember(id: MemberId): Promise<string> {
    return (await this.api.query.members.membershipById(id)).controller_account.toString()
  }

  public async getBalance(address: string): Promise<Balance> {
    const accountData: AccountInfo = await this.api.query.system.account<AccountInfo>(address)
    return accountData.data.free
  }

  public async getStakedBalance(address: string | AccountId, lockId?: string): Promise<BN> {
    const locks = await this.api.query.balances.locks(address)
    if (lockId) {
      const foundLock = locks.find((l) => l.id.eq(lockId))
      return foundLock ? foundLock.amount : new BN(0)
    }
    return locks.reduce((sum, lock) => sum.add(lock.amount), new BN(0))
  }

  public async transferBalance({
    from,
    to,
    amount,
  }: {
    from: string
    to: string
    amount: BN
  }): Promise<ISubmittableResult> {
    return this.sender.signAndSend(this.api.tx.balances.transfer(to, amount), from)
  }

  public async treasuryTransferBalance(to: string, amount: BN): Promise<ISubmittableResult> {
    return this.transferBalance({ from: this.treasuryAccount, to, amount })
  }

  public treasuryTransferBalanceToAccounts(destinations: string[], amount: BN): Promise<ISubmittableResult[]> {
    return Promise.all(
      destinations.map((account) => this.transferBalance({ from: this.treasuryAccount, to: account, amount }))
    )
  }

  public async prepareAccountsForFeeExpenses(
    accountOrAccounts: string | string[],
    extrinsics: SubmittableExtrinsic<'promise'>[]
  ): Promise<void> {
    const fees = await Promise.all(
      extrinsics.map((tx, i) =>
        this.estimateTxFee(tx, Array.isArray(accountOrAccounts) ? accountOrAccounts[i] : accountOrAccounts)
      )
    )

    if (Array.isArray(accountOrAccounts)) {
      await Promise.all(fees.map((fee, i) => this.treasuryTransferBalance(accountOrAccounts[i], fee)))
    } else {
      await this.treasuryTransferBalance(
        accountOrAccounts,
        fees.reduce((a, b) => a.add(b), new BN(0))
      )
    }
  }

  public async getMembershipFee(): Promise<BN> {
    return this.api.query.members.membershipPrice()
  }

  // This method does not take into account weights and the runtime weight to fees computation!
  public async estimateTxFee(tx: SubmittableExtrinsic<'promise'>, account: string): Promise<Balance> {
    const paymentInfo = await tx.paymentInfo(account)
    return paymentInfo.partialFee
  }

  public existentialDeposit(): Balance {
    return this.api.consts.balances.existentialDeposit
  }

  // TODO: Augmentations comming with new @polkadot/typegen!

  public findEventRecord(events: EventRecord[], section: string, method: string): EventRecord | undefined {
    return events.find((record) => record.event.section === section && record.event.method === method)
  }

  public async retrieveEventDetails(
    result: ISubmittableResult,
    section: string,
    method: string
  ): Promise<EventDetails | undefined> {
    const { status, events } = result
    const record = this.findEventRecord(events, section, method)
    if (!record) {
      return
    }

    const blockHash = status.asInBlock.toString()
    const blockNumber = (await this.api.rpc.chain.getHeader(blockHash)).number.toNumber()
    const blockTimestamp = (await this.api.query.timestamp.now.at(blockHash)).toNumber()
    const blockEvents = await this.api.query.system.events.at(blockHash)
    const indexInBlock = blockEvents.findIndex(({ event: blockEvent }) => blockEvent.hash.eq(record.event.hash))

    return {
      event: record.event,
      blockNumber,
      blockHash,
      blockTimestamp,
      indexInBlock,
    }
  }

  public async retrieveMembershipEventDetails(
    result: ISubmittableResult,
    eventName: MembershipEventName
  ): Promise<EventDetails> {
    const details = await this.retrieveEventDetails(result, 'members', eventName)
    if (!details) {
      throw new Error(`${eventName} event details not found in result: ${JSON.stringify(result.toHuman())}`)
    }
    return details
  }

  public async retrieveWorkingGroupsEventDetails(
    result: ISubmittableResult,
    moduleName: WorkingGroupModuleName,
    eventName: WorkingGroupsEventName
  ): Promise<EventDetails> {
    const details = await this.retrieveEventDetails(result, moduleName, eventName)
    if (!details) {
      throw new Error(`${eventName} event details not found in result: ${JSON.stringify(result.toHuman())}`)
    }
    return details
  }

  public async retrieveMembershipBoughtEventDetails(result: ISubmittableResult): Promise<MembershipBoughtEventDetails> {
    const details = await this.retrieveMembershipEventDetails(result, 'MembershipBought')
    return {
      ...details,
      memberId: details.event.data[0] as MemberId,
    }
  }

  public async retrieveMemberInvitedEventDetails(result: ISubmittableResult): Promise<MemberInvitedEventDetails> {
    const details = await this.retrieveMembershipEventDetails(result, 'MemberInvited')
    return {
      ...details,
      newMemberId: details.event.data[0] as MemberId,
    }
  }

  public async retrieveOpeningAddedEventDetails(
    result: ISubmittableResult,
    moduleName: WorkingGroupModuleName
  ): Promise<OpeningAddedEventDetails> {
    const details = await this.retrieveWorkingGroupsEventDetails(result, moduleName, 'OpeningAdded')
    return {
      ...details,
      openingId: details.event.data[0] as OpeningId,
    }
  }

  public async retrieveAppliedOnOpeningEventDetails(
    result: ISubmittableResult,
    moduleName: WorkingGroupModuleName
  ): Promise<AppliedOnOpeningEventDetails> {
    const details = await this.retrieveWorkingGroupsEventDetails(result, moduleName, 'AppliedOnOpening')
    return {
      ...details,
      params: details.event.data[0] as ApplyOnOpeningParameters,
      applicationId: details.event.data[1] as ApplicationId,
    }
  }

  public async retrieveOpeningFilledEventDetails(
    result: ISubmittableResult,
    moduleName: WorkingGroupModuleName
  ): Promise<OpeningFilledEventDetails> {
    const details = await this.retrieveWorkingGroupsEventDetails(result, moduleName, 'OpeningFilled')
    return {
      ...details,
      applicationIdToWorkerIdMap: details.event.data[1] as BTreeMap<ApplicationId, WorkerId>,
    }
  }

  public getErrorNameFromExtrinsicFailedRecord(result: ISubmittableResult): string | undefined {
    const failed = result.findRecord('system', 'ExtrinsicFailed')
    if (!failed) {
      return
    }
    const record = failed as EventRecord
    const {
      event: { data },
    } = record
    const err = data[0] as DispatchError
    if (err.isModule) {
      try {
        const { name } = this.api.registry.findMetaError(err.asModule)
        return name
      } catch (findmetaerror) {
        //
      }
    }
  }

  public async getOpening(group: WorkingGroupModuleName, id: OpeningId): Promise<Opening> {
    const opening = await this.api.query[group].openingById(id)
    if (opening.isEmpty) {
      throw new Error(`Opening by id ${id} not found!`)
    }
    return opening
  }

  public async getLeader(group: WorkingGroupModuleName): Promise<Worker> {
    const leadId = await this.api.query[group].currentLead()
    if (leadId.isNone) {
      throw new Error('Cannot get lead role key: Lead not yet hired!')
    }
    return await this.api.query[group].workerById(leadId.unwrap())
  }

  public async getLeadRoleKey(group: WorkingGroupModuleName): Promise<string> {
    return (await this.getLeader(group)).role_account_id.toString()
  }

  public async getLeaderStakingKey(group: WorkingGroupModuleName): Promise<string> {
    return (await this.getLeader(group)).staking_account_id.toString()
  }

  public async retrieveForumEventDetails(result: ISubmittableResult, eventName: ForumEventName): Promise<EventDetails> {
    const details = await this.retrieveEventDetails(result, 'forum', eventName)
    if (!details) {
      throw new Error(`${eventName} event details not found in result: ${JSON.stringify(result.toHuman())}`)
    }
    return details
  }

  public async retrieveCategoryCreatedEventDetails(result: ISubmittableResult): Promise<CategoryCreatedEventDetails> {
    const details = await this.retrieveForumEventDetails(result, 'CategoryCreated')
    return {
      ...details,
      categoryId: details.event.data[0] as CategoryId,
    }
  }

  public async retrieveThreadCreatedEventDetails(result: ISubmittableResult): Promise<ThreadCreatedEventDetails> {
    const details = await this.retrieveForumEventDetails(result, 'ThreadCreated')
    return {
      ...details,
      threadId: details.event.data[0] as ThreadId,
    }
  }

  public async retrievePostAddedEventDetails(result: ISubmittableResult): Promise<PostAddedEventDetails> {
    const details = await this.retrieveForumEventDetails(result, 'PostAdded')
    return {
      ...details,
      postId: details.event.data[0] as PostId,
    }
  }
}
