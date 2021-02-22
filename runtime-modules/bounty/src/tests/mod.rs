#![cfg(test)]

pub(crate) mod fixtures;
pub(crate) mod mocks;

use frame_support::storage::StorageMap;
use frame_system::RawOrigin;
use sp_runtime::DispatchError;

use crate::tests::fixtures::DEFAULT_BOUNTY_CHERRY;
use crate::{BountyActor, BountyMilestone, Error, RawEvent};
use common::council::CouncilBudgetManager;
use fixtures::{
    increase_account_balance, increase_total_balance_issuance_using_account_id, run_to_block,
    set_council_budget, AnnounceWorkEntryFixture, CancelBountyFixture, CreateBountyFixture,
    EventFixture, FundBountyFixture, VetoBountyFixture, WithdrawCreatorFundingFixture,
    WithdrawFundingFixture, WithdrawWorkEntryFixture,
};
use mocks::{build_test_externalities, Balances, Bounty, Test, COUNCIL_BUDGET_ACCOUNT_ID};

#[test]
fn create_bounty_succeeds() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let starting_block = 1;
        run_to_block(starting_block);

        let text = b"Bounty text".to_vec();

        let create_bounty_fixture = CreateBountyFixture::default().with_metadata(text);
        create_bounty_fixture.call_and_assert(Ok(()));

        let bounty_id = 1u64;

        EventFixture::assert_last_crate_event(RawEvent::BountyCreated(
            bounty_id,
            create_bounty_fixture.get_bounty_creation_parameters(),
        ));
    });
}

#[test]
fn create_bounty_fails_with_insufficient_cherry_value() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_cherry(0)
            .call_and_assert(Err(Error::<Test>::CherryLessThenMinimumAllowed.into()));
    });
}

#[test]
fn create_bounty_transfers_member_balance_correctly() {
    build_test_externalities().execute_with(|| {
        let member_id = 1;
        let account_id = 1;
        let cherry = 100;
        let initial_balance = 500;
        let creator_funding = 200;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        // Insufficient member controller account balance.
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_cherry(cherry)
            .with_creator_funding(creator_funding)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - cherry - creator_funding
        );

        let bounty_id = 1;

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            cherry + creator_funding
        );
    });
}

#[test]
fn create_bounty_transfers_the_council_balance_correctly() {
    build_test_externalities().execute_with(|| {
        let cherry = 100;
        let initial_balance = 500;
        let creator_funding = 200;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        // Insufficient member controller account balance.
        CreateBountyFixture::default()
            .with_cherry(cherry)
            .with_creator_funding(creator_funding)
            .call_and_assert(Ok(()));

        assert_eq!(
            <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::get_budget(),
            initial_balance - cherry - creator_funding
        );

        let bounty_id = 1;

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            cherry + creator_funding
        );
    });
}

#[test]
fn created_bounty_gets_fully_funded_by_creator() {
    build_test_externalities().execute_with(|| {
        let starting_block = 0;

        let cherry = 100;
        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        // Insufficient member controller account balance.
        CreateBountyFixture::default()
            .with_cherry(cherry)
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn create_bounty_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        // For a council bounty.
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(1))
            .call_and_assert(Err(DispatchError::BadOrigin));

        // For a member bounty.
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .with_creator_member_id(1)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn create_bounty_fails_with_invalid_min_max_amounts() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_min_amount(100)
            .with_max_amount(0)
            .call_and_assert(Err(
                Error::<Test>::MinFundingAmountCannotBeGreaterThanMaxAmount.into(),
            ));
    });
}

#[test]
fn create_bounty_fails_with_invalid_periods() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_work_period(0)
            .call_and_assert(Err(Error::<Test>::WorkPeriodCannotBeZero.into()));

        CreateBountyFixture::default()
            .with_judging_period(0)
            .call_and_assert(Err(Error::<Test>::JudgingPeriodCannotBeZero.into()));
    });
}

#[test]
fn create_bounty_fails_with_insufficient_balances() {
    build_test_externalities().execute_with(|| {
        let member_id = 1;
        let account_id = 1;
        let cherry = 100;

        // Insufficient council budget.
        CreateBountyFixture::default()
            .with_cherry(cherry)
            .call_and_assert(Err(Error::<Test>::InsufficientBalanceForBounty.into()));

        // Insufficient member controller account balance.
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_cherry(cherry)
            .call_and_assert(Err(Error::<Test>::InsufficientBalanceForBounty.into()));
    });
}

