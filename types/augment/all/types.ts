// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { BTreeMap, BTreeSet, Bytes, Enum, GenericAccountId, Null, Option, Struct, Text, U8aFixed, Vec, bool, i16, i32, i64, u128, u16, u32, u64 } from '@polkadot/types';
import type { ITuple } from '@polkadot/types/types';
import type { AccountId, Balance, Hash } from '@polkadot/types/interfaces/runtime';

/** @name AcceptingApplications */
export interface AcceptingApplications extends Struct {
  readonly started_accepting_applicants_at_block: u32;
}

/** @name ActivateOpeningAt */
export interface ActivateOpeningAt extends Enum {
  readonly isCurrentBlock: boolean;
  readonly isExactBlock: boolean;
  readonly asExactBlock: u32;
}

/** @name ActiveOpeningStage */
export interface ActiveOpeningStage extends Enum {
  readonly isAcceptingApplications: boolean;
  readonly asAcceptingApplications: AcceptingApplications;
  readonly isReviewPeriod: boolean;
  readonly asReviewPeriod: ReviewPeriod;
  readonly isDeactivated: boolean;
  readonly asDeactivated: Deactivated;
}

/** @name ActiveOpeningStageVariant */
export interface ActiveOpeningStageVariant extends Struct {
  readonly stage: ActiveOpeningStage;
  readonly applications_added: Vec<ApplicationId>;
  readonly active_application_count: u32;
  readonly unstaking_application_count: u32;
  readonly deactivated_application_count: u32;
}

/** @name ActiveStake */
export interface ActiveStake extends Struct {
  readonly stake_id: StakeId;
  readonly source_account_id: GenericAccountId;
}

/** @name Actor */
export interface Actor extends Enum {
  readonly isCurator: boolean;
  readonly asCurator: ITuple<[CuratorGroupId, u64]>;
  readonly isMember: boolean;
  readonly asMember: MemberId;
  readonly isLead: boolean;
}

/** @name ActorId */
export interface ActorId extends u64 {}

/** @name AddOpeningParameters */
export interface AddOpeningParameters extends Struct {
  readonly activate_at: ActivateOpeningAt;
  readonly commitment: WorkingGroupOpeningPolicyCommitment;
  readonly human_readable_text: Bytes;
  readonly working_group: WorkingGroup;
}

/** @name Address */
export interface Address extends AccountId {}

/** @name AddSchemaSupportToEntityOperation */
export interface AddSchemaSupportToEntityOperation extends Struct {
  readonly entity_id: ParameterizedEntity;
  readonly schema_id: SchemaId;
  readonly parametrized_property_values: Vec<ParametrizedClassPropertyValue>;
}

/** @name AdjustCapacityBy */
export interface AdjustCapacityBy extends Enum {
  readonly isSetting: boolean;
  readonly asSetting: u128;
  readonly isAdding: boolean;
  readonly asAdding: u128;
  readonly isReducing: boolean;
  readonly asReducing: u128;
}

/** @name AdjustOnInterval */
export interface AdjustOnInterval extends Struct {
  readonly block_interval: u32;
  readonly adjustment_type: AdjustCapacityBy;
}

/** @name Application */
export interface Application extends Struct {
  readonly opening_id: OpeningId;
  readonly application_index_in_opening: u32;
  readonly add_to_opening_in_block: u32;
  readonly active_role_staking_id: Option<StakeId>;
  readonly active_application_staking_id: Option<StakeId>;
  readonly stage: ApplicationStage;
  readonly human_readable_text: Text;
}

/** @name ApplicationDeactivationCause */
export interface ApplicationDeactivationCause extends Enum {
  readonly isExternal: boolean;
  readonly isHired: boolean;
  readonly isNotHired: boolean;
  readonly isCrowdedOut: boolean;
  readonly isOpeningCancelled: boolean;
  readonly isReviewPeriodExpired: boolean;
  readonly isOpeningFilled: boolean;
}

/** @name ApplicationId */
export interface ApplicationId extends u64 {}

/** @name ApplicationIdSet */
export interface ApplicationIdSet extends BTreeSet<ApplicationId> {}

/** @name ApplicationIdToWorkerIdMap */
export interface ApplicationIdToWorkerIdMap extends BTreeMap<ApplicationId, WorkerId> {}

/** @name ApplicationOf */
export interface ApplicationOf extends Struct {
  readonly role_account_id: GenericAccountId;
  readonly opening_id: OpeningId;
  readonly member_id: MemberId;
  readonly application_id: ApplicationId;
}

/** @name ApplicationRationingPolicy */
export interface ApplicationRationingPolicy extends Struct {
  readonly max_active_applicants: u32;
}

/** @name ApplicationStage */
export interface ApplicationStage extends Enum {
  readonly isActive: boolean;
  readonly isUnstaking: boolean;
  readonly asUnstaking: UnstakingApplicationStage;
  readonly isInactive: boolean;
  readonly asInactive: InactiveApplicationStage;
}

/** @name Approved */
export interface Approved extends Enum {
  readonly isPendingExecution: boolean;
  readonly isExecuted: boolean;
  readonly isExecutionFailed: boolean;
  readonly asExecutionFailed: ExecutionFailed;
}

/** @name Backer */
export interface Backer extends Struct {
  readonly member: GenericAccountId;
  readonly stake: u128;
}

/** @name Backers */
export interface Backers extends Vec<Backer> {}

/** @name BagId */
export interface BagId extends Enum {
  readonly isStatic: boolean;
  readonly asStatic: Static;
  readonly isDynamic: boolean;
  readonly asDynamic: DynamicBagIdType;
}

/** @name BagIdType */
export interface BagIdType extends Enum {
  readonly isStatic: boolean;
  readonly asStatic: Static;
  readonly isDynamic: boolean;
  readonly asDynamic: DynamicBagIdType;
}

/** @name BalanceOfMint */
export interface BalanceOfMint extends u128 {}

/** @name BlockAndTime */
export interface BlockAndTime extends Struct {
  readonly block: u32;
  readonly time: u64;
}

