import { ApolloClient, DocumentNode, NormalizedCacheObject } from '@apollo/client'
import { MemberId, PostId, ThreadId } from '@joystream/types/common'
import Debugger from 'debug'
import { ApplicationId, OpeningId, WorkerId } from '@joystream/types/working-group'
import { EventDetails, WorkingGroupModuleName } from './types'
import {
  GetMemberByIdQuery,
  GetMemberByIdQueryVariables,
  GetMemberById,
  GetMemberProfileUpdatedEventsByMemberIdQuery,
  GetMemberProfileUpdatedEventsByMemberIdQueryVariables,
  GetMemberProfileUpdatedEventsByMemberId,
  GetMemberAccountsUpdatedEventsByMemberIdQuery,
  GetMemberAccountsUpdatedEventsByMemberIdQueryVariables,
  GetMemberAccountsUpdatedEventsByMemberId,
  GetInvitesTransferredEventsBySourceMemberIdQuery,
  GetInvitesTransferredEventsBySourceMemberIdQueryVariables,
  GetInvitesTransferredEventsBySourceMemberId,
  GetStakingAccountAddedEventsByMemberIdQuery,
  GetStakingAccountAddedEventsByMemberIdQueryVariables,
  GetStakingAccountAddedEventsByMemberId,
  GetStakingAccountConfirmedEventsByMemberIdQuery,
  GetStakingAccountConfirmedEventsByMemberIdQueryVariables,
  GetStakingAccountConfirmedEventsByMemberId,
  GetStakingAccountRemovedEventsByMemberIdQuery,
  GetStakingAccountRemovedEventsByMemberIdQueryVariables,
  GetStakingAccountRemovedEventsByMemberId,
  GetMembershipSystemSnapshotAtQuery,
  GetMembershipSystemSnapshotAtQueryVariables,
  GetMembershipSystemSnapshotAt,
  GetMembershipSystemSnapshotBeforeQuery,
  GetMembershipSystemSnapshotBeforeQueryVariables,
  GetMembershipSystemSnapshotBefore,
  GetReferralCutUpdatedEventsByEventIdQuery,
  GetReferralCutUpdatedEventsByEventIdQueryVariables,
  GetReferralCutUpdatedEventsByEventId,
  GetMembershipPriceUpdatedEventsByEventIdQuery,
  GetMembershipPriceUpdatedEventsByEventIdQueryVariables,
  GetMembershipPriceUpdatedEventsByEventId,
  GetInitialInvitationBalanceUpdatedEventsByEventIdQuery,
  GetInitialInvitationBalanceUpdatedEventsByEventIdQueryVariables,
  GetInitialInvitationBalanceUpdatedEventsByEventId,
  GetInitialInvitationCountUpdatedEventsByEventIdQuery,
  GetInitialInvitationCountUpdatedEventsByEventIdQueryVariables,
  GetInitialInvitationCountUpdatedEventsByEventId,
  GetOpeningByIdQuery,
  GetOpeningByIdQueryVariables,
  GetOpeningById,
  GetApplicationByIdQuery,
  GetApplicationByIdQueryVariables,
  GetApplicationById,
  GetAppliedOnOpeningEventsByEventIdsQuery,
  GetAppliedOnOpeningEventsByEventIdsQueryVariables,
  GetAppliedOnOpeningEventsByEventIds,
  GetOpeningAddedEventsByEventIdsQuery,
  GetOpeningAddedEventsByEventIdsQueryVariables,
  GetOpeningAddedEventsByEventIds,
  GetOpeningFilledEventsByEventIdsQuery,
  GetOpeningFilledEventsByEventIdsQueryVariables,
  GetOpeningFilledEventsByEventIds,
  GetApplicationWithdrawnEventsByEventIdsQuery,
  GetApplicationWithdrawnEventsByEventIdsQueryVariables,
  GetApplicationWithdrawnEventsByEventIds,
  GetOpeningCancelledEventsByEventIdsQuery,
  GetOpeningCancelledEventsByEventIdsQueryVariables,
  GetOpeningCancelledEventsByEventIds,
  GetStatusTextChangedEventsByEventIdsQuery,
  GetStatusTextChangedEventsByEventIdsQueryVariables,
  GetStatusTextChangedEventsByEventIds,
  GetUpcomingOpeningsByCreatedInEventIdsQuery,
  GetUpcomingOpeningsByCreatedInEventIdsQueryVariables,
  GetUpcomingOpeningsByCreatedInEventIds,
  GetWorkingGroupByNameQuery,
  GetWorkingGroupByNameQueryVariables,
  GetWorkingGroupByName,
  GetWorkingGroupMetadataSnapshotsByTimeAsc,
  GetWorkingGroupMetadataSnapshotsByTimeAscQuery,
  GetWorkingGroupMetadataSnapshotsByTimeAscQueryVariables,
  MembershipFieldsFragment,
  MembershipBoughtEventFieldsFragment,
  MemberProfileUpdatedEventFieldsFragment,
  MemberAccountsUpdatedEventFieldsFragment,
  MemberInvitedEventFieldsFragment,
  InvitesTransferredEventFieldsFragment,
  StakingAccountAddedEventFieldsFragment,
  StakingAccountConfirmedEventFieldsFragment,
  StakingAccountRemovedEventFieldsFragment,
  MembershipSystemSnapshotFieldsFragment,
  ReferralCutUpdatedEventFieldsFragment,
  MembershipPriceUpdatedEventFieldsFragment,
  InitialInvitationBalanceUpdatedEventFieldsFragment,
  InitialInvitationCountUpdatedEventFieldsFragment,
  OpeningFieldsFragment,
  ApplicationFieldsFragment,
  AppliedOnOpeningEventFieldsFragment,
  OpeningAddedEventFieldsFragment,
  OpeningFilledEventFieldsFragment,
  ApplicationWithdrawnEventFieldsFragment,
  OpeningCanceledEventFieldsFragment,
  StatusTextChangedEventFieldsFragment,
  UpcomingOpeningFieldsFragment,
  WorkingGroupFieldsFragment,
  WorkingGroupMetadataFieldsFragment,
  GetUpcomingOpeningByIdQuery,
  GetUpcomingOpeningByIdQueryVariables,
  GetUpcomingOpeningById,
  GetOpeningsByIdsQuery,
  GetOpeningsByIdsQueryVariables,
  GetOpeningsByIds,
  GetApplicationsByIdsQuery,
  GetApplicationsByIdsQueryVariables,
  GetApplicationsByIds,
  GetWorkerRoleAccountUpdatedEventsByEventIdsQuery,
  GetWorkerRoleAccountUpdatedEventsByEventIdsQueryVariables,
  WorkerRoleAccountUpdatedEventFieldsFragment,
  GetWorkerRoleAccountUpdatedEventsByEventIds,
  GetWorkerRewardAccountUpdatedEventsByEventIdsQuery,
  GetWorkerRewardAccountUpdatedEventsByEventIdsQueryVariables,
  WorkerRewardAccountUpdatedEventFieldsFragment,
  GetWorkerRewardAccountUpdatedEventsByEventIds,
  StakeIncreasedEventFieldsFragment,
  GetStakeIncreasedEventsByEventIdsQuery,
  GetStakeIncreasedEventsByEventIdsQueryVariables,
  GetStakeIncreasedEventsByEventIds,
  WorkerFieldsFragment,
  GetWorkersByRuntimeIdsQuery,
  GetWorkersByRuntimeIdsQueryVariables,
  GetWorkersByRuntimeIds,
  GetWorkerStartedLeavingEventsByEventIdsQuery,
  GetWorkerStartedLeavingEventsByEventIdsQueryVariables,
  GetWorkerStartedLeavingEventsByEventIds,
  WorkerStartedLeavingEventFieldsFragment,
  TerminatedWorkerEventFieldsFragment,
  GetTerminatedWorkerEventsByEventIdsQuery,
  GetTerminatedWorkerEventsByEventIdsQueryVariables,
  GetTerminatedWorkerEventsByEventIds,
  TerminatedLeaderEventFieldsFragment,
  GetTerminatedLeaderEventsByEventIdsQuery,
  GetTerminatedLeaderEventsByEventIdsQueryVariables,
  GetTerminatedLeaderEventsByEventIds,
  WorkerRewardAmountUpdatedEventFieldsFragment,
  GetWorkerRewardAmountUpdatedEventsByEventIdsQuery,
  GetWorkerRewardAmountUpdatedEventsByEventIdsQueryVariables,
  GetWorkerRewardAmountUpdatedEventsByEventIds,
  StakeSlashedEventFieldsFragment,
  GetStakeSlashedEventsByEventIdsQuery,
  GetStakeSlashedEventsByEventIdsQueryVariables,
  GetStakeSlashedEventsByEventIds,
  StakeDecreasedEventFieldsFragment,
  GetStakeDecreasedEventsByEventIdsQuery,
  GetStakeDecreasedEventsByEventIdsQueryVariables,
  GetStakeDecreasedEventsByEventIds,
  BudgetSetEventFieldsFragment,
  GetBudgetSetEventsByEventIdsQuery,
  GetBudgetSetEventsByEventIdsQueryVariables,
  GetBudgetSetEventsByEventIds,
  BudgetSpendingEventFieldsFragment,
  GetBudgetSpendingEventsByEventIdsQuery,
  GetBudgetSpendingEventsByEventIdsQueryVariables,
  GetBudgetSpendingEventsByEventIds,
  LeaderUnsetEventFieldsFragment,
  GetLeaderUnsetEventsByEventIdsQuery,
  GetLeaderUnsetEventsByEventIdsQueryVariables,
  GetLeaderUnsetEventsByEventIds,
  LeaderSetEventFieldsFragment,
  GetLeaderSetEventsByEventIdsQuery,
  GetLeaderSetEventsByEventIdsQueryVariables,
  GetLeaderSetEventsByEventIds,
  ForumCategoryFieldsFragment,
  GetCategoriesByIdsQuery,
  GetCategoriesByIdsQueryVariables,
  GetCategoriesByIds,
  CategoryCreatedEventFieldsFragment,
  GetCategoryCreatedEventsByEventIdsQuery,
  GetCategoryCreatedEventsByEventIdsQueryVariables,
  GetCategoryCreatedEventsByEventIds,
  GetCategoryArchivalStatusUpdatedEventsByEventIds,
  GetCategoryArchivalStatusUpdatedEventsByEventIdsQuery,
  GetCategoryArchivalStatusUpdatedEventsByEventIdsQueryVariables,
  CategoryDeletedEventFieldsFragment,
  GetCategoryDeletedEventsByEventIdsQuery,
  GetCategoryDeletedEventsByEventIdsQueryVariables,
  GetCategoryDeletedEventsByEventIds,
  ThreadCreatedEventFieldsFragment,
  GetThreadCreatedEventsByEventIdsQuery,
  GetThreadCreatedEventsByEventIds,
  GetThreadCreatedEventsByEventIdsQueryVariables,
  VoteOnPollEventFieldsFragment,
  GetVoteOnPollEventsByEventIdsQuery,
  GetVoteOnPollEventsByEventIdsQueryVariables,
  GetVoteOnPollEventsByEventIds,
  ThreadDeletedEventFieldsFragment,
  GetThreadDeletedEventsByEventIdsQuery,
  GetThreadDeletedEventsByEventIdsQueryVariables,
  GetThreadDeletedEventsByEventIds,
  ForumThreadWithPostsFieldsFragment,
  GetThreadsWithPostsByIdsQuery,
  GetThreadsWithPostsByIdsQueryVariables,
  GetThreadsWithPostsByIds,
  GetMembershipBoughtEventsByEventIdsQuery,
  GetMembershipBoughtEventsByEventIdsQueryVariables,
  GetMembershipBoughtEventsByEventIds,
  GetMembersByIdsQuery,
  GetMembersByIdsQueryVariables,
  GetMembersByIds,
  GetMemberInvitedEventsByEventIdsQuery,
  GetMemberInvitedEventsByEventIdsQueryVariables,
  GetMemberInvitedEventsByEventIds,
  ForumPostFieldsFragment,
  GetPostsByIdsQuery,
  GetPostsByIdsQueryVariables,
  GetPostsByIds,
  PostAddedEventFieldsFragment,
  GetPostAddedEventsByEventIdsQuery,
  GetPostAddedEventsByEventIdsQueryVariables,
  GetPostAddedEventsByEventIds,
  ThreadTitleUpdatedEventFieldsFragment,
  GetThreadTitleUpdatedEventsByEventIdsQuery,
  GetThreadTitleUpdatedEventsByEventIdsQueryVariables,
  GetThreadTitleUpdatedEventsByEventIds,
  ThreadMovedEventFieldsFragment,
  GetThreadMovedEventsByEventIdsQuery,
  GetThreadMovedEventsByEventIdsQueryVariables,
  GetThreadMovedEventsByEventIds,
  CategoryStickyThreadUpdateEventFieldsFragment,
  GetCategoryStickyThreadUpdateEventsByEventIdsQuery,
  GetCategoryStickyThreadUpdateEventsByEventIdsQueryVariables,
  GetCategoryStickyThreadUpdateEventsByEventIds,
  CategoryMembershipOfModeratorUpdatedEventFieldsFragment,
  GetCategoryMembershipOfModeratorUpdatedEventsByEventIdsQuery,
  GetCategoryMembershipOfModeratorUpdatedEventsByEventIdsQueryVariables,
  GetCategoryMembershipOfModeratorUpdatedEventsByEventIds,
  ThreadModeratedEventFieldsFragment,
  GetThreadModeratedEventsByEventIdsQuery,
  GetThreadModeratedEventsByEventIdsQueryVariables,
  GetThreadModeratedEventsByEventIds,
  PostModeratedEventFieldsFragment,
  GetPostModeratedEventsByEventIdsQuery,
  GetPostModeratedEventsByEventIdsQueryVariables,
  GetPostModeratedEventsByEventIds,
  PostReactedEventFieldsFragment,
  GetPostReactedEventsByEventIdsQuery,
  GetPostReactedEventsByEventIdsQueryVariables,
  GetPostReactedEventsByEventIds,
  PostTextUpdatedEventFieldsFragment,
  GetPostTextUpdatedEventsByEventIdsQuery,
  GetPostTextUpdatedEventsByEventIdsQueryVariables,
  GetPostTextUpdatedEventsByEventIds,
  PostDeletedEventFieldsFragment,
  GetPostDeletedEventsByEventIdsQuery,
  GetPostDeletedEventsByEventIdsQueryVariables,
  GetPostDeletedEventsByEventIds,
  CategoryArchivalStatusUpdatedEventFieldsFragment,
} from './graphql/generated/queries'
import { Maybe } from './graphql/generated/schema'
import { OperationDefinitionNode } from 'graphql'
import { CategoryId } from '@joystream/types/forum'
import { Utils } from './utils'
export class QueryNodeApi {
  private readonly queryNodeProvider: ApolloClient<NormalizedCacheObject>
  private readonly debug: Debugger.Debugger
  private readonly queryDebug: Debugger.Debugger
  private readonly tryDebug: Debugger.Debugger

