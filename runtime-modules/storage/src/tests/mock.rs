#![cfg(test)]

use frame_support::parameter_types;
use frame_support::storage::StorageMap;
use frame_support::traits::{OnFinalize, OnInitialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

use crate::data_directory::ContentIdExists;
pub use crate::data_directory::Voucher;
pub use crate::data_directory::{ContentParameters, StorageObjectOwner};
use crate::data_object_type_registry::IsActiveDataObjectType;
use crate::ContentId;
pub use crate::StorageWorkingGroupInstance;
pub use crate::{data_directory, data_object_storage_registry, data_object_type_registry};
use common::currency::GovernanceCurrency;
use frame_support::StorageValue;
use membership;

pub use crate::data_directory::{
    DEFAULT_GLOBAL_VOUCHER, DEFAULT_UPLOADING_BLOCKED_STATUS, DEFAULT_VOUCHER,
    DEFAULT_VOUCHER_OBJECTS_LIMIT_UPPER_BOUND, DEFAULT_VOUCHER_SIZE_LIMIT_UPPER_BOUND,
};

pub const DEFAULT_LEADER_ACCOUNT_ID: u64 = 1;
pub const DEFAULT_LEADER_MEMBER_ID: u64 = 1;
pub const DEFAULT_LEADER_WORKER_ID: u32 = 1;

pub struct SetLeadFixture;
impl SetLeadFixture {
    pub fn set_default_lead() {
        let worker = working_group::Worker {
            member_id: DEFAULT_LEADER_MEMBER_ID,
            role_account_id: DEFAULT_LEADER_ACCOUNT_ID,
            reward_relationship: None,
            role_stake_profile: None,
        };

        // Create the worker.
        <working_group::WorkerById<Test, StorageWorkingGroupInstance>>::insert(
            DEFAULT_LEADER_WORKER_ID,
            worker,
        );

        // Update current lead.
        <working_group::CurrentLead<Test, StorageWorkingGroupInstance>>::put(
            DEFAULT_LEADER_WORKER_ID,
        );
    }
}

pub const TEST_FIRST_DATA_OBJECT_TYPE_ID: u64 = 1000;
pub const TEST_FIRST_RELATIONSHIP_ID: u64 = 3000;

pub const TEST_MOCK_LIAISON_STORAGE_PROVIDER_ID: u32 = 1;
pub const TEST_MOCK_EXISTING_CID: u64 = 42;

pub struct AnyDataObjectTypeIsActive {}
impl<T: data_object_type_registry::Config> IsActiveDataObjectType<T> for AnyDataObjectTypeIsActive {
    fn is_active_data_object_type(_which: &T::DataObjectTypeId) -> bool {
        true
    }
}

pub struct MockContent {}
impl ContentIdExists<Test> for MockContent {
    fn has_content(which: &ContentId<Test>) -> bool {
        *which == TEST_MOCK_EXISTING_CID
    }

    fn get_data_object(
        which: &ContentId<Test>,
    ) -> Result<data_directory::DataObject<Test>, data_directory::Error<Test>> {
        match *which {
            TEST_MOCK_EXISTING_CID => Ok(data_directory::DataObjectInternal {
                type_id: 1,
                size: 1234,
                added_at: data_directory::BlockAndTime {
                    block: 10,
                    time: 1024,
                },
                owner: StorageObjectOwner::Member(1),
                liaison: Some(TEST_MOCK_LIAISON_STORAGE_PROVIDER_ID),
                liaison_judgement: data_directory::LiaisonJudgement::Pending,
                ipfs_content_id: vec![],
            }),
            _ => Err(data_directory::Error::<Test>::CidNotFound),
        }
    }
}

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
        Rewards: recurringrewards::{Pallet, Call, Storage},
        Minting: minting::{Pallet, Call, Storage},
        DataObjectTypeRegistry: data_object_type_registry::{Pallet, Call, Storage, Event<T>, Config<T>},
        DataDirectory: data_directory::{Pallet, Call, Storage, Event<T>, Config<T>},
        DataObjectStorageRegistry: data_object_storage_registry::{Pallet, Call, Storage, Event<T>, Config<T>},
        Members: membership::{Pallet, Call, Storage, Event<T>, Config<T>},
        StorageWorkingGroup: working_group::<Instance2>::{Pallet, Call, Storage, Config<T>, Event<T>},
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

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl common::MembershipTypes for Test {
    type MemberId = u64;
    type ActorId = u64;
}

impl common::StorageOwnership for Test {
    type ChannelId = u64;
    type DAOId = u64;
    type ContentId = u64;
    type DataObjectTypeId = u64;
}

parameter_types! {
    pub const ExistentialDeposit: u32 = 0;
    pub const StakePoolId: [u8; 8] = *b"joystake";
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
    pub const MaxWorkerNumberLimit: u32 = 3;
}

impl working_group::Config<StorageWorkingGroupInstance> for Test {
    type Event = Event;
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
}

impl data_object_type_registry::Config for Test {
    type Event = Event;
}

impl data_directory::Config for Test {
    type Event = Event;
    type IsActiveDataObjectType = AnyDataObjectTypeIsActive;
    type MemberOriginValidator = ();
}

impl common::origin::ActorOriginValidator<Origin, u64, u64> for () {
    fn ensure_actor_origin(origin: Origin, _account_id: u64) -> Result<u64, &'static str> {
        let signed_account_id = frame_system::ensure_signed(origin)?;

        Ok(signed_account_id)
    }
}