#[test]
fn cancel_bounty_succeeds() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let starting_block = 1;
        run_to_block(starting_block);

        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::BountyCanceled(
            bounty_id,
            BountyActor::Council,
        ));
    });
}

#[test]
fn cancel_bounty_by_member_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let member_id = 1;
        let account_id = 1;
        let initial_balance = 500;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::BountyCanceled(
            bounty_id,
            BountyActor::Member(member_id),
        ));
    });
}

#[test]
fn cancel_bounty_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        CancelBountyFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn cancel_bounty_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let member_id = 1;
        let account_id = 1;
        let initial_balance = 500;

        increase_account_balance(&account_id, initial_balance);
        set_council_budget(initial_balance);

        // Created by council - try to cancel with bad origin
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;
        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Err(DispatchError::BadOrigin));

        // Created by a member - try to cancel with invalid member_id
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 2u64;
        let invalid_member_id = 2;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::NotBountyActor.into()));

        // Created by a member - try to cancel with bad origin
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 3u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(DispatchError::BadOrigin));

        // Created by a member  - try to cancel by council
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 4u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(Error::<Test>::NotBountyActor.into()));
    });
}

#[test]
fn cancel_bounty_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        // Test already cancelled bounty.
        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));

        // Test bounty that was funded.
        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .call_and_assert(Ok(()));

        let bounty_id = 2u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn veto_bounty_succeeds() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let starting_block = 1;
        run_to_block(starting_block);

        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        VetoBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::BountyVetoed(bounty_id));
    });
}

#[test]
fn veto_bounty_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        VetoBountyFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn veto_bounty_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let account_id = 1;

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        VetoBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn veto_bounty_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        // Test already vetoed bounty.
        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        VetoBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));

        // Test bounty that was funded.
        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .call_and_assert(Ok(()));

        let bounty_id = 2u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        VetoBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn fund_bounty_succeeds_by_member() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = DEFAULT_BOUNTY_CHERRY;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);
        increase_total_balance_issuance_using_account_id(
            COUNCIL_BUDGET_ACCOUNT_ID,
            initial_balance,
        );

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - amount
        );

        assert_eq!(
            crate::Module::<Test>::contribution_by_bounty_by_actor(
                bounty_id,
                BountyActor::Member(member_id)
            ),
            amount
        );

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding + amount + cherry
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyFunded(
            bounty_id,
            BountyActor::Member(member_id),
            amount,
        ));
    });
}

#[test]
fn fund_bounty_succeeds_by_council() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = DEFAULT_BOUNTY_CHERRY;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_council()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance - amount - cherry - creator_funding
        );

        assert_eq!(
            crate::Module::<Test>::contribution_by_bounty_by_actor(bounty_id, BountyActor::Council),
            amount
        );

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding + amount + cherry
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyFunded(
            bounty_id,
            BountyActor::Council,
            amount,
        ));
    });
}

#[test]
fn fund_bounty_succeeds_with_reaching_max_funding_amount() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 50;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - amount
        );

        let bounty = Bounty::bounties(&bounty_id);
        assert_eq!(
            bounty.milestone,
            BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: false
            }
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyMaxFundingReached(bounty_id));
    });
}

#[test]
fn multiple_fund_bounty_succeed() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let max_amount = 5000;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = DEFAULT_BOUNTY_CHERRY;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);
        increase_total_balance_issuance_using_account_id(
            COUNCIL_BUDGET_ACCOUNT_ID,
            initial_balance,
        );

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - 2 * amount
        );

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding + 2 * amount + cherry
        );
    });
}

#[test]
fn fund_bounty_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        FundBountyFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn fund_bounty_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn fund_bounty_fails_with_insufficient_balance() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let member_id = 1;
        let account_id = 1;
        let amount = 100;

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Err(Error::<Test>::InsufficientBalanceForBounty.into()));
    });
}

#[test]
fn fund_bounty_fails_with_zero_amount() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let member_id = 1;
        let account_id = 1;
        let amount = 0;

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Err(Error::<Test>::ZeroFundingAmount.into()));
    });
}

