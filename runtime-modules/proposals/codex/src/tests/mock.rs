#![cfg(test)]
// Internal substrate warning.
#![allow(non_fmt_panic)]

use frame_support::{parameter_types, weights::Weight};
pub use frame_system;
use sp_core::H256;
use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use sp_staking::SessionIndex;

use crate as proposals_codex;
use governance::council;
use governance::election;

use crate::{ProposalDetailsOf, ProposalEncoder};
use frame_election_provider_support::onchain;
use proposals_engine::VotersParameters;
use sp_runtime::testing::TestXt;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Staking: staking::{Pallet, Call, Config<T>, Storage, Event<T>},
        ProposalsEngine: proposals_engine::{Pallet, Call, Storage, Event<T>},
        ProposalsCodex: proposals_codex::{Pallet, Call, Storage, Config<T>},
        ProposalsDiscussion: proposals_discussion::{Pallet, Call, Storage, Event<T>},
        Election: election::{Pallet, Call, Storage, Event<T>, Config<T>},
        Council: council::{Pallet, Call, Storage, Event<T>, Config<T>},
        Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
        StorageWorkingGroup: working_group::<Instance2>::{Pallet, Call, Storage, Config<T>, Event<T>},
        ContentDirectoryWorkingGroup: working_group::<Instance3>::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);

impl common::currency::GovernanceCurrency for Test {
    type Currency = balances::Pallet<Self>;
}

parameter_types! {
    pub const ScreenedMemberMaxInitialBalance: u64 = 500;
}

impl membership::Config for Test {
    type Event = Event;
    type MemberId = u64;
    type PaidTermId = u64;
    type SubscriptionId = u64;
    type ActorId = u64;
    type ScreenedMemberMaxInitialBalance = ScreenedMemberMaxInitialBalance;
}

parameter_types! {
    pub const ExistentialDeposit: u32 = 0;
}

type Balance = u64;

impl balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Test>;
    type WeightInfo = ();
    type MaxLocks = ();
}

parameter_types! {
    pub const StakePoolId: [u8; 8] = *b"joystake";
}

impl stake::Config for Test {
    type Currency = Balances;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = ();
    type StakeId = u64;
    type SlashId = u64;
}

parameter_types! {
    pub const CancellationFee: u64 = 5;
    pub const RejectionFee: u64 = 3;
    pub const TitleMaxLength: u32 = 100;
    pub const DescriptionMaxLength: u32 = 10000;
    pub const MaxActiveProposalLimit: u32 = 100;
}

impl proposals_engine::Config for Test {
    type Event = Event;
    type ProposerOriginValidator = ();
    type VoterOriginValidator = ();
    type TotalVotersCounter = MockVotersParameters;
    type ProposalId = u32;
    type StakeHandlerProvider = proposals_engine::DefaultStakeHandlerProvider;
    type CancellationFee = CancellationFee;
    type RejectionFee = RejectionFee;
    type TitleMaxLength = TitleMaxLength;
    type DescriptionMaxLength = DescriptionMaxLength;
    type MaxActiveProposalLimit = MaxActiveProposalLimit;
    type DispatchableCallCode = crate::Call<Test>;
}

impl Default for crate::Call<Test> {
    fn default() -> Self {
        panic!("shouldn't call default for Call");
    }
}

impl minting::Config for Test {
    type Currency = Balances;
    type MintId = u64;
}

impl governance::council::Config for Test {
    type Event = Event;
    type CouncilTermEnded = ();
}

impl common::origin::ActorOriginValidator<Origin, u64, u64> for () {
    fn ensure_actor_origin(origin: Origin, _: u64) -> Result<u64, &'static str> {
        let account_id = frame_system::ensure_signed(origin)?;

        Ok(account_id)
    }
}

parameter_types! {
    pub const MaxPostEditionNumber: u32 = 5;
    pub const MaxThreadInARowNumber: u32 = 3;
    pub const ThreadTitleLengthLimit: u32 = 200;
    pub const PostLengthLimit: u32 = 2000;
}

impl proposals_discussion::Config for Test {
    type Event = Event;
    type PostAuthorOriginValidator = ();
    type ThreadId = u64;
    type PostId = u64;
    type MaxPostEditionNumber = MaxPostEditionNumber;
    type ThreadTitleLengthLimit = ThreadTitleLengthLimit;
    type PostLengthLimit = PostLengthLimit;
    type MaxThreadInARowNumber = MaxThreadInARowNumber;
}

pub struct MockVotersParameters;
impl VotersParameters for MockVotersParameters {
    fn total_voters_count() -> u32 {
        4
    }
}

parameter_types! {
    pub const TextProposalMaxLength: u32 = 20_000;
    pub const RuntimeUpgradeWasmProposalMaxLength: u32 = 20_000;
}

impl governance::election::Config for Test {
    type Event = Event;
    type CouncilElected = ();
}

// The content directory working group instance alias.
pub type ContentDirectoryWorkingGroupInstance = working_group::Instance3;

// The storage working group instance alias.
pub type StorageWorkingGroupInstance = working_group::Instance2;

parameter_types! {
    pub const MaxWorkerNumberLimit: u32 = 100;
}

impl working_group::Config<ContentDirectoryWorkingGroupInstance> for Test {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl working_group::Config<StorageWorkingGroupInstance> for Test {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl recurring_rewards::Config for Test {
    type PayoutStatusHandler = ();
    type RecipientId = u64;
    type RewardRelationshipId = u64;
}

impl hiring::Config for Test {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = ();
    type StakeHandlerProvider = hiring::Module<Self>;
}

pallet_staking_reward_curve::build! {
    const I_NPOS: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}

parameter_types! {
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const SessionsPerEra: SessionIndex = 3;
    pub const BondingDuration: staking::EraIndex = 3;
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &I_NPOS;
}

impl onchain::Config for Test {
    type AccountId = u64;
    type BlockNumber = u64;
    type BlockWeights = BlockWeights;
    type Accuracy = Perbill;
    type DataProvider = Staking;
}

impl staking::Config for Test {
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
    type EraPayout = staking::ConvertCurve<RewardCurve>;
    type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;
    type SlashDeferDuration = ();
    type SlashCancelOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type NextNewSession = ();
    type MaxNominatorRewardedPerValidator = ();
    type WeightInfo = ();
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

pub type Extrinsic = TestXt<Call, ()>;

impl staking::SessionInterface<u64> for Test {
    fn disable_validator(_: &u64) -> Result<bool, ()> {
        unimplemented!()
    }

    fn validators() -> Vec<u64> {
        unimplemented!()
    }

    fn prune_historical_up_to(_: u32) {
        unimplemented!()
    }
}

impl crate::Config for Test {
    type TextProposalMaxLength = TextProposalMaxLength;
    type RuntimeUpgradeWasmProposalMaxLength = RuntimeUpgradeWasmProposalMaxLength;
    type MembershipOriginValidator = ();
    type ProposalEncoder = ();
}

impl ProposalEncoder<Test> for () {
    fn encode_proposal(_proposal_details: ProposalDetailsOf<Test>) -> Vec<u8> {
        Vec::new()
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

pub fn initial_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    t.into()
}