/** @name Category */
export interface Category extends Struct {
  readonly id: CategoryId;
  readonly title: Text;
  readonly description: Text;
  readonly created_at: BlockAndTime;
  readonly deleted: bool;
  readonly archived: bool;
  readonly num_direct_subcategories: u32;
  readonly num_direct_unmoderated_threads: u32;
  readonly num_direct_moderated_threads: u32;
  readonly position_in_parent_category: Option<ChildPositionInParentCategory>;
  readonly moderator_id: GenericAccountId;
}

/** @name CategoryId */
export interface CategoryId extends u64 {}

/** @name Channel */
export interface Channel extends Struct {
  readonly verified: bool;
  readonly handle: Text;
  readonly title: OptionalText;
  readonly description: OptionalText;
  readonly avatar: OptionalText;
  readonly banner: OptionalText;
  readonly content: ChannelContentType;
  readonly owner: MemberId;
  readonly role_account: GenericAccountId;
  readonly publication_status: ChannelPublicationStatus;
  readonly curation_status: ChannelCurationStatus;
  readonly created: u32;
  readonly principal_id: PrincipalId;
}

/** @name ChannelContentType */
export interface ChannelContentType extends Enum {
  readonly isVideo: boolean;
  readonly isMusic: boolean;
  readonly isEbook: boolean;
}

/** @name ChannelCurationStatus */
export interface ChannelCurationStatus extends Enum {
  readonly isNormal: boolean;
  readonly isCensored: boolean;
}

/** @name ChannelId */
export interface ChannelId extends u64 {}

/** @name ChannelPublicationStatus */
export interface ChannelPublicationStatus extends Enum {
  readonly isPublic: boolean;
  readonly isUnlisted: boolean;
}

/** @name ChildPositionInParentCategory */
export interface ChildPositionInParentCategory extends Struct {
  readonly parent_id: CategoryId;
  readonly child_nr_in_parent_category: u32;
}

/** @name Class */
export interface Class extends Struct {
  readonly class_permissions: ClassPermissions;
  readonly properties: Vec<Property>;
  readonly schemas: Vec<Schema>;
  readonly name: Text;
  readonly description: Text;
  readonly maximum_entities_count: EntityId;
  readonly current_number_of_entities: EntityId;
  readonly default_entity_creation_voucher_upper_bound: EntityId;
}

/** @name ClassId */
export interface ClassId extends u64 {}

/** @name ClassOf */
export interface ClassOf extends Struct {
  readonly class_permissions: ClassPermissions;
  readonly properties: Vec<Property>;
  readonly schemas: Vec<Schema>;
  readonly name: Text;
  readonly description: Text;
  readonly maximum_entities_count: EntityId;
  readonly current_number_of_entities: EntityId;
  readonly default_entity_creation_voucher_upper_bound: EntityId;
}

/** @name ClassPermissions */
export interface ClassPermissions extends Struct {
  readonly any_member: bool;
  readonly entity_creation_blocked: bool;
  readonly all_entity_property_values_locked: bool;
  readonly maintainers: Vec<CuratorGroupId>;
}

/** @name ClassPermissionsType */
export interface ClassPermissionsType extends Null {}

/** @name ClassPropertyValue */
export interface ClassPropertyValue extends Null {}

/** @name ContentId */
export interface ContentId extends Text {}

/** @name ContentIdSet */
export interface ContentIdSet extends BTreeSet<ContentId> {}

/** @name CreateEntityOperation */
export interface CreateEntityOperation extends Struct {
  readonly class_id: ClassId;
}

/** @name Credential */
export interface Credential extends u64 {}

/** @name CredentialSet */
export interface CredentialSet extends BTreeSet<Credential> {}

/** @name CurationActor */
export interface CurationActor extends Enum {
  readonly isLead: boolean;
  readonly isCurator: boolean;
  readonly asCurator: CuratorId;
}

/** @name Curator */
export interface Curator extends Struct {
  readonly role_account: GenericAccountId;
  readonly reward_relationship: Option<RewardRelationshipId>;
  readonly role_stake_profile: Option<CuratorRoleStakeProfile>;
  readonly stage: CuratorRoleStage;
  readonly induction: CuratorInduction;
  readonly principal_id: PrincipalId;
}

/** @name CuratorApplication */
export interface CuratorApplication extends Struct {
  readonly role_account: GenericAccountId;
  readonly curator_opening_id: CuratorOpeningId;
  readonly member_id: MemberId;
  readonly application_id: ApplicationId;
}

/** @name CuratorApplicationId */
export interface CuratorApplicationId extends u64 {}

/** @name CuratorApplicationIdSet */
export interface CuratorApplicationIdSet extends BTreeSet<CuratorApplicationId> {}

/** @name CuratorApplicationIdToCuratorIdMap */
export interface CuratorApplicationIdToCuratorIdMap extends BTreeMap<ApplicationId, CuratorId> {}

/** @name CuratorExitInitiationOrigin */
export interface CuratorExitInitiationOrigin extends Enum {
  readonly isLead: boolean;
  readonly isCurator: boolean;
}

/** @name CuratorExitSummary */
export interface CuratorExitSummary extends Struct {
  readonly origin: CuratorExitInitiationOrigin;
  readonly initiated_at_block_number: u32;
  readonly rationale_text: Text;
}

/** @name CuratorGroup */
export interface CuratorGroup extends Struct {
  readonly curators: Vec<u64>;
  readonly active: bool;
  readonly number_of_classes_maintained: u32;
}

/** @name CuratorGroupId */
export interface CuratorGroupId extends u64 {}

/** @name CuratorId */
export interface CuratorId extends u64 {}

/** @name CuratorInduction */
export interface CuratorInduction extends Struct {
  readonly lead: LeadId;
  readonly curator_application_id: CuratorApplicationId;
  readonly at_block: u32;
}

/** @name CuratorOpening */
export interface CuratorOpening extends Struct {
  readonly opening_id: OpeningId;
  readonly curator_applications: Vec<CuratorApplicationId>;
  readonly policy_commitment: OpeningPolicyCommitment;
}

/** @name CuratorOpeningId */
export interface CuratorOpeningId extends u64 {}

/** @name CuratorRoleStage */
export interface CuratorRoleStage extends Enum {
  readonly isActive: boolean;
  readonly isUnstaking: boolean;
  readonly asUnstaking: CuratorExitSummary;
  readonly isExited: boolean;
  readonly asExited: CuratorExitSummary;
}