#[test]
fn fund_bounty_fails_with_less_than_minimum_amount() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let member_id = 1;
        let account_id = 1;
        let amount = 10;

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Err(Error::<Test>::FundingLessThenMinimumAllowed.into()));
    });
}

#[test]
fn fund_bounty_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn fund_bounty_fails_with_expired_funding_period() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let funding_period = 10;

        increase_total_balance_issuance_using_account_id(account_id, initial_balance);

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .with_funding_period(funding_period)
            .call_and_assert(Ok(()));

        run_to_block(funding_period + 1);

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn withdraw_member_funding_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        run_to_block(funding_period + starting_block + 1);

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        // The whole cherry
        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance + cherry
        );

        // Only funding amount left.
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            amount
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyFundingWithdrawal(
            bounty_id,
            BountyActor::Member(member_id),
        ));
    });
}

#[test]
fn withdraw_council_funding_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .with_council()
            .with_amount(amount)
            .call_and_assert(Ok(()));

        run_to_block(funding_period + starting_block + 1);

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_council()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance - creator_funding
        );

        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyFundingWithdrawal(
            bounty_id,
            BountyActor::Council,
        ));
    });
}

#[test]
fn withdraw_member_funding_with_half_cherry() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let account_id1 = 1;
        let member_id1 = 1;
        let account_id2 = 2;
        let member_id2 = 2;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id1, initial_balance);
        increase_account_balance(&account_id2, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id1))
            .with_member_id(member_id1)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id2))
            .with_member_id(member_id2)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        run_to_block(funding_period + starting_block + 1);

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(member_id1)
            .with_origin(RawOrigin::Signed(account_id1))
            .call_and_assert(Ok(()));

        // A half of the cherry
        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id1),
            initial_balance + cherry / 2
        );

        // On funding amount + creation funding + half of the cherry left.
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding + amount + cherry / 2
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyFundingWithdrawal(
            bounty_id,
            BountyActor::Member(member_id1),
        ));
    });
}

#[test]
fn withdraw_member_funding_removes_bounty() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id, initial_balance);

        println!(
            "Initial: {}",
            Balances::usable_balance(&Bounty::bounty_account_id(1u64))
        );

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        println!(
            "After creation: {}",
            Balances::usable_balance(&Bounty::bounty_account_id(bounty_id))
        );

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        run_to_block(funding_period + starting_block + 1);

        println!(
            "Before creator withdrawal: {}",
            Balances::usable_balance(&Bounty::bounty_account_id(bounty_id))
        );

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        println!(
            "Before member withdrawal: {}",
            Balances::usable_balance(&Bounty::bounty_account_id(bounty_id))
        );

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::BountyRemoved(bounty_id));
    });
}

#[test]
fn withdraw_member_funding_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        WithdrawFundingFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn withdraw_member_funding_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn withdraw_member_funding_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn withdraw_member_funding_fails_with_invalid_bounty_funder() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let max_amount = 500;
        let amount = 100;
        let account_id = 1;
        let member_id = 1;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_period = 10;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id, initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_funding_period(funding_period)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_amount(amount)
            .call_and_assert(Ok(()));

        // Bounty failed because of the funding period
        run_to_block(starting_block + funding_period + 1);

        let invalid_account_id = 2;
        let invalid_member_id = 2;

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(invalid_member_id)
            .with_origin(RawOrigin::Signed(invalid_account_id))
            .call_and_assert(Err(Error::<Test>::NotBountyFunder.into()));
    });
}

#[test]
fn withdraw_creator_funding_by_council_succeeds() {
    build_test_externalities().execute_with(|| {
        let account_id = 1;
        let member_id = 1;
        let funding_period = 10;
        let max_amount = 500;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let funding_amount = 200;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);
        increase_account_balance(&account_id, initial_balance);

        let starting_block = 1;
        run_to_block(starting_block);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_funding_period(funding_period)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance - creator_funding - cherry
        );

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(funding_amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        // Bounty failed because of the funding period
        run_to_block(starting_block + funding_period + 1);

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance - cherry
        );

        assert_eq!(
            Bounty::bounties(bounty_id).milestone,
            BountyMilestone::CreatorFundsWithdrawn
        );

        // On funding amount + unspent cherry left.
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            funding_amount + cherry
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyCreatorFundingWithdrawal(
            bounty_id,
            BountyActor::Council,
        ));
    });
}

