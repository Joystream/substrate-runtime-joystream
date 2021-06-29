//! Mock runtime for the module testing.
//!
//! Submodules:
//! - stakes: contains support for mocking external 'stake' module
//! - balance_restorator: restores balances after unstaking
//! - proposals: provides types for proposal execution tests
//!

#![cfg(test)]

use frame_support::parameter_types;
pub use frame_system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

mod balance_manager;
pub(crate) mod proposals;
mod stakes;

use crate as proposals_engine;
use balance_manager::*;
pub use proposals::*;
pub use stakes::*;

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
        Balances: balances::{Pallet, Call, Storage, Event<T>},
        ProposalsEngine: proposals_engine::{Pallet, Call, Storage, Event<T>},
        Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
    }
);

parameter_types! {
    pub const ExistentialDeposit: u32 = 0;
}

type Balance = u64;

impl balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl common::currency::GovernanceCurrency for Test {
    type Currency = balances::Pallet<Self>;
}

impl proposals::Config for Test {}

impl stake::Config for Test {
    type Currency = Balances;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = BalanceManagerStakingEventsHandler;
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

impl crate::Config for Test {
    type Event = Event;
    type ProposerOriginValidator = ();
    type VoterOriginValidator = ();
    type TotalVotersCounter = ();
    type ProposalId = u32;
    type StakeHandlerProvider = stakes::TestStakeHandlerProvider;
    type CancellationFee = CancellationFee;
    type RejectionFee = RejectionFee;
    type TitleMaxLength = TitleMaxLength;
    type DescriptionMaxLength = DescriptionMaxLength;
    type MaxActiveProposalLimit = MaxActiveProposalLimit;
    type DispatchableCallCode = proposals::Call<Test>;
}

impl Default for proposals::Call<Test> {
    fn default() -> Self {
        panic!("shouldn't call default for Call");
    }
}

impl common::origin::ActorOriginValidator<Origin, u64, u64> for () {
    fn ensure_actor_origin(origin: Origin, _account_id: u64) -> Result<u64, &'static str> {
        let signed_account_id = frame_system::ensure_signed(origin)?;

        Ok(signed_account_id)
    }
}

// If changing count is required, we can upgrade the implementation as shown here:
// https://substrate.dev/recipes/3-entrees/testing/externalities.html
impl crate::VotersParameters for () {
    fn total_voters_count() -> u32 {
        4
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const MinimumPeriod: u64 = 5;
    pub const StakePoolId: [u8; 8] = *b"joystake";
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