/** @name CuratorRoleStakeProfile */
export interface CuratorRoleStakeProfile extends Struct {
  readonly stake_id: StakeId;
  readonly termination_unstaking_period: Option<u32>;
  readonly exit_unstaking_period: Option<u32>;
}

/** @name DataObject */
export interface DataObject extends Struct {
  readonly owner: MemberId;
  readonly added_at: BlockAndTime;
  readonly type_id: DataObjectTypeId;
  readonly liaison: StorageProviderId;
  readonly liaison_judgement: LiaisonJudgement;
  readonly ipfs_content_id: Text;
}

/** @name DataObjectCreationParameters */
export interface DataObjectCreationParameters extends Struct {
  readonly ipfsContentId: Text;
}

/** @name DataObjectId */
export interface DataObjectId extends u64 {}

/** @name DataObjectIdSet */
export interface DataObjectIdSet extends BTreeSet<DataObjectId> {}

/** @name DataObjectsMap */
export interface DataObjectsMap extends BTreeMap<U8aFixed, DataObject> {}

/** @name DataObjectStorageRelationship */
export interface DataObjectStorageRelationship extends Struct {
  readonly content_id: Hash;
  readonly storage_provider: StorageProviderId;
  readonly ready: bool;
}

/** @name DataObjectStorageRelationshipId */
export interface DataObjectStorageRelationshipId extends u64 {}

/** @name DataObjectType */
export interface DataObjectType extends Struct {
  readonly description: Text;
  readonly active: bool;
}

/** @name DataObjectTypeId */
export interface DataObjectTypeId extends u64 {}

/** @name Deactivated */
export interface Deactivated extends Struct {
  readonly cause: OpeningDeactivationCause;
  readonly deactivated_at_block: u32;
  readonly started_accepting_applicants_at_block: u32;
  readonly started_review_period_at_block: Option<u32>;
}

/** @name DiscussionPost */
export interface DiscussionPost extends Struct {
  readonly text: Bytes;
  readonly created_at: u32;
  readonly updated_at: u32;
  readonly author_id: MemberId;
  readonly thread_id: ThreadId;
  readonly edition_number: u32;
}

/** @name DiscussionThread */
export interface DiscussionThread extends Struct {
  readonly title: Bytes;
  readonly created_at: u32;
  readonly author_id: MemberId;
}

/** @name DynamicBag */
export interface DynamicBag extends u64 {}

/** @name DynamicBagCreationPolicy */
export interface DynamicBagCreationPolicy extends Struct {
  readonly numberOfStorageBuckets: u64;
}

/** @name DynamicBagId */
export interface DynamicBagId extends Enum {
  readonly isMember: boolean;
  readonly asMember: MemberId;
  readonly isChannel: boolean;
  readonly asChannel: ChannelId;
}

/** @name DynamicBagIdType */
export interface DynamicBagIdType extends Enum {
  readonly isMember: boolean;
  readonly asMember: MemberId;
  readonly isChannel: boolean;
  readonly asChannel: ChannelId;
}

/** @name DynamicBagType */
export interface DynamicBagType extends Enum {
  readonly isMember: boolean;
  readonly isChannel: boolean;
}

/** @name ElectionParameters */
export interface ElectionParameters extends Struct {
  readonly announcing_period: u32;
  readonly voting_period: u32;
  readonly revealing_period: u32;
  readonly council_size: u32;
  readonly candidacy_limit: u32;
  readonly new_term_duration: u32;
  readonly min_council_stake: u128;
  readonly min_voting_stake: u128;
}

/** @name ElectionStage */
export interface ElectionStage extends Enum {
  readonly isAnnouncing: boolean;
  readonly asAnnouncing: u32;
  readonly isVoting: boolean;
  readonly asVoting: u32;
  readonly isRevealing: boolean;
  readonly asRevealing: u32;
}

/** @name ElectionStake */
export interface ElectionStake extends Struct {
  readonly new: u128;
  readonly transferred: u128;
}

/** @name Entity */
export interface Entity extends Struct {
  readonly entity_permissions: EntityPermissions;
  readonly class_id: ClassId;
  readonly supported_schemas: Vec<SchemaId>;
  readonly reference_counter: InboundReferenceCounter;
}

/** @name EntityController */
export interface EntityController extends Enum {
  readonly isMaintainers: boolean;
  readonly isMember: boolean;
  readonly asMember: MemberId;
  readonly isLead: boolean;
}

/** @name EntityCreationVoucher */
export interface EntityCreationVoucher extends Struct {
  readonly maximum_entities_count: EntityId;
  readonly entities_created: EntityId;
}

/** @name EntityId */
export interface EntityId extends u64 {}

/** @name EntityOf */
export interface EntityOf extends Struct {
  readonly entity_permissions: EntityPermissions;
  readonly class_id: ClassId;
  readonly supported_schemas: Vec<SchemaId>;
  readonly reference_counter: InboundReferenceCounter;
}

/** @name EntityPermissions */
export interface EntityPermissions extends Struct {
  readonly controller: EntityController;
  readonly frozen: bool;
  readonly referenceable: bool;
}

/** @name EntityReferenceCounterSideEffect */
export interface EntityReferenceCounterSideEffect extends Struct {
  readonly total: i32;
  readonly same_owner: i32;
}

/** @name EntryMethod */
export interface EntryMethod extends Enum {
  readonly isPaid: boolean;
  readonly asPaid: u64;
  readonly isScreening: boolean;
  readonly asScreening: AccountId;
  readonly isGenesis: boolean;
}

/** @name ExecutionFailed */
export interface ExecutionFailed extends Struct {
  readonly error: Text;
}

/** @name ExitedLeadRole */
export interface ExitedLeadRole extends Struct {
  readonly initiated_at_block_number: u32;
}

/** @name FailedAt */
export interface FailedAt extends u32 {}

/** @name FillOpeningParameters */
export interface FillOpeningParameters extends Struct {
  readonly opening_id: OpeningId;
  readonly successful_application_id: ApplicationId;
  readonly reward_policy: Option<RewardPolicy>;
  readonly working_group: WorkingGroup;
}

