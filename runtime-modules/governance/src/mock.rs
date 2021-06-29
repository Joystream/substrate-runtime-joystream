#![cfg(test)]

pub use super::{council, election};
pub use common::currency::GovernanceCurrency;

use frame_support::{parameter_types, weights::Weight};
pub use frame_system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Election: election::{Pallet, Call, Storage, Event<T>, Config<T>},
        Council: council::{Pallet, Call, Storage, Event<T>, Config<T>},
        Balances: balances::{Pallet, Call, Storage, Event<T>},
        Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
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

impl council::Config for Test {
    type Event = Event;

    type CouncilTermEnded = (Election,);
}

impl election::Config for Test {
    type Event = Event;

    type CouncilElected = (Council,);
}

parameter_types! {
    pub const ScreenedMemberMaxInitialBalance: u64 = 500;
}

impl membership::Config for Test {
    type Event = Event;
    type MemberId = u64;
    type SubscriptionId = u32;
    type PaidTermId = u32;
    type ActorId = u32;
    type ScreenedMemberMaxInitialBalance = ScreenedMemberMaxInitialBalance;
}

impl minting::Config for Test {
    type Currency = Balances;
    type MintId = u64;
}

impl recurringrewards::Config for Test {
    type PayoutStatusHandler = ();
    type RecipientId = u64;
    type RewardRelationshipId = u64;
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

// TODO add a Hook type to capture TriggerElection and CouncilElected hooks

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn initial_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let members_config_builder = membership::genesis::GenesisConfigBuilder::<Test>::default()
        .default_paid_membership_fee(0)
        .members(vec![
            // member_id, account_id
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (9, 10),
            (10, 11),
            (11, 12),
            (12, 13),
            (13, 14),
            (14, 15),
            (15, 16),
            (16, 17),
            (17, 18),
            (18, 19),
            (19, 20),
        ]);

    members_config_builder
        .build()
        .assimilate_storage(&mut t)
        .unwrap();

    // build the council config to initialize the mint
    let council_config = council::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    council_config.assimilate_storage(&mut t).unwrap();

    t.into()
}
