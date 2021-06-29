#![cfg(test)]

use crate as forum;
use crate::*;
use common::BlockAndTime;

use frame_support::{parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

/// Module which has a full Substrate module for
/// mocking behaviour of MembershipRegistry
pub mod registry {

    use super::*;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
    pub struct Member<AccountId> {
        pub id: AccountId,
    }

    decl_storage! {
        trait Store for Module<T: Config> as MockForumUserRegistry {
            pub ForumUserById get(fn forum_user_by_id) config(): map hasher(blake2_128_concat)
                T::AccountId => Member<T::AccountId>;
        }
    }

    decl_module! {
        pub struct Module<T: Config> for enum Call where origin: T::Origin {}
    }

    impl<T: Config> Module<T> {
        pub fn add_member(member: &Member<T::AccountId>) {
            <ForumUserById<T>>::insert(member.id.clone(), member.clone());
        }
    }

    impl<T: Config> ForumUserRegistry<T::AccountId> for Module<T> {
        fn get_forum_user(id: &T::AccountId) -> Option<ForumUser<T::AccountId>> {
            if <ForumUserById<T>>::contains_key(id) {
                let m = <ForumUserById<T>>::get(id);

                Some(ForumUser { id: m.id })
            } else {
                None
            }
        }
    }

    pub type TestMembershipRegistryModule = Module<Runtime>;
}

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Forum: forum::{Pallet, Call, Storage, Event<T>, Config<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
}

impl frame_system::Config for Runtime {
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
    type AccountId = u128;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl Config for Runtime {
    type Event = Event;
    type MembershipRegistry = registry::TestMembershipRegistryModule;
    type ThreadId = u64;
    type PostId = u64;
}

#[derive(Clone)]
pub enum OriginType {
    Signed(<Runtime as frame_system::Config>::AccountId),
    //Inherent, <== did not find how to make such an origin yet
    Root,
}

pub fn mock_origin(origin: OriginType) -> mock::Origin {
    match origin {
        OriginType::Signed(account_id) => Origin::signed(account_id),
        //OriginType::Inherent => Origin::inherent,
        OriginType::Root => frame_system::RawOrigin::Root.into(), //Origin::root
    }
}

pub const NOT_FORUM_SUDO_ORIGIN: OriginType = OriginType::Signed(111);

pub const NOT_MEMBER_ORIGIN: OriginType = OriginType::Signed(222);

pub const INVLAID_CATEGORY_ID: CategoryId = 333;

pub const INVLAID_THREAD_ID: RuntimeThreadId = 444;

pub const INVLAID_POST_ID: RuntimePostId = 555;

pub fn generate_text(len: usize) -> Vec<u8> {
    vec![b'x'; len]
}

pub fn good_category_title() -> Vec<u8> {
    b"Great new category".to_vec()
}

pub fn good_category_description() -> Vec<u8> {
    b"This is a great new category for the forum".to_vec()
}

pub fn good_thread_title() -> Vec<u8> {
    b"Great new thread".to_vec()
}

pub fn good_thread_text() -> Vec<u8> {
    b"The first post in this thread".to_vec()
}

pub fn good_post_text() -> Vec<u8> {
    b"A response in the thread".to_vec()
}

pub fn good_rationale() -> Vec<u8> {
    b"This post violates our community rules".to_vec()
}

/*
 * These test fixtures can be heavily refactored to avoid repotition, needs macros, and event
 * assertions are also missing.
 */

pub struct CreateCategoryFixture {
    pub origin: OriginType,
    pub parent: Option<CategoryId>,
    pub title: Vec<u8>,
    pub description: Vec<u8>,
    pub result: DispatchResult,
}

impl CreateCategoryFixture {
    pub fn call_and_assert(&self) {
        assert_eq!(
            TestForumModule::create_category(
                mock_origin(self.origin.clone()),
                self.parent,
                self.title.clone(),
                self.description.clone()
            ),
            self.result
        )
    }
}

pub struct UpdateCategoryFixture {
    pub origin: OriginType,
    pub category_id: CategoryId,
    pub new_archival_status: Option<bool>,
    pub new_deletion_status: Option<bool>,
    pub result: DispatchResult,
}

impl UpdateCategoryFixture {
    pub fn call_and_assert(&self) {
        assert_eq!(
            TestForumModule::update_category(
                mock_origin(self.origin.clone()),
                self.category_id,
                self.new_archival_status.clone(),
                self.new_deletion_status.clone()
            ),
            self.result
        )
    }
}

pub struct CreateThreadFixture {
    pub origin: OriginType,
    pub category_id: CategoryId,
    pub title: Vec<u8>,
    pub text: Vec<u8>,
    pub result: DispatchResult,
}

impl CreateThreadFixture {
    pub fn call_and_assert(&self) {
        assert_eq!(
            TestForumModule::create_thread(
                mock_origin(self.origin.clone()),
                self.category_id,
                self.title.clone(),
                self.text.clone()
            ),
            self.result
        )
    }
}

pub struct CreatePostFixture {
    pub origin: OriginType,
    pub thread_id: RuntimeThreadId,
    pub text: Vec<u8>,
    pub result: DispatchResult,
}

impl CreatePostFixture {
    pub fn call_and_assert(&self) {
        assert_eq!(
            TestForumModule::add_post(
                mock_origin(self.origin.clone()),
                self.thread_id,
                self.text.clone()
            ),
            self.result
        )
    }
}

pub fn create_forum_member() -> OriginType {
    let member_id = 123;
    let new_member = registry::Member { id: member_id };
    registry::TestMembershipRegistryModule::add_member(&new_member);
    OriginType::Signed(member_id)
}

pub fn assert_create_category(
    forum_sudo: OriginType,
    parent_category_id: Option<CategoryId>,
    expected_result: DispatchResult,
) {
    CreateCategoryFixture {
        origin: forum_sudo,
        parent: parent_category_id,
        title: good_category_title(),
        description: good_category_description(),
        result: expected_result,
    }
    .call_and_assert();
}

pub fn assert_create_thread(
    forum_sudo: OriginType,
    category_id: CategoryId,
    expected_result: DispatchResult,
) {
    CreateThreadFixture {
        origin: forum_sudo,
        category_id,
        title: good_thread_title(),
        text: good_thread_text(),
        result: expected_result,
    }
    .call_and_assert();
}

pub fn assert_create_post(
    forum_sudo: OriginType,
    thread_id: RuntimeThreadId,
    expected_result: DispatchResult,
) {
    CreatePostFixture {
        origin: forum_sudo,
        thread_id,
        text: good_thread_text(),
        result: expected_result,
    }
    .call_and_assert();
}

pub fn create_category(
    forum_sudo: OriginType,
    parent_category_id: Option<CategoryId>,
) -> CategoryId {
    let category_id = TestForumModule::next_category_id();
    assert_create_category(forum_sudo, parent_category_id, Ok(()));
    category_id
}

pub fn create_root_category(forum_sudo: OriginType) -> CategoryId {
    create_category(forum_sudo, None)
}

pub fn create_root_category_and_thread(
    forum_sudo: OriginType,
) -> (OriginType, CategoryId, RuntimeThreadId) {
    let member_origin = create_forum_member();
    let category_id = create_root_category(forum_sudo);
    let thread_id = TestForumModule::next_thread_id();

    CreateThreadFixture {
        origin: member_origin.clone(),
        category_id,
        title: good_thread_title(),
        text: good_thread_text(),
        result: Ok(()),
    }
    .call_and_assert();

    (member_origin, category_id, thread_id)
}

pub fn create_root_category_and_thread_and_post(
    forum_sudo: OriginType,
) -> (OriginType, CategoryId, RuntimeThreadId, RuntimePostId) {
    let (member_origin, category_id, thread_id) = create_root_category_and_thread(forum_sudo);
    let post_id = TestForumModule::next_post_id();

    CreatePostFixture {
        origin: member_origin.clone(),
        thread_id: thread_id.clone(),
        text: good_post_text(),
        result: Ok(()),
    }
    .call_and_assert();

    (member_origin, category_id, thread_id, post_id)
}

pub fn moderate_thread(
    forum_sudo: OriginType,
    thread_id: RuntimeThreadId,
    rationale: Vec<u8>,
) -> DispatchResult {
    TestForumModule::moderate_thread(mock_origin(forum_sudo), thread_id, rationale)
}

pub fn moderate_post(
    forum_sudo: OriginType,
    post_id: RuntimePostId,
    rationale: Vec<u8>,
) -> DispatchResult {
    TestForumModule::moderate_post(mock_origin(forum_sudo), post_id, rationale)
}

pub fn archive_category(forum_sudo: OriginType, category_id: CategoryId) -> DispatchResult {
    TestForumModule::update_category(mock_origin(forum_sudo), category_id, Some(true), None)
}

pub fn unarchive_category(forum_sudo: OriginType, category_id: CategoryId) -> DispatchResult {
    TestForumModule::update_category(mock_origin(forum_sudo), category_id, Some(false), None)
}

pub fn delete_category(forum_sudo: OriginType, category_id: CategoryId) -> DispatchResult {
    TestForumModule::update_category(mock_origin(forum_sudo), category_id, None, Some(true))
}

pub fn undelete_category(forum_sudo: OriginType, category_id: CategoryId) -> DispatchResult {
    TestForumModule::update_category(mock_origin(forum_sudo), category_id, None, Some(false))
}

pub fn assert_not_forum_sudo_cannot_update_category(
    update_operation: fn(OriginType, CategoryId) -> DispatchResult,
) {
    let config = default_genesis_config();
    let origin = OriginType::Signed(config.forum_sudo);

    build_test_externalities(config).execute_with(|| {
        let category_id = create_root_category(origin.clone());
        assert_eq!(
            update_operation(NOT_FORUM_SUDO_ORIGIN, category_id),
            Err(ERROR_ORIGIN_NOT_FORUM_SUDO)
        );
    });
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.

// refactor
/// - add each config as parameter, then
///

pub fn default_genesis_config() -> forum::GenesisConfig<Runtime> {
    forum::GenesisConfig::<Runtime> {
        category_by_id: vec![], // endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
        next_category_id: 1,
        thread_by_id: vec![],
        next_thread_id: 1,
        post_by_id: vec![],
        next_post_id: 1,

        forum_sudo: 33,

        category_title_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 140,
        },

        category_description_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 140,
        },

        thread_title_constraint: InputValidationLengthConstraint {
            min: 3,
            max_min_diff: 43,
        },

        post_text_constraint: InputValidationLengthConstraint {
            min: 1,
            max_min_diff: 1001,
        },

        thread_moderation_rationale_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 2000,
        },

        post_moderation_rationale_constraint: InputValidationLengthConstraint {
            min: 10,
            max_min_diff: 2000,
        }, // JUST GIVING UP ON ALL THIS FOR NOW BECAUSE ITS TAKING TOO LONG

           // Extra genesis fields
           //initial_forum_sudo: Some(143)
    }
}