impl data_object_storage_registry::Config for Test {
    type Event = Event;
    type DataObjectStorageRelationshipId = u64;
    type ContentIdExists = MockContent;
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

impl stake::Config for Test {
    type Currency = Balances;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = ();
    type StakeId = u64;
    type SlashId = u64;
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

impl hiring::Config for Test {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = ();
    type StakeHandlerProvider = hiring::Module<Self>;
}

#[allow(dead_code)]
pub struct ExtBuilder {
    voucher_objects_limit_upper_bound: u64,
    voucher_size_limit_upper_bound: u64,
    global_voucher: Voucher,
    default_voucher: Voucher,
    first_data_object_type_id: u64,
    first_relationship_id: u64,
    uploading_blocked: bool,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            voucher_objects_limit_upper_bound: DEFAULT_VOUCHER_OBJECTS_LIMIT_UPPER_BOUND,
            voucher_size_limit_upper_bound: DEFAULT_VOUCHER_SIZE_LIMIT_UPPER_BOUND,
            global_voucher: DEFAULT_GLOBAL_VOUCHER,
            default_voucher: DEFAULT_VOUCHER,
            first_data_object_type_id: 1,
            first_relationship_id: 3,
            uploading_blocked: DEFAULT_UPLOADING_BLOCKED_STATUS,
        }
    }
}

impl ExtBuilder {
    pub fn first_data_object_type_id(mut self, first_data_object_type_id: u64) -> Self {
        self.first_data_object_type_id = first_data_object_type_id;
        self
    }

    pub fn first_relationship_id(mut self, first_relationship_id: u64) -> Self {
        self.first_relationship_id = first_relationship_id;
        self
    }

    pub fn uploading_blocked_status(mut self, uploading_blocked: bool) -> Self {
        self.uploading_blocked = uploading_blocked;
        self
    }

    pub fn global_voucher(mut self, global_voucher: Voucher) -> Self {
        self.global_voucher = global_voucher;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        data_directory::GenesisConfig::<Test> {
            voucher_size_limit_upper_bound: self.voucher_size_limit_upper_bound,
            voucher_objects_limit_upper_bound: self.voucher_objects_limit_upper_bound,
            global_voucher: self.global_voucher,
            default_voucher: self.default_voucher,
            data_object_by_content_id: vec![],
            vouchers: vec![],
            uploading_blocked: self.uploading_blocked,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        data_object_type_registry::GenesisConfig::<Test> {
            first_data_object_type_id: self.first_data_object_type_id,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        data_object_storage_registry::GenesisConfig::<Test> {
            first_relationship_id: self.first_relationship_id,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        membership::GenesisConfig::<Test> {
            default_paid_membership_fee: 0,
            members: vec![membership::genesis::Member {
                member_id: 0,
                root_account: 1,
                controller_account: 1,
                handle: "alice".into(),
                avatar_uri: "".into(),
                about: "".into(),
                registered_at_time: 0,
            }],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}

pub type TestDataObjectType = data_object_type_registry::DataObjectType;

pub type TestDataObjectTypeRegistry = data_object_type_registry::Module<Test>;
pub type TestDataDirectory = data_directory::Module<Test>;
pub type TestDataObjectStorageRegistry = data_object_storage_registry::Module<Test>;

pub fn with_default_mock_builder<R, F: FnOnce() -> R>(f: F) -> R {
    ExtBuilder::default()
        .first_data_object_type_id(TEST_FIRST_DATA_OBJECT_TYPE_ID)
        .first_relationship_id(TEST_FIRST_RELATIONSHIP_ID)
        .build()
        .execute_with(|| f())
}

pub(crate) fn hire_storage_provider() -> (u64, u32) {
    let storage_provider_id = 1;
    let role_account_id = 1;

    let storage_provider = working_group::Worker {
        member_id: 1,
        role_account_id,
        reward_relationship: None,
        role_stake_profile: None,
    };

    <working_group::WorkerById<Test, StorageWorkingGroupInstance>>::insert(
        storage_provider_id,
        storage_provider,
    );

    (role_account_id, storage_provider_id)
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <TestDataObjectTypeRegistry as OnFinalize<u64>>::on_finalize(System::block_number());
        <TestDataDirectory as OnFinalize<u64>>::on_finalize(System::block_number());
        <TestDataObjectStorageRegistry as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <TestDataObjectTypeRegistry as OnInitialize<u64>>::on_initialize(System::block_number());
        <TestDataDirectory as OnInitialize<u64>>::on_initialize(System::block_number());
        <TestDataObjectStorageRegistry as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}