#[test]
fn withdraw_creator_funding_removes_the_bounty() {
    build_test_externalities().execute_with(|| {
        let max_amount = 500;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;

        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);

        let starting_block = 1;
        run_to_block(starting_block);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_cherry(cherry)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance - creator_funding - cherry
        );

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&COUNCIL_BUDGET_ACCOUNT_ID),
            initial_balance
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyRemoved(bounty_id));
    });
}

#[test]
fn withdraw_creator_funding_by_member_succeeds() {
    build_test_externalities().execute_with(|| {
        let max_amount = 500;
        let funding_amount = 66;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 200;
        let account_id = 1;
        let member_id = 1;
        let funding_period = 10;

        let funding_account_id = 2;
        let funding_member_id = 2;

        increase_account_balance(&account_id, initial_balance);
        increase_account_balance(&funding_account_id, initial_balance);

        let starting_block = 1;
        run_to_block(starting_block);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_cherry(cherry)
            .with_funding_period(funding_period)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - creator_funding - cherry
        );

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(funding_amount)
            .with_member_id(funding_member_id)
            .with_origin(RawOrigin::Signed(funding_account_id))
            .call_and_assert(Ok(()));

        // Bounty failed because of the funding period
        run_to_block(starting_block + funding_period + 1);

        WithdrawCreatorFundingFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - cherry
        );

        // On funding amount + unspent cherry left.
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            funding_amount + cherry
        );

        EventFixture::assert_last_crate_event(RawEvent::BountyCreatorFundingWithdrawal(
            bounty_id,
            BountyActor::Member(member_id),
        ));
    });
}

#[test]
fn withdraw_creator_funding_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        CancelBountyFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn withdraw_creator_funding_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 500;
        let member_id = 1;
        let account_id = 1;

        increase_account_balance(&account_id, initial_balance);
        set_council_budget(initial_balance);

        // Created by council - try to cancel with bad origin
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Err(DispatchError::BadOrigin));

        // Created by a member - try to cancel with invalid member_id
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 2u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let invalid_member_id = 2;

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::NotBountyActor.into()));

        // Created by a member - try to cancel with bad origin
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 3u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(DispatchError::BadOrigin));

        // Created by a member  - try to cancel by council
        CreateBountyFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        let bounty_id = 4u64;

        CancelBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .call_and_assert(Ok(()));

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(Error::<Test>::NotBountyActor.into()));
    });
}

#[test]
fn withdraw_creator_funding_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn withdraw_creator_funding_fails_when_nothing_to_withdraw() {
    build_test_externalities().execute_with(|| {
        let max_amount = 500;
        let funding_amount = 100;
        let initial_balance = 500;
        let cherry = 100;
        let account_id = 1;
        let member_id = 1;
        let funding_period = 10;

        increase_account_balance(&account_id, initial_balance);
        increase_account_balance(&COUNCIL_BUDGET_ACCOUNT_ID, initial_balance);

        // No creator funding and cherry goes to another funder.
        // Create bounty
        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_cherry(cherry)
            .with_funding_period(funding_period)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(funding_amount)
            .with_member_id(member_id)
            .with_origin(RawOrigin::Signed(account_id))
            .call_and_assert(Ok(()));

        // Bounty failed.
        run_to_block(funding_period + 1);

        // Cannot withdraw cherry.
        WithdrawCreatorFundingFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::NothingToWithdraw.into()));
    });
}