/** @name Finalized */
export interface Finalized extends Struct {
  readonly proposalStatus: ProposalDecisionStatus;
  readonly finalizedAt: u32;
  readonly encodedUnstakingErrorDueToBrokenRuntime: Option<Bytes>;
  readonly stakeDataAfterUnstakingError: Option<ActiveStake>;
}

/** @name HashedTextMaxLength */
export interface HashedTextMaxLength extends Option<u16> {}

/** @name HiringApplicationId */
export interface HiringApplicationId extends u64 {}

/** @name InactiveApplicationStage */
export interface InactiveApplicationStage extends Struct {
  readonly deactivation_initiated: u32;
  readonly deactivated: u32;
  readonly cause: ApplicationDeactivationCause;
}

/** @name InboundReferenceCounter */
export interface InboundReferenceCounter extends Struct {
  readonly total: u32;
  readonly same_owner: u32;
}

/** @name InputEntityValuesMap */
export interface InputEntityValuesMap extends BTreeMap<PropertyId, InputPropertyValue> {}

/** @name InputPropertyValue */
export interface InputPropertyValue extends Enum {
  readonly isSingle: boolean;
  readonly asSingle: InputValue;
  readonly isVector: boolean;
  readonly asVector: VecInputValue;
}

/** @name InputValidationLengthConstraint */
export interface InputValidationLengthConstraint extends Struct {
  readonly min: u16;
  readonly max_min_diff: u16;
}

/** @name InputValue */
export interface InputValue extends Enum {
  readonly isBool: boolean;
  readonly asBool: bool;
  readonly isUint16: boolean;
  readonly asUint16: u16;
  readonly isUint32: boolean;
  readonly asUint32: u32;
  readonly isUint64: boolean;
  readonly asUint64: u64;
  readonly isInt16: boolean;
  readonly asInt16: i16;
  readonly isInt32: boolean;
  readonly asInt32: i32;
  readonly isInt64: boolean;
  readonly asInt64: i64;
  readonly isText: boolean;
  readonly asText: Text;
  readonly isTextToHash: boolean;
  readonly asTextToHash: Text;
  readonly isReference: boolean;
  readonly asReference: EntityId;
}

/** @name IPNSIdentity */
export interface IPNSIdentity extends Text {}

/** @name Lead */
export interface Lead extends Struct {
  readonly member_id: MemberId;
  readonly role_account: GenericAccountId;
  readonly reward_relationship: Option<RewardRelationshipId>;
  readonly inducted: u32;
  readonly stage: LeadRoleState;
}

/** @name LeadId */
export interface LeadId extends u64 {}

/** @name LeadRoleState */
export interface LeadRoleState extends Enum {
  readonly isActive: boolean;
  readonly isExited: boolean;
  readonly asExited: ExitedLeadRole;
}

/** @name LiaisonJudgement */
export interface LiaisonJudgement extends Enum {
  readonly isPending: boolean;
  readonly isAccepted: boolean;
  readonly isRejected: boolean;
}

/** @name LookupSource */
export interface LookupSource extends AccountId {}

/** @name MemberId */
export interface MemberId extends u64 {}

/** @name Membership */
export interface Membership extends Struct {
  readonly handle: Text;
  readonly avatar_uri: Text;
  readonly about: Text;
  readonly registered_at_block: u32;
  readonly registered_at_time: u64;
  readonly entry: EntryMethod;
  readonly suspended: bool;
  readonly subscription: Option<SubscriptionId>;
  readonly root_account: GenericAccountId;
  readonly controller_account: GenericAccountId;
}

/** @name MemoText */
export interface MemoText extends Text {}

/** @name Mint */
export interface Mint extends Struct {
  readonly capacity: u128;
  readonly next_adjustment: Option<NextAdjustment>;
  readonly created_at: u32;
  readonly total_minted: u128;
}

/** @name MintBalanceOf */
export interface MintBalanceOf extends u128 {}

/** @name MintId */
export interface MintId extends u64 {}

/** @name ModerationAction */
export interface ModerationAction extends Struct {
  readonly moderated_at: BlockAndTime;
  readonly moderator_id: GenericAccountId;
  readonly rationale: Text;
}

/** @name NextAdjustment */
export interface NextAdjustment extends Struct {
  readonly adjustment: AdjustOnInterval;
  readonly at_block: u32;
}

/** @name Nonce */
export interface Nonce extends u64 {}

/** @name Opening */
export interface Opening extends Struct {
  readonly created: u32;
  readonly stage: OpeningStage;
  readonly max_review_period_length: u32;
  readonly application_rationing_policy: Option<ApplicationRationingPolicy>;
  readonly application_staking_policy: Option<StakingPolicy>;
  readonly role_staking_policy: Option<StakingPolicy>;
  readonly human_readable_text: Text;
}

/** @name OpeningDeactivationCause */
export interface OpeningDeactivationCause extends Enum {
  readonly isCancelledBeforeActivation: boolean;
  readonly isCancelledAcceptingApplications: boolean;
  readonly isCancelledInReviewPeriod: boolean;
  readonly isReviewPeriodExpired: boolean;
  readonly isFilled: boolean;
}

/** @name OpeningId */
export interface OpeningId extends u64 {}

/** @name OpeningOf */
export interface OpeningOf extends Struct {
  readonly hiring_opening_id: OpeningId;
  readonly applications: Vec<ApplicationId>;
  readonly policy_commitment: WorkingGroupOpeningPolicyCommitment;
  readonly opening_type: OpeningType;
}

/** @name OpeningPolicyCommitment */
export interface OpeningPolicyCommitment extends Struct {
  readonly application_rationing_policy: Option<ApplicationRationingPolicy>;
  readonly max_review_period_length: u32;
  readonly application_staking_policy: Option<StakingPolicy>;
  readonly role_staking_policy: Option<StakingPolicy>;
  readonly role_slashing_terms: SlashingTerms;
  readonly fill_opening_successful_applicant_application_stake_unstaking_period: Option<u32>;
  readonly fill_opening_failed_applicant_application_stake_unstaking_period: Option<u32>;
  readonly fill_opening_failed_applicant_role_stake_unstaking_period: Option<u32>;
  readonly terminate_curator_application_stake_unstaking_period: Option<u32>;
  readonly terminate_curator_role_stake_unstaking_period: Option<u32>;
  readonly exit_curator_role_application_stake_unstaking_period: Option<u32>;
  readonly exit_curator_role_stake_unstaking_period: Option<u32>;
}

