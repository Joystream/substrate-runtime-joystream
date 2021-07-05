// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { Bytes, Option, Vec, bool, u32, u64 } from '@polkadot/types';
import type { AnyNumber, ITuple, Observable } from '@polkadot/types/types';
import type { Application, ApplicationId, ApplicationOf, Category, CategoryId, Channel, ChannelId, Class, ClassId, ClassOf, ClassPermissionsType, ContentId, Credential, Curator, CuratorApplication, CuratorApplicationId, CuratorGroup, CuratorGroupId, CuratorId, CuratorOpening, CuratorOpeningId, DataObjectId, DiscussionPost, DiscussionThread, DynamicBag, DynamicBagCreationPolicy, DynamicBagId, DynamicBagType, ElectionStage, ElectionStake, Entity, EntityController, EntityCreationVoucher, EntityId, EntityOf, HiringApplicationId, InputValidationLengthConstraint, Lead, LeadId, MemberId, Membership, MemoText, Mint, MintId, Opening, OpeningId, OpeningOf, PaidMembershipTerms, PaidTermId, Post, PostId, Principal, PrincipalId, PropertyId, ProposalDetailsOf, ProposalId, ProposalOf, Recipient, RecipientId, RewardRelationship, RewardRelationshipId, SealedVote, Seats, Stake, StakeId, StaticBag, StaticBagId, StorageBucket, StorageBucketId, Thread, ThreadCounter, ThreadId, TransferableStake, VoteKind, WorkerId, WorkerOf, WorkingGroupUnstaker } from './all';
import type { UncleEntryItem } from '@polkadot/types/interfaces/authorship';
import type { BabeAuthorityWeight, MaybeRandomness, NextConfigDescriptor, Randomness } from '@polkadot/types/interfaces/babe';
import type { AccountData, BalanceLock } from '@polkadot/types/interfaces/balances';
import type { AuthorityId } from '@polkadot/types/interfaces/consensus';
import type { SetId, StoredPendingChange, StoredState } from '@polkadot/types/interfaces/grandpa';
import type { AuthIndex } from '@polkadot/types/interfaces/imOnline';
import type { DeferredOffenceOf, Kind, OffenceDetails, OpaqueTimeSlot, ReportIdOf } from '@polkadot/types/interfaces/offences';
import type { AccountId, Balance, BalanceOf, BlockNumber, ExtrinsicsWeight, Hash, KeyTypeId, Moment, Perbill, Releases, ValidatorId } from '@polkadot/types/interfaces/runtime';
import type { Keys, SessionIndex } from '@polkadot/types/interfaces/session';
import type { ActiveEraInfo, ElectionResult, ElectionScore, ElectionStatus, EraIndex, EraRewardPoints, Exposure, Forcing, Nominations, RewardDestination, SlashingSpans, SpanIndex, SpanRecord, StakingLedger, UnappliedSlash, ValidatorPrefs } from '@polkadot/types/interfaces/staking';
import type { AccountInfo, DigestOf, EventIndex, EventRecord, LastRuntimeUpgradeInfo, Phase } from '@polkadot/types/interfaces/system';
import type { Multiplier } from '@polkadot/types/interfaces/txpayment';
import type { ApiTypes } from '@polkadot/api/types';