  constructor(queryNodeProvider: ApolloClient<NormalizedCacheObject>) {
    this.queryNodeProvider = queryNodeProvider
    this.debug = Debugger('query-node-api')
    this.queryDebug = this.debug.extend('query')
    this.tryDebug = this.debug.extend('try')
  }

  public async tryQueryWithTimeout<QueryResultT>(
    query: () => Promise<QueryResultT>,
    assertResultIsValid: (res: QueryResultT) => void,
    retryTimeMs = 15000,
    retries = 3
  ): Promise<QueryResultT> {
    const label = query.toString().replace(/^.*\.([A-za-z0-9]+\(.*\))$/g, '$1')
    const debug = this.tryDebug.extend(label)
    let retryCounter = 0
    const retry = async (error: any) => {
      if (retryCounter === retries) {
        debug(`Max number of query retries (${retries}) reached!`)
        throw error
      }
      debug(`Retrying query in ${retryTimeMs}ms...`)
      ++retryCounter
      await Utils.wait(retryTimeMs)
    }
    while (true) {
      let result: QueryResultT
      try {
        result = await query()
      } catch (e) {
        debug(`Query node unreachable`)
        await retry(e)
        continue
      }

      try {
        assertResultIsValid(result)
      } catch (e) {
        debug(`Unexpected query result${e && e.message ? ` (${e.message})` : ''}`)
        await retry(e)
        continue
      }

      return result
    }
  }

