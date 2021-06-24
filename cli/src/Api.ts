import BN from 'bn.js'
import { types } from '@joystream/types/'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { SubmittableExtrinsic, AugmentedQuery } from '@polkadot/api/types'
import { formatBalance } from '@polkadot/util'
import { Balance } from '@polkadot/types/interfaces'
import { KeyringPair } from '@polkadot/keyring/types'
import { Codec, Observable } from '@polkadot/types/types'
import { UInt } from '@polkadot/types'
import {
  AccountSummary,
  WorkingGroups,
  Reward,
  GroupMember,
  ApplicationDetails,
  OpeningDetails,
  UnaugmentedApiPromise,
  MemberDetails,
} from './Types'
import { DeriveBalancesAll } from '@polkadot/api-derive/types'
import { CLIError } from '@oclif/errors'
import {
  Worker,
  WorkerId,
  OpeningId,
  Application,
  ApplicationId,
  StorageProviderId,
  Opening,
} from '@joystream/types/working-group'
import { Membership, StakingAccountMemberBinding } from '@joystream/types/members'
import { AccountId, MemberId } from '@joystream/types/common'
import { Class, ClassId, CuratorGroup, CuratorGroupId, Entity, EntityId } from '@joystream/types/content-directory'
import { ContentId, DataObject } from '@joystream/types/media'
import { ServiceProviderRecord } from '@joystream/types/discovery'
import _ from 'lodash'
import { ApolloClient, InMemoryCache, HttpLink, NormalizedCacheObject, DocumentNode } from '@apollo/client'
import fetch from 'cross-fetch'
import { Maybe } from './graphql/generated/schema'
import {
  GetMemberById,
  GetMemberByIdQuery,
  GetMemberByIdQueryVariables,
  MembershipFieldsFragment,
} from './graphql/generated/queries'

export const DEFAULT_API_URI = 'ws://localhost:9944/'
const DEFAULT_DECIMALS = new BN(12)

// Mapping of working group to api module
export const apiModuleByGroup = {
  [WorkingGroups.StorageProviders]: 'storageWorkingGroup',
  [WorkingGroups.Curators]: 'contentDirectoryWorkingGroup',
  [WorkingGroups.Forum]: 'forumWorkingGroup',
  [WorkingGroups.Membership]: 'membershipWorkingGroup',
} as const

export const lockIdByWorkingGroup: { [K in WorkingGroups]: string } = {
  [WorkingGroups.StorageProviders]: '0x0606060606060606',
  [WorkingGroups.Curators]: '0x0707070707070707',
  [WorkingGroups.Forum]: '0x0808080808080808',
  [WorkingGroups.Membership]: '0x0909090909090909',
}

// Api wrapper for handling most common api calls and allowing easy API implementation switch in the future
export default class Api {
  private _api: ApiPromise
  private _queryNode?: ApolloClient<NormalizedCacheObject>
  private _cdClassesCache: [ClassId, Class][] | null = null
  public isDevelopment = false

  private constructor(
    originalApi: ApiPromise,
    isDevelopment: boolean,
    queryNodeClient?: ApolloClient<NormalizedCacheObject>
  ) {
    this.isDevelopment = isDevelopment
    this._api = originalApi
    this._queryNode = queryNodeClient
  }

  public getOriginalApi(): ApiPromise {
    return this._api
  }

  // Get api for use-cases where no type augmentations are desirable
  public getUnaugmentedApi(): UnaugmentedApiPromise {
    return (this._api as unknown) as UnaugmentedApiPromise
  }

