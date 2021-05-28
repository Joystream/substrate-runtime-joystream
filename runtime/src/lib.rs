//! The Joystream Substrate Node runtime.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
//Substrate internal issues.
#![allow(clippy::large_enum_variant)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(non_fmt_panic)]
#![allow(clippy::from_over_into)]

// Make the WASM binary available.
// This is required only by the node build.
// A dummy wasm_binary.rs will be built for the IDE.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(feature = "std")]
/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
pub fn wasm_binary_unwrap() -> &'static [u8] {
    WASM_BINARY.expect(
        "Development wasm binary is not available. This means the client is \
        built with `BUILD_DUMMY_WASM_BINARY` flag and it is only usable for \
        production chains. Please rebuild with the flag disabled.",
    )
}

mod constants;
mod integration;
pub mod primitives;
mod runtime_api;
#[cfg(test)]
mod tests;
mod weights; // Runtime integration tests

use frame_support::dispatch::DispatchResult;
use frame_support::traits::{Currency, KeyOwnerProofSystem, OnUnbalanced};
use frame_support::weights::{
    constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
    Weight,
};
use frame_support::weights::{WeightToFeeCoefficients, WeightToFeePolynomial};
use frame_support::{construct_runtime, parameter_types};
use frame_system::EnsureRoot;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_session::historical as pallet_session_historical;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::crypto::KeyTypeId;
use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::traits::{BlakeTwo256, Block as BlockT, IdentityLookup, OpaqueKeys, Saturating};
use sp_runtime::{create_runtime_str, generic, impl_opaque_keys, ModuleId, Perbill};
use sp_std::boxed::Box;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use constants::*;
pub use primitives::*;
pub use runtime_api::*;

use integration::proposals::{CouncilManager, ExtrinsicProposalEncoder, MembershipOriginValidator};

use governance::{council, election};
use storage::DynamicBagCreationPolicy;

// Node dependencies
pub use common;
pub use content_directory;
pub use content_directory::{
    HashedTextMaxLength, InputValidationLengthConstraint, MaxNumber, TextMaxLength, VecMaxLength,
};
pub use content_working_group as content_wg;
pub use forum;
pub use governance::election_params::ElectionParameters;
pub use membership;
#[cfg(any(feature = "std", test))]
pub use pallet_balances::Call as BalancesCall;
pub use pallet_staking::StakerStatus;
pub use proposals_codex::ProposalsConfigParameters;
pub use versioned_store;
pub use versioned_store_permissions;
pub use working_group;

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("joystream-node"),
    impl_name: create_runtime_str!("joystream-node"),
    authoring_version: 9,
    spec_version: 3,
    impl_version: 0,
    apis: crate::runtime_api::EXPORTED_RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub const MaximumBlockWeight: Weight = 2 * frame_support::weights::constants::WEIGHT_PER_SECOND;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const Version: RuntimeVersion = VERSION;
    /// Assume 10% of weight for average on_initialize calls.
    pub MaximumExtrinsicWeight: Weight =
        AvailableBlockRatio::get().saturating_sub(AVERAGE_ON_INITIALIZE_WEIGHT)
        * MaximumBlockWeight::get();
}
const AVERAGE_ON_INITIALIZE_WEIGHT: Perbill = Perbill::from_percent(10);

// TODO: adjust weight
impl frame_system::Trait for Runtime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = Call;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = RocksDbWeight;
    type BlockExecutionWeight = BlockExecutionWeight;
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = weights::frame_system::WeightInfo;
}

impl substrate_utility::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = weights::substrate_utility::WeightInfo;
}

parameter_types! {
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
}

impl pallet_babe::Trait for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::IdentificationTuple;

    type HandleEquivocation =
        pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences>;

    type WeightInfo = ();
}