  private debugQuery(query: DocumentNode, args: Record<string, unknown>): void {
    const queryDef = query.definitions.find((d) => d.kind === 'OperationDefinition') as OperationDefinitionNode
    this.queryDebug(`${queryDef.name!.value}(${JSON.stringify(args)})`)
  }

  // Query entity by unique input
  private async uniqueEntityQuery<
    QueryT extends { [k: string]: Maybe<Record<string, unknown>> | undefined },
    VariablesT extends Record<string, unknown>
  >(
    query: DocumentNode,
    variables: VariablesT,
    resultKey: keyof QueryT
  ): Promise<Required<QueryT>[keyof QueryT] | null> {
    this.debugQuery(query, variables)
    return (await this.queryNodeProvider.query<QueryT, VariablesT>({ query, variables })).data[resultKey] || null
  }

  // Query entities by "non-unique" input and return first result
  private async firstEntityQuery<QueryT extends { [k: string]: unknown[] }, VariablesT extends Record<string, unknown>>(
    query: DocumentNode,
    variables: VariablesT,
    resultKey: keyof QueryT
  ): Promise<QueryT[keyof QueryT][number] | null> {
    this.debugQuery(query, variables)
    return (await this.queryNodeProvider.query<QueryT, VariablesT>({ query, variables })).data[resultKey][0] || null
  }