pub type RuntimeMap<K, V> = std::vec::Vec<(K, V)>;
pub type RuntimeCategory = Category<
    <Runtime as frame_system::Config>::BlockNumber,
    <Runtime as pallet_timestamp::Config>::Moment,
    <Runtime as frame_system::Config>::AccountId,
>;
pub type RuntimeThread = Thread<
    <Runtime as frame_system::Config>::BlockNumber,
    <Runtime as pallet_timestamp::Config>::Moment,
    <Runtime as frame_system::Config>::AccountId,
    RuntimeThreadId,
>;
pub type RuntimePost = Post<
    <Runtime as frame_system::Config>::BlockNumber,
    <Runtime as pallet_timestamp::Config>::Moment,
    <Runtime as frame_system::Config>::AccountId,
    RuntimeThreadId,
    RuntimePostId,
>;
pub type RuntimeBlockchainTimestamp = BlockAndTime<
    <Runtime as frame_system::Config>::BlockNumber,
    <Runtime as pallet_timestamp::Config>::Moment,
>;

pub type RuntimeThreadId = <Runtime as Config>::ThreadId;
pub type RuntimePostId = <Runtime as Config>::PostId;

pub fn genesis_config(
    category_by_id: &RuntimeMap<CategoryId, RuntimeCategory>,
    next_category_id: u64,
    thread_by_id: &RuntimeMap<RuntimeThreadId, RuntimeThread>,
    next_thread_id: u64,
    post_by_id: &RuntimeMap<RuntimePostId, RuntimePost>,
    next_post_id: u64,
    forum_sudo: <Runtime as frame_system::Config>::AccountId,
    category_title_constraint: &InputValidationLengthConstraint,
    category_description_constraint: &InputValidationLengthConstraint,
    thread_title_constraint: &InputValidationLengthConstraint,
    post_text_constraint: &InputValidationLengthConstraint,
    thread_moderation_rationale_constraint: &InputValidationLengthConstraint,
    post_moderation_rationale_constraint: &InputValidationLengthConstraint,
) -> forum::GenesisConfig<Runtime> {
    forum::GenesisConfig::<Runtime> {
        category_by_id: category_by_id.clone(),
        next_category_id,
        thread_by_id: thread_by_id.clone(),
        next_thread_id,
        post_by_id: post_by_id.clone(),
        next_post_id,
        forum_sudo,
        category_title_constraint: category_title_constraint.clone(),
        category_description_constraint: category_description_constraint.clone(),
        thread_title_constraint: thread_title_constraint.clone(),
        post_text_constraint: post_text_constraint.clone(),
        thread_moderation_rationale_constraint: thread_moderation_rationale_constraint.clone(),
        post_moderation_rationale_constraint: post_moderation_rationale_constraint.clone(),
    }
}

// MockForumUserRegistry
pub fn default_mock_forum_user_registry_genesis_config() -> registry::GenesisConfig<Runtime> {
    registry::GenesisConfig::<Runtime> {
        forum_user_by_id: vec![],
    }
}

// NB!:
// Wanted to have payload: a: &GenesisConfig<Test>
// but borrow checker made my life miserabl, so giving up for now.
pub fn build_test_externalities(config: forum::GenesisConfig<Runtime>) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    config.assimilate_storage(&mut t).unwrap();

    // Add mock registry configuration
    default_mock_forum_user_registry_genesis_config()
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}

// pub type System = frame_system::Module<Runtime>;

/// Export forum module on a test runtime
pub type TestForumModule = Module<Runtime>;
