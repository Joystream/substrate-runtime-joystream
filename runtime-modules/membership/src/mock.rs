#![cfg(test)]

pub use crate::{Config, DEFAULT_PAID_TERM_ID};

pub use frame_support::traits::Currency;
use frame_support::{parameter_types, weights::Weight};
pub use frame_system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub use common::currency::GovernanceCurrency;

use crate as membership;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
        Balances: balances::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
    }
);

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

impl GovernanceCurrency for Test {
    type Currency = balances::Pallet<Self>;
}

parameter_types! {
    pub const ScreenedMemberMaxInitialBalance: u64 = 500;
}

impl Config for Test {
    type Event = Event;
    type MemberId = u64;
    type PaidTermId = u32;
    type SubscriptionId = u32;
    type ActorId = u32;
    type ScreenedMemberMaxInitialBalance = ScreenedMemberMaxInitialBalance;
}

pub struct TestExternalitiesBuilder<T: Config> {
    system_config: Option<frame_system::GenesisConfig>,
    membership_config: Option<membership::GenesisConfig<T>>,
}

impl<T: Config> Default for TestExternalitiesBuilder<T> {
    fn default() -> Self {
        Self {
            system_config: None,
            membership_config: None,
        }
    }
}

impl<T: Config> TestExternalitiesBuilder<T> {
    pub fn set_membership_config(
        mut self,
        membership_config: membership::GenesisConfig<T>,
    ) -> Self {
        self.membership_config = Some(membership_config);
        self
    }
    pub fn build(self) -> sp_io::TestExternalities {
        // Add frame_system
        let mut t = self
            .system_config
            .unwrap_or(frame_system::GenesisConfig::default())
            .build_storage::<T>()
            .unwrap();

        // Add membership
        self.membership_config
            .unwrap_or(membership::GenesisConfig::default())
            .assimilate_storage(&mut t)
            .unwrap();

        t.into()
    }
}