/** @name OpeningStage */
export interface OpeningStage extends Enum {
  readonly isWaitingToBegin: boolean;
  readonly asWaitingToBegin: WaitingToBeingOpeningStageVariant;
  readonly isActive: boolean;
  readonly asActive: ActiveOpeningStageVariant;
}

/** @name OpeningType */
export interface OpeningType extends Enum {
  readonly isLeader: boolean;
  readonly isWorker: boolean;
}

/** @name Operation */
export interface Operation extends Null {}

/** @name OperationType */
export interface OperationType extends Enum {
  readonly isCreateEntity: boolean;
  readonly asCreateEntity: CreateEntityOperation;
  readonly isUpdatePropertyValues: boolean;
  readonly asUpdatePropertyValues: UpdatePropertyValuesOperation;
  readonly isAddSchemaSupportToEntity: boolean;
  readonly asAddSchemaSupportToEntity: AddSchemaSupportToEntityOperation;
}

/** @name OptionalText */
export interface OptionalText extends Option<Text> {}

/** @name PaidMembershipTerms */
export interface PaidMembershipTerms extends Struct {
  readonly fee: u128;
  readonly text: Text;
}

/** @name PaidTermId */
export interface PaidTermId extends u64 {}

/** @name ParameterizedEntity */
export interface ParameterizedEntity extends Enum {
  readonly isInternalEntityJustAdded: boolean;
  readonly asInternalEntityJustAdded: u32;
  readonly isExistingEntity: boolean;
  readonly asExistingEntity: EntityId;
}

/** @name ParametrizedClassPropertyValue */
export interface ParametrizedClassPropertyValue extends Struct {
  readonly in_class_index: PropertyId;
  readonly value: ParametrizedPropertyValue;
}

/** @name ParametrizedPropertyValue */
export interface ParametrizedPropertyValue extends Enum {
  readonly isInputPropertyValue: boolean;
  readonly asInputPropertyValue: InputPropertyValue;
  readonly isInternalEntityJustAdded: boolean;
  readonly asInternalEntityJustAdded: u32;
  readonly isInternalEntityVec: boolean;
  readonly asInternalEntityVec: Vec<ParameterizedEntity>;
}

/** @name Post */
export interface Post extends Struct {
  readonly id: PostId;
  readonly thread_id: ThreadId;
  readonly nr_in_thread: u32;
  readonly current_text: Text;
  readonly moderation: Option<ModerationAction>;
  readonly text_change_history: Vec<PostTextChange>;
  readonly created_at: BlockAndTime;
  readonly author_id: GenericAccountId;
}

/** @name PostId */
export interface PostId extends u64 {}

/** @name PostTextChange */
export interface PostTextChange extends Struct {
  readonly expired_at: BlockAndTime;
  readonly text: Text;
}

/** @name Principal */
export interface Principal extends Enum {
  readonly isLead: boolean;
  readonly isCurator: boolean;
  readonly asCurator: CuratorId;
  readonly isChannelOwner: boolean;
  readonly asChannelOwner: ChannelId;
}

/** @name PrincipalId */
export interface PrincipalId extends u64 {}

/** @name Property */
export interface Property extends Struct {
  readonly property_type: PropertyType;
  readonly required: bool;
  readonly unique: bool;
  readonly name: Text;
  readonly description: Text;
  readonly locking_policy: PropertyLockingPolicy;
}

/** @name PropertyId */
export interface PropertyId extends u16 {}

/** @name PropertyLockingPolicy */
export interface PropertyLockingPolicy extends Struct {
  readonly is_locked_from_maintainer: bool;
  readonly is_locked_from_controller: bool;
}

/** @name PropertyType */
export interface PropertyType extends Enum {
  readonly isSingle: boolean;
  readonly asSingle: PropertyTypeSingle;
  readonly isVector: boolean;
  readonly asVector: PropertyTypeVector;
}

/** @name PropertyTypeSingle */
export interface PropertyTypeSingle extends Enum {
  readonly isBool: boolean;
  readonly isUint16: boolean;
  readonly isUint32: boolean;
  readonly isUint64: boolean;
  readonly isInt16: boolean;
  readonly isInt32: boolean;
  readonly isInt64: boolean;
  readonly isText: boolean;
  readonly asText: TextMaxLength;
  readonly isHash: boolean;
  readonly asHash: HashedTextMaxLength;
  readonly isReference: boolean;
  readonly asReference: ITuple<[ClassId, SameController]>;
}

/** @name PropertyTypeVector */
export interface PropertyTypeVector extends Struct {
  readonly vec_type: PropertyTypeSingle;
  readonly max_length: VecMaxLength;
}

/** @name ProposalDecisionStatus */
export interface ProposalDecisionStatus extends Enum {
  readonly isCanceled: boolean;
  readonly isVetoed: boolean;
  readonly isRejected: boolean;
  readonly isSlashed: boolean;
  readonly isExpired: boolean;
  readonly isApproved: boolean;
  readonly asApproved: Approved;
}

