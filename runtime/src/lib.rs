//! The Joystream Substrate Node runtime.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// The `large_enum_variant` warning originates from `construct_runtime` macro.
#![allow(clippy::large_enum_variant)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::from_over_into)]
#![allow(clippy::upper_case_acronyms)]

// Make the WASM binary available.
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
mod tests; // Runtime integration tests
mod weights;

pub use frame_support::{
    construct_runtime, log, match_type, parameter_types,
    traits::{
        All, Contains, EnsureOrigin, Filter, Get, IsInVec, IsType, KeyOwnerProofSystem,
        LockIdentifier, Randomness, U128CurrencyToVote, WithdrawReasons,
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchClass, IdentityFee, Weight,
    },
    StorageValue,
};
use frame_system::limits::{BlockLength, BlockWeights};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256},
};
use sp_std::prelude::*;

use crate::runtime_api::CustomOnRuntimeUpgrade;
pub use constants::*;
pub use primitives::*;
pub use runtime_api::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use integration::proposals::{CouncilManager, ExtrinsicProposalEncoder, MembershipOriginValidator};

use governance::{council, election};
use storage::data_object_storage_registry;

// Node dependencies
pub use common;
pub use forum;
pub use governance::election_params::ElectionParameters;
pub use membership;
#[cfg(any(feature = "std", test))]
pub use pallet_balances::Call as BalancesCall;
pub use proposals_codex::ProposalsConfigParameters;
pub use storage::{data_directory, data_object_type_registry};
pub use working_group;

pub use content;
pub use content::MaxNumber;

pub use pallet_grandpa::AuthorityId as GrandpaId;
pub use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;

#[cfg(feature = "standalone")]
use standalone_use::*;
#[cfg(feature = "standalone")]
mod standalone_use {
    pub use frame_election_provider_support::onchain;
    pub use frame_system::EnsureRoot;
    pub use pallet_session::historical as pallet_session_historical;
    pub use pallet_staking::StakerStatus;
    pub use sp_core::crypto::KeyTypeId;
    pub use sp_runtime::{
        curve::PiecewiseLinear, traits::OpaqueKeys, transaction_validity::TransactionPriority,
    };
}

#[cfg(not(feature = "standalone"))]
use parachain_use::*;
#[cfg(not(feature = "standalone"))]
mod parachain_use {
    pub use cumulus_primitives_core::ParaId;
    pub use polkadot_parachain::primitives::Sibling;
    pub use sp_runtime::traits::{Convert, Identity};
    pub use sp_std::collections::btree_set::BTreeSet;
    pub use xcm::v0::{
        BodyId,
        Junction::{GeneralKey, Parachain, Parent, Plurality},
        MultiAsset,
        MultiLocation::{self, X1, X2, X3},
        NetworkId, Xcm,
    };
    pub use xcm_builder::{
        AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
        CurrencyAdapter, EnsureXcmOrigin, FixedRateOfConcreteFungible, FixedWeightBounds,
        IsConcrete, LocationInverter, NativeAsset, ParentAsSuperuser, ParentIsDefault,
        RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
        SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
        TakeWeightCredit, UsingComponents,
    };
    pub use xcm_executor::{Config, XcmExecutor};

    pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
}

pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Percent, Permill, Perquintill};

pub use primitives::{AccountId, AccountIndex, Balance, BlockNumber, Hash, Moment, Signature};

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("joystream-node"),
    impl_name: create_runtime_str!("joystream-node"),
    authoring_version: 9,
    spec_version: 7,
    impl_version: 0,
    apis: crate::runtime_api::EXPORTED_RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// The version infromation used to identify this runtime when compiled
/// natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

#[cfg(not(feature = "standalone"))]
impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

/// We assume that ~10% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND * 2;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 900; // mortal tx can be valid up to 1 hour after signing
    pub const Version: RuntimeVersion = VERSION;
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u8 = 42; // Ss58AddressFormat::SubstrateAccount
}

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a Pallet to the index of the Pallet in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    #[cfg(feature = "standalone")]
    type OnSetCode = ();
    #[cfg(not(feature = "standalone"))]
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = weights::pallet_utility::WeightInfo;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = Moment;
    #[cfg(not(feature = "standalone"))]
    type OnTimestampSet = ();
    #[cfg(feature = "standalone")]
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const NativeTokenExistentialDeposit: Balance = 0;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = NativeTokenExistentialDeposit;
    type AccountStore = frame_system::Pallet<Runtime>;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

#[cfg(feature = "standalone")]
impl pallet_authority_discovery::Config for Runtime {}

parameter_types! {
    pub const MaxNumberOfCuratorsPerGroup: MaxNumber = 50;
    pub const ChannelOwnershipPaymentEscrowId: [u8; 8] = *b"chescrow";
}

impl content::Config for Runtime {
    type Event = Event;
    type ChannelOwnershipPaymentEscrowId = ChannelOwnershipPaymentEscrowId;
    type ChannelCategoryId = ChannelCategoryId;
    type VideoId = VideoId;
    type VideoCategoryId = VideoCategoryId;
    type PlaylistId = PlaylistId;
    type PersonId = PersonId;
    type SeriesId = SeriesId;
    type ChannelOwnershipTransferRequestId = ChannelOwnershipTransferRequestId;
    type MaxNumberOfCuratorsPerGroup = MaxNumberOfCuratorsPerGroup;
    type StorageSystem = data_directory::Pallet<Self>;
}

impl hiring::Config for Runtime {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = (); // TODO - what needs to happen?
    type StakeHandlerProvider = hiring::Pallet<Self>;
}

impl minting::Config for Runtime {
    type Currency = <Self as common::currency::GovernanceCurrency>::Currency;
    type MintId = u64;
}

impl recurring_rewards::Config for Runtime {
    type PayoutStatusHandler = (); // TODO - deal with successful and failed payouts
    type RecipientId = u64;
    type RewardRelationshipId = u64;
}

parameter_types! {
    pub const StakePoolId: [u8; 8] = *b"joystake";
}

impl stake::Config for Runtime {
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

impl common::currency::GovernanceCurrency for Runtime {
    type Currency = pallet_balances::Pallet<Self>;
}

impl common::MembershipTypes for Runtime {
    type MemberId = MemberId;
    type ActorId = ActorId;
}

impl common::StorageOwnership for Runtime {
    type ChannelId = ChannelId;
    type DAOId = DAOId;
    type ContentId = ContentId;
    type DataObjectTypeId = DataObjectTypeId;
}

impl governance::election::Config for Runtime {
    type Event = Event;
    type CouncilElected = (Council, integration::proposals::CouncilElectedHandler);
}

impl governance::council::Config for Runtime {
    type Event = Event;
    type CouncilTermEnded = (CouncilElection,);
}

impl memo::Config for Runtime {
    type Event = Event;
}

impl storage::data_object_type_registry::Config for Runtime {
    type Event = Event;
}

impl storage::data_directory::Config for Runtime {
    type Event = Event;
    type IsActiveDataObjectType = DataObjectTypeRegistry;
    type MemberOriginValidator = MembershipOriginValidator<Self>;
}

impl storage::data_object_storage_registry::Config for Runtime {
    type Event = Event;
    type DataObjectStorageRelationshipId = u64;
    type ContentIdExists = DataDirectory;
}

parameter_types! {
    pub const ScreenedMemberMaxInitialBalance: u128 = 5000;
}

impl membership::Config for Runtime {
    type Event = Event;
    type MemberId = MemberId;
    type PaidTermId = u64;
    type SubscriptionId = u64;
    type ActorId = ActorId;
    type ScreenedMemberMaxInitialBalance = ScreenedMemberMaxInitialBalance;
}

impl forum::Config for Runtime {
    type Event = Event;
    type MembershipRegistry = integration::forum::ShimMembershipRegistry;
    type ThreadId = ThreadId;
    type PostId = PostId;
}

// The storage working group instance alias.
pub type StorageWorkingGroupInstance = working_group::Instance2;

// The content directory working group instance alias.
pub type ContentDirectoryWorkingGroupInstance = working_group::Instance3;

// The builder working group instance alias.
pub type OperationsWorkingGroupInstance = working_group::Instance4;

// The gateway working group instance alias.
pub type GatewayWorkingGroupInstance = working_group::Instance5;

parameter_types! {
    pub const MaxWorkerNumberLimit: u32 = 100;
}

impl working_group::Config<StorageWorkingGroupInstance> for Runtime {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl working_group::Config<ContentDirectoryWorkingGroupInstance> for Runtime {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl working_group::Config<OperationsWorkingGroupInstance> for Runtime {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl working_group::Config<GatewayWorkingGroupInstance> for Runtime {
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

impl proposals_engine::Config for Runtime {
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

impl proposals_discussion::Config for Runtime {
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

impl proposals_codex::Config for Runtime {
    type MembershipOriginValidator = MembershipOriginValidator<Self>;
    type TextProposalMaxLength = TextProposalMaxLength;
    type RuntimeUpgradeWasmProposalMaxLength = RuntimeUpgradeWasmProposalMaxLength;
    type ProposalEncoder = ExtrinsicProposalEncoder;
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
    pub const TransactionByteFee: u128 = 1_000_000; // TODO: adjust fee
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

#[cfg(feature = "standalone")]
pub use standalone_impl::*;

#[cfg(feature = "standalone")]
mod standalone_impl {
    use super::*;

    parameter_types! {
        pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
        pub const ReportLongevity: u64 =
            BondingDuration::get() as u64 * SessionsPerEra::get() as u64 *
    EpochDuration::get(); }

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub grandpa: Grandpa,
            pub babe: Babe,
            pub im_online: ImOnline,
            pub authority_discovery: AuthorityDiscovery,
        }
    }

    parameter_types! {
        pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    }

    /// The BABE epoch configuration at genesis.
    pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
        sp_consensus_babe::BabeEpochConfiguration {
            c: PRIMARY_PROBABILITY,
            allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
        };

    impl pallet_babe::Config for Runtime {
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

        type HandleEquivocation = pallet_babe::EquivocationHandler<
            Self::KeyOwnerIdentification,
            Offences,
            ReportLongevity,
        >;

        type WeightInfo = ();
    }

    impl pallet_grandpa::Config for Runtime {
        type Event = Event;
        type Call = Call;

        type KeyOwnerProofSystem = Historical;

        type KeyOwnerProof =
            <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

        type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
            KeyTypeId,
            GrandpaId,
        )>>::IdentificationTuple;

        type HandleEquivocation =
            pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, (), ReportLongevity>;

        type WeightInfo = ();
    }

    parameter_types! {
        pub const UncleGenerations: BlockNumber = 5;
    }

    impl pallet_authorship::Config for Runtime {
        type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
        type UncleGenerations = UncleGenerations;
        type FilterUncle = ();
        type EventHandler = (Staking, ()); // ImOnline
    }

    parameter_types! {
        pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
    }

    impl pallet_session::Config for Runtime {
        type Event = Event;
        type ValidatorId = <Self as frame_system::Config>::AccountId;
        type ValidatorIdOf = pallet_staking::StashOf<Self>;
        type ShouldEndSession = Babe;
        type NextSessionRotation = Babe;
        type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
        type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
        type Keys = SessionKeys;
        type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
        type WeightInfo = ();
    }

    impl pallet_session::historical::Config for Runtime {
        type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
        type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
    }

    impl pallet_offences::Config for Runtime {
        type Event = Event;
        type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
        type OnOffenceHandler = Staking;
    }

    parameter_types! {
        pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    }

    impl pallet_im_online::Config for Runtime {
        type AuthorityId = ImOnlineId;
        type Event = Event;
        type ValidatorSet = Historical;
        type ReportUnresponsiveness = Offences;
        type NextSessionRotation = ();
        // Using the default weights until we check if we can run the benchmarks for this pallet in
        // the reference machine in an acceptable time.
        type WeightInfo = ();
        type UnsignedPriority = ImOnlineUnsignedPriority;
    }

    parameter_types! {
        pub const SessionsPerEra: sp_staking::SessionIndex = 6;
        pub const BondingDuration: pallet_staking::EraIndex = BONDING_DURATION;
        pub const SlashDeferDuration: pallet_staking::EraIndex = BONDING_DURATION - 1; // 'slightly less' than the bonding duration.
        pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
        pub const MaxNominatorRewardedPerValidator: u32 = 64;
    }

    impl onchain::Config for Runtime {
        type AccountId = AccountId;
        type BlockNumber = BlockNumber;
        type BlockWeights = RuntimeBlockWeights;
        type Accuracy = Perbill;
        type DataProvider = Staking;
    }

    impl pallet_staking::Config for Runtime {
        const MAX_NOMINATIONS: u32 = 16;
        type Currency = Balances;
        type UnixTime = Timestamp;
        type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
        type RewardRemainder = ();
        type Event = Event;
        type Slash = ();
        type Reward = ();
        type SessionsPerEra = SessionsPerEra;
        type BondingDuration = BondingDuration;
        type SessionInterface = Self;
        type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
        type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;
        type SlashDeferDuration = ();
        type SlashCancelOrigin = EnsureRoot<Self::AccountId>;
        type NextNewSession = ();
        type MaxNominatorRewardedPerValidator = ();
        type WeightInfo = ();
    }

    pallet_staking_reward_curve::build! {
        const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
            min_inflation: 0_025_000,
            max_inflation: 0_100_000,
            ideal_stake: 0_500_000,
            falloff: 0_050_000,
            max_piece_count: 40,
            test_precision: 0_005_000,
        );
    }

    sp_npos_elections::generate_solution_type!(
        #[compact]
        pub struct NposCompactSolution16::<
            VoterIndex = u32,
            TargetIndex = u16,
            Accuracy = sp_runtime::PerU16,
        >(16)
    );

    pub const MAX_NOMINATIONS: u32 =
        <NposCompactSolution16 as sp_npos_elections::CompactSolution>::LIMIT as u32;
}

#[cfg(not(feature = "standalone"))]
pub use parachain_impl::*;
#[cfg(not(feature = "standalone"))]
mod parachain_impl {
    use super::*;

    parameter_types! {
        pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
        pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
    }

    impl cumulus_pallet_parachain_system::Config for Runtime {
        type Event = Event;
        type OnValidationData = ();
        type SelfParaId = parachain_info::Pallet<Runtime>;
        type OutboundXcmpMessageSource = XcmpQueue;
        type DmpMessageHandler = DmpQueue;
        type ReservedDmpWeight = ReservedDmpWeight;
        type XcmpMessageHandler = XcmpQueue;
        type ReservedXcmpWeight = ReservedXcmpWeight;
    }

    impl parachain_info::Config for Runtime {}

    impl pallet_aura::Config for Runtime {
        type AuthorityId = AuraId;
    }

    impl cumulus_pallet_aura_ext::Config for Runtime {}

    parameter_types! {
        pub const RelayLocation: MultiLocation = X1(Parent);
        pub const RelayNetwork: NetworkId = NetworkId::Polkadot;
        pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
        pub Ancestry: MultiLocation = X1(Parachain(ParachainInfo::parachain_id().into()));
    }

    /// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
    /// when determining ownership of accounts for asset transacting and when attempting to use XCM
    /// `Transact` in order to determine the dispatch Origin.
    pub type LocationToAccountId = (
        // The parent (Relay-chain) origin converts to the default `AccountId`.
        ParentIsDefault<AccountId>,
        // Sibling parachain origins convert to AccountId via the `ParaId::into`.
        SiblingParachainConvertsVia<Sibling, AccountId>,
        // Straight up local `AccountId32` origins just alias directly to `AccountId`.
        AccountId32Aliases<RelayNetwork, AccountId>,
    );

    /// Means for transacting assets on this chain.
    pub type LocalAssetTransactor = CurrencyAdapter<
        // Use this currency:
        Balances,
        // Use this currency when it is a fungible asset matching the given location or name:
        IsConcrete<RelayLocation>,
        // Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
        LocationToAccountId,
        // Our chain's account ID type (we can't get away without mentioning it explicitly):
        AccountId,
        // We don't track any teleports.
        (),
    >;

    /// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
    /// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
    /// biases the kind of local `Origin` it will become.
    pub type XcmOriginToTransactDispatchOrigin = (
        // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
        // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
        // foreign chains who want to have a local sovereign account on this chain which they control.
        SovereignSignedViaLocation<LocationToAccountId, Origin>,
        // Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
        // recognised.
        RelayChainAsNative<RelayOrigin, Origin>,
        // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
        // recognised.
        SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
        // Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
        // transaction from the Root origin.
        ParentAsSuperuser<Origin>,
        // Native signed account converter; this just converts an `AccountId32` origin into a normal
        // `Origin::Signed` origin of the same 32-byte value.
        SignedAccountId32AsNative<RelayNetwork, Origin>,
    );

    parameter_types! {
        pub UnitWeightCost: Weight = 1_000;
    }

    match_type! {
        pub type ParentOrParentsUnitPlurality: impl Contains<MultiLocation> = {
            X1(Parent) | X2(Parent, Plurality { id: BodyId::Unit, .. })
        };
    }

    pub type Barrier = (
        TakeWeightCredit,
        AllowTopLevelPaidExecutionFrom<All<MultiLocation>>,
        AllowUnpaidExecutionFrom<ParentOrParentsUnitPlurality>,
        // ^^^ Parent & its unit plurality gets free execution
    );

    pub struct XcmConfig;
    impl Config for XcmConfig {
        type Call = Call;
        type XcmSender = XcmRouter;
        // How to withdraw and deposit an asset.
        type AssetTransactor = LocalAssetTransactor;
        type OriginConverter = XcmOriginToTransactDispatchOrigin;
        type IsReserve = NativeAsset;
        type IsTeleporter = NativeAsset; // <- should be enough to allow teleportation of ROC
        type LocationInverter = LocationInverter<Ancestry>;
        type Barrier = Barrier;
        type Weigher = FixedWeightBounds<UnitWeightCost, Call>;
        type Trader = UsingComponents<IdentityFee<Balance>, RelayLocation, AccountId, Balances, ()>;
        type ResponseHandler = (); // Don't handle responses for now.
    }

    /// No local origins on this chain are allowed to dispatch XCM sends/executions.
    pub type LocalOriginToLocation = (SignedToAccountId32<Origin, AccountId, RelayNetwork>,);

    /// The means for routing XCM messages which are not for local execution into the right message
    /// queues.
    pub type XcmRouter = (
        // Two routers - use UMP to communicate with the relay chain:
        cumulus_primitives_utility::ParentAsUmp<ParachainSystem>,
        // ..and XCMP to communicate with the sibling chains.
        XcmpQueue,
    );

    impl pallet_xcm::Config for Runtime {
        type Event = Event;
        type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
        type XcmRouter = XcmRouter;
        type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
        type XcmExecuteFilter = All<(MultiLocation, Xcm<Call>)>;
        type XcmExecutor = XcmExecutor<XcmConfig>;
        type XcmTeleportFilter = All<(MultiLocation, Vec<MultiAsset>)>;
        type XcmReserveTransferFilter = ();
        type Weigher = FixedWeightBounds<UnitWeightCost, Call>;
    }

    impl cumulus_pallet_xcm::Config for Runtime {
        type Event = Event;
        type XcmExecutor = XcmExecutor<XcmConfig>;
    }

    impl cumulus_pallet_xcmp_queue::Config for Runtime {
        type Event = Event;
        type XcmExecutor = XcmExecutor<XcmConfig>;
        type ChannelInfo = ParachainSystem;
    }

    impl cumulus_pallet_dmp_queue::Config for Runtime {
        type Event = Event;
        type XcmExecutor = XcmExecutor<XcmConfig>;
        type ExecuteOverweightOrigin = frame_system::EnsureRoot<AccountId>;
    }
}

macro_rules! construct_joystream_runtime {
	($( $Pallets:tt )*) => {
		#[allow(clippy::large_enum_variant)]
		construct_runtime! {
			pub enum Runtime where
				Block = Block,
				NodeBlock = generic::Block<Header, sp_runtime::OpaqueExtrinsic>,
				UncheckedExtrinsic = UncheckedExtrinsic,
			{
				// Core
				System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,
				Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 1,
				RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Call, Storage} = 2,
                Utility: pallet_utility::{Pallet, Call, Event} = 3,

				// Tokens & Related
				Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 4,
				TransactionPayment: pallet_transaction_payment::{Pallet, Storage} = 5,

                // Joystream
                CouncilElection: election::{Pallet, Call, Storage, Event<T>, Config<T>} = 6,
                Council: council::{Pallet, Call, Storage, Event<T>, Config<T>} = 7,
                Memo: memo::{Pallet, Call, Storage, Event<T>} = 8,
                Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>} = 9,
                Forum: forum::{Pallet, Call, Storage, Event<T>, Config<T>} = 10,
                Stake: stake::{Pallet, Call, Storage} = 11,
                Minting: minting::{Pallet, Call, Storage} = 12,
                RecurringRewards: recurring_rewards::{Pallet, Call, Storage} = 13,
                Hiring: hiring::{Pallet, Call, Storage} = 14,
                Content: content::{Pallet, Call, Storage, Event<T>, Config<T>} = 15,
                // --- Storage
                DataObjectTypeRegistry: data_object_type_registry::{Pallet, Call, Storage, Event<T>, Config<T>} = 16,
                DataDirectory: data_directory::{Pallet, Call, Storage, Event<T>, Config<T>} = 17,
                DataObjectStorageRegistry: data_object_storage_registry::{Pallet, Call, Storage, Event<T>, Config<T>} = 18,
                // --- Proposals
                ProposalsEngine: proposals_engine::{Pallet, Call, Storage, Event<T>} = 19,
                ProposalsDiscussion: proposals_discussion::{Pallet, Call, Storage, Event<T>} = 20,
                ProposalsCodex: proposals_codex::{Pallet, Call, Storage, Config<T>} = 21,
                // --- Working groups
                // reserved for the future use: ForumWorkingGroup: working_group::<Instance1>::{Pallet, Call, Storage, Event<T>},
                StorageWorkingGroup: working_group::<Instance2>::{Pallet, Call, Storage, Config<T>, Event<T>} = 22,
                ContentDirectoryWorkingGroup: working_group::<Instance3>::{Pallet, Call, Storage, Config<T>, Event<T>} = 23,
                OperationsWorkingGroup: working_group::<Instance4>::{Pallet, Call, Storage, Config<T>, Event<T>} = 24,
                GatewayWorkingGroup: working_group::<Instance5>::{Pallet, Call, Storage, Config<T>, Event<T>} = 25,

                // Dev
				Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 55,

				$($Pallets)*
			}
		}
	}
}

#[cfg(feature = "standalone")]
construct_joystream_runtime! {
    // Consensus & Staking
    Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent} = 57,
    Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event, ValidateUnsigned} = 58,
    Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>} = 60,
    Offences: pallet_offences::{Pallet, Call, Storage, Event} = 61,
    Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 62,
    Historical: pallet_session_historical::{Pallet} = 63,
    AuthorityDiscovery: pallet_authority_discovery::{Pallet, Call, Config} = 64,
    ImOnline: pallet_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>} = 65,

    // Block authoring logic
    Babe: pallet_babe::{Pallet, Call, Storage, Config, ValidateUnsigned} = 66,
}

#[cfg(not(feature = "standalone"))]
construct_joystream_runtime! {
    // Parachain
    ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Storage, Inherent, Event<T>} = 57,
    ParachainInfo: parachain_info::{Pallet, Storage, Config} = 58,
    XcmHandler: cumulus_pallet_xcm::{Pallet, Call, Event<T>, Origin} = 59,
    PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 60,
    XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 61,
    DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 62,

    // Block authoring logic
    Aura: pallet_aura::{Pallet, Config<T>} = 63,
    AuraExt: cumulus_pallet_aura_ext::{Pallet, Config} = 64,
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
    CustomOnRuntimeUpgrade,
>;

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(&block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
}
