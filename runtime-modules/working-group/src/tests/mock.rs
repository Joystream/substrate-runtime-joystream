use frame_support::parameter_types;
use frame_support::storage::StorageMap;
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use std::marker::PhantomData;

use crate as working_group;
use crate::{BalanceOf, Config, NegativeImbalance};
use common::constraints::InputValidationLengthConstraint;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: balances::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Minting: minting::{Pallet, Call, Storage},
        Membership: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
        TestWorkingGroup: working_group::<Instance1>::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
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

impl hiring::Config for Test {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = ();
    type StakeHandlerProvider = hiring::Module<Self>;
}

impl minting::Config for Test {
    type Currency = Balances;
    type MintId = u64;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
    pub const StakePoolId: [u8; 8] = *b"joystake";
    pub const ExistentialDeposit: u32 = 0;
}

impl stake::Config for Test {
    type Currency = Balances;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = StakingEventsHandler<Test>;
    type StakeId = u64;
    type SlashId = u64;
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

impl common::currency::GovernanceCurrency for Test {
    type Currency = Balances;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
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

impl recurringrewards::Config for Test {
    type PayoutStatusHandler = ();
    type RecipientId = u64;
    type RewardRelationshipId = u64;
}

parameter_types! {
    pub const MaxWorkerNumberLimit: u32 = 3;
}

impl Config<TestWorkingGroupInstance> for Test {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

pub type TestWorkingGroupInstance = crate::Instance1;

pub(crate) const WORKING_GROUP_MINT_CAPACITY: u64 = 40000;
pub(crate) const WORKING_GROUP_CONSTRAINT_MIN: u16 = 1;
pub(crate) const WORKING_GROUP_CONSTRAINT_DIFF: u16 = 40;

pub fn build_test_externalities() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    crate::GenesisConfig::<Test, TestWorkingGroupInstance> {
        phantom: Default::default(),
        working_group_mint_capacity: WORKING_GROUP_MINT_CAPACITY,
        opening_human_readable_text_constraint: InputValidationLengthConstraint::new(
            WORKING_GROUP_CONSTRAINT_MIN,
            WORKING_GROUP_CONSTRAINT_DIFF,
        ),
        worker_application_human_readable_text_constraint: InputValidationLengthConstraint::new(
            WORKING_GROUP_CONSTRAINT_MIN,
            WORKING_GROUP_CONSTRAINT_DIFF,
        ),
        worker_exit_rationale_text_constraint: InputValidationLengthConstraint::new(
            WORKING_GROUP_CONSTRAINT_MIN,
            WORKING_GROUP_CONSTRAINT_DIFF,
        ),
        worker_storage_size_constraint: crate::default_storage_size_constraint(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

pub struct StakingEventsHandler<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Config + crate::Config<TestWorkingGroupInstance>> stake::StakingEventsHandler<T>
    for StakingEventsHandler<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Config>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if crate::MemberIdByHiringApplicationId::<T, TestWorkingGroupInstance>::contains_key(
            hiring_application_id,
        ) {
            return <crate::Module<T, TestWorkingGroupInstance>>::refund_working_group_stake(
                *stake_id,
                remaining_imbalance,
            );
        }

        remaining_imbalance
    }

    /// Empty handler for slashing
    fn slashed(
        _: &<T as stake::Config>::StakeId,
        _: Option<<T as stake::Config>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <TestWorkingGroup as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <TestWorkingGroup as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}