impl pallet_grandpa::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type KeyOwnerProofSystem = Historical;

    type HandleEquivocation =
        pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, Offences>;
    type WeightInfo = ();
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        public: <Signature as sp_runtime::traits::Verify>::Signer,
        account: AccountId,
        nonce: Index,
    ) -> Option<(
        Call,
        <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
    )> {
        integration::transactions::create_transaction::<C>(call, public, account, nonce)
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as sp_runtime::traits::Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

parameter_types! {
    pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Trait for Runtime {
    type Moment = Moment;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = weights::pallet_timestamp::WeightInfo;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 0;
    pub const TransferFee: u128 = 0;
    pub const CreationFee: u128 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Trait for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::WeightInfo;
    type MaxLocks = MaxLocks;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 0;
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
    fn on_nonzero_unbalanced(amount: NegativeImbalance) {
        Balances::resolve_creating(&Authorship::author(), amount);
    }
}

/// Stub for zero transaction weights.
pub struct NoWeights;
impl WeightToFeePolynomial for NoWeights {
    type Balance = Balance;

    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        Default::default()
    }

    fn calc(_weight: &u64) -> Self::Balance {
        Default::default()
    }
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = NoWeights;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Trait for Runtime {
    type Event = Event;
    type Call = Call;
}

parameter_types! {
    pub const UncleGenerations: BlockNumber = 0;
}

impl pallet_authorship::Trait for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = (Staking, ImOnline);
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,
        pub babe: Babe,
        pub im_online: ImOnline,
        pub authority_discovery: AuthorityDiscovery,
    }
}
// NOTE: `SessionHandler` and `SessionKeys` are co-dependent: One key will be used for each handler.
// The number and order of items in `SessionHandler` *MUST* be the same number and order of keys in
// `SessionKeys`.
// TODO: Introduce some structure to tie these together to make it a bit less of a footgun. This
// should be easy, since OneSessionHandler trait provides the `Key` as an associated type. #2858
parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl pallet_session::Trait for Runtime {
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = weights::pallet_session::WeightInfo;
}

impl pallet_session::historical::Trait for Runtime {
    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

pallet_staking_reward_curve::build! {
    const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_050_000,
        max_inflation: 0_750_000,
        ideal_stake: 0_300_000,
        falloff: 0_050_000,
        max_piece_count: 100,
        test_precision: 0_005_000,
    );
}

parameter_types! {
    pub const SessionDuration: BlockNumber = EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    /// We prioritize im-online heartbeats over election solution submission.
    pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
}

parameter_types! {
    pub const SessionsPerEra: sp_staking::SessionIndex = 6;
    pub const BondingDuration: pallet_staking::EraIndex = BONDING_DURATION;
    pub const SlashDeferDuration: pallet_staking::EraIndex = BONDING_DURATION - 1; // 'slightly less' than the bonding duration.
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
    pub const MaxNominatorRewardedPerValidator: u32 = 64;
    pub const ElectionLookahead: BlockNumber = EPOCH_DURATION_IN_BLOCKS / 4;
    pub const MaxIterations: u32 = 10;
    // 0.05%. The higher the value, the more strict solution acceptance becomes.
    pub MinSolutionScoreBump: Perbill = Perbill::from_rational_approximation(5u32, 10_000);
}

impl pallet_staking::Trait for Runtime {
    type Currency = Balances;
    type UnixTime = Timestamp;
    type CurrencyToVote = common::currency::CurrencyToVoteHandler;
    type RewardRemainder = (); // Could be Treasury.
    type Event = Event;
    type Slash = (); // Where to send the slashed funds. Could be Treasury.
    type Reward = (); // Rewards are minted from the void.
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    type SlashCancelOrigin = EnsureRoot<AccountId>; // Requires sudo. Parity recommends: a super-majority of the council can cancel the slash.
    type SessionInterface = Self;
    type RewardCurve = RewardCurve;
    type NextNewSession = Session;
    type ElectionLookahead = ElectionLookahead;
    type Call = Call;
    type MaxIterations = MaxIterations;
    type MinSolutionScoreBump = MinSolutionScoreBump;
    type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
    type UnsignedPriority = StakingUnsignedPriority;
    type WeightInfo = weights::pallet_staking::WeightInfo;
}