  private static async initApi(apiUri: string = DEFAULT_API_URI, metadataCache: Record<string, any>) {
    const wsProvider: WsProvider = new WsProvider(apiUri)
    const api = await ApiPromise.create({ provider: wsProvider, types, metadata: metadataCache })

    // Initializing some api params based on pioneer/packages/react-api/Api.tsx
    const [properties, chainType] = await Promise.all([api.rpc.system.properties(), api.rpc.system.chainType()])

    const tokenSymbol = properties.tokenSymbol.unwrapOr('DEV').toString()
    const tokenDecimals = properties.tokenDecimals.unwrapOr(DEFAULT_DECIMALS).toNumber()

    // formatBlanace config
    formatBalance.setDefaults({
      decimals: tokenDecimals,
      unit: tokenSymbol,
    })

    return { api, properties, chainType }
  }

  private static async createQueryNodeClient(uri: string) {
    return new ApolloClient({
      link: new HttpLink({ uri, fetch }),
      cache: new InMemoryCache(),
      defaultOptions: { query: { fetchPolicy: 'no-cache', errorPolicy: 'all' } },
    })
  }

  static async create(
    apiUri = DEFAULT_API_URI,
    metadataCache: Record<string, any>,
    queryNodeUri?: string
  ): Promise<Api> {
    const { api, chainType } = await Api.initApi(apiUri, metadataCache)
    const queryNodeClient = queryNodeUri ? await this.createQueryNodeClient(queryNodeUri) : undefined
    return new Api(api, chainType.isDevelopment || chainType.isLocal, queryNodeClient)
  }

  // Query-node: get entity by unique input
  protected async uniqueEntityQuery<
    QueryT extends { [k: string]: Maybe<Record<string, unknown>> | undefined },
    VariablesT extends Record<string, unknown>
  >(
    query: DocumentNode,
    variables: VariablesT,
    resultKey: keyof QueryT
  ): Promise<Required<QueryT>[keyof QueryT] | null | undefined> {
    if (!this._queryNode) {
      return
    }
    return (await this._queryNode.query<QueryT, VariablesT>({ query, variables })).data[resultKey] || null
  }

  // Query-node: get entities by "non-unique" input and return first result
  protected async firstEntityQuery<
    QueryT extends { [k: string]: unknown[] },
    VariablesT extends Record<string, unknown>
  >(
    query: DocumentNode,
    variables: VariablesT,
    resultKey: keyof QueryT
  ): Promise<QueryT[keyof QueryT][number] | null | undefined> {
    if (!this._queryNode) {
      return
    }
    return (await this._queryNode.query<QueryT, VariablesT>({ query, variables })).data[resultKey][0] || null
  }

  // Query-node: get multiple entities
  protected async multipleEntitiesQuery<
    QueryT extends { [k: string]: unknown[] },
    VariablesT extends Record<string, unknown>
  >(query: DocumentNode, variables: VariablesT, resultKey: keyof QueryT): Promise<QueryT[keyof QueryT] | undefined> {
    if (!this._queryNode) {
      return
    }
    return (await this._queryNode.query<QueryT, VariablesT>({ query, variables })).data[resultKey]
  }

  async bestNumber(): Promise<number> {
    return (await this._api.derive.chain.bestNumber()).toNumber()
  }

  async getAccountsBalancesInfo(accountAddresses: string[]): Promise<DeriveBalancesAll[]> {
    const accountsBalances: DeriveBalancesAll[] = await Promise.all(
      accountAddresses.map((addr) => this._api.derive.balances.all(addr))
    )

    return accountsBalances
  }

  // Get on-chain data related to given account.
  // For now it's just account balances
  async getAccountSummary(accountAddresses: string): Promise<AccountSummary> {
    const balances: DeriveBalancesAll = (await this.getAccountsBalancesInfo([accountAddresses]))[0]
    // TODO: Some more information can be fetched here in the future

    return { balances }
  }

  async estimateFee(account: KeyringPair, tx: SubmittableExtrinsic<'promise'>): Promise<Balance> {
    const paymentInfo = await tx.paymentInfo(account)
    return paymentInfo.partialFee
  }

  createTransferTx(recipient: string, amount: BN) {
    return this._api.tx.balances.transfer(recipient, amount)
  }