#[test]
fn bounty_removal_succeeds() {
    build_test_externalities().execute_with(|| {
        let max_amount = 500;
        let amount = 100;
        let initial_balance = 500;
        let creator_funding = 100;
        let cherry = 100;
        let account_id = 1;
        let member_id = 1;
        let funding_period = 10;

        increase_account_balance(&account_id, initial_balance);

        // Increment block in order to get Substrate events (no events on block 0).
        let starting_block = 1;
        run_to_block(starting_block);

        // Create bounty
        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_min_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_cherry(cherry)
            .with_funding_period(funding_period)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        // Member funding
        let funding_account_id1 = 2;
        let funding_member_id1 = 2;
        increase_account_balance(&funding_account_id1, initial_balance);

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(funding_member_id1)
            .with_origin(RawOrigin::Signed(funding_account_id1))
            .call_and_assert(Ok(()));

        let funding_account_id2 = 3;
        let funding_member_id2 = 3;
        increase_account_balance(&funding_account_id2, initial_balance);

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(funding_member_id2)
            .with_origin(RawOrigin::Signed(funding_account_id2))
            .call_and_assert(Ok(()));

        let funding_account_id3 = 4;
        let funding_member_id3 = 4;
        increase_account_balance(&funding_account_id3, initial_balance);

        FundBountyFixture::default()
            .with_bounty_id(bounty_id)
            .with_amount(amount)
            .with_member_id(funding_member_id3)
            .with_origin(RawOrigin::Signed(funding_account_id3))
            .call_and_assert(Ok(()));

        // Bounty failed because of the funding period
        run_to_block(starting_block + funding_period + 1);

        // Withdraw member funding
        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(funding_member_id1)
            .with_origin(RawOrigin::Signed(funding_account_id1))
            .call_and_assert(Ok(()));

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(funding_member_id2)
            .with_origin(RawOrigin::Signed(funding_account_id2))
            .call_and_assert(Ok(()));

        WithdrawFundingFixture::default()
            .with_bounty_id(bounty_id)
            .with_member_id(funding_member_id3)
            .with_origin(RawOrigin::Signed(funding_account_id3))
            .call_and_assert(Ok(()));

        let cherry_remaining_fraction = 1;
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            creator_funding + cherry_remaining_fraction
        );

        // Creator withdrawal
        WithdrawCreatorFundingFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_creator_member_id(member_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            balances::Module::<Test>::usable_balance(&account_id),
            initial_balance - cherry
        );

        // Bounty removal effects.
        assert_eq!(
            balances::Module::<Test>::usable_balance(&Bounty::bounty_account_id(bounty_id)),
            0
        );

        assert!(!crate::Bounties::<Test>::contains_key(bounty_id));
        assert!(!Bounty::contributions_exist(&bounty_id));

        EventFixture::assert_last_crate_event(RawEvent::BountyRemoved(bounty_id));
    });
}

#[test]
fn announce_work_entry_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 37;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        increase_account_balance(&account_id, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id),
            initial_balance - entrant_stake
        );

        let entry_id = 1;

        EventFixture::assert_last_crate_event(RawEvent::WorkEntryAnnounced(
            entry_id,
            bounty_id,
            member_id,
            Some(account_id),
        ));
    });
}

#[test]
fn announce_work_entry_fails_with_exceeding_the_entry_limit() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 0;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        increase_account_balance(&account_id, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::MaxWorkEntryLimitReached.into()));
    });
}

#[test]
fn announce_work_entry_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        AnnounceWorkEntryFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn announce_work_entry_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));

        let bounty_id = 1u64;

        AnnounceWorkEntryFixture::default()
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn announce_work_entry_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        set_council_budget(500);

        CreateBountyFixture::default().call_and_assert(Ok(()));

        let bounty_id = 1u64;

        AnnounceWorkEntryFixture::default()
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}

#[test]
fn announce_work_entry_fails_with_invalid_staking_data() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 37;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::NoStakingAccountProvided.into()));

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InsufficientBalanceForStake.into()));

        increase_account_balance(&account_id, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::ConflictingStakes.into()));
    });
}

#[test]
fn withdraw_work_entry_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 37;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        increase_account_balance(&account_id, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id),
            initial_balance - entrant_stake
        );

        let entry_id = 1;

        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_entry_id(entry_id)
            .call_and_assert(Ok(()));

        assert_eq!(Balances::usable_balance(&account_id), initial_balance);

        EventFixture::assert_last_crate_event(RawEvent::WorkEntryWithdrawn(
            bounty_id, entry_id, member_id,
        ));
    });
}