impl pallet_im_online::Trait for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type SessionDuration = SessionDuration;
    type ReportUnresponsiveness = Offences;
    // Using the default weights until we check if we can run the benchmarks for this pallet in
    // the reference machine in an acceptable time.
    type WeightInfo = ();
    type UnsignedPriority = ImOnlineUnsignedPriority;
}

parameter_types! {
    pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * MaximumBlockWeight::get();
}

impl pallet_offences::Trait for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
    type WeightSoftLimit = OffencesWeightSoftLimit;
}

impl pallet_authority_discovery::Trait for Runtime {}

parameter_types! {
    pub const WindowSize: BlockNumber = 101;
    pub const ReportLatency: BlockNumber = 1000;
}

impl pallet_finality_tracker::Trait for Runtime {
    type OnFinalizationStalled = ();
    type WindowSize = WindowSize;
    type ReportLatency = ReportLatency;
}

impl versioned_store::Trait for Runtime {
    type Event = Event;
}

impl versioned_store_permissions::Trait for Runtime {
    type Credential = Credential;
    type CredentialChecker = (
        integration::content_working_group::ContentWorkingGroupCredentials,
        integration::versioned_store_permissions::SudoKeyHasAllCredentials,
    );
    type CreateClassPermissionsChecker =
        integration::versioned_store_permissions::ContentLeadOrSudoKeyCanCreateClasses;
}

type EntityId = <Runtime as content_directory::Trait>::EntityId;

parameter_types! {
    pub const PropertyNameLengthConstraint: InputValidationLengthConstraint = InputValidationLengthConstraint::new(1, 49);
    pub const PropertyDescriptionLengthConstraint: InputValidationLengthConstraint = InputValidationLengthConstraint::new(1, 500);
    pub const ClassNameLengthConstraint: InputValidationLengthConstraint = InputValidationLengthConstraint::new(1, 49);
    pub const ClassDescriptionLengthConstraint: InputValidationLengthConstraint = InputValidationLengthConstraint::new(1, 500);
    pub const MaxNumberOfClasses: MaxNumber = 100;
    pub const MaxNumberOfMaintainersPerClass: MaxNumber = 10;
    pub const MaxNumberOfSchemasPerClass: MaxNumber = 20;
    pub const MaxNumberOfPropertiesPerSchema: MaxNumber = 40;
    pub const MaxNumberOfEntitiesPerClass: MaxNumber = 5000;
    pub const MaxNumberOfCuratorsPerGroup: MaxNumber = 50;
    pub const MaxNumberOfOperationsDuringAtomicBatching: MaxNumber = 500;
    pub const VecMaxLengthConstraint: VecMaxLength = 200;
    pub const TextMaxLengthConstraint: TextMaxLength = 5000;
    pub const HashedTextMaxLengthConstraint: HashedTextMaxLength = Some(25000);
    pub const IndividualEntitiesCreationLimit: EntityId = 500;
}

impl content_directory::Trait for Runtime {
    type Event = Event;
    type Nonce = u64;
    type ClassId = u64;
    type EntityId = u64;
    type PropertyNameLengthConstraint = PropertyNameLengthConstraint;
    type PropertyDescriptionLengthConstraint = PropertyDescriptionLengthConstraint;
    type ClassNameLengthConstraint = ClassNameLengthConstraint;
    type ClassDescriptionLengthConstraint = ClassDescriptionLengthConstraint;
    type MaxNumberOfClasses = MaxNumberOfClasses;
    type MaxNumberOfMaintainersPerClass = MaxNumberOfMaintainersPerClass;
    type MaxNumberOfSchemasPerClass = MaxNumberOfSchemasPerClass;
    type MaxNumberOfPropertiesPerSchema = MaxNumberOfPropertiesPerSchema;
    type MaxNumberOfEntitiesPerClass = MaxNumberOfEntitiesPerClass;
    type MaxNumberOfCuratorsPerGroup = MaxNumberOfCuratorsPerGroup;
    type MaxNumberOfOperationsDuringAtomicBatching = MaxNumberOfOperationsDuringAtomicBatching;
    type VecMaxLengthConstraint = VecMaxLengthConstraint;
    type TextMaxLengthConstraint = TextMaxLengthConstraint;
    type HashedTextMaxLengthConstraint = HashedTextMaxLengthConstraint;
    type IndividualEntitiesCreationLimit = IndividualEntitiesCreationLimit;
}

impl hiring::Trait for Runtime {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = (); // TODO - what needs to happen?
    type StakeHandlerProvider = hiring::Module<Self>;
}

impl minting::Trait for Runtime {
    type Currency = <Self as common::currency::GovernanceCurrency>::Currency;
    type MintId = u64;
}

impl recurring_rewards::Trait for Runtime {
    type PayoutStatusHandler = (); // TODO - deal with successful and failed payouts
    type RecipientId = u64;
    type RewardRelationshipId = u64;
}

parameter_types! {
    pub const StakePoolId: [u8; 8] = *b"joystake";
}

impl stake::Trait for Runtime {
    type Currency = <Self as common::currency::GovernanceCurrency>::Currency;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = (
        crate::integration::proposals::StakingEventsHandler<Self>,
        (
            crate::integration::working_group::ContentDirectoryWGStakingEventsHandler<Self>,
            crate::integration::working_group::StorageWgStakingEventsHandler<Self>,
        ),
    );
    type StakeId = u64;
    type SlashId = u64;
}

impl content_wg::Trait for Runtime {
    type Event = Event;
}

impl common::currency::GovernanceCurrency for Runtime {
    type Currency = pallet_balances::Module<Self>;
}

impl governance::election::Trait for Runtime {
    type Event = Event;
    type CouncilElected = (Council, integration::proposals::CouncilElectedHandler);
}

impl governance::council::Trait for Runtime {
    type Event = Event;
    type CouncilTermEnded = (CouncilElection,);
}

impl memo::Trait for Runtime {
    type Event = Event;
}

impl membership::Trait for Runtime {
    type Event = Event;
    type MemberId = MemberId;
    type PaidTermId = u64;
    type SubscriptionId = u64;
    type ActorId = ActorId;
}

impl forum::Trait for Runtime {
    type Event = Event;
    type MembershipRegistry = integration::forum::ShimMembershipRegistry;
    type ThreadId = ThreadId;
    type PostId = PostId;
}

// The storage working group instance alias.
pub type StorageWorkingGroupInstance = working_group::Instance2;

// The content directory working group instance alias.
pub type ContentDirectoryWorkingGroupInstance = working_group::Instance3;

parameter_types! {
    pub const MaxWorkerNumberLimit: u32 = 100;
}

impl working_group::Trait<StorageWorkingGroupInstance> for Runtime {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl working_group::Trait<ContentDirectoryWorkingGroupInstance> for Runtime {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

parameter_types! {
    pub const ProposalCancellationFee: u64 = 10000;
    pub const ProposalRejectionFee: u64 = 5000;
    pub const ProposalTitleMaxLength: u32 = 40;
    pub const ProposalDescriptionMaxLength: u32 = 3000;
    pub const ProposalMaxActiveProposalLimit: u32 = 20;
}

impl proposals_engine::Trait for Runtime {
    type Event = Event;
    type ProposerOriginValidator = MembershipOriginValidator<Self>;
    type VoterOriginValidator = CouncilManager<Self>;
    type TotalVotersCounter = CouncilManager<Self>;
    type ProposalId = u32;
    type StakeHandlerProvider = proposals_engine::DefaultStakeHandlerProvider;
    type CancellationFee = ProposalCancellationFee;
    type RejectionFee = ProposalRejectionFee;
    type TitleMaxLength = ProposalTitleMaxLength;
    type DescriptionMaxLength = ProposalDescriptionMaxLength;
    type MaxActiveProposalLimit = ProposalMaxActiveProposalLimit;
    type DispatchableCallCode = Call;
}
impl Default for Call {
    fn default() -> Self {
        panic!("shouldn't call default for Call");
    }
}

parameter_types! {
    pub const ProposalMaxPostEditionNumber: u32 = 0; // post update is disabled
    pub const ProposalMaxThreadInARowNumber: u32 = 100_000; // will not be used
    pub const ProposalThreadTitleLengthLimit: u32 = 40;
    pub const ProposalPostLengthLimit: u32 = 1000;
}

impl proposals_discussion::Trait for Runtime {
    type Event = Event;
    type PostAuthorOriginValidator = MembershipOriginValidator<Self>;
    type ThreadId = ThreadId;
    type PostId = PostId;
    type MaxPostEditionNumber = ProposalMaxPostEditionNumber;
    type ThreadTitleLengthLimit = ProposalThreadTitleLengthLimit;
    type PostLengthLimit = ProposalPostLengthLimit;
    type MaxThreadInARowNumber = ProposalMaxThreadInARowNumber;
}

parameter_types! {
    pub const TextProposalMaxLength: u32 = 5_000;
    pub const RuntimeUpgradeWasmProposalMaxLength: u32 = 3_000_000;
}

impl proposals_codex::Trait for Runtime {
    type MembershipOriginValidator = MembershipOriginValidator<Self>;
    type TextProposalMaxLength = TextProposalMaxLength;
    type RuntimeUpgradeWasmProposalMaxLength = RuntimeUpgradeWasmProposalMaxLength;
    type ProposalEncoder = ExtrinsicProposalEncoder;
}

parameter_types! {
    pub const TombstoneDeposit: Balance = 1; // TODO: adjust fee
    pub const RentByteFee: Balance = 1; // TODO: adjust fee
    pub const RentDepositOffset: Balance = 0; // no rent deposit
    pub const SurchargeReward: Balance = 0; // no reward
}

parameter_types! {
    pub const MaxStorageBucketNumber: u64 = 20; //TODO: adjust value
    pub const MaxNumberOfDataObjectsPerBag: u64 = 1000; //TODO: adjust value
    pub const DataObjectDeletionPrize: Balance = 10; //TODO: adjust value
    pub const BlacklistSizeLimit: u64 = 10000; //TODO: adjust value
    pub const MaxRandomIterationNumber: u64 = 30; //TODO: adjust value
    pub const StorageModuleId: ModuleId = ModuleId(*b"mstorage"); // module storage
    pub const StorageBucketsPerBagValueConstraint: storage::StorageBucketsPerBagValueConstraint =
        storage::StorageBucketsPerBagValueConstraint {min: 3, max_min_diff: 7}; //TODO: adjust value
    pub const DefaultMemberDynamicBagCreationPolicy: DynamicBagCreationPolicy = DynamicBagCreationPolicy{
        number_of_storage_buckets: 4
    }; //TODO: adjust value
    pub const DefaultChannelDynamicBagCreationPolicy: DynamicBagCreationPolicy = DynamicBagCreationPolicy{
        number_of_storage_buckets: 4
    }; //TODO: adjust value
}

impl storage::Trait for Runtime {
    type Event = Event;
    type DataObjectId = DataObjectId;
    type StorageBucketId = StorageBucketId;
    type DistributionBucketId = DistributionBucketId;
    type ChannelId = ChannelId;
    type MaxStorageBucketNumber = MaxStorageBucketNumber;
    type MaxNumberOfDataObjectsPerBag = MaxNumberOfDataObjectsPerBag;
    type DataObjectDeletionPrize = DataObjectDeletionPrize;
    type BlacklistSizeLimit = BlacklistSizeLimit;
    type ModuleId = StorageModuleId;
    type MemberOriginValidator = MembershipOriginValidator<Self>;
    type StorageBucketsPerBagValueConstraint = StorageBucketsPerBagValueConstraint;
    type DefaultMemberDynamicBagCreationPolicy = DefaultMemberDynamicBagCreationPolicy;
    type DefaultChannelDynamicBagCreationPolicy = DefaultChannelDynamicBagCreationPolicy;
    type Randomness = RandomnessCollectiveFlip;
    type MaxRandomIterationNumber = MaxRandomIterationNumber;

    fn ensure_working_group_leader_origin(origin: Self::Origin) -> DispatchResult {
        StorageWorkingGroup::ensure_origin_is_active_leader(origin)
    }

    fn ensure_worker_origin(origin: Self::Origin, worker_id: ActorId) -> DispatchResult {
        StorageWorkingGroup::ensure_worker_signed(origin, &worker_id).map(|_| ())
    }

    fn ensure_worker_exists(worker_id: &ActorId) -> DispatchResult {
        StorageWorkingGroup::ensure_worker_exists(&worker_id)
            .map(|_| ())
            .map_err(|err| err.into())
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Substrate
        System: frame_system::{Module, Call, Storage, Config, Event<T>},
        Utility: substrate_utility::{Module, Call, Event},
        Babe: pallet_babe::{Module, Call, Storage, Config, Inherent, ValidateUnsigned},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Authorship: pallet_authorship::{Module, Call, Storage, Inherent},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
        Staking: pallet_staking::{Module, Call, Config<T>, Storage, Event<T>, ValidateUnsigned},
        Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
        Historical: pallet_session_historical::{Module},
        FinalityTracker: pallet_finality_tracker::{Module, Call, Inherent},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        ImOnline: pallet_im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
        AuthorityDiscovery: pallet_authority_discovery::{Module, Call, Config},
        Offences: pallet_offences::{Module, Call, Storage, Event},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
        // Joystream
        CouncilElection: election::{Module, Call, Storage, Event<T>, Config<T>},
        Council: council::{Module, Call, Storage, Event<T>, Config<T>},
        Memo: memo::{Module, Call, Storage, Event<T>},
        Members: membership::{Module, Call, Storage, Event<T>, Config<T>},
        Forum: forum::{Module, Call, Storage, Event<T>, Config<T>},
        VersionedStore: versioned_store::{Module, Call, Storage, Event<T>, Config},
        VersionedStorePermissions: versioned_store_permissions::{Module, Call, Storage, Config<T>},
        Stake: stake::{Module, Call, Storage},
        Minting: minting::{Module, Call, Storage},
        RecurringRewards: recurring_rewards::{Module, Call, Storage},
        Hiring: hiring::{Module, Call, Storage},
        ContentWorkingGroup: content_wg::{Module, Call, Storage, Event<T>, Config<T>},
        ContentDirectory: content_directory::{Module, Call, Storage, Event<T>, Config<T>},
        // --- Proposals
        ProposalsEngine: proposals_engine::{Module, Call, Storage, Event<T>},
        ProposalsDiscussion: proposals_discussion::{Module, Call, Storage, Event<T>},
        ProposalsCodex: proposals_codex::{Module, Call, Storage, Config<T>},
        // --- Working groups
        // reserved for the future use: ForumWorkingGroup: working_group::<Instance1>::{Module, Call, Storage, Event<T>},
        StorageWorkingGroup: working_group::<Instance2>::{Module, Call, Storage, Config<T>, Event<T>},
        ContentDirectoryWorkingGroup: working_group::<Instance3>::{Module, Call, Storage, Config<T>, Event<T>},
        //
        Storage: storage::{Module, Call, Storage, Event<T>},
    }
);