/** @name ProposalDetails */
export interface ProposalDetails extends Enum {
  readonly isText: boolean;
  readonly asText: Text;
  readonly isRuntimeUpgrade: boolean;
  readonly asRuntimeUpgrade: Bytes;
  readonly isSetElectionParameters: boolean;
  readonly asSetElectionParameters: ElectionParameters;
  readonly isSpending: boolean;
  readonly asSpending: ITuple<[Balance, AccountId]>;
  readonly isSetLead: boolean;
  readonly asSetLead: Option<SetLeadParams>;
  readonly isSetContentWorkingGroupMintCapacity: boolean;
  readonly asSetContentWorkingGroupMintCapacity: u128;
  readonly isEvictStorageProvider: boolean;
  readonly asEvictStorageProvider: GenericAccountId;
  readonly isSetValidatorCount: boolean;
  readonly asSetValidatorCount: u32;
  readonly isSetStorageRoleParameters: boolean;
  readonly asSetStorageRoleParameters: RoleParameters;
  readonly isAddWorkingGroupLeaderOpening: boolean;
  readonly asAddWorkingGroupLeaderOpening: AddOpeningParameters;
  readonly isBeginReviewWorkingGroupLeaderApplication: boolean;
  readonly asBeginReviewWorkingGroupLeaderApplication: ITuple<[OpeningId, WorkingGroup]>;
  readonly isFillWorkingGroupLeaderOpening: boolean;
  readonly asFillWorkingGroupLeaderOpening: FillOpeningParameters;
  readonly isSetWorkingGroupMintCapacity: boolean;
  readonly asSetWorkingGroupMintCapacity: ITuple<[Balance, WorkingGroup]>;
  readonly isDecreaseWorkingGroupLeaderStake: boolean;
  readonly asDecreaseWorkingGroupLeaderStake: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isSlashWorkingGroupLeaderStake: boolean;
  readonly asSlashWorkingGroupLeaderStake: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isSetWorkingGroupLeaderReward: boolean;
  readonly asSetWorkingGroupLeaderReward: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isTerminateWorkingGroupLeaderRole: boolean;
  readonly asTerminateWorkingGroupLeaderRole: TerminateRoleParameters;
}

/** @name ProposalDetailsOf */
export interface ProposalDetailsOf extends Enum {
  readonly isText: boolean;
  readonly asText: Text;
  readonly isRuntimeUpgrade: boolean;
  readonly asRuntimeUpgrade: Bytes;
  readonly isSetElectionParameters: boolean;
  readonly asSetElectionParameters: ElectionParameters;
  readonly isSpending: boolean;
  readonly asSpending: ITuple<[Balance, AccountId]>;
  readonly isSetLead: boolean;
  readonly asSetLead: Option<SetLeadParams>;
  readonly isSetContentWorkingGroupMintCapacity: boolean;
  readonly asSetContentWorkingGroupMintCapacity: u128;
  readonly isEvictStorageProvider: boolean;
  readonly asEvictStorageProvider: GenericAccountId;
  readonly isSetValidatorCount: boolean;
  readonly asSetValidatorCount: u32;
  readonly isSetStorageRoleParameters: boolean;
  readonly asSetStorageRoleParameters: RoleParameters;
  readonly isAddWorkingGroupLeaderOpening: boolean;
  readonly asAddWorkingGroupLeaderOpening: AddOpeningParameters;
  readonly isBeginReviewWorkingGroupLeaderApplication: boolean;
  readonly asBeginReviewWorkingGroupLeaderApplication: ITuple<[OpeningId, WorkingGroup]>;
  readonly isFillWorkingGroupLeaderOpening: boolean;
  readonly asFillWorkingGroupLeaderOpening: FillOpeningParameters;
  readonly isSetWorkingGroupMintCapacity: boolean;
  readonly asSetWorkingGroupMintCapacity: ITuple<[Balance, WorkingGroup]>;
  readonly isDecreaseWorkingGroupLeaderStake: boolean;
  readonly asDecreaseWorkingGroupLeaderStake: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isSlashWorkingGroupLeaderStake: boolean;
  readonly asSlashWorkingGroupLeaderStake: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isSetWorkingGroupLeaderReward: boolean;
  readonly asSetWorkingGroupLeaderReward: ITuple<[WorkerId, Balance, WorkingGroup]>;
  readonly isTerminateWorkingGroupLeaderRole: boolean;
  readonly asTerminateWorkingGroupLeaderRole: TerminateRoleParameters;
}

/** @name ProposalId */
export interface ProposalId extends u32 {}

/** @name ProposalOf */
export interface ProposalOf extends Struct {
  readonly parameters: ProposalParameters;
  readonly proposerId: MemberId;
  readonly title: Text;
  readonly description: Text;
  readonly createdAt: u32;
  readonly status: ProposalStatus;
  readonly votingResults: VotingResults;
}

/** @name ProposalParameters */
export interface ProposalParameters extends Struct {
  readonly votingPeriod: u32;
  readonly gracePeriod: u32;
  readonly approvalQuorumPercentage: u32;
  readonly approvalThresholdPercentage: u32;
  readonly slashingQuorumPercentage: u32;
  readonly slashingThresholdPercentage: u32;
  readonly requiredStake: Option<u128>;
}

/** @name ProposalStatus */
export interface ProposalStatus extends Enum {
  readonly isActive: boolean;
  readonly asActive: Option<ActiveStake>;
  readonly isFinalized: boolean;
  readonly asFinalized: Finalized;
}

/** @name RationaleText */
export interface RationaleText extends Bytes {}

/** @name Recipient */
export interface Recipient extends Struct {
  readonly total_reward_received: u128;
  readonly total_reward_missed: u128;
}

/** @name RecipientId */
export interface RecipientId extends u64 {}

/** @name ReferenceConstraint */
export interface ReferenceConstraint extends Null {}

/** @name ReferenceCounterSideEffects */
export interface ReferenceCounterSideEffects extends BTreeMap<EntityId, EntityReferenceCounterSideEffect> {}

/** @name Reply */
export interface Reply extends Struct {
  readonly owner: GenericAccountId;
  readonly thread_id: ThreadId;
  readonly text: Text;
  readonly moderation: Option<ModerationAction>;
}

/** @name ReplyId */
export interface ReplyId extends u64 {}

/** @name ReviewPeriod */
export interface ReviewPeriod extends Struct {
  readonly started_accepting_applicants_at_block: u32;
  readonly started_review_period_at_block: u32;
}

/** @name RewardPolicy */
export interface RewardPolicy extends Struct {
  readonly amount_per_payout: u128;
  readonly next_payment_at_block: u32;
  readonly payout_interval: Option<u32>;
}

/** @name RewardRelationship */
export interface RewardRelationship extends Struct {
  readonly recipient: RecipientId;
  readonly mint_id: MintId;
  readonly account: GenericAccountId;
  readonly amount_per_payout: u128;
  readonly next_payment_at_block: Option<u32>;
  readonly payout_interval: Option<u32>;
  readonly total_reward_received: u128;
  readonly total_reward_missed: u128;
}

/** @name RewardRelationshipId */
export interface RewardRelationshipId extends u64 {}