  // Working groups
  // TODO: This is a lot of repeated logic from "/pioneer/joy-utils/transport"
  // It will be refactored to "joystream-js" soon
  async entriesByIds<IDType extends UInt, ValueType extends Codec>(
    apiMethod: AugmentedQuery<'promise', (key: IDType) => Observable<ValueType>>
  ): Promise<[IDType, ValueType][]> {
    const entries: [IDType, ValueType][] = (await apiMethod.entries()).map(([storageKey, value]) => [
      storageKey.args[0] as IDType,
      value,
    ])

    return entries.sort((a, b) => a[0].toNumber() - b[0].toNumber())
  }

  protected async blockHash(height: number): Promise<string> {
    const blockHash = await this._api.rpc.chain.getBlockHash(height)

    return blockHash.toString()
  }

  protected async blockTimestamp(height: number): Promise<Date> {
    const blockTime = await this._api.query.timestamp.now.at(await this.blockHash(height))

    return new Date(blockTime.toNumber())
  }

  protected workingGroupApiQuery(group: WorkingGroups) {
    const module = apiModuleByGroup[group]
    return this._api.query[module]
  }

  protected async fetchMemberQueryNodeData(memberId: MemberId): Promise<MembershipFieldsFragment | null | undefined> {
    return this.uniqueEntityQuery<GetMemberByIdQuery, GetMemberByIdQueryVariables>(
      GetMemberById,
      {
        id: memberId.toString(),
      },
      'membershipByUniqueInput'
    )
  }

  async memberDetails(memberId: MemberId, membership: Membership): Promise<MemberDetails> {
    const memberData = await this.fetchMemberQueryNodeData(memberId)

    return {
      id: memberId,
      name: memberData?.metadata.name,
      handle: memberData?.handle,
      membership,
    }
  }

  protected async membershipById(memberId: MemberId): Promise<MemberDetails | null> {
    const membership = await this._api.query.members.membershipById(memberId)
    return membership.isEmpty ? null : await this.memberDetails(memberId, membership)
  }

  protected async expectedMembershipById(memberId: MemberId): Promise<MemberDetails> {
    const member = await this.membershipById(memberId)
    if (!member) {
      throw new CLIError(`Expected member was not found by id: ${memberId.toString()}`)
    }

    return member
  }

  async groupLead(group: WorkingGroups): Promise<GroupMember | null> {
    const optLeadId = await this.workingGroupApiQuery(group).currentLead()

    if (!optLeadId.isSome) {
      return null
    }

    const leadWorkerId = optLeadId.unwrap()
    const leadWorker = await this.workerByWorkerId(group, leadWorkerId.toNumber())

    return await this.parseGroupMember(group, leadWorkerId, leadWorker)
  }

  protected async fetchStake(account: AccountId | string, group: WorkingGroups): Promise<Balance> {
    return this._api.createType(
      'Balance',
      new BN(
        (await this._api.query.balances.locks(account)).find((lock) => lock.id.eq(lockIdByWorkingGroup[group]))
          ?.amount || 0
      )
    )
  }

  protected async parseGroupMember(group: WorkingGroups, id: WorkerId, worker: Worker): Promise<GroupMember> {
    const roleAccount = worker.role_account_id
    const stakingAccount = worker.staking_account_id
    const memberId = worker.member_id

    const profile = await this.membershipById(memberId)

    if (!profile) {
      throw new Error(`Group member profile not found! (member id: ${memberId.toNumber()})`)
    }

    const stake = await this.fetchStake(worker.staking_account_id, group)

    const reward: Reward = {
      valuePerBlock: worker.reward_per_block.unwrapOr(undefined),
      totalMissed: worker.missed_reward.unwrapOr(undefined),
    }

    return {
      workerId: id,
      roleAccount,
      stakingAccount,
      memberId,
      profile,
      stake,
      reward,
    }
  }

  async workerByWorkerId(group: WorkingGroups, workerId: number): Promise<Worker> {
    const nextId = await this.workingGroupApiQuery(group).nextWorkerId<WorkerId>()

    // This is chain specfic, but if next id is still 0, it means no workers have been added yet
    if (workerId < 0 || workerId >= nextId.toNumber()) {
      throw new CLIError('Invalid worker id!')
    }

    const worker = await this.workingGroupApiQuery(group).workerById<Worker>(workerId)

    if (worker.isEmpty) {
      throw new CLIError('This worker is not active anymore')
    }

    return worker
  }

  async groupMember(group: WorkingGroups, workerId: number) {
    const worker = await this.workerByWorkerId(group, workerId)
    return await this.parseGroupMember(group, this._api.createType('WorkerId', workerId), worker)
  }

  async groupMembers(group: WorkingGroups): Promise<GroupMember[]> {
    const workerEntries = await this.groupWorkers(group)

    const groupMembers: GroupMember[] = await Promise.all(
      workerEntries.map(([id, worker]) => this.parseGroupMember(group, id, worker))
    )

    return groupMembers.reverse() // Sort by newest
  }

  groupWorkers(group: WorkingGroups): Promise<[WorkerId, Worker][]> {
    return this.entriesByIds<WorkerId, Worker>(this.workingGroupApiQuery(group).workerById)
  }

  async openingsByGroup(group: WorkingGroups): Promise<OpeningDetails[]> {
    const openings = await this.entriesByIds<OpeningId, Opening>(this.workingGroupApiQuery(group).openingById)

    return Promise.all(openings.map(([id, opening]) => this.fetchOpeningDetails(group, opening, id.toNumber())))
  }

  async applicationById(group: WorkingGroups, applicationId: number): Promise<Application> {
    const nextAppId = await this.workingGroupApiQuery(group).nextApplicationId<ApplicationId>()

    if (applicationId < 0 || applicationId >= nextAppId.toNumber()) {
      throw new CLIError('Invalid working group application ID!')
    }

    const result = await this.workingGroupApiQuery(group).applicationById(applicationId)

    if (result.isEmpty) {
      throw new CLIError(`Application of ID=${applicationId} no longer exists!`)
    }

    return result
  }

  protected async fetchApplicationDetails(
    applicationId: number,
    application: Application
  ): Promise<ApplicationDetails> {
    return {
      applicationId,
      member: await this.expectedMembershipById(application.member_id),
      roleAccout: application.role_account_id,
      rewardAccount: application.reward_account_id,
      stakingAccount: application.staking_account_id,
      descriptionHash: application.description_hash.toString(),
      openingId: application.opening_id.toNumber(),
    }
  }

  async groupApplication(group: WorkingGroups, applicationId: number): Promise<ApplicationDetails> {
    const application = await this.applicationById(group, applicationId)
    return await this.fetchApplicationDetails(applicationId, application)
  }

  protected async groupOpeningApplications(group: WorkingGroups, openingId: number): Promise<ApplicationDetails[]> {
    const applicationEntries = await this.entriesByIds<ApplicationId, Application>(
      this.workingGroupApiQuery(group).applicationById
    )

    return Promise.all(
      applicationEntries
        .filter(([, application]) => application.opening_id.eqn(openingId))
        .map(([id, application]) => this.fetchApplicationDetails(id.toNumber(), application))
    )
  }

  async fetchOpening(group: WorkingGroups, openingId: number): Promise<Opening> {
    const nextId = (await this.workingGroupApiQuery(group).nextOpeningId()).toNumber()

    if (openingId < 0 || openingId >= nextId) {
      throw new CLIError('Invalid working group opening ID!')
    }

    const opening = await this.workingGroupApiQuery(group).openingById(openingId)

    if (opening.isEmpty) {
      throw new CLIError(`Opening of ID=${openingId} no longer exists!`)
    }

    return opening
  }