declare module '@polkadot/api/types/storage' {
  export interface AugmentedQueries<ApiType> {
    authorship: {
      /**
       * Author of current block.
       **/
      author: AugmentedQuery<ApiType, () => Observable<Option<AccountId>>, []>;
      /**
       * Whether uncles were already set in this block.
       **/
      didSetUncles: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Uncles
       **/
      uncles: AugmentedQuery<ApiType, () => Observable<Vec<UncleEntryItem>>, []>;
    };
    babe: {
      /**
       * Current epoch authorities.
       **/
      authorities: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[AuthorityId, BabeAuthorityWeight]>>>, []>;
      /**
       * Current slot number.
       **/
      currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Current epoch index.
       **/
      epochIndex: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * The slot at which the first epoch actually started. This is 0
       * until the first block of the chain.
       **/
      genesisSlot: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Temporary value (cleared at block finalization) which is `Some`
       * if per-block initialization has already been called for current block.
       **/
      initialized: AugmentedQuery<ApiType, () => Observable<Option<MaybeRandomness>>, []>;
      /**
       * How late the current block is compared to its parent.
       * 
       * This entry is populated as part of block execution and is cleaned up
       * on block finalization. Querying this storage entry outside of block
       * execution context should always yield zero.
       **/
      lateness: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Next epoch configuration, if changed.
       **/
      nextEpochConfig: AugmentedQuery<ApiType, () => Observable<Option<NextConfigDescriptor>>, []>;
      /**
       * Next epoch randomness.
       **/
      nextRandomness: AugmentedQuery<ApiType, () => Observable<Randomness>, []>;
      /**
       * The epoch randomness for the *current* epoch.
       * 
       * # Security
       * 
       * This MUST NOT be used for gambling, as it can be influenced by a
       * malicious validator in the short term. It MAY be used in many
       * cryptographic protocols, however, so long as one remembers that this
       * (like everything else on-chain) it is public. For example, it can be
       * used where a number is needed that cannot have been chosen by an
       * adversary, for purposes such as public-coin zero-knowledge proofs.
       **/
      randomness: AugmentedQuery<ApiType, () => Observable<Randomness>, []>;
      /**
       * Randomness under construction.
       * 
       * We make a tradeoff between storage accesses and list length.
       * We store the under-construction randomness in segments of up to
       * `UNDER_CONSTRUCTION_SEGMENT_LENGTH`.
       * 
       * Once a segment reaches this length, we begin the next one.
       * We reset all segments and return to `0` at the beginning of every
       * epoch.
       **/
      segmentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay.
       **/
      underConstruction: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Randomness>>, [u32]>;
    };
    balances: {
      /**
       * The balance of an account.
       * 
       * NOTE: This is only used in the case that this module is used to store balances.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<AccountData>, [AccountId]>;
      /**
       * Any liquidity locks on some account balances.
       * NOTE: Should only be accessed when setting, changing and freeing a lock.
       **/
      locks: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Vec<BalanceLock>>, [AccountId]>;
      /**
       * Storage version of the pallet.
       * 
       * This is set to v2.0.0 for new networks.
       **/
      storageVersion: AugmentedQuery<ApiType, () => Observable<Releases>, []>;
      /**
       * The total units issued in the system.
       **/
      totalIssuance: AugmentedQuery<ApiType, () => Observable<Balance>, []>;
    };
    contentDirectory: {
      /**
       * Map, representing ClassId -> Class relation
       **/
      classById: AugmentedQuery<ApiType, (arg: ClassId | AnyNumber | Uint8Array) => Observable<ClassOf>, [ClassId]>;
      /**
       * Map, representing  CuratorGroupId -> CuratorGroup relation
       **/
      curatorGroupById: AugmentedQuery<ApiType, (arg: CuratorGroupId | AnyNumber | Uint8Array) => Observable<CuratorGroup>, [CuratorGroupId]>;
      /**
       * Map, representing EntityId -> Entity relation
       **/
      entityById: AugmentedQuery<ApiType, (arg: EntityId | AnyNumber | Uint8Array) => Observable<EntityOf>, [EntityId]>;
      entityCreationVouchers: AugmentedQueryDoubleMap<ApiType, (key1: ClassId | AnyNumber | Uint8Array, key2: EntityController | { Maintainers: any } | { Member: any } | { Lead: any } | string | Uint8Array) => Observable<EntityCreationVoucher>, [ClassId, EntityController]>;
      /**
       * Next runtime storage values used to maintain next id value, used on creation of respective curator groups, classes and entities
       **/
      nextClassId: AugmentedQuery<ApiType, () => Observable<ClassId>, []>;
      nextCuratorGroupId: AugmentedQuery<ApiType, () => Observable<CuratorGroupId>, []>;
      nextEntityId: AugmentedQuery<ApiType, () => Observable<EntityId>, []>;
      /**
       * Mapping of class id and its property id to the respective entity id and property value hash.
       **/
      uniquePropertyValueHashes: AugmentedQueryDoubleMap<ApiType, (key1: ITuple<[ClassId, PropertyId]> | [ClassId | AnyNumber | Uint8Array, PropertyId | AnyNumber | Uint8Array], key2: Hash | string | Uint8Array) => Observable<ITuple<[]>>, [ITuple<[ClassId, PropertyId]>, Hash]>;
    };
    contentDirectoryWorkingGroup: {
      /**
       * Count of active workers.
       **/
      activeWorkerCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Maps identifier to worker application on opening.
       **/
      applicationById: AugmentedQuery<ApiType, (arg: ApplicationId | AnyNumber | Uint8Array) => Observable<ApplicationOf>, [ApplicationId]>;
      /**
       * The current lead.
       **/
      currentLead: AugmentedQuery<ApiType, () => Observable<Option<WorkerId>>, []>;
      /**
       * Map member id by hiring application id.
       * Required by StakingEventsHandler callback call to refund the balance on unstaking.
       **/
      memberIdByHiringApplicationId: AugmentedQuery<ApiType, (arg: HiringApplicationId | AnyNumber | Uint8Array) => Observable<MemberId>, [HiringApplicationId]>;
      /**
       * The mint currently funding the rewards for this module.
       **/
      mint: AugmentedQuery<ApiType, () => Observable<MintId>, []>;
      /**
       * Next identifier value for new worker application.
       **/
      nextApplicationId: AugmentedQuery<ApiType, () => Observable<ApplicationId>, []>;
      /**
       * Next identifier value for new worker opening.
       **/
      nextOpeningId: AugmentedQuery<ApiType, () => Observable<OpeningId>, []>;
      /**
       * Next identifier for new worker.
       **/
      nextWorkerId: AugmentedQuery<ApiType, () => Observable<WorkerId>, []>;
      /**
       * Maps identifier to worker opening.
       **/
      openingById: AugmentedQuery<ApiType, (arg: OpeningId | AnyNumber | Uint8Array) => Observable<OpeningOf>, [OpeningId]>;
      /**
       * Opening human readable text length limits
       **/
      openingHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Worker application human readable text length limits
       **/
      workerApplicationHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to corresponding worker.
       **/
      workerById: AugmentedQuery<ApiType, (arg: WorkerId | AnyNumber | Uint8Array) => Observable<WorkerOf>, [WorkerId]>;
      /**
       * Worker exit rationale text length limits.
       **/
      workerExitRationaleText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
    };
    contentWorkingGroup: {
      channelAvatarConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      channelBannerConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to corresponding channel.
       **/
      channelById: AugmentedQuery<ApiType, (arg: ChannelId | AnyNumber | Uint8Array) => Observable<Channel>, [ChannelId]>;
      /**
       * Whether it is currently possible to create a channel via `create_channel` extrinsic.
       **/
      channelCreationEnabled: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      channelDescriptionConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      channelHandleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps (unique) channel handle to the corresponding identifier for the channel.
       * Mapping is required to allow efficient (O(log N)) on-chain verification that a proposed handle is indeed unique
       * at the time it is being proposed.
       **/
      channelIdByHandle: AugmentedQuery<ApiType, (arg: Bytes | string | Uint8Array) => Observable<ChannelId>, [Bytes]>;
      channelTitleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to curator application on opening.
       **/
      curatorApplicationById: AugmentedQuery<ApiType, (arg: CuratorApplicationId | AnyNumber | Uint8Array) => Observable<CuratorApplication>, [CuratorApplicationId]>;
      curatorApplicationHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to corresponding curator.
       **/
      curatorById: AugmentedQuery<ApiType, (arg: CuratorId | AnyNumber | Uint8Array) => Observable<Curator>, [CuratorId]>;
      curatorExitRationaleText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifeir to curator opening.
       **/
      curatorOpeningById: AugmentedQuery<ApiType, (arg: CuratorOpeningId | AnyNumber | Uint8Array) => Observable<CuratorOpening>, [CuratorOpeningId]>;
      /**
       * The current lead.
       **/
      currentLeadId: AugmentedQuery<ApiType, () => Observable<Option<LeadId>>, []>;
      /**
       * Maps identifier to corresponding lead.
       **/
      leadById: AugmentedQuery<ApiType, (arg: LeadId | AnyNumber | Uint8Array) => Observable<Lead>, [LeadId]>;
      /**
       * The mint currently funding the rewards for this module.
       **/
      mint: AugmentedQuery<ApiType, () => Observable<MintId>, []>;
      /**
       * Identifier to be used by the next channel introduced.
       **/
      nextChannelId: AugmentedQuery<ApiType, () => Observable<ChannelId>, []>;
      /**
       * Next identifier value for new curator application.
       **/
      nextCuratorApplicationId: AugmentedQuery<ApiType, () => Observable<CuratorApplicationId>, []>;
      /**
       * Next identifier for new curator.
       **/
      nextCuratorId: AugmentedQuery<ApiType, () => Observable<CuratorId>, []>;
      /**
       * Next identifier valuefor new curator opening.
       **/
      nextCuratorOpeningId: AugmentedQuery<ApiType, () => Observable<CuratorOpeningId>, []>;
      /**
       * Next identifier for new current lead.
       **/
      nextLeadId: AugmentedQuery<ApiType, () => Observable<LeadId>, []>;
      /**
       * Next identifier for
       **/
      nextPrincipalId: AugmentedQuery<ApiType, () => Observable<PrincipalId>, []>;
      openingHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to principal.
       **/
      principalById: AugmentedQuery<ApiType, (arg: PrincipalId | AnyNumber | Uint8Array) => Observable<Principal>, [PrincipalId]>;
      /**
       * Recover curator by the role stake which is currently unstaking.
       **/
      unstakerByStakeId: AugmentedQuery<ApiType, (arg: StakeId | AnyNumber | Uint8Array) => Observable<WorkingGroupUnstaker>, [StakeId]>;
    };
    council: {
      activeCouncil: AugmentedQuery<ApiType, () => Observable<Seats>, []>;
      /**
       * Reward amount paid out at each PayoutInterval
       **/
      amountPerPayout: AugmentedQuery<ApiType, () => Observable<BalanceOf>, []>;
      /**
       * The mint that funds council member rewards and spending proposals budget
       **/
      councilMint: AugmentedQuery<ApiType, () => Observable<MintId>, []>;
      /**
       * How many blocks after the reward is created, the first payout will be made
       **/
      firstPayoutAfterRewardCreated: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Optional interval in blocks on which a reward payout will be made to each council member
       **/
      payoutInterval: AugmentedQuery<ApiType, () => Observable<Option<BlockNumber>>, []>;
      /**
       * The reward relationships currently in place. There may not necessarily be a 1-1 correspondance with
       * the active council, since there are multiple ways of setting/adding/removing council members, some of which
       * do not involve creating a relationship.
       **/
      rewardRelationships: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<RewardRelationshipId>, [AccountId]>;
      termEndsAt: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
    };
    councilElection: {
      announcingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      applicants: AugmentedQuery<ApiType, () => Observable<Vec<AccountId>>, []>;
      applicantStakes: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<ElectionStake>, [AccountId]>;
      autoStart: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      candidacyLimit: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      commitments: AugmentedQuery<ApiType, () => Observable<Vec<Hash>>, []>;
      councilSize: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      existingStakeHolders: AugmentedQuery<ApiType, () => Observable<Vec<AccountId>>, []>;
      minCouncilStake: AugmentedQuery<ApiType, () => Observable<BalanceOf>, []>;
      minVotingStake: AugmentedQuery<ApiType, () => Observable<BalanceOf>, []>;
      newTermDuration: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      revealingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      round: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      stage: AugmentedQuery<ApiType, () => Observable<Option<ElectionStage>>, []>;
      transferableStakes: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<TransferableStake>, [AccountId]>;
      votes: AugmentedQuery<ApiType, (arg: Hash | string | Uint8Array) => Observable<SealedVote>, [Hash]>;
      votingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
    };
    forum: {
      /**
       * Map category identifier to corresponding category.
       **/
      categoryById: AugmentedQuery<ApiType, (arg: CategoryId | AnyNumber | Uint8Array) => Observable<Category>, [CategoryId]>;
      categoryDescriptionConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Input constraints
       * These are all forward looking, that is they are enforced on all
       * future calls.
       **/
      categoryTitleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Account of forum sudo.
       **/
      forumSudo: AugmentedQuery<ApiType, () => Observable<Option<AccountId>>, []>;
      /**
       * Category identifier value to be used for the next Category created.
       **/
      nextCategoryId: AugmentedQuery<ApiType, () => Observable<CategoryId>, []>;
      /**
       * Post identifier value to be used for for next post created.
       **/
      nextPostId: AugmentedQuery<ApiType, () => Observable<PostId>, []>;
      /**
       * Thread identifier value to be used for next Thread in threadById.
       **/
      nextThreadId: AugmentedQuery<ApiType, () => Observable<ThreadId>, []>;
      /**
       * Map post identifier to corresponding post.
       **/
      postById: AugmentedQuery<ApiType, (arg: PostId | AnyNumber | Uint8Array) => Observable<Post>, [PostId]>;
      postModerationRationaleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      postTextConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Map thread identifier to corresponding thread.
       **/
      threadById: AugmentedQuery<ApiType, (arg: ThreadId | AnyNumber | Uint8Array) => Observable<Thread>, [ThreadId]>;
      threadModerationRationaleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      threadTitleConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
    };
    grandpa: {
      /**
       * The number of changes (both in terms of keys and underlying economic responsibilities)
       * in the "set" of Grandpa validators from genesis.
       **/
      currentSetId: AugmentedQuery<ApiType, () => Observable<SetId>, []>;
      /**
       * next block number where we can force a change.
       **/
      nextForced: AugmentedQuery<ApiType, () => Observable<Option<BlockNumber>>, []>;
      /**
       * Pending change: (signaled at, scheduled change).
       **/
      pendingChange: AugmentedQuery<ApiType, () => Observable<Option<StoredPendingChange>>, []>;
      /**
       * A mapping from grandpa set ID to the index of the *most recent* session for which its
       * members were responsible.
       * 
       * TWOX-NOTE: `SetId` is not under user control.
       **/
      setIdSession: AugmentedQuery<ApiType, (arg: SetId | AnyNumber | Uint8Array) => Observable<Option<SessionIndex>>, [SetId]>;
      /**
       * `true` if we are currently stalled.
       **/
      stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[BlockNumber, BlockNumber]>>>, []>;
      /**
       * State of the current authority set.
       **/
      state: AugmentedQuery<ApiType, () => Observable<StoredState>, []>;
    };
    hiring: {
      /**
       * Applications
       **/
      applicationById: AugmentedQuery<ApiType, (arg: ApplicationId | AnyNumber | Uint8Array) => Observable<Application>, [ApplicationId]>;
      /**
       * Internal purpose of given stake, i.e. fro what application, and whether for the role or for the application.
       **/
      applicationIdByStakingId: AugmentedQuery<ApiType, (arg: StakeId | AnyNumber | Uint8Array) => Observable<ApplicationId>, [StakeId]>;
      /**
       * Identifier for next application to be added.
       **/
      nextApplicationId: AugmentedQuery<ApiType, () => Observable<ApplicationId>, []>;
      /**
       * Identifier for next opening to be added.
       **/
      nextOpeningId: AugmentedQuery<ApiType, () => Observable<OpeningId>, []>;
      /**
       * Openings.
       **/
      openingById: AugmentedQuery<ApiType, (arg: OpeningId | AnyNumber | Uint8Array) => Observable<Opening>, [OpeningId]>;
    };
    imOnline: {
      /**
       * For each session index, we keep a mapping of `T::ValidatorId` to the
       * number of blocks authored by the given authority.
       **/
      authoredBlocks: AugmentedQueryDoubleMap<ApiType, (key1: SessionIndex | AnyNumber | Uint8Array, key2: ValidatorId | string | Uint8Array) => Observable<u32>, [SessionIndex, ValidatorId]>;
      /**
       * The block number after which it's ok to send heartbeats in current session.
       * 
       * At the beginning of each session we set this to a value that should
       * fall roughly in the middle of the session duration.
       * The idea is to first wait for the validators to produce a block
       * in the current session, so that the heartbeat later on will not be necessary.
       **/
      heartbeatAfter: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * The current set of keys that may issue a heartbeat.
       **/
      keys: AugmentedQuery<ApiType, () => Observable<Vec<AuthorityId>>, []>;
      /**
       * For each session index, we keep a mapping of `AuthIndex` to
       * `offchain::OpaqueNetworkState`.
       **/
      receivedHeartbeats: AugmentedQueryDoubleMap<ApiType, (key1: SessionIndex | AnyNumber | Uint8Array, key2: AuthIndex | AnyNumber | Uint8Array) => Observable<Option<Bytes>>, [SessionIndex, AuthIndex]>;
    };
    members: {
      /**
       * Active Paid membership terms
       **/
      activePaidMembershipTerms: AugmentedQuery<ApiType, () => Observable<Vec<PaidTermId>>, []>;
      maxAboutTextLength: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      maxAvatarUriLength: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      maxHandleLength: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Registered unique handles and their mapping to their owner
       **/
      memberIdByHandle: AugmentedQuery<ApiType, (arg: Bytes | string | Uint8Array) => Observable<MemberId>, [Bytes]>;
      /**
       * Mapping of a controller account id to vector of member ids it controls
       **/
      memberIdsByControllerAccountId: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Vec<MemberId>>, [AccountId]>;
      /**
       * Mapping of a root account id to vector of member ids it controls.
       **/
      memberIdsByRootAccountId: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Vec<MemberId>>, [AccountId]>;
      /**
       * Mapping of member's id to their membership profile
       **/
      membershipById: AugmentedQuery<ApiType, (arg: MemberId | AnyNumber | Uint8Array) => Observable<Membership>, [MemberId]>;
      minHandleLength: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Is the platform is accepting new members or not
       **/
      newMembershipsAllowed: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * MemberId to assign to next member that is added to the registry, and is also the
       * total number of members created. MemberIds start at Zero.
       **/
      nextMemberId: AugmentedQuery<ApiType, () => Observable<MemberId>, []>;
      /**
       * Next paid membership terms id
       **/
      nextPaidMembershipTermsId: AugmentedQuery<ApiType, () => Observable<PaidTermId>, []>;
      /**
       * Paid membership terms record
       **/
      paidMembershipTermsById: AugmentedQuery<ApiType, (arg: PaidTermId | AnyNumber | Uint8Array) => Observable<PaidMembershipTerms>, [PaidTermId]>;
      screeningAuthority: AugmentedQuery<ApiType, () => Observable<AccountId>, []>;
    };
    memo: {
      maxMemoLength: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      memo: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<MemoText>, [AccountId]>;
    };
    minting: {
      /**
       * Mints
       **/
      mints: AugmentedQuery<ApiType, (arg: MintId | AnyNumber | Uint8Array) => Observable<Mint>, [MintId]>;
      /**
       * The number of mints created.
       **/
      mintsCreated: AugmentedQuery<ApiType, () => Observable<MintId>, []>;
    };
    offences: {
      /**
       * A vector of reports of the same kind that happened at the same time slot.
       **/
      concurrentReportsIndex: AugmentedQueryDoubleMap<ApiType, (key1: Kind | string | Uint8Array, key2: OpaqueTimeSlot | string | Uint8Array) => Observable<Vec<ReportIdOf>>, [Kind, OpaqueTimeSlot]>;
      /**
       * Deferred reports that have been rejected by the offence handler and need to be submitted
       * at a later time.
       **/
      deferredOffences: AugmentedQuery<ApiType, () => Observable<Vec<DeferredOffenceOf>>, []>;
      /**
       * The primary structure that holds all offence records keyed by report identifiers.
       **/
      reports: AugmentedQuery<ApiType, (arg: ReportIdOf | string | Uint8Array) => Observable<Option<OffenceDetails>>, [ReportIdOf]>;
      /**
       * Enumerates all reports of a kind along with the time they happened.
       * 
       * All reports are sorted by the time of offence.
       * 
       * Note that the actual type of this mapping is `Vec<u8>`, this is because values of
       * different types are not supported at the moment so we are doing the manual serialization.
       **/
      reportsByKindIndex: AugmentedQuery<ApiType, (arg: Kind | string | Uint8Array) => Observable<Bytes>, [Kind]>;
    };
    proposalsCodex: {
      /**
       * Grace period for the 'add working group opening' proposal
       **/
      addWorkingGroupOpeningProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'add working group opening' proposal
       **/
      addWorkingGroupOpeningProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'begin review working group leader applications' proposal
       **/
      beginReviewWorkingGroupLeaderApplicationsProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'begin review working group leader applications' proposal
       **/
      beginReviewWorkingGroupLeaderApplicationsProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'decrease working group leader stake' proposal
       **/
      decreaseWorkingGroupLeaderStakeProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'decrease working group leader stake' proposal
       **/
      decreaseWorkingGroupLeaderStakeProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'fill working group leader opening' proposal
       **/
      fillWorkingGroupLeaderOpeningProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'fill working group leader opening' proposal
       **/
      fillWorkingGroupLeaderOpeningProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Map proposal id to proposal details
       **/
      proposalDetailsByProposalId: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<ProposalDetailsOf>, [ProposalId]>;
      /**
       * Grace period for the 'runtime upgrade' proposal
       **/
      runtimeUpgradeProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'runtime upgrade' proposal
       **/
      runtimeUpgradeProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'set election parameters' proposal
       **/
      setElectionParametersProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'set election parameters' proposal
       **/
      setElectionParametersProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'set validator count' proposal
       **/
      setValidatorCountProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'set validator count' proposal
       **/
      setValidatorCountProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'set working group leader reward' proposal
       **/
      setWorkingGroupLeaderRewardProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'set working group leader reward' proposal
       **/
      setWorkingGroupLeaderRewardProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'set working group mint capacity' proposal
       **/
      setWorkingGroupMintCapacityProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'set working group mint capacity' proposal
       **/
      setWorkingGroupMintCapacityProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'slash working group leader stake' proposal
       **/
      slashWorkingGroupLeaderStakeProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'slash working group leader stake' proposal
       **/
      slashWorkingGroupLeaderStakeProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'spending' proposal
       **/
      spendingProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'spending' proposal
       **/
      spendingProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'terminate working group leader role' proposal
       **/
      terminateWorkingGroupLeaderRoleProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'terminate working group leader role' proposal
       **/
      terminateWorkingGroupLeaderRoleProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Grace period for the 'text' proposal
       **/
      textProposalGracePeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Voting period for the 'text' proposal
       **/
      textProposalVotingPeriod: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Map proposal id to its discussion thread id
       **/
      threadIdByProposalId: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<ThreadId>, [ProposalId]>;
    };
    proposalsDiscussion: {
      /**
       * Last author thread counter (part of the antispam mechanism)
       **/
      lastThreadAuthorCounter: AugmentedQuery<ApiType, () => Observable<Option<ThreadCounter>>, []>;
      /**
       * Count of all posts that have been created.
       **/
      postCount: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Map thread id and post id to corresponding post.
       **/
      postThreadIdByPostId: AugmentedQueryDoubleMap<ApiType, (key1: ThreadId | AnyNumber | Uint8Array, key2: PostId | AnyNumber | Uint8Array) => Observable<DiscussionPost>, [ThreadId, PostId]>;
      /**
       * Map thread identifier to corresponding thread.
       **/
      threadById: AugmentedQuery<ApiType, (arg: ThreadId | AnyNumber | Uint8Array) => Observable<DiscussionThread>, [ThreadId]>;
      /**
       * Count of all threads that have been created.
       **/
      threadCount: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    proposalsEngine: {
      /**
       * Count of active proposals.
       **/
      activeProposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Ids of proposals that are open for voting (have not been finalized yet).
       **/
      activeProposalIds: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<ITuple<[]>>, [ProposalId]>;
      /**
       * Map proposal executable code by proposal id.
       **/
      dispatchableCallCode: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<Bytes>, [ProposalId]>;
      /**
       * Ids of proposals that were approved and theirs grace period was not expired.
       **/
      pendingExecutionProposalIds: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<ITuple<[]>>, [ProposalId]>;
      /**
       * Count of all proposals that have been created.
       **/
      proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Map proposal by its id.
       **/
      proposals: AugmentedQuery<ApiType, (arg: ProposalId | AnyNumber | Uint8Array) => Observable<ProposalOf>, [ProposalId]>;
      /**
       * Map proposal id by stake id. Required by StakingEventsHandler callback call
       **/
      stakesProposals: AugmentedQuery<ApiType, (arg: StakeId | AnyNumber | Uint8Array) => Observable<ProposalId>, [StakeId]>;
      /**
       * Double map for preventing duplicate votes. Should be cleaned after usage.
       **/
      voteExistsByProposalByVoter: AugmentedQueryDoubleMap<ApiType, (key1: ProposalId | AnyNumber | Uint8Array, key2: MemberId | AnyNumber | Uint8Array) => Observable<VoteKind>, [ProposalId, MemberId]>;
    };
    randomnessCollectiveFlip: {
      /**
       * Series of block headers from the last 81 blocks that acts as random seed material. This
       * is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of
       * the oldest hash.
       **/
      randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<Hash>>, []>;
    };
    recurringRewards: {
      recipients: AugmentedQuery<ApiType, (arg: RecipientId | AnyNumber | Uint8Array) => Observable<Recipient>, [RecipientId]>;
      recipientsCreated: AugmentedQuery<ApiType, () => Observable<RecipientId>, []>;
      rewardRelationships: AugmentedQuery<ApiType, (arg: RewardRelationshipId | AnyNumber | Uint8Array) => Observable<RewardRelationship>, [RewardRelationshipId]>;
      rewardRelationshipsCreated: AugmentedQuery<ApiType, () => Observable<RewardRelationshipId>, []>;
    };
    session: {
      /**
       * Current index of the session.
       **/
      currentIndex: AugmentedQuery<ApiType, () => Observable<SessionIndex>, []>;
      /**
       * Indices of disabled validators.
       * 
       * The set is cleared when `on_session_ending` returns a new set of identities.
       **/
      disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []>;
      /**
       * The owner of a key. The key is the `KeyTypeId` + the encoded key.
       **/
      keyOwner: AugmentedQuery<ApiType, (arg: ITuple<[KeyTypeId, Bytes]> | [KeyTypeId | AnyNumber | Uint8Array, Bytes | string | Uint8Array]) => Observable<Option<ValidatorId>>, [ITuple<[KeyTypeId, Bytes]>]>;
      /**
       * The next session keys for a validator.
       **/
      nextKeys: AugmentedQuery<ApiType, (arg: ValidatorId | string | Uint8Array) => Observable<Option<Keys>>, [ValidatorId]>;
      /**
       * True if the underlying economic identities or weighting behind the validators
       * has changed in the queued validator set.
       **/
      queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * The queued keys for the next session. When the next session begins, these keys
       * will be used to determine the validator's session keys.
       **/
      queuedKeys: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[ValidatorId, Keys]>>>, []>;
      /**
       * The current set of validators.
       **/
      validators: AugmentedQuery<ApiType, () => Observable<Vec<ValidatorId>>, []>;
    };
    stake: {
      /**
       * Maps identifiers to a stake.
       **/
      stakes: AugmentedQuery<ApiType, (arg: StakeId | AnyNumber | Uint8Array) => Observable<Stake>, [StakeId]>;
      /**
       * Identifier value for next stake, and count of total stakes created (not necessarily the number of current
       * stakes in the Stakes map as stakes can be removed.)
       **/
      stakesCreated: AugmentedQuery<ApiType, () => Observable<StakeId>, []>;
    };
    staking: {
      /**
       * The active era information, it holds index and start.
       * 
       * The active era is the era currently rewarded.
       * Validator set of this era must be equal to `SessionInterface::validators`.
       **/
      activeEra: AugmentedQuery<ApiType, () => Observable<Option<ActiveEraInfo>>, []>;
      /**
       * Map from all locked "stash" accounts to the controller account.
       **/
      bonded: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<AccountId>>, [AccountId]>;
      /**
       * A mapping from still-bonded eras to the first session index of that era.
       * 
       * Must contains information for eras for the range:
       * `[active_era - bounding_duration; active_era]`
       **/
      bondedEras: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[EraIndex, SessionIndex]>>>, []>;
      /**
       * The amount of currency given to reporters of a slash event which was
       * canceled by extraordinary circumstances (e.g. governance).
       **/
      canceledSlashPayout: AugmentedQuery<ApiType, () => Observable<BalanceOf>, []>;
      /**
       * The current era index.
       * 
       * This is the latest planned era, depending on how the Session pallet queues the validator
       * set, it might be active or not.
       **/
      currentEra: AugmentedQuery<ApiType, () => Observable<Option<EraIndex>>, []>;
      /**
       * The earliest era for which we have a pending, unapplied slash.
       **/
      earliestUnappliedSlash: AugmentedQuery<ApiType, () => Observable<Option<EraIndex>>, []>;
      /**
       * Flag to control the execution of the offchain election. When `Open(_)`, we accept
       * solutions to be submitted.
       **/
      eraElectionStatus: AugmentedQuery<ApiType, () => Observable<ElectionStatus>, []>;
      /**
       * Rewards for the last `HISTORY_DEPTH` eras.
       * If reward hasn't been set or has been removed then 0 reward is returned.
       **/
      erasRewardPoints: AugmentedQuery<ApiType, (arg: EraIndex | AnyNumber | Uint8Array) => Observable<EraRewardPoints>, [EraIndex]>;
      /**
       * Exposure of validator at era.
       * 
       * This is keyed first by the era index to allow bulk deletion and then the stash account.
       * 
       * Is it removed after `HISTORY_DEPTH` eras.
       * If stakers hasn't been set or has been removed then empty exposure is returned.
       **/
      erasStakers: AugmentedQueryDoubleMap<ApiType, (key1: EraIndex | AnyNumber | Uint8Array, key2: AccountId | string | Uint8Array) => Observable<Exposure>, [EraIndex, AccountId]>;
      /**
       * Clipped Exposure of validator at era.
       * 
       * This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the
       * `T::MaxNominatorRewardedPerValidator` biggest stakers.
       * (Note: the field `total` and `own` of the exposure remains unchanged).
       * This is used to limit the i/o cost for the nominator payout.
       * 
       * This is keyed fist by the era index to allow bulk deletion and then the stash account.
       * 
       * Is it removed after `HISTORY_DEPTH` eras.
       * If stakers hasn't been set or has been removed then empty exposure is returned.
       **/
      erasStakersClipped: AugmentedQueryDoubleMap<ApiType, (key1: EraIndex | AnyNumber | Uint8Array, key2: AccountId | string | Uint8Array) => Observable<Exposure>, [EraIndex, AccountId]>;
      /**
       * The session index at which the era start for the last `HISTORY_DEPTH` eras.
       **/
      erasStartSessionIndex: AugmentedQuery<ApiType, (arg: EraIndex | AnyNumber | Uint8Array) => Observable<Option<SessionIndex>>, [EraIndex]>;
      /**
       * The total amount staked for the last `HISTORY_DEPTH` eras.
       * If total hasn't been set or has been removed then 0 stake is returned.
       **/
      erasTotalStake: AugmentedQuery<ApiType, (arg: EraIndex | AnyNumber | Uint8Array) => Observable<BalanceOf>, [EraIndex]>;
      /**
       * Similar to `ErasStakers`, this holds the preferences of validators.
       * 
       * This is keyed first by the era index to allow bulk deletion and then the stash account.
       * 
       * Is it removed after `HISTORY_DEPTH` eras.
       **/
      erasValidatorPrefs: AugmentedQueryDoubleMap<ApiType, (key1: EraIndex | AnyNumber | Uint8Array, key2: AccountId | string | Uint8Array) => Observable<ValidatorPrefs>, [EraIndex, AccountId]>;
      /**
       * The total validator era payout for the last `HISTORY_DEPTH` eras.
       * 
       * Eras that haven't finished yet or has been removed doesn't have reward.
       **/
      erasValidatorReward: AugmentedQuery<ApiType, (arg: EraIndex | AnyNumber | Uint8Array) => Observable<Option<BalanceOf>>, [EraIndex]>;
      /**
       * Mode of era forcing.
       **/
      forceEra: AugmentedQuery<ApiType, () => Observable<Forcing>, []>;
      /**
       * Number of eras to keep in history.
       * 
       * Information is kept for eras in `[current_era - history_depth; current_era]`.
       * 
       * Must be more than the number of eras delayed by session otherwise. I.e. active era must
       * always be in history. I.e. `active_era > current_era - history_depth` must be
       * guaranteed.
       **/
      historyDepth: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
       * easy to initialize and the performance hit is minimal (we expect no more than four
       * invulnerables) and restricted to testnets.
       **/
      invulnerables: AugmentedQuery<ApiType, () => Observable<Vec<AccountId>>, []>;
      /**
       * True if the current **planned** session is final. Note that this does not take era
       * forcing into account.
       **/
      isCurrentSessionFinal: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Map from all (unlocked) "controller" accounts to the info regarding the staking.
       **/
      ledger: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<StakingLedger>>, [AccountId]>;
      /**
       * Minimum number of staking participants before emergency conditions are imposed.
       **/
      minimumValidatorCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The map from nominator stash key to the set of stash keys of all validators to nominate.
       **/
      nominators: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<Nominations>>, [AccountId]>;
      /**
       * All slashing events on nominators, mapped by era to the highest slash value of the era.
       **/
      nominatorSlashInEra: AugmentedQueryDoubleMap<ApiType, (key1: EraIndex | AnyNumber | Uint8Array, key2: AccountId | string | Uint8Array) => Observable<Option<BalanceOf>>, [EraIndex, AccountId]>;
      /**
       * Where the reward payment should be made. Keyed by stash.
       **/
      payee: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<RewardDestination>, [AccountId]>;
      /**
       * The next validator set. At the end of an era, if this is available (potentially from the
       * result of an offchain worker), it is immediately used. Otherwise, the on-chain election
       * is executed.
       **/
      queuedElected: AugmentedQuery<ApiType, () => Observable<Option<ElectionResult>>, []>;
      /**
       * The score of the current [`QueuedElected`].
       **/
      queuedScore: AugmentedQuery<ApiType, () => Observable<Option<ElectionScore>>, []>;
      /**
       * Slashing spans for stash accounts.
       **/
      slashingSpans: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<Option<SlashingSpans>>, [AccountId]>;
      /**
       * The percentage of the slash that is distributed to reporters.
       * 
       * The rest of the slashed value is handled by the `Slash`.
       **/
      slashRewardFraction: AugmentedQuery<ApiType, () => Observable<Perbill>, []>;
      /**
       * Snapshot of nominators at the beginning of the current election window. This should only
       * have a value when [`EraElectionStatus`] == `ElectionStatus::Open(_)`.
       **/
      snapshotNominators: AugmentedQuery<ApiType, () => Observable<Option<Vec<AccountId>>>, []>;
      /**
       * Snapshot of validators at the beginning of the current election window. This should only
       * have a value when [`EraElectionStatus`] == `ElectionStatus::Open(_)`.
       **/
      snapshotValidators: AugmentedQuery<ApiType, () => Observable<Option<Vec<AccountId>>>, []>;
      /**
       * Records information about the maximum slash of a stash within a slashing span,
       * as well as how much reward has been paid out.
       **/
      spanSlash: AugmentedQuery<ApiType, (arg: ITuple<[AccountId, SpanIndex]> | [AccountId | string | Uint8Array, SpanIndex | AnyNumber | Uint8Array]) => Observable<SpanRecord>, [ITuple<[AccountId, SpanIndex]>]>;
      /**
       * True if network has been upgraded to this version.
       * Storage version of the pallet.
       * 
       * This is set to v3.0.0 for new networks.
       **/
      storageVersion: AugmentedQuery<ApiType, () => Observable<Releases>, []>;
      /**
       * All unapplied slashes that are queued for later.
       **/
      unappliedSlashes: AugmentedQuery<ApiType, (arg: EraIndex | AnyNumber | Uint8Array) => Observable<Vec<UnappliedSlash>>, [EraIndex]>;
      /**
       * The ideal number of staking participants.
       **/
      validatorCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * The map from (wannabe) validator stash key to the preferences of that validator.
       **/
      validators: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<ValidatorPrefs>, [AccountId]>;
      /**
       * All slashing events on validators, mapped by era to the highest slash proportion
       * and slash value of the era.
       **/
      validatorSlashInEra: AugmentedQueryDoubleMap<ApiType, (key1: EraIndex | AnyNumber | Uint8Array, key2: AccountId | string | Uint8Array) => Observable<Option<ITuple<[Perbill, BalanceOf]>>>, [EraIndex, AccountId]>;
    };
    storage: {
      /**
       * Blacklisted data object hashes.
       **/
      blacklist: AugmentedQuery<ApiType, (arg: ContentId | string) => Observable<ITuple<[]>>, [ContentId]>;
      /**
       * Blacklist collection counter.
       **/
      currentBlacklistSize: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Size based pricing of new objects uploaded.
       **/
      dataObjectPerMegabyteFee: AugmentedQuery<ApiType, () => Observable<BalanceOf>, []>;
      /**
       * DynamicBagCreationPolicy by bag type storage map.
       **/
      dynamicBagCreationPolicies: AugmentedQuery<ApiType, (arg: DynamicBagType | 'Member' | 'Channel' | number | Uint8Array) => Observable<DynamicBagCreationPolicy>, [DynamicBagType]>;
      /**
       * Dynamic bag storage map.
       **/
      dynamicBags: AugmentedQuery<ApiType, (arg: DynamicBagId | { Member: any } | { Channel: any } | string | Uint8Array) => Observable<DynamicBag>, [DynamicBagId]>;
      /**
       * Data object id counter. Starts at zero.
       **/
      nextDataObjectId: AugmentedQuery<ApiType, () => Observable<DataObjectId>, []>;
      /**
       * Storage bucket id counter. Starts at zero.
       **/
      nextStorageBucketId: AugmentedQuery<ApiType, () => Observable<StorageBucketId>, []>;
      /**
       * Working groups' and council's bags storage map.
       **/
      staticBags: AugmentedQuery<ApiType, (arg: StaticBagId | { Council: any } | { WorkingGroup: any } | string | Uint8Array) => Observable<StaticBag>, [StaticBagId]>;
      /**
       * Storage buckets.
       **/
      storageBucketById: AugmentedQuery<ApiType, (arg: StorageBucketId | AnyNumber | Uint8Array) => Observable<StorageBucket>, [StorageBucketId]>;
      /**
       * Total number of the storage buckets in the system.
       **/
      storageBucketsNumber: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * "Storage buckets per bag" number limit.
       **/
      storageBucketsPerBagLimit: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * Defines whether all new uploads blocked
       **/
      uploadingBlocked: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * "Max objects number for a storage bucket voucher" number limit.
       **/
      voucherMaxObjectsNumberLimit: AugmentedQuery<ApiType, () => Observable<u64>, []>;
      /**
       * "Max objects size for a storage bucket voucher" number limit.
       **/
      voucherMaxObjectsSizeLimit: AugmentedQuery<ApiType, () => Observable<u64>, []>;
    };
    storageWorkingGroup: {
      /**
       * Count of active workers.
       **/
      activeWorkerCount: AugmentedQuery<ApiType, () => Observable<u32>, []>;
      /**
       * Maps identifier to worker application on opening.
       **/
      applicationById: AugmentedQuery<ApiType, (arg: ApplicationId | AnyNumber | Uint8Array) => Observable<ApplicationOf>, [ApplicationId]>;
      /**
       * The current lead.
       **/
      currentLead: AugmentedQuery<ApiType, () => Observable<Option<WorkerId>>, []>;
      /**
       * Map member id by hiring application id.
       * Required by StakingEventsHandler callback call to refund the balance on unstaking.
       **/
      memberIdByHiringApplicationId: AugmentedQuery<ApiType, (arg: HiringApplicationId | AnyNumber | Uint8Array) => Observable<MemberId>, [HiringApplicationId]>;
      /**
       * The mint currently funding the rewards for this module.
       **/
      mint: AugmentedQuery<ApiType, () => Observable<MintId>, []>;
      /**
       * Next identifier value for new worker application.
       **/
      nextApplicationId: AugmentedQuery<ApiType, () => Observable<ApplicationId>, []>;
      /**
       * Next identifier value for new worker opening.
       **/
      nextOpeningId: AugmentedQuery<ApiType, () => Observable<OpeningId>, []>;
      /**
       * Next identifier for new worker.
       **/
      nextWorkerId: AugmentedQuery<ApiType, () => Observable<WorkerId>, []>;
      /**
       * Maps identifier to worker opening.
       **/
      openingById: AugmentedQuery<ApiType, (arg: OpeningId | AnyNumber | Uint8Array) => Observable<OpeningOf>, [OpeningId]>;
      /**
       * Opening human readable text length limits
       **/
      openingHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Worker application human readable text length limits
       **/
      workerApplicationHumanReadableText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      /**
       * Maps identifier to corresponding worker.
       **/
      workerById: AugmentedQuery<ApiType, (arg: WorkerId | AnyNumber | Uint8Array) => Observable<WorkerOf>, [WorkerId]>;
      /**
       * Worker exit rationale text length limits.
       **/
      workerExitRationaleText: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
    };
    sudo: {
      /**
       * The `AccountId` of the sudo key.
       **/
      key: AugmentedQuery<ApiType, () => Observable<AccountId>, []>;
    };
    system: {
      /**
       * The full account information for a particular account ID.
       **/
      account: AugmentedQuery<ApiType, (arg: AccountId | string | Uint8Array) => Observable<AccountInfo>, [AccountId]>;
      /**
       * Total length (in bytes) for all extrinsics put together, for the current block.
       **/
      allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Map of block numbers to block hashes.
       **/
      blockHash: AugmentedQuery<ApiType, (arg: BlockNumber | AnyNumber | Uint8Array) => Observable<Hash>, [BlockNumber]>;
      /**
       * The current weight for the block.
       **/
      blockWeight: AugmentedQuery<ApiType, () => Observable<ExtrinsicsWeight>, []>;
      /**
       * Digest of the current block, also part of the block header.
       **/
      digest: AugmentedQuery<ApiType, () => Observable<DigestOf>, []>;
      /**
       * The number of events in the `Events<T>` list.
       **/
      eventCount: AugmentedQuery<ApiType, () => Observable<EventIndex>, []>;
      /**
       * Events deposited for the current block.
       **/
      events: AugmentedQuery<ApiType, () => Observable<Vec<EventRecord>>, []>;
      /**
       * Mapping between a topic (represented by T::Hash) and a vector of indexes
       * of events in the `<Events<T>>` list.
       * 
       * All topic vectors have deterministic storage locations depending on the topic. This
       * allows light-clients to leverage the changes trie storage tracking mechanism and
       * in case of changes fetch the list of events of interest.
       * 
       * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
       * the `EventIndex` then in case if the topic has the same contents on the next block
       * no notification will be triggered thus the event might be lost.
       **/
      eventTopics: AugmentedQuery<ApiType, (arg: Hash | string | Uint8Array) => Observable<Vec<ITuple<[BlockNumber, EventIndex]>>>, [Hash]>;
      /**
       * The execution phase of the block.
       **/
      executionPhase: AugmentedQuery<ApiType, () => Observable<Option<Phase>>, []>;
      /**
       * Total extrinsics count for the current block.
       **/
      extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []>;
      /**
       * Extrinsics data for the current block (maps an extrinsic's index to its data).
       **/
      extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]>;
      /**
       * Extrinsics root of the current block, also part of the block header.
       **/
      extrinsicsRoot: AugmentedQuery<ApiType, () => Observable<Hash>, []>;
      /**
       * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
       **/
      lastRuntimeUpgrade: AugmentedQuery<ApiType, () => Observable<Option<LastRuntimeUpgradeInfo>>, []>;
      /**
       * The current block number being processed. Set by `execute_block`.
       **/
      number: AugmentedQuery<ApiType, () => Observable<BlockNumber>, []>;
      /**
       * Hash of the previous block.
       **/
      parentHash: AugmentedQuery<ApiType, () => Observable<Hash>, []>;
      /**
       * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
       **/
      upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []>;
    };
    timestamp: {
      /**
       * Did the timestamp get updated in this block?
       **/
      didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []>;
      /**
       * Current time for the current block.
       **/
      now: AugmentedQuery<ApiType, () => Observable<Moment>, []>;
    };
    transactionPayment: {
      nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<Multiplier>, []>;
      storageVersion: AugmentedQuery<ApiType, () => Observable<Releases>, []>;
    };
    versionedStore: {
      classById: AugmentedQuery<ApiType, (arg: ClassId | AnyNumber | Uint8Array) => Observable<Class>, [ClassId]>;
      classDescriptionConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      classNameConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      entityById: AugmentedQuery<ApiType, (arg: EntityId | AnyNumber | Uint8Array) => Observable<Entity>, [EntityId]>;
      nextClassId: AugmentedQuery<ApiType, () => Observable<ClassId>, []>;
      nextEntityId: AugmentedQuery<ApiType, () => Observable<EntityId>, []>;
      propertyDescriptionConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
      propertyNameConstraint: AugmentedQuery<ApiType, () => Observable<InputValidationLengthConstraint>, []>;
    };
    versionedStorePermissions: {
      /**
       * ClassPermissions of corresponding Classes in the versioned store
       **/
      classPermissionsByClassId: AugmentedQuery<ApiType, (arg: ClassId | AnyNumber | Uint8Array) => Observable<ClassPermissionsType>, [ClassId]>;
      /**
       * Owner of an entity in the versioned store. If it is None then it is owned by the frame_system.
       **/
      entityMaintainerByEntityId: AugmentedQuery<ApiType, (arg: EntityId | AnyNumber | Uint8Array) => Observable<Option<Credential>>, [EntityId]>;
    };
  }

  export interface QueryableStorage<ApiType extends ApiTypes> extends AugmentedQueries<ApiType> {
  }
}