/** @name RoleParameters */
export interface RoleParameters extends Struct {
  readonly min_stake: u128;
  readonly min_actors: u32;
  readonly max_actors: u32;
  readonly reward: u128;
  readonly reward_period: u32;
  readonly bonding_period: u32;
  readonly unbonding_period: u32;
  readonly min_service_period: u32;
  readonly startup_grace_period: u32;
  readonly entry_request_fee: u128;
}

/** @name RoleStakeProfile */
export interface RoleStakeProfile extends Struct {
  readonly stake_id: StakeId;
  readonly termination_unstaking_period: Option<u32>;
  readonly exit_unstaking_period: Option<u32>;
}

/** @name SameController */
export interface SameController extends bool {}

/** @name Schema */
export interface Schema extends Struct {
  readonly properties: Vec<PropertyId>;
  readonly is_active: bool;
}

/** @name SchemaId */
export interface SchemaId extends u16 {}

/** @name SealedVote */
export interface SealedVote extends Struct {
  readonly voter: GenericAccountId;
  readonly commitment: Hash;
  readonly stake: ElectionStake;
  readonly vote: Option<GenericAccountId>;
}

/** @name Seat */
export interface Seat extends Struct {
  readonly member: GenericAccountId;
  readonly stake: u128;
  readonly backers: Backers;
}

/** @name Seats */
export interface Seats extends Vec<Seat> {}

/** @name ServiceProviderRecord */
export interface ServiceProviderRecord extends Struct {
  readonly identity: IPNSIdentity;
  readonly expires_at: u32;
}

/** @name SetLeadParams */
export interface SetLeadParams extends ITuple<[MemberId, GenericAccountId]> {}

/** @name SideEffect */
export interface SideEffect extends Option<ITuple<[EntityId, EntityReferenceCounterSideEffect]>> {}

/** @name SideEffects */
export interface SideEffects extends Option<ReferenceCounterSideEffects> {}

/** @name Slash */
export interface Slash extends Struct {
  readonly started_at_block: u32;
  readonly is_active: bool;
  readonly blocks_remaining_in_active_period_for_slashing: u32;
  readonly slash_amount: u128;
}

/** @name SlashableTerms */
export interface SlashableTerms extends Struct {
  readonly max_count: u16;
  readonly max_percent_pts_per_time: u16;
}

/** @name SlashingTerms */
export interface SlashingTerms extends Enum {
  readonly isUnslashable: boolean;
  readonly isSlashable: boolean;
  readonly asSlashable: SlashableTerms;
}

/** @name Stake */
export interface Stake extends Struct {
  readonly created: u32;
  readonly staking_status: StakingStatus;
}

/** @name Staked */
export interface Staked extends Struct {
  readonly staked_amount: u128;
  readonly staked_status: StakedStatus;
  readonly next_slash_id: u64;
  readonly ongoing_slashes: BTreeMap<u64, Slash>;
}

/** @name StakedStatus */
export interface StakedStatus extends Enum {
  readonly isNormal: boolean;
  readonly isUnstaking: boolean;
  readonly asUnstaking: Unstaking;
}

/** @name StakeId */
export interface StakeId extends u64 {}

/** @name StakingAmountLimitMode */
export interface StakingAmountLimitMode extends Enum {
  readonly isAtLeast: boolean;
  readonly isExact: boolean;
}

/** @name StakingPolicy */
export interface StakingPolicy extends Struct {
  readonly amount: u128;
  readonly amount_mode: StakingAmountLimitMode;
  readonly crowded_out_unstaking_period_length: Option<u32>;
  readonly review_period_expired_unstaking_period_length: Option<u32>;
}

/** @name StakingStatus */
export interface StakingStatus extends Enum {
  readonly isNotStaked: boolean;
  readonly isStaked: boolean;
  readonly asStaked: Staked;
}

/** @name Static */
export interface Static extends Enum {
  readonly isCouncil: boolean;
  readonly isWorkingGroup: boolean;
  readonly asWorkingGroup: WorkingGroup;
}

/** @name StaticBag */
export interface StaticBag extends u64 {}

/** @name StaticBagId */
export interface StaticBagId extends Enum {
  readonly isCouncil: boolean;
  readonly isWorkingGroup: boolean;
  readonly asWorkingGroup: WorkingGroup;
}

/** @name Status */
export interface Status extends bool {}

/** @name StorageBucket */
export interface StorageBucket extends u64 {}

/** @name StorageBucketId */
export interface StorageBucketId extends u64 {}

/** @name StorageBucketIdSet */
export interface StorageBucketIdSet extends BTreeSet<StorageBucketId> {}

/** @name StorageBucketsPerBagValueConstraint */
export interface StorageBucketsPerBagValueConstraint extends Struct {
  readonly min: u64;
  readonly max_min_diff: u64;
}

/** @name StorageProviderId */
export interface StorageProviderId extends u64 {}

/** @name StoredPropertyValue */
export interface StoredPropertyValue extends Enum {
  readonly isSingle: boolean;
  readonly asSingle: StoredValue;
  readonly isVector: boolean;
  readonly asVector: VecStoredPropertyValue;
}

/** @name StoredValue */
export interface StoredValue extends Enum {
  readonly isBool: boolean;
  readonly asBool: bool;
  readonly isUint16: boolean;
  readonly asUint16: u16;
  readonly isUint32: boolean;
  readonly asUint32: u32;
  readonly isUint64: boolean;
  readonly asUint64: u64;
  readonly isInt16: boolean;
  readonly asInt16: i16;
  readonly isInt32: boolean;
  readonly asInt32: i32;
  readonly isInt64: boolean;
  readonly asInt64: i64;
  readonly isText: boolean;
  readonly asText: Text;
  readonly isHash: boolean;
  readonly asHash: Hash;
  readonly isReference: boolean;
  readonly asReference: EntityId;
}

/** @name SubscriptionId */
export interface SubscriptionId extends u64 {}

/** @name TerminateRoleParameters */
export interface TerminateRoleParameters extends Struct {
  readonly worker_id: WorkerId;
  readonly rationale: Bytes;
  readonly slash: bool;
  readonly working_group: WorkingGroup;
}

/** @name TextMaxLength */
export interface TextMaxLength extends u16 {}