  async fetchOpeningDetails(group: WorkingGroups, opening: Opening, openingId: number): Promise<OpeningDetails> {
    const applications = await this.groupOpeningApplications(group, openingId)
    const type = opening.opening_type
    const stake = {
      unstakingPeriod: opening.stake_policy.leaving_unstaking_period.toNumber(),
      value: opening.stake_policy.stake_amount,
    }

    return {
      openingId,
      applications,
      type,
      stake,
      createdAtBlock: opening.created.toNumber(),
      rewardPerBlock: opening.reward_per_block.unwrapOr(undefined),
    }
  }

  async groupOpening(group: WorkingGroups, openingId: number): Promise<OpeningDetails> {
    const opening = await this.fetchOpening(group, openingId)
    return this.fetchOpeningDetails(group, opening, openingId)
  }

  async allMembers(): Promise<[MemberId, Membership][]> {
    return this.entriesByIds<MemberId, Membership>(this._api.query.members.membershipById)
  }

  // Content directory
  async availableClasses(useCache = true): Promise<[ClassId, Class][]> {
    return useCache && this._cdClassesCache
      ? this._cdClassesCache
      : (this._cdClassesCache = await this.entriesByIds<ClassId, Class>(this._api.query.contentDirectory.classById))
  }

  availableCuratorGroups(): Promise<[CuratorGroupId, CuratorGroup][]> {
    return this.entriesByIds<CuratorGroupId, CuratorGroup>(this._api.query.contentDirectory.curatorGroupById)
  }

  async curatorGroupById(id: number): Promise<CuratorGroup | null> {
    const exists = !!(await this._api.query.contentDirectory.curatorGroupById.size(id)).toNumber()
    return exists ? await this._api.query.contentDirectory.curatorGroupById(id) : null
  }

  async nextCuratorGroupId(): Promise<number> {
    return (await this._api.query.contentDirectory.nextCuratorGroupId()).toNumber()
  }

  async classById(id: number): Promise<Class | null> {
    const c = await this._api.query.contentDirectory.classById(id)
    return c.isEmpty ? null : c
  }

  async entitiesByClassId(classId: number): Promise<[EntityId, Entity][]> {
    const entityEntries = await this.entriesByIds<EntityId, Entity>(this._api.query.contentDirectory.entityById)
    return entityEntries.filter(([, entity]) => entity.class_id.toNumber() === classId)
  }

  async entityById(id: number): Promise<Entity | null> {
    const exists = !!(await this._api.query.contentDirectory.entityById.size(id)).toNumber()
    return exists ? await this._api.query.contentDirectory.entityById(id) : null
  }

  async dataObjectByContentId(contentId: ContentId): Promise<DataObject | null> {
    const dataObject = await this._api.query.dataDirectory.dataObjectByContentId(contentId)
    return dataObject.unwrapOr(null)
  }

  async ipnsIdentity(storageProviderId: number): Promise<string | null> {
    const accountInfo = await this._api.query.discovery.accountInfoByStorageProviderId(storageProviderId)
    return accountInfo.isEmpty || accountInfo.expires_at.toNumber() <= (await this.bestNumber())
      ? null
      : accountInfo.identity.toString()
  }

  async getRandomBootstrapEndpoint(): Promise<string | null> {
    const endpoints = await this._api.query.discovery.bootstrapEndpoints()
    const randomEndpoint = _.sample(endpoints.toArray())
    return randomEndpoint ? randomEndpoint.toString() : null
  }

  async isAnyProviderAvailable(): Promise<boolean> {
    const accounInfoEntries = await this.entriesByIds<StorageProviderId, ServiceProviderRecord>(
      this._api.query.discovery.accountInfoByStorageProviderId
    )

    const bestNumber = await this.bestNumber()
    return !!accounInfoEntries.filter(([, info]) => info.expires_at.toNumber() > bestNumber).length
  }

  async stakingAccountStatus(account: string): Promise<StakingAccountMemberBinding | null> {
    const status = await this.getOriginalApi().query.members.stakingAccountIdMemberStatus(account)
    return status.isEmpty ? null : status
  }
}