  // Query multiple entities
  private async multipleEntitiesQuery<
    QueryT extends { [k: string]: unknown[] },
    VariablesT extends Record<string, unknown>
  >(query: DocumentNode, variables: VariablesT, resultKey: keyof QueryT): Promise<QueryT[keyof QueryT]> {
    this.debugQuery(query, variables)
    return (await this.queryNodeProvider.query<QueryT, VariablesT>({ query, variables })).data[resultKey]
  }

  public getQueryNodeEventId(blockNumber: number, indexInBlock: number): string {
    return `OLYMPIA-${blockNumber}-${indexInBlock}`
  }

  public async getMemberById(id: MemberId): Promise<MembershipFieldsFragment | null> {
    return this.uniqueEntityQuery<GetMemberByIdQuery, GetMemberByIdQueryVariables>(
      GetMemberById,
      { id: id.toString() },
      'membershipByUniqueInput'
    )
  }

  public async getMembersByIds(ids: MemberId[]): Promise<MembershipFieldsFragment[]> {
    return this.multipleEntitiesQuery<GetMembersByIdsQuery, GetMembersByIdsQueryVariables>(
      GetMembersByIds,
      { ids: ids.map((id) => id.toString()) },
      'memberships'
    )
  }

  public async getMembershipBoughtEvents(events: EventDetails[]): Promise<MembershipBoughtEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetMembershipBoughtEventsByEventIdsQuery,
      GetMembershipBoughtEventsByEventIdsQueryVariables
    >(GetMembershipBoughtEventsByEventIds, { eventIds }, 'membershipBoughtEvents')
  }

  public async getMemberProfileUpdatedEvents(memberId: MemberId): Promise<MemberProfileUpdatedEventFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetMemberProfileUpdatedEventsByMemberIdQuery,
      GetMemberProfileUpdatedEventsByMemberIdQueryVariables
    >(GetMemberProfileUpdatedEventsByMemberId, { memberId: memberId.toString() }, 'memberProfileUpdatedEvents')
  }

  public async getMemberAccountsUpdatedEvents(memberId: MemberId): Promise<MemberAccountsUpdatedEventFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetMemberAccountsUpdatedEventsByMemberIdQuery,
      GetMemberAccountsUpdatedEventsByMemberIdQueryVariables
    >(GetMemberAccountsUpdatedEventsByMemberId, { memberId: memberId.toString() }, 'memberAccountsUpdatedEvents')
  }

  public async getMemberInvitedEvents(events: EventDetails[]): Promise<MemberInvitedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetMemberInvitedEventsByEventIdsQuery,
      GetMemberInvitedEventsByEventIdsQueryVariables
    >(GetMemberInvitedEventsByEventIds, { eventIds }, 'memberInvitedEvents')
  }

  // TODO: Use event id
  public async getInvitesTransferredEvent(
    sourceMemberId: MemberId
  ): Promise<InvitesTransferredEventFieldsFragment | null> {
    return this.firstEntityQuery<
      GetInvitesTransferredEventsBySourceMemberIdQuery,
      GetInvitesTransferredEventsBySourceMemberIdQueryVariables
    >(
      GetInvitesTransferredEventsBySourceMemberId,
      { sourceMemberId: sourceMemberId.toString() },
      'invitesTransferredEvents'
    )
  }

  public async getStakingAccountAddedEvents(memberId: MemberId): Promise<StakingAccountAddedEventFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetStakingAccountAddedEventsByMemberIdQuery,
      GetStakingAccountAddedEventsByMemberIdQueryVariables
    >(GetStakingAccountAddedEventsByMemberId, { memberId: memberId.toString() }, 'stakingAccountAddedEvents')
  }

  public async getStakingAccountConfirmedEvents(
    memberId: MemberId
  ): Promise<StakingAccountConfirmedEventFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetStakingAccountConfirmedEventsByMemberIdQuery,
      GetStakingAccountConfirmedEventsByMemberIdQueryVariables
    >(GetStakingAccountConfirmedEventsByMemberId, { memberId: memberId.toString() }, 'stakingAccountConfirmedEvents')
  }

  public async getStakingAccountRemovedEvents(memberId: MemberId): Promise<StakingAccountRemovedEventFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetStakingAccountRemovedEventsByMemberIdQuery,
      GetStakingAccountRemovedEventsByMemberIdQueryVariables
    >(GetStakingAccountRemovedEventsByMemberId, { memberId: memberId.toString() }, 'stakingAccountRemovedEvents')
  }

  // FIXME: Cross-filtering is not enabled yet, so we have to use timestamp workaround
  public async getMembershipSystemSnapshotAt(
    timestamp: number
  ): Promise<MembershipSystemSnapshotFieldsFragment | null> {
    return this.firstEntityQuery<GetMembershipSystemSnapshotAtQuery, GetMembershipSystemSnapshotAtQueryVariables>(
      GetMembershipSystemSnapshotAt,
      { time: new Date(timestamp) },
      'membershipSystemSnapshots'
    )
  }

  public async getMembershipSystemSnapshotBefore(
    timestamp: number
  ): Promise<MembershipSystemSnapshotFieldsFragment | null> {
    return this.firstEntityQuery<
      GetMembershipSystemSnapshotBeforeQuery,
      GetMembershipSystemSnapshotBeforeQueryVariables
    >(GetMembershipSystemSnapshotBefore, { time: new Date(timestamp) }, 'membershipSystemSnapshots')
  }

  public async getReferralCutUpdatedEvent(
    blockNumber: number,
    indexInBlock: number
  ): Promise<ReferralCutUpdatedEventFieldsFragment | null> {
    return this.firstEntityQuery<
      GetReferralCutUpdatedEventsByEventIdQuery,
      GetReferralCutUpdatedEventsByEventIdQueryVariables
    >(
      GetReferralCutUpdatedEventsByEventId,
      { eventId: this.getQueryNodeEventId(blockNumber, indexInBlock) },
      'referralCutUpdatedEvents'
    )
  }

  public async getMembershipPriceUpdatedEvent(
    blockNumber: number,
    indexInBlock: number
  ): Promise<MembershipPriceUpdatedEventFieldsFragment | null> {
    return this.firstEntityQuery<
      GetMembershipPriceUpdatedEventsByEventIdQuery,
      GetMembershipPriceUpdatedEventsByEventIdQueryVariables
    >(
      GetMembershipPriceUpdatedEventsByEventId,
      { eventId: this.getQueryNodeEventId(blockNumber, indexInBlock) },
      'membershipPriceUpdatedEvents'
    )
  }

  public async getInitialInvitationBalanceUpdatedEvent(
    blockNumber: number,
    indexInBlock: number
  ): Promise<InitialInvitationBalanceUpdatedEventFieldsFragment | null> {
    return this.firstEntityQuery<
      GetInitialInvitationBalanceUpdatedEventsByEventIdQuery,
      GetInitialInvitationBalanceUpdatedEventsByEventIdQueryVariables
    >(
      GetInitialInvitationBalanceUpdatedEventsByEventId,
      { eventId: this.getQueryNodeEventId(blockNumber, indexInBlock) },
      'initialInvitationBalanceUpdatedEvents'
    )
  }

  public async getInitialInvitationCountUpdatedEvent(
    blockNumber: number,
    indexInBlock: number
  ): Promise<InitialInvitationCountUpdatedEventFieldsFragment | null> {
    return this.firstEntityQuery<
      GetInitialInvitationCountUpdatedEventsByEventIdQuery,
      GetInitialInvitationCountUpdatedEventsByEventIdQueryVariables
    >(
      GetInitialInvitationCountUpdatedEventsByEventId,
      { eventId: this.getQueryNodeEventId(blockNumber, indexInBlock) },
      'initialInvitationCountUpdatedEvents'
    )
  }

  public async getOpeningById(id: OpeningId, group: WorkingGroupModuleName): Promise<OpeningFieldsFragment | null> {
    return this.uniqueEntityQuery<GetOpeningByIdQuery, GetOpeningByIdQueryVariables>(
      GetOpeningById,
      { openingId: `${group}-${id.toString()}` },
      'workingGroupOpeningByUniqueInput'
    )
  }

  public async getOpeningsByIds(ids: OpeningId[], group: WorkingGroupModuleName): Promise<OpeningFieldsFragment[]> {
    const openingIds = ids.map((id) => `${group}-${id.toString()}`)
    return this.multipleEntitiesQuery<GetOpeningsByIdsQuery, GetOpeningsByIdsQueryVariables>(
      GetOpeningsByIds,
      { openingIds },
      'workingGroupOpenings'
    )
  }

  public async getApplicationById(
    id: ApplicationId,
    group: WorkingGroupModuleName
  ): Promise<ApplicationFieldsFragment | null> {
    return this.uniqueEntityQuery<GetApplicationByIdQuery, GetApplicationByIdQueryVariables>(
      GetApplicationById,
      { applicationId: `${group}-${id.toString()}` },
      'workingGroupApplicationByUniqueInput'
    )
  }

  public async getApplicationsByIds(
    ids: ApplicationId[],
    group: WorkingGroupModuleName
  ): Promise<ApplicationFieldsFragment[]> {
    const applicationIds = ids.map((id) => `${group}-${id.toString()}`)
    return this.multipleEntitiesQuery<GetApplicationsByIdsQuery, GetApplicationsByIdsQueryVariables>(
      GetApplicationsByIds,
      { applicationIds },
      'workingGroupApplications'
    )
  }

  public async getAppliedOnOpeningEvents(events: EventDetails[]): Promise<AppliedOnOpeningEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetAppliedOnOpeningEventsByEventIdsQuery,
      GetAppliedOnOpeningEventsByEventIdsQueryVariables
    >(GetAppliedOnOpeningEventsByEventIds, { eventIds }, 'appliedOnOpeningEvents')
  }

  public async getOpeningAddedEvents(events: EventDetails[]): Promise<OpeningAddedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetOpeningAddedEventsByEventIdsQuery,
      GetOpeningAddedEventsByEventIdsQueryVariables
    >(GetOpeningAddedEventsByEventIds, { eventIds }, 'openingAddedEvents')
  }

  public async getOpeningFilledEvents(events: EventDetails[]): Promise<OpeningFilledEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetOpeningFilledEventsByEventIdsQuery,
      GetOpeningFilledEventsByEventIdsQueryVariables
    >(GetOpeningFilledEventsByEventIds, { eventIds }, 'openingFilledEvents')
  }

  public async getApplicationWithdrawnEvents(
    events: EventDetails[]
  ): Promise<ApplicationWithdrawnEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetApplicationWithdrawnEventsByEventIdsQuery,
      GetApplicationWithdrawnEventsByEventIdsQueryVariables
    >(GetApplicationWithdrawnEventsByEventIds, { eventIds }, 'applicationWithdrawnEvents')
  }

  public async getOpeningCancelledEvents(events: EventDetails[]): Promise<OpeningCanceledEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetOpeningCancelledEventsByEventIdsQuery,
      GetOpeningCancelledEventsByEventIdsQueryVariables
    >(GetOpeningCancelledEventsByEventIds, { eventIds }, 'openingCanceledEvents')
  }

  public async getStatusTextChangedEvents(events: EventDetails[]): Promise<StatusTextChangedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetStatusTextChangedEventsByEventIdsQuery,
      GetStatusTextChangedEventsByEventIdsQueryVariables
    >(GetStatusTextChangedEventsByEventIds, { eventIds }, 'statusTextChangedEvents')
  }

  public async getUpcomingOpeningById(id: string): Promise<UpcomingOpeningFieldsFragment | null> {
    return this.uniqueEntityQuery<GetUpcomingOpeningByIdQuery, GetUpcomingOpeningByIdQueryVariables>(
      GetUpcomingOpeningById,
      { id },
      'upcomingWorkingGroupOpeningByUniqueInput'
    )
  }

  public async getUpcomingOpeningsByCreatedInEventIds(eventIds: string[]): Promise<UpcomingOpeningFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetUpcomingOpeningsByCreatedInEventIdsQuery,
      GetUpcomingOpeningsByCreatedInEventIdsQueryVariables
    >(GetUpcomingOpeningsByCreatedInEventIds, { createdInEventIds: eventIds }, 'upcomingWorkingGroupOpenings')
  }

  public async getWorkingGroup(name: WorkingGroupModuleName): Promise<WorkingGroupFieldsFragment | null> {
    return this.uniqueEntityQuery<GetWorkingGroupByNameQuery, GetWorkingGroupByNameQueryVariables>(
      GetWorkingGroupByName,
      { name },
      'workingGroupByUniqueInput'
    )
  }

  public async getGroupMetaSnapshotsByTimeAsc(groupId: string): Promise<WorkingGroupMetadataFieldsFragment[]> {
    return this.multipleEntitiesQuery<
      GetWorkingGroupMetadataSnapshotsByTimeAscQuery,
      GetWorkingGroupMetadataSnapshotsByTimeAscQueryVariables
    >(GetWorkingGroupMetadataSnapshotsByTimeAsc, { groupId }, 'workingGroupMetadata')
  }

  public async getWorkerRoleAccountUpdatedEvents(
    events: EventDetails[]
  ): Promise<WorkerRoleAccountUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetWorkerRoleAccountUpdatedEventsByEventIdsQuery,
      GetWorkerRoleAccountUpdatedEventsByEventIdsQueryVariables
    >(GetWorkerRoleAccountUpdatedEventsByEventIds, { eventIds }, 'workerRoleAccountUpdatedEvents')
  }

  public async getWorkerRewardAccountUpdatedEvents(
    events: EventDetails[]
  ): Promise<WorkerRewardAccountUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetWorkerRewardAccountUpdatedEventsByEventIdsQuery,
      GetWorkerRewardAccountUpdatedEventsByEventIdsQueryVariables
    >(GetWorkerRewardAccountUpdatedEventsByEventIds, { eventIds }, 'workerRewardAccountUpdatedEvents')
  }

  public async getStakeIncreasedEvents(events: EventDetails[]): Promise<StakeIncreasedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetStakeIncreasedEventsByEventIdsQuery,
      GetStakeIncreasedEventsByEventIdsQueryVariables
    >(GetStakeIncreasedEventsByEventIds, { eventIds }, 'stakeIncreasedEvents')
  }

  public async getWorkersByIds(ids: WorkerId[], group: WorkingGroupModuleName): Promise<WorkerFieldsFragment[]> {
    return this.multipleEntitiesQuery<GetWorkersByRuntimeIdsQuery, GetWorkersByRuntimeIdsQueryVariables>(
      GetWorkersByRuntimeIds,
      { workerIds: ids.map((id) => id.toNumber()), groupId: group },
      'workers'
    )
  }

  public async getWorkerStartedLeavingEvents(
    events: EventDetails[]
  ): Promise<WorkerStartedLeavingEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetWorkerStartedLeavingEventsByEventIdsQuery,
      GetWorkerStartedLeavingEventsByEventIdsQueryVariables
    >(GetWorkerStartedLeavingEventsByEventIds, { eventIds }, 'workerStartedLeavingEvents')
  }

  public async getTerminatedWorkerEvents(events: EventDetails[]): Promise<TerminatedWorkerEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetTerminatedWorkerEventsByEventIdsQuery,
      GetTerminatedWorkerEventsByEventIdsQueryVariables
    >(GetTerminatedWorkerEventsByEventIds, { eventIds }, 'terminatedWorkerEvents')
  }

  public async getTerminatedLeaderEvents(events: EventDetails[]): Promise<TerminatedLeaderEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetTerminatedLeaderEventsByEventIdsQuery,
      GetTerminatedLeaderEventsByEventIdsQueryVariables
    >(GetTerminatedLeaderEventsByEventIds, { eventIds }, 'terminatedLeaderEvents')
  }

  public async getWorkerRewardAmountUpdatedEvents(
    events: EventDetails[]
  ): Promise<WorkerRewardAmountUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetWorkerRewardAmountUpdatedEventsByEventIdsQuery,
      GetWorkerRewardAmountUpdatedEventsByEventIdsQueryVariables
    >(GetWorkerRewardAmountUpdatedEventsByEventIds, { eventIds }, 'workerRewardAmountUpdatedEvents')
  }

  public async getStakeSlashedEvents(events: EventDetails[]): Promise<StakeSlashedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetStakeSlashedEventsByEventIdsQuery,
      GetStakeSlashedEventsByEventIdsQueryVariables
    >(GetStakeSlashedEventsByEventIds, { eventIds }, 'stakeSlashedEvents')
  }

  public async getStakeDecreasedEvents(events: EventDetails[]): Promise<StakeDecreasedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetStakeDecreasedEventsByEventIdsQuery,
      GetStakeDecreasedEventsByEventIdsQueryVariables
    >(GetStakeDecreasedEventsByEventIds, { eventIds }, 'stakeDecreasedEvents')
  }

  public async getBudgetSetEvents(events: EventDetails[]): Promise<BudgetSetEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<GetBudgetSetEventsByEventIdsQuery, GetBudgetSetEventsByEventIdsQueryVariables>(
      GetBudgetSetEventsByEventIds,
      { eventIds },
      'budgetSetEvents'
    )
  }

  public async getBudgetSpendingEvents(events: EventDetails[]): Promise<BudgetSpendingEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetBudgetSpendingEventsByEventIdsQuery,
      GetBudgetSpendingEventsByEventIdsQueryVariables
    >(GetBudgetSpendingEventsByEventIds, { eventIds }, 'budgetSpendingEvents')
  }

  public async getLeaderSetEvent(event: EventDetails): Promise<LeaderSetEventFieldsFragment | null> {
    const eventId = this.getQueryNodeEventId(event.blockNumber, event.indexInBlock)
    return this.firstEntityQuery<GetLeaderSetEventsByEventIdsQuery, GetLeaderSetEventsByEventIdsQueryVariables>(
      GetLeaderSetEventsByEventIds,
      { eventIds: [eventId] },
      'leaderSetEvents'
    )
  }

  public async getLeaderUnsetEvent(event: EventDetails): Promise<LeaderUnsetEventFieldsFragment | null> {
    const eventId = this.getQueryNodeEventId(event.blockNumber, event.indexInBlock)
    return this.firstEntityQuery<GetLeaderUnsetEventsByEventIdsQuery, GetLeaderUnsetEventsByEventIdsQueryVariables>(
      GetLeaderUnsetEventsByEventIds,
      { eventIds: [eventId] },
      'leaderUnsetEvents'
    )
  }

  public async getCategoriesByIds(ids: CategoryId[]): Promise<ForumCategoryFieldsFragment[]> {
    return this.multipleEntitiesQuery<GetCategoriesByIdsQuery, GetCategoriesByIdsQueryVariables>(
      GetCategoriesByIds,
      { ids: ids.map((id) => id.toString()) },
      'forumCategories'
    )
  }

  public async getCategoryCreatedEvents(events: EventDetails[]): Promise<CategoryCreatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetCategoryCreatedEventsByEventIdsQuery,
      GetCategoryCreatedEventsByEventIdsQueryVariables
    >(GetCategoryCreatedEventsByEventIds, { eventIds }, 'categoryCreatedEvents')
  }

  public async getCategoryArchivalStatusUpdatedEvents(
    events: EventDetails[]
  ): Promise<CategoryArchivalStatusUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetCategoryArchivalStatusUpdatedEventsByEventIdsQuery,
      GetCategoryArchivalStatusUpdatedEventsByEventIdsQueryVariables
    >(GetCategoryArchivalStatusUpdatedEventsByEventIds, { eventIds }, 'categoryArchivalStatusUpdatedEvents')
  }

  public async getCategoryDeletedEvents(events: EventDetails[]): Promise<CategoryDeletedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetCategoryDeletedEventsByEventIdsQuery,
      GetCategoryDeletedEventsByEventIdsQueryVariables
    >(GetCategoryDeletedEventsByEventIds, { eventIds }, 'categoryDeletedEvents')
  }

  public async getThreadCreatedEvents(events: EventDetails[]): Promise<ThreadCreatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetThreadCreatedEventsByEventIdsQuery,
      GetThreadCreatedEventsByEventIdsQueryVariables
    >(GetThreadCreatedEventsByEventIds, { eventIds }, 'threadCreatedEvents')
  }

  public async getThreadTitleUpdatedEvents(events: EventDetails[]): Promise<ThreadTitleUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetThreadTitleUpdatedEventsByEventIdsQuery,
      GetThreadTitleUpdatedEventsByEventIdsQueryVariables
    >(GetThreadTitleUpdatedEventsByEventIds, { eventIds }, 'threadTitleUpdatedEvents')
  }

  public async getThreadsWithPostsByIds(ids: ThreadId[]): Promise<ForumThreadWithPostsFieldsFragment[]> {
    return this.multipleEntitiesQuery<GetThreadsWithPostsByIdsQuery, GetThreadsWithPostsByIdsQueryVariables>(
      GetThreadsWithPostsByIds,
      { ids: ids.map((id) => id.toString()) },
      'forumThreads'
    )
  }

  public async getVoteOnPollEvents(events: EventDetails[]): Promise<VoteOnPollEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<GetVoteOnPollEventsByEventIdsQuery, GetVoteOnPollEventsByEventIdsQueryVariables>(
      GetVoteOnPollEventsByEventIds,
      { eventIds },
      'voteOnPollEvents'
    )
  }

  public async getThreadDeletedEvents(events: EventDetails[]): Promise<ThreadDeletedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetThreadDeletedEventsByEventIdsQuery,
      GetThreadDeletedEventsByEventIdsQueryVariables
    >(GetThreadDeletedEventsByEventIds, { eventIds }, 'threadDeletedEvents')
  }

  public async getPostsByIds(ids: PostId[]): Promise<ForumPostFieldsFragment[]> {
    return this.multipleEntitiesQuery<GetPostsByIdsQuery, GetPostsByIdsQueryVariables>(
      GetPostsByIds,
      { ids: ids.map((id) => id.toString()) },
      'forumPosts'
    )
  }

  public async getPostAddedEvents(events: EventDetails[]): Promise<PostAddedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<GetPostAddedEventsByEventIdsQuery, GetPostAddedEventsByEventIdsQueryVariables>(
      GetPostAddedEventsByEventIds,
      { eventIds },
      'postAddedEvents'
    )
  }

  public async getThreadMovedEvents(events: EventDetails[]): Promise<ThreadMovedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetThreadMovedEventsByEventIdsQuery,
      GetThreadMovedEventsByEventIdsQueryVariables
    >(GetThreadMovedEventsByEventIds, { eventIds }, 'threadMovedEvents')
  }

  public async getCategoryStickyThreadUpdateEvents(
    events: EventDetails[]
  ): Promise<CategoryStickyThreadUpdateEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetCategoryStickyThreadUpdateEventsByEventIdsQuery,
      GetCategoryStickyThreadUpdateEventsByEventIdsQueryVariables
    >(GetCategoryStickyThreadUpdateEventsByEventIds, { eventIds }, 'categoryStickyThreadUpdateEvents')
  }

  public async getCategoryMembershipOfModeratorUpdatedEvents(
    events: EventDetails[]
  ): Promise<CategoryMembershipOfModeratorUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetCategoryMembershipOfModeratorUpdatedEventsByEventIdsQuery,
      GetCategoryMembershipOfModeratorUpdatedEventsByEventIdsQueryVariables
    >(
      GetCategoryMembershipOfModeratorUpdatedEventsByEventIds,
      { eventIds },
      'categoryMembershipOfModeratorUpdatedEvents'
    )
  }

  public async getThreadModeratedEvents(events: EventDetails[]): Promise<ThreadModeratedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetThreadModeratedEventsByEventIdsQuery,
      GetThreadModeratedEventsByEventIdsQueryVariables
    >(GetThreadModeratedEventsByEventIds, { eventIds }, 'threadModeratedEvents')
  }

  public async getPostModeratedEvents(events: EventDetails[]): Promise<PostModeratedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetPostModeratedEventsByEventIdsQuery,
      GetPostModeratedEventsByEventIdsQueryVariables
    >(GetPostModeratedEventsByEventIds, { eventIds }, 'postModeratedEvents')
  }

  public async getPostReactedEvents(events: EventDetails[]): Promise<PostReactedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetPostReactedEventsByEventIdsQuery,
      GetPostReactedEventsByEventIdsQueryVariables
    >(GetPostReactedEventsByEventIds, { eventIds }, 'postReactedEvents')
  }

  public async getPostTextUpdatedEvents(events: EventDetails[]): Promise<PostTextUpdatedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetPostTextUpdatedEventsByEventIdsQuery,
      GetPostTextUpdatedEventsByEventIdsQueryVariables
    >(GetPostTextUpdatedEventsByEventIds, { eventIds }, 'postTextUpdatedEvents')
  }

  public async getPostDeletedEvents(events: EventDetails[]): Promise<PostDeletedEventFieldsFragment[]> {
    const eventIds = events.map((e) => this.getQueryNodeEventId(e.blockNumber, e.indexInBlock))
    return this.multipleEntitiesQuery<
      GetPostDeletedEventsByEventIdsQuery,
      GetPostDeletedEventsByEventIdsQueryVariables
    >(GetPostDeletedEventsByEventIds, { eventIds }, 'postDeletedEvents')
  }
}