#[test]
fn withdraw_work_slashes_successfully1() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 100;
        let work_period = 1000;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_work_period(work_period)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;

        // Announcing entry with no slashes
        let member_id1 = 1;
        let account_id1 = 1;

        increase_account_balance(&account_id1, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id1))
            .with_member_id(member_id1)
            .with_staking_account_id(account_id1)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id1),
            initial_balance - entrant_stake
        );

        let entry_id1 = 1;

        // Announcing entry with half slashing.

        let member_id2 = 2;
        let account_id2 = 2;

        increase_account_balance(&account_id2, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id2))
            .with_member_id(member_id2)
            .with_staking_account_id(account_id2)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id2),
            initial_balance - entrant_stake
        );

        let entry_id2 = 2;

        // No slashes
        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id1))
            .with_member_id(member_id1)
            .with_entry_id(entry_id1)
            .call_and_assert(Ok(()));

        assert_eq!(Balances::usable_balance(&account_id1), initial_balance);

        // Slashes half.
        let half_period = work_period / 2;
        run_to_block(starting_block + half_period);

        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id2))
            .with_member_id(member_id2)
            .with_entry_id(entry_id2)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id2),
            initial_balance - entrant_stake / 2
        );
    });
}

#[test]
fn withdraw_work_slashes_successfully2() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 100;
        let work_period = 1000;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_work_period(work_period)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;

        // Announcing entry with 33%
        let member_id1 = 1;
        let account_id1 = 1;

        increase_account_balance(&account_id1, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id1))
            .with_member_id(member_id1)
            .with_staking_account_id(account_id1)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id1),
            initial_balance - entrant_stake
        );

        let entry_id1 = 1;

        // Announcing entry with full slashing.

        let member_id2 = 2;
        let account_id2 = 2;

        increase_account_balance(&account_id2, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id2))
            .with_member_id(member_id2)
            .with_staking_account_id(account_id2)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id2),
            initial_balance - entrant_stake
        );

        let entry_id2 = 2;

        // Slashes half.
        let one_third_period = work_period / 3;
        run_to_block(starting_block + one_third_period);

        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id1))
            .with_member_id(member_id1)
            .with_entry_id(entry_id1)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id1),
            initial_balance - entrant_stake / 3
        );

        // Slashes all.
        run_to_block(starting_block + work_period);

        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id2))
            .with_member_id(member_id2)
            .with_entry_id(entry_id2)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id2),
            initial_balance - entrant_stake
        );
    });
}

#[test]
fn withdraw_work_entry_fails_with_invalid_bounty_id() {
    build_test_externalities().execute_with(|| {
        let invalid_bounty_id = 11u64;

        WithdrawWorkEntryFixture::default()
            .with_bounty_id(invalid_bounty_id)
            .call_and_assert(Err(Error::<Test>::BountyDoesntExist.into()));
    });
}

#[test]
fn withdraw_work_entry_fails_with_invalid_entry_id() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 37;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let invalid_entry_id = 11u64;

        WithdrawWorkEntryFixture::default()
            .with_bounty_id(bounty_id)
            .with_entry_id(invalid_entry_id)
            .call_and_assert(Err(Error::<Test>::WorkEntryDoesntExist.into()));
    });
}

#[test]
fn withdraw_work_entry_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        let entry_id = 1;

        WithdrawWorkEntryFixture::default()
            .with_entry_id(entry_id)
            .with_bounty_id(bounty_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn withdraw_work_entry_fails_with_invalid_stage() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 500;
        let creator_funding = 200;
        let max_amount = 100;
        let entrant_stake = 37;
        let work_period = 10;

        <mocks::CouncilBudgetManager as CouncilBudgetManager<u64>>::set_budget(initial_balance);

        CreateBountyFixture::default()
            .with_max_amount(max_amount)
            .with_creator_funding(creator_funding)
            .with_entrant_stake(entrant_stake)
            .with_expected_milestone(BountyMilestone::BountyMaxFundingReached {
                max_funding_reached_at: starting_block,
                reached_on_creation: true,
            })
            .with_work_period(work_period)
            .call_and_assert(Ok(()));

        let bounty_id = 1;
        let member_id = 1;
        let account_id = 1;

        increase_account_balance(&account_id, initial_balance);

        AnnounceWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_staking_account_id(account_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Ok(()));

        assert_eq!(
            Balances::usable_balance(&account_id),
            initial_balance - entrant_stake
        );

        let entry_id = 1;

        run_to_block(starting_block + work_period + 1);

        WithdrawWorkEntryFixture::default()
            .with_origin(RawOrigin::Signed(account_id))
            .with_member_id(member_id)
            .with_entry_id(entry_id)
            .with_bounty_id(bounty_id)
            .call_and_assert(Err(Error::<Test>::InvalidBountyStage.into()));
    });
}