#![cfg(test)]

use frame_support::{parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::cell::{Cell, RefCell};
use sp_std::rc::Rc;
use std::panic;

use crate::hiring::ApplicationDeactivationCause;
use crate::Config;
use balances;
use stake;

use crate as hiring;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Hiring: hiring::{Pallet, Call, Storage},
        Balances: balances::{Pallet, Call, Storage, Event<T>},
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
    type AccountId = u128;
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
    pub const ExistentialDeposit: u32 = 100;
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
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

impl Config for Test {
    type OpeningId = u64;
    type ApplicationId = u64;
    type ApplicationDeactivatedHandler = TestApplicationDeactivatedHandler;
    type StakeHandlerProvider = TestStakeHandlerProvider;
}

impl stake::Config for Test {
    type Currency = Balances;
    type StakePoolId = StakePoolId;
    type StakingEventsHandler = ();
    type StakeId = u64;
    type SlashId = u64;
}

// Intercepts panic method
// Returns: whether panic occurred
fn panics<F: std::panic::RefUnwindSafe + Fn()>(could_panic_func: F) -> bool {
    {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            println!("{}", info);
        }));

        // intercept panic
        let result = panic::catch_unwind(|| could_panic_func());

        //restore default behaviour
        panic::set_hook(default_hook);

        result.is_err()
    }
}

pub struct TestStakeHandlerProvider;
impl crate::StakeHandlerProvider<Test> for TestStakeHandlerProvider {
    /// Returns StakeHandler. Mock entry point for stake module.
    fn staking() -> Rc<RefCell<dyn crate::StakeHandler<Test>>> {
        THREAD_LOCAL_STAKE_HANDLER.with(|f| f.borrow().clone())
    }
}

// 1. RefCell - thread_local! mutation pattern
// 2. Rc - ability to have multiple references
// 3. Refcell - interior mutability to provide extra-mock capabilities (like mockall checkpoint).
thread_local! {
    pub static THREAD_LOCAL_STAKE_HANDLER:
      RefCell<Rc<RefCell<dyn crate::StakeHandler<Test>>>> = RefCell::new(Rc::new(RefCell::new(crate::HiringStakeHandler {})));
}

// Sets stake handler implementation in hiring module. Mockall frameworks integration
pub(crate) fn set_stake_handler_impl(
    mock: Rc<sp_std::cell::RefCell<dyn crate::StakeHandler<Test>>>,
) {
    // Hiring::staking.mock_safe(move || MockResult::Return(mock.clone()));
    THREAD_LOCAL_STAKE_HANDLER.with(|f| {
        *f.borrow_mut() = mock.clone();
    });
}

// Tests mock expectation and restores default behaviour
pub(crate) fn test_expectation_and_clear_mock() {
    set_stake_handler_impl(Rc::new(RefCell::new(crate::HiringStakeHandler {})));
}

pub fn build_test_externalities() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    t.into()
}

// Intercepts panic in provided function, test mock expectation and restores default behaviour
pub(crate) fn handle_mock<F: std::panic::RefUnwindSafe + Fn()>(func: F) {
    let panicked = panics(func);

    test_expectation_and_clear_mock();

    assert!(!panicked);
}

//
// ******* ApplicationDeactivatedHandler mocks ********************
//
thread_local! {
    pub static LAST_DEACTIVATED_APPLICATION:
        Cell<Option<(<Test as Config>::ApplicationId, ApplicationDeactivationCause)>> = Cell::new(None);
}

pub struct TestApplicationDeactivatedHandler;
impl crate::ApplicationDeactivatedHandler<Test> for TestApplicationDeactivatedHandler {
    fn deactivated(
        application_id: &<Test as Config>::ApplicationId,
        cause: ApplicationDeactivationCause,
    ) {
        LAST_DEACTIVATED_APPLICATION.with(|f| {
            f.replace(Some((*application_id, cause)));
        });
    }
}

impl TestApplicationDeactivatedHandler {
    pub(crate) fn assert_deactivated_application(
        expected_application_id: <Test as Config>::ApplicationId,
        expected_cause: ApplicationDeactivationCause,
    ) {
        let mut actual_deactivated_application = None;
        LAST_DEACTIVATED_APPLICATION.with(|f| {
            actual_deactivated_application = f.replace(None);
        });

        assert_eq!(
            Some((expected_application_id, expected_cause)),
            actual_deactivated_application
        );
    }
}

// Test fixtures starting block.
pub(crate) static FIRST_BLOCK_HEIGHT: <Test as frame_system::Config>::BlockNumber = 0;