/** @name Thread */
export interface Thread extends Struct {
  readonly id: ThreadId;
  readonly title: Text;
  readonly category_id: CategoryId;
  readonly nr_in_category: u32;
  readonly moderation: Option<ModerationAction>;
  readonly num_unmoderated_posts: u32;
  readonly num_moderated_posts: u32;
  readonly created_at: BlockAndTime;
  readonly author_id: GenericAccountId;
}

/** @name ThreadCounter */
export interface ThreadCounter extends Struct {
  readonly author_id: MemberId;
  readonly counter: u32;
}

/** @name ThreadId */
export interface ThreadId extends u64 {}

/** @name TransferableStake */
export interface TransferableStake extends Struct {
  readonly seat: u128;
  readonly backing: u128;
}

/** @name Unstaking */
export interface Unstaking extends Struct {
  readonly started_at_block: u32;
  readonly is_active: bool;
  readonly blocks_remaining_in_active_period_for_unstaking: u32;
}

/** @name UnstakingApplicationStage */
export interface UnstakingApplicationStage extends Struct {
  readonly deactivation_initiated: u32;
  readonly cause: ApplicationDeactivationCause;
}

/** @name UpdatePropertyValuesOperation */
export interface UpdatePropertyValuesOperation extends Struct {
  readonly entity_id: ParameterizedEntity;
  readonly new_parametrized_property_values: Vec<ParametrizedClassPropertyValue>;
}

/** @name UploadParameters */
export interface UploadParameters extends Struct {
  readonly authenticationKey: Text;
  readonly bagId: BagId;
  readonly objectCreationList: Vec<DataObjectCreationParameters>;
  readonly deletionPrizeSourceAccountId: GenericAccountId;
}

/** @name Url */
export interface Url extends Text {}

/** @name VecInputValue */
export interface VecInputValue extends Enum {
  readonly isBool: boolean;
  readonly asBool: Vec<bool>;
  readonly isUint16: boolean;
  readonly asUint16: Vec<u16>;
  readonly isUint32: boolean;
  readonly asUint32: Vec<u32>;
  readonly isUint64: boolean;
  readonly asUint64: Vec<u64>;
  readonly isInt16: boolean;
  readonly asInt16: Vec<i16>;
  readonly isInt32: boolean;
  readonly asInt32: Vec<i32>;
  readonly isInt64: boolean;
  readonly asInt64: Vec<i64>;
  readonly isTextToHash: boolean;
  readonly asTextToHash: Vec<Text>;
  readonly isText: boolean;
  readonly asText: Vec<Text>;
  readonly isReference: boolean;
  readonly asReference: Vec<EntityId>;
}

/** @name VecMaxLength */
export interface VecMaxLength extends u16 {}

/** @name VecStoredPropertyValue */
export interface VecStoredPropertyValue extends Struct {
  readonly vec_value: VecStoredValue;
  readonly nonce: Nonce;
}

/** @name VecStoredValue */
export interface VecStoredValue extends Enum {
  readonly isBool: boolean;
  readonly asBool: Vec<bool>;
  readonly isUint16: boolean;
  readonly asUint16: Vec<u16>;
  readonly isUint32: boolean;
  readonly asUint32: Vec<u32>;
  readonly isUint64: boolean;
  readonly asUint64: Vec<u64>;
  readonly isInt16: boolean;
  readonly asInt16: Vec<i16>;
  readonly isInt32: boolean;
  readonly asInt32: Vec<i32>;
  readonly isInt64: boolean;
  readonly asInt64: Vec<i64>;
  readonly isHash: boolean;
  readonly asHash: Vec<Hash>;
  readonly isText: boolean;
  readonly asText: Vec<Text>;
  readonly isReference: boolean;
  readonly asReference: Vec<EntityId>;
}

/** @name VoteKind */
export interface VoteKind extends Enum {
  readonly isApprove: boolean;
  readonly isReject: boolean;
  readonly isSlash: boolean;
  readonly isAbstain: boolean;
}

/** @name VotingResults */
export interface VotingResults extends Struct {
  readonly abstensions: u32;
  readonly approvals: u32;
  readonly rejections: u32;
  readonly slashes: u32;
}

/** @name Voucher */
export interface Voucher extends Struct {
  readonly sizeLimit: u64;
  readonly objectsLimit: u64;
  readonly sizeUsed: u64;
  readonly objectsUsed: u64;
}

/** @name WaitingToBeingOpeningStageVariant */
export interface WaitingToBeingOpeningStageVariant extends Struct {
  readonly begins_at_block: u32;
}

/** @name WorkerId */
export interface WorkerId extends u64 {}

/** @name WorkerOf */
export interface WorkerOf extends Struct {
  readonly member_id: MemberId;
  readonly role_account_id: GenericAccountId;
  readonly reward_relationship: Option<RewardRelationshipId>;
  readonly role_stake_profile: Option<RoleStakeProfile>;
}

/** @name WorkingGroup */
export interface WorkingGroup extends Enum {
  readonly isStorage: boolean;
  readonly isContent: boolean;
}

/** @name WorkingGroupOpeningPolicyCommitment */
export interface WorkingGroupOpeningPolicyCommitment extends Struct {
  readonly application_rationing_policy: Option<ApplicationRationingPolicy>;
  readonly max_review_period_length: u32;
  readonly application_staking_policy: Option<StakingPolicy>;
  readonly role_staking_policy: Option<StakingPolicy>;
  readonly role_slashing_terms: SlashingTerms;
  readonly fill_opening_successful_applicant_application_stake_unstaking_period: Option<u32>;
  readonly fill_opening_failed_applicant_application_stake_unstaking_period: Option<u32>;
  readonly fill_opening_failed_applicant_role_stake_unstaking_period: Option<u32>;
  readonly terminate_application_stake_unstaking_period: Option<u32>;
  readonly terminate_role_stake_unstaking_period: Option<u32>;
  readonly exit_role_application_stake_unstaking_period: Option<u32>;
  readonly exit_role_stake_unstaking_period: Option<u32>;
}

/** @name WorkingGroupUnstaker */
export interface WorkingGroupUnstaker extends Enum {
  readonly isLead: boolean;
  readonly asLead: LeadId;
  readonly isCurator: boolean;
  readonly asCurator: CuratorId;
}

export type PHANTOM_ALL = 'all';
