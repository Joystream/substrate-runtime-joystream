mod fixtures;
mod hiring_workflow;
mod mock;

use frame_support::dispatch::DispatchError;
use frame_support::storage::{StorageMap, StorageValue};
use frame_system::RawOrigin;
use std::collections::BTreeMap;

use crate::tests::hiring_workflow::HiringWorkflow;
use crate::types::{OpeningPolicyCommitment, OpeningType, RewardPolicy};
use crate::{Error, RawEvent, Worker, WorkerId};
use common::constraints::InputValidationLengthConstraint;
use fixtures::*;
use mock::{
    build_test_externalities, run_to_block, Test, TestWorkingGroup, TestWorkingGroupInstance,
    WORKING_GROUP_CONSTRAINT_DIFF, WORKING_GROUP_CONSTRAINT_MIN, WORKING_GROUP_MINT_CAPACITY,
};

#[test]
fn hire_lead_succeeds() {
    build_test_externalities().execute_with(|| {
        // Ensure that lead is default
        assert_eq!(TestWorkingGroup::current_lead(), None);

        HireLeadFixture::default().hire_lead();

        assert!(TestWorkingGroup::current_lead().is_some());
    });
}

#[test]
fn hire_lead_fails_with_existing_lead() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let hiring_workflow = HiringWorkflow::default()
            .disable_setup_environment()
            .with_opening_type(OpeningType::Leader)
            .add_application(b"leader_handle".to_vec())
            .expect(Err(
                Error::<Test, TestWorkingGroupInstance>::CannotHireLeaderWhenLeaderExists.into(),
            ));

        hiring_workflow.execute();
    });
}

#[test]
fn hire_lead_fails_multiple_applications() {
    build_test_externalities().execute_with(|| {
        let hiring_workflow = HiringWorkflow::default()
            .with_opening_type(OpeningType::Leader)
            .add_application_with_origin(b"leader_handle".to_vec(), RawOrigin::Signed(1), 1)
            .add_application_with_origin(b"leader_handle2".to_vec(), RawOrigin::Signed(2), 2)
            .expect(Err(
                Error::<Test, TestWorkingGroupInstance>::CannotHireMultipleLeaders.into(),
            ));

        hiring_workflow.execute();
    });
}

#[test]
fn add_opening_fails_with_incorrect_unstaking_periods() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                fill_opening_failed_applicant_role_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::FillOpeningFailedApplicantRoleStakeUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                fill_opening_failed_applicant_application_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::FillOpeningFailedApplicantApplicationStakeUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                fill_opening_successful_applicant_application_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::FillOpeningSuccessfulApplicantApplicationStakeUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                exit_role_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(Error::<Test, TestWorkingGroupInstance>::ExitRoleStakeUnstakingPeriodIsZero.into()));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                exit_role_application_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture
            .call_and_assert(Err(Error::<Test, TestWorkingGroupInstance>::ExitRoleApplicationStakeUnstakingPeriodIsZero.into()));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                terminate_role_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(Error::<Test, TestWorkingGroupInstance>::TerminateRoleStakeUnstakingPeriodIsZero.into()));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                terminate_application_stake_unstaking_period: Some(0),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture
            .call_and_assert(Err(Error::<Test, TestWorkingGroupInstance>::TerminateApplicationStakeUnstakingPeriodIsZero.into()));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    crowded_out_unstaking_period_length: Some(0),
                    ..Default::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture
            .call_and_assert(Err(Error::<Test, TestWorkingGroupInstance>::RoleStakingPolicyCrowdedOutUnstakingPeriodIsZero.into()));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    review_period_expired_unstaking_period_length: Some(0),
                    ..Default::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::RoleStakingPolicyReviewPeriodUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                application_staking_policy: Some(hiring::StakingPolicy {
                    crowded_out_unstaking_period_length: Some(0),
                    ..Default::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::ApplicationStakingPolicyCrowdedOutUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                application_staking_policy: Some(hiring::StakingPolicy {
                    review_period_expired_unstaking_period_length: Some(0),
                    ..Default::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::ApplicationStakingPolicyReviewPeriodUnstakingPeriodIsZero.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                application_rationing_policy: Some(hiring::ApplicationRationingPolicy {
                    max_active_applicants: 0,
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::ApplicationRationingPolicyMaxActiveApplicantsIsZero.into(),
        ));
    });
}

#[test]
fn add_opening_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();

        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::OpeningAdded(opening_id));
    });
}

#[test]
fn add_leader_opening_succeeds_fails_with_incorrect_origin_for_opening_type() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_opening_type(OpeningType::Leader);

        add_opening_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn add_leader_opening_succeeds() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_opening_type(OpeningType::Leader)
            .with_origin(RawOrigin::Root);

        add_opening_fixture.call_and_assert(Ok(()));
    });
}

#[test]
fn add_opening_fails_with_lead_is_not_set() {
    build_test_externalities().execute_with(|| {
        let add_opening_fixture = AddWorkerOpeningFixture::default();

        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::CurrentLeadNotSet.into(),
        ));
    });
}

#[test]
fn add_opening_fails_with_invalid_human_readable_text() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        <crate::OpeningHumanReadableText<TestWorkingGroupInstance>>::put(
            InputValidationLengthConstraint {
                min: 1,
                max_min_diff: 5,
            },
        );

        let add_opening_fixture = AddWorkerOpeningFixture::default().with_text(Vec::new());

        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OpeningTextTooShort.into(),
        ));

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_text(b"Long text".to_vec());

        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OpeningTextTooLong.into(),
        ));
    });
}

#[test]
fn add_opening_fails_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_activate_at(hiring::ActivateOpeningAt::ExactBlock(0));

        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::AddWorkerOpeningActivatesInThePast.into(),
        ));
    });
}

#[test]
fn accept_applications_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_activate_at(hiring::ActivateOpeningAt::ExactBlock(5));
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let accept_applications_fixture =
            AcceptWorkerApplicationsFixture::default_for_opening_id(opening_id);
        accept_applications_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::AcceptedApplications(opening_id));
    });
}

#[test]
fn accept_applications_fails_for_invalid_opening_type() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_origin(RawOrigin::Root)
            .with_opening_type(OpeningType::Leader)
            .with_activate_at(hiring::ActivateOpeningAt::ExactBlock(5));
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let accept_applications_fixture =
            AcceptWorkerApplicationsFixture::default_for_opening_id(opening_id);
        accept_applications_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn accept_applications_fails_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let accept_applications_fixture =
            AcceptWorkerApplicationsFixture::default_for_opening_id(opening_id);
        accept_applications_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::AcceptWorkerApplicationsOpeningIsNotWaitingToBegin.into(),
        ));
    });
}

#[test]
fn accept_applications_fails_with_not_lead() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        SetLeadFixture::set_lead_with_ids(2, 2, 2);

        let accept_applications_fixture =
            AcceptWorkerApplicationsFixture::default_for_opening_id(opening_id);
        accept_applications_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::IsNotLeadAccount.into(),
        ));
    });
}

#[test]
fn accept_applications_fails_with_no_opening() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let opening_id = 55; // random opening id

        let accept_applications_fixture =
            AcceptWorkerApplicationsFixture::default_for_opening_id(opening_id);
        accept_applications_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OpeningDoesNotExist.into(),
        ));
    });
}

#[test]
fn apply_on_opening_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::AppliedOnOpening(
            opening_id,
            application_id,
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_no_opening() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let opening_id = 123; // random opening id

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OpeningDoesNotExist.into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_not_set_members() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_origin(RawOrigin::Signed(55), 55);
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OriginIsNeitherMemberControllerOrRoot.into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 500000);
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_application_stake(100);
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::AddWorkerOpeningStakeProvidedWhenRedundant
                .into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_invalid_application_stake() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let stake = 100;

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                application_staking_policy: Some(hiring::StakingPolicy {
                    amount: stake,
                    ..hiring::StakingPolicy::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_origin(RawOrigin::Signed(2), 2)
                .with_application_stake(stake);
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::InsufficientBalanceToApply.into(),
        ));
    });
}

#[test]
fn add_opening_fails_with_invalid_zero_application_stake() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let zero_stake = 0;

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                application_staking_policy: Some(hiring::StakingPolicy {
                    amount: zero_stake,
                    amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                    ..hiring::StakingPolicy::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        add_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::AddWorkerOpeningApplicationStakeCannotBeZero
                .into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_invalid_role_stake() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let stake = 100;

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    amount: stake,
                    ..hiring::StakingPolicy::default()
                }),
                ..OpeningPolicyCommitment::default()
            });
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_role_stake(Some(stake))
                .with_origin(RawOrigin::Signed(2), 2);
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::InsufficientBalanceToApply.into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_invalid_text() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        <crate::WorkerApplicationHumanReadableText<TestWorkingGroupInstance>>::put(
            InputValidationLengthConstraint {
                min: 1,
                max_min_diff: 5,
            },
        );

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id).with_text(Vec::new());
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerApplicationTextTooShort.into(),
        ));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_text(b"Long text".to_vec());
        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerApplicationTextTooLong.into(),
        ));
    });
}

#[test]
fn apply_on_opening_fails_with_already_active_application() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        apply_on_opening_fixture.call_and_assert(Ok(()));

        apply_on_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::MemberHasActiveApplicationOnOpening.into(),
        ));
    });
}

#[test]
fn withdraw_worker_application_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let withdraw_application_fixture =
            WithdrawApplicationFixture::default_for_application_id(application_id);
        withdraw_application_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::ApplicationWithdrawn(application_id));
    });
}

#[test]
fn withdraw_worker_application_fails_invalid_application_id() {
    build_test_externalities().execute_with(|| {
        let invalid_application_id = 6;

        let withdraw_application_fixture =
            WithdrawApplicationFixture::default_for_application_id(invalid_application_id);
        withdraw_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerApplicationDoesNotExist.into(),
        ));
    });
}

#[test]
fn withdraw_worker_application_fails_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let withdraw_application_fixture =
            WithdrawApplicationFixture::default_for_application_id(application_id)
                .with_origin(RawOrigin::None);
        withdraw_application_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn withdraw_worker_application_fails_with_invalid_application_author() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let invalid_author_account_id = 55;
        let withdraw_application_fixture =
            WithdrawApplicationFixture::default_for_application_id(application_id)
                .with_signer(invalid_author_account_id);
        withdraw_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OriginIsNotApplicant.into(),
        ));
    });
}

#[test]
fn withdraw_worker_application_fails_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let withdraw_application_fixture =
            WithdrawApplicationFixture::default_for_application_id(application_id);
        withdraw_application_fixture.call_and_assert(Ok(()));
        withdraw_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WithdrawWorkerApplicationApplicationNotActive
                .into(),
        ));
    });
}

#[test]
fn terminate_worker_application_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let terminate_application_fixture =
            TerminateApplicationFixture::default_for_application_id(application_id);
        terminate_application_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::ApplicationTerminated(application_id));
    });
}

#[test]
fn terminate_worker_application_fails_with_invalid_application_author() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let invalid_author_account_id = 55;
        let terminate_application_fixture =
            TerminateApplicationFixture::default_for_application_id(application_id)
                .with_signer(invalid_author_account_id);
        terminate_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::IsNotLeadAccount.into(),
        ));
    });
}

#[test]
fn terminate_worker_application_fails_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let terminate_application_fixture =
            TerminateApplicationFixture::default_for_application_id(application_id)
                .with_origin(RawOrigin::None);
        terminate_application_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn terminate_worker_application_fails_invalid_application_id() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let invalid_application_id = 6;

        let terminate_application_fixture =
            TerminateApplicationFixture::default_for_application_id(invalid_application_id);
        terminate_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerApplicationDoesNotExist.into(),
        ));
    });
}

#[test]
fn terminate_worker_application_fails_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let terminate_application_fixture =
            TerminateApplicationFixture::default_for_application_id(application_id);
        terminate_application_fixture.call_and_assert(Ok(()));
        terminate_application_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WithdrawWorkerApplicationApplicationNotActive
                .into(),
        ));
    });
}

#[test]
fn begin_review_worker_applications_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::BeganApplicationReview(opening_id));
    });
}

#[test]
fn begin_review_worker_applications_fails_with_invalid_origin_for_opening_type() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_origin(RawOrigin::Root)
            .with_opening_type(OpeningType::Leader);
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn begin_review_worker_applications_fails_with_not_a_lead() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        SetLeadFixture::set_lead_with_ids(2, 2, 2);

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Err(Error::<
            Test,
            TestWorkingGroupInstance,
        >::IsNotLeadAccount
            .into()));
    });
}

#[test]
fn begin_review_worker_applications_fails_with_invalid_opening() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let invalid_opening_id = 6;

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(invalid_opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Err(Error::<
            Test,
            TestWorkingGroupInstance,
        >::OpeningDoesNotExist
            .into()));
    });
}

#[test]
fn begin_review_worker_applications_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));
        begin_review_worker_applications_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::BeginWorkerApplicantReviewOpeningOpeningIsNotWaitingToBegin.into(),
        ));
    });
}

#[test]
fn begin_review_worker_applications_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id)
                .with_origin(RawOrigin::None);
        begin_review_worker_applications_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn fill_opening_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();
        increase_total_balance_issuance_using_account_id(1, 10000);

        let add_opening_fixture =
            AddWorkerOpeningFixture::default().with_policy_commitment(OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    amount: 10,
                    amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                    crowded_out_unstaking_period_length: None,
                    review_period_expired_unstaking_period_length: None,
                }),
                ..OpeningPolicyCommitment::default()
            });
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_role_stake(Some(10));
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));

        let mint_id = create_mint();
        set_mint_id(mint_id);

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, vec![application_id])
                .with_reward_policy(RewardPolicy {
                    amount_per_payout: 1000,
                    next_payment_at_block: 20,
                    payout_interval: None,
                });
        let worker_id = fill_opening_fixture.call_and_assert(Ok(()));

        let mut worker_application_dictionary = BTreeMap::new();
        worker_application_dictionary.insert(application_id, worker_id);

        EventFixture::assert_last_crate_event(RawEvent::OpeningFilled(
            opening_id,
            worker_application_dictionary,
        ));
    });
}

#[test]
fn fill_opening_fails_with_invalid_origin_for_opening_type() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();
        increase_total_balance_issuance_using_account_id(1, 10000);

        let add_opening_fixture = AddWorkerOpeningFixture::default()
            .with_policy_commitment(OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    amount: 10,
                    amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                    crowded_out_unstaking_period_length: None,
                    review_period_expired_unstaking_period_length: None,
                }),
                ..OpeningPolicyCommitment::default()
            })
            .with_opening_type(OpeningType::Leader)
            .with_origin(RawOrigin::Root);
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id)
                .with_role_stake(Some(10));
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id)
                .with_origin(RawOrigin::Root);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));

        set_mint_id(create_mint());

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, vec![application_id])
                .with_reward_policy(RewardPolicy {
                    amount_per_payout: 1000,
                    next_payment_at_block: 20,
                    payout_interval: None,
                });
        fill_opening_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn fill_opening_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, Vec::new())
                .with_origin(RawOrigin::None);
        fill_opening_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn fill_opening_fails_with_not_a_lead() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        SetLeadFixture::set_lead_with_ids(2, 2, 2);

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, Vec::new());
        fill_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::IsNotLeadAccount.into(),
        ));
    });
}

#[test]
fn fill_opening_fails_with_invalid_opening() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let invalid_opening_id = 6;

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(invalid_opening_id, Vec::new());
        fill_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::OpeningDoesNotExist.into(),
        ));
    });
}

#[test]
fn fill_opening_fails_with_invalid_application_list() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));

        let invalid_application_id = 66;
        let fill_opening_fixture = FillWorkerOpeningFixture::default_for_ids(
            opening_id,
            vec![application_id, invalid_application_id],
        );
        fill_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::SuccessfulWorkerApplicationDoesNotExist.into(),
        ));
    });
}

#[test]
fn fill_opening_fails_with_invalid_application_with_hiring_error() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, Vec::new());
        fill_opening_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::FullWorkerOpeningOpeningNotInReviewPeriodStage
                .into(),
        ));
    });
}

#[test]
fn fill_opening_fails_with_invalid_reward_policy() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let add_opening_fixture = AddWorkerOpeningFixture::default();
        let opening_id = add_opening_fixture.call_and_assert(Ok(()));

        let apply_on_opening_fixture =
            ApplyOnWorkerOpeningFixture::default_for_opening_id(opening_id);
        let application_id = apply_on_opening_fixture.call_and_assert(Ok(()));

        let begin_review_worker_applications_fixture =
            BeginReviewWorkerApplicationsFixture::default_for_opening_id(opening_id);
        begin_review_worker_applications_fixture.call_and_assert(Ok(()));

        let fill_opening_fixture =
            FillWorkerOpeningFixture::default_for_ids(opening_id, vec![application_id])
                .with_reward_policy(RewardPolicy {
                    amount_per_payout: 10000,
                    // Invalid next payment at block zero
                    next_payment_at_block: 0,
                    payout_interval: None,
                });
        fill_opening_fixture
    });
}

#[test]
fn update_worker_role_account_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let new_account_id = 10;
        let worker_id = fill_default_worker_position();

        let update_worker_account_fixture =
            UpdateWorkerRoleAccountFixture::default_with_ids(worker_id, new_account_id);

        update_worker_account_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::WorkerRoleAccountUpdated(
            worker_id,
            new_account_id,
        ));
    });
}

#[test]
fn update_worker_role_account_by_leader_succeeds() {
    build_test_externalities().execute_with(|| {
        let new_account_id = 10;
        let worker_id = HireLeadFixture::default().hire_lead();

        let old_lead = TestWorkingGroup::worker_by_id(worker_id);

        let update_worker_account_fixture =
            UpdateWorkerRoleAccountFixture::default_with_ids(worker_id, new_account_id);

        update_worker_account_fixture.call_and_assert(Ok(()));

        let new_lead = TestWorkingGroup::worker_by_id(worker_id);

        assert_eq!(
            new_lead,
            Worker {
                role_account_id: new_account_id,
                ..old_lead
            }
        );
    });
}

#[test]
fn update_worker_role_account_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();
        let update_worker_account_fixture =
            UpdateWorkerRoleAccountFixture::default_with_ids(worker_id, 1)
                .with_origin(RawOrigin::None);

        update_worker_account_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::MembershipUnsignedOrigin.into(),
        ));
    });
}

#[test]
fn update_worker_reward_account_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_default_worker_position();

        let new_role_account = 22;
        let update_worker_account_fixture =
            UpdateWorkerRewardAccountFixture::default_with_ids(worker_id, new_role_account);

        update_worker_account_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::WorkerRewardAccountUpdated(
            worker_id,
            new_role_account,
        ));
    });
}

#[test]
fn update_worker_reward_account_succeeds_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default()
            .with_reward_policy(RewardPolicy {
                amount_per_payout: 1000,
                next_payment_at_block: 20,
                payout_interval: None,
            })
            .hire_lead();

        let new_reward_account = 22;
        let update_worker_account_fixture =
            UpdateWorkerRewardAccountFixture::default_with_ids(worker_id, new_reward_account);

        update_worker_account_fixture.call_and_assert(Ok(()));
    });
}

#[test]
fn update_worker_reward_account_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let update_worker_account_fixture =
            UpdateWorkerRewardAccountFixture::default_with_ids(1, 1).with_origin(RawOrigin::None);

        update_worker_account_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_worker_reward_account_fails_with_invalid_origin_signed_account() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();
        let worker = get_worker_by_id(worker_id);

        let invalid_role_account = 23333;
        let update_worker_account_fixture =
            UpdateWorkerRewardAccountFixture::default_with_ids(worker_id, worker.role_account_id)
                .with_origin(RawOrigin::Signed(invalid_role_account));

        update_worker_account_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::SignerIsNotWorkerRoleAccount.into(),
        ));
    });
}

#[test]
fn update_worker_reward_account_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 1;
        fill_default_worker_position();

        let new_reward_account = 2;
        let update_worker_account_fixture = UpdateWorkerRewardAccountFixture::default_with_ids(
            invalid_worker_id,
            new_reward_account,
        );

        update_worker_account_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn update_worker_reward_account_fails_with_no_recurring_reward() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_no_reward();

        let new_reward_account = 343;

        let update_worker_account_fixture =
            UpdateWorkerRewardAccountFixture::default_with_ids(worker_id, new_reward_account);

        update_worker_account_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerHasNoReward.into(),
        ));
    });
}

#[test]
fn update_worker_reward_amount_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_default_worker_position();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id);

        update_worker_amount_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::WorkerRewardAmountUpdated(worker_id));
    });
}

#[test]
fn update_worker_reward_amount_succeeds_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default()
            .with_reward_policy(RewardPolicy {
                amount_per_payout: 1000,
                next_payment_at_block: 20,
                payout_interval: None,
            })
            .hire_lead();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::Root);

        update_worker_amount_fixture.call_and_assert(Ok(()));
    });
}

#[test]
fn update_worker_reward_amount_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let worker_id = 22; // random worker id

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::None);

        update_worker_amount_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_worker_reward_amount_fails_with_invalid_origin_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().hire_lead();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::None);

        update_worker_amount_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_worker_reward_amount_fails_with_invalid_origin_signed_account() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::Signed(2));

        update_worker_amount_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::IsNotLeadAccount.into(),
        ));
    });
}

#[test]
fn update_worker_reward_amount_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 12;
        fill_default_worker_position();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(invalid_worker_id);

        update_worker_amount_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn update_worker_reward_amount_fails_with_no_recurring_reward() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_no_reward();

        let update_worker_amount_fixture =
            UpdateWorkerRewardAmountFixture::default_for_worker_id(worker_id);

        update_worker_amount_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerHasNoReward.into(),
        ));
    });
}

fn fill_default_worker_position() -> u64 {
    fill_worker_position(
        Some(RewardPolicy {
            amount_per_payout: 1000,
            next_payment_at_block: 20,
            payout_interval: None,
        }),
        None,
        true,
        OpeningType::Worker,
        None,
    )
}

fn fill_worker_position_with_no_reward() -> u64 {
    fill_worker_position(None, None, true, OpeningType::Worker, None)
}

fn fill_worker_position_with_stake(stake: u64) -> u64 {
    fill_worker_position(
        Some(RewardPolicy {
            amount_per_payout: 1000,
            next_payment_at_block: 20,
            payout_interval: None,
        }),
        Some(stake),
        true,
        OpeningType::Worker,
        None,
    )
}

fn fill_worker_position(
    reward_policy: Option<RewardPolicy<u64, u64>>,
    role_stake: Option<u64>,
    setup_environment: bool,
    opening_type: OpeningType,
    worker_handle: Option<Vec<u8>>,
) -> u64 {
    let mut hiring_workflow = HiringWorkflow::default()
        .with_role_stake(role_stake)
        .with_setup_environment(setup_environment)
        .with_opening_type(opening_type)
        .with_reward_policy(reward_policy);

    hiring_workflow = if let Some(worker_handle) = worker_handle {
        hiring_workflow.add_application(worker_handle)
    } else {
        hiring_workflow.add_default_application()
    };

    hiring_workflow.execute().unwrap()
}

#[test]
fn leave_worker_role_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_default_worker_position();

        let leave_worker_role_fixture = LeaveWorkerRoleFixture::default_for_worker_id(worker_id);

        leave_worker_role_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::WorkerExited(
            worker_id,
            b"rationale_text".to_vec(),
        ));
    });
}

#[test]
fn leave_worker_role_by_leader_succeeds() {
    build_test_externalities().execute_with(|| {
        // Ensure that lead is default
        assert_eq!(TestWorkingGroup::current_lead(), None);
        let worker_id = HireLeadFixture::default().hire_lead();

        assert!(TestWorkingGroup::current_lead().is_some());

        let leave_worker_role_fixture = LeaveWorkerRoleFixture::default_for_worker_id(worker_id);

        leave_worker_role_fixture.call_and_assert(Ok(()));

        assert_eq!(TestWorkingGroup::current_lead(), None);
    });
}

#[test]
fn leave_worker_role_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let leave_worker_role_fixture =
            LeaveWorkerRoleFixture::default_for_worker_id(1).with_origin(RawOrigin::None);

        leave_worker_role_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn leave_worker_role_fails_with_invalid_origin_signed_account() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let leave_worker_role_fixture = LeaveWorkerRoleFixture::default_for_worker_id(worker_id)
            .with_origin(RawOrigin::Signed(2));

        leave_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::SignerIsNotWorkerRoleAccount.into(),
        ));
    });
}

#[test]
fn leave_worker_role_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 1;
        fill_default_worker_position();

        let leave_worker_role_fixture =
            LeaveWorkerRoleFixture::default_for_worker_id(invalid_worker_id);

        leave_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn leave_worker_role_fails_with_invalid_recurring_reward_relationships() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let mut worker = TestWorkingGroup::worker_by_id(worker_id);
        worker.reward_relationship = Some(2);

        <crate::WorkerById<Test, TestWorkingGroupInstance>>::insert(worker_id, worker);

        let leave_worker_role_fixture = LeaveWorkerRoleFixture::default_for_worker_id(worker_id);

        leave_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::RelationshipMustExist.into(),
        ));
    });
}

#[test]
fn leave_worker_role_succeeds_with_stakes() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_worker_position_with_stake(100);

        let leave_worker_role_fixture = LeaveWorkerRoleFixture::default_for_worker_id(worker_id);

        leave_worker_role_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::WorkerExited(
            worker_id,
            b"rationale_text".to_vec(),
        ));
    });
}

#[test]
fn terminate_worker_role_succeeds_with_stakes() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let total_balance = 10000;
        let stake_balance = 100;

        let worker_account_id = 2;
        let worker_member_id = 2;
        increase_total_balance_issuance_using_account_id(worker_account_id, total_balance);

        HireLeadFixture::default().hire_lead();

        let worker_id = HiringWorkflow::default()
            .disable_setup_environment()
            .with_role_stake(Some(stake_balance))
            .add_application_with_origin(
                b"worker_handle".to_vec(),
                RawOrigin::Signed(worker_account_id),
                worker_member_id,
            )
            .execute()
            .unwrap();

        // Balance was staked.
        assert_eq!(
            get_balance(worker_account_id),
            total_balance - stake_balance
        );

        let stake_id = 0;
        let old_stake = <stake::Module<Test>>::stakes(stake_id);

        assert_eq!(get_stake_balance(old_stake), stake_balance);

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id);

        terminate_worker_role_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::TerminatedWorker(
            worker_id,
            b"rationale_text".to_vec(),
        ));

        // Balance was restored.

        assert_eq!(get_balance(worker_account_id), total_balance);

        let new_stake = <stake::Module<Test>>::stakes(stake_id);
        assert!(matches!(
            new_stake.staking_status,
            stake::StakingStatus::NotStaked
        ));
    });
}

#[test]
fn terminate_worker_role_succeeds_with_slashing() {
    build_test_externalities().execute_with(|| {
        let total_balance = 10000;
        let stake_balance = 100;

        let worker_account_id = 2;
        let worker_member_id = 2;
        increase_total_balance_issuance_using_account_id(worker_account_id, total_balance);

        assert_eq!(get_balance(worker_account_id), total_balance);

        HireLeadFixture::default().hire_lead();

        let worker_id = HiringWorkflow::default()
            .disable_setup_environment()
            .with_role_stake(Some(stake_balance))
            .add_application_with_origin(
                b"worker_handle".to_vec(),
                RawOrigin::Signed(worker_account_id),
                worker_member_id,
            )
            .execute()
            .unwrap();

        // Balance was staked.

        assert_eq!(
            get_balance(worker_account_id),
            total_balance - stake_balance
        );

        let stake_id = 0;
        let old_stake = <stake::Module<Test>>::stakes(stake_id);

        assert_eq!(get_stake_balance(old_stake), stake_balance);

        // Terminate with slashing.

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id).with_slashing();

        terminate_worker_role_fixture.call_and_assert(Ok(()));

        // Balance was slashed.

        assert_eq!(
            get_balance(worker_account_id),
            total_balance - stake_balance
        );

        let new_stake = <stake::Module<Test>>::stakes(stake_id);
        assert!(matches!(
            new_stake.staking_status,
            stake::StakingStatus::NotStaked
        ));
    });
}

#[test]
fn terminate_worker_role_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        HireLeadFixture::default().hire_lead();

        let worker_id = HiringWorkflow::default()
            .disable_setup_environment()
            .add_application_with_origin(b"worker_handle".to_vec(), RawOrigin::Signed(2), 2)
            .execute()
            .unwrap();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id);

        terminate_worker_role_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::TerminatedWorker(
            worker_id,
            b"rationale_text".to_vec(),
        ));
    });
}

#[test]
fn fire_leader_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = HireLeadFixture::default().hire_lead();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::Root);

        terminate_worker_role_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::TerminatedLeader(
            worker_id,
            b"rationale_text".to_vec(),
        ));

        assert_eq!(TestWorkingGroup::current_lead(), None);
    });
}

#[test]
fn terminate_worker_role_fails_with_invalid_text() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let worker_id = HiringWorkflow::default()
            .disable_setup_environment()
            .add_application_with_origin(b"worker_handle".to_vec(), RawOrigin::Signed(2), 2)
            .execute()
            .unwrap();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id).with_text(Vec::new());
        terminate_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerExitRationaleTextTooShort.into(),
        ));

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id)
                .with_text(b"MSG_WORKER_EXIT_RATIONALE_TEXT_TOO_LONG".to_vec());
        terminate_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerExitRationaleTextTooLong.into(),
        ));
    });
}

#[test]
fn terminate_worker_role_fails_with_unset_lead() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        SetLeadFixture::unset_lead();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id);

        terminate_worker_role_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::CurrentLeadNotSet.into(),
        ));
    });
}

#[test]
fn terminate_worker_role_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let worker_id = HiringWorkflow::default()
            .disable_setup_environment()
            .add_application_with_origin(b"worker_handle".to_vec(), RawOrigin::Signed(2), 2)
            .execute()
            .unwrap();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::None);

        terminate_worker_role_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn fire_leader_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().hire_lead();

        let terminate_worker_role_fixture =
            TerminateWorkerRoleFixture::default_for_worker_id(worker_id)
                .with_origin(RawOrigin::None);

        terminate_worker_role_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn increase_worker_stake_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_worker_position_with_stake(100);

        let increase_stake_fixture = IncreaseWorkerStakeFixture::default_for_worker_id(worker_id);

        increase_stake_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StakeIncreased(worker_id));
    });
}

#[test]
fn increase_worker_stake_succeeds_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().with_stake(100).hire_lead();

        let increase_stake_fixture = IncreaseWorkerStakeFixture::default_for_worker_id(worker_id);

        increase_stake_fixture.call_and_assert(Ok(()));
    });
}

#[test]
fn increase_worker_stake_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let worker_id = 0;
        let increase_stake_fixture = IncreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_origin(RawOrigin::None);

        increase_stake_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn increase_worker_stake_fails_with_zero_balance() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_stake(100);

        let increase_stake_fixture =
            IncreaseWorkerStakeFixture::default_for_worker_id(worker_id).with_balance(0);

        increase_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::StakeBalanceCannotBeZero.into(),
        ));
    });
}

#[test]
fn increase_worker_stake_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 11;

        let increase_stake_fixture =
            IncreaseWorkerStakeFixture::default_for_worker_id(invalid_worker_id);

        increase_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn increase_worker_stake_fails_with_invalid_balance() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_stake(100);
        let invalid_balance = 100000000;
        let increase_stake_fixture = IncreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_balance(invalid_balance);

        increase_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::StakingErrorInsufficientBalanceInSourceAccount
                .into(),
        ));
    });
}

#[test]
fn increase_worker_stake_fails_with_no_stake_profile() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let increase_stake_fixture = IncreaseWorkerStakeFixture::default_for_worker_id(worker_id);

        increase_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::NoWorkerStakeProfile.into(),
        ));
    });
}

#[test]
fn decrease_worker_stake_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_worker_position_with_stake(100);

        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id);

        decrease_stake_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StakeDecreased(worker_id));
    });
}

#[test]
fn decrease_worker_stake_succeeds_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().with_stake(100).hire_lead();

        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_origin(RawOrigin::Root);

        decrease_stake_fixture.call_and_assert(Ok(()));
    });
}

#[test]
fn decrease_worker_stake_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let worker_id = 22; // random worker id
        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_origin(RawOrigin::None);

        decrease_stake_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn decrease_worker_stake_fails_with_invalid_origin_for_leader() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().hire_lead();
        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_origin(RawOrigin::None);

        decrease_stake_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn decrease_worker_stake_fails_with_zero_balance() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_stake(100);

        let decrease_stake_fixture =
            DecreaseWorkerStakeFixture::default_for_worker_id(worker_id).with_balance(0);

        decrease_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::StakeBalanceCannotBeZero.into(),
        ));
    });
}

#[test]
fn decrease_worker_stake_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();
        let invalid_worker_id = 11;

        let decrease_stake_fixture =
            DecreaseWorkerStakeFixture::default_for_worker_id(invalid_worker_id);

        decrease_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn decrease_worker_stake_fails_with_invalid_balance() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_stake(100);
        let invalid_balance = 100000000;
        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id)
            .with_balance(invalid_balance);

        decrease_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::StakingErrorInsufficientStake.into(),
        ));
    });
}

#[test]
fn decrease_worker_stake_fails_with_no_stake_profile() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let decrease_stake_fixture = DecreaseWorkerStakeFixture::default_for_worker_id(worker_id);

        decrease_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::NoWorkerStakeProfile.into(),
        ));
    });
}

#[test]
fn decrease_worker_stake_fails_with_not_set_lead() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 11;

        let decrease_stake_fixture =
            DecreaseWorkerStakeFixture::default_for_worker_id(invalid_worker_id);

        decrease_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::CurrentLeadNotSet.into(),
        ));
    });
}

#[test]
fn slash_worker_stake_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
          Events are not emitted on block 0.
          So any dispatchable calls made during genesis block formation will have no events emitted.
          https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let worker_id = fill_worker_position_with_stake(100);

        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(worker_id);

        slash_stake_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StakeSlashed(worker_id));
    });
}

#[test]
fn slash_leader_stake_succeeds() {
    build_test_externalities().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let leader_worker_id = HiringWorkflow::default()
            .with_role_stake(Some(100))
            .with_opening_type(OpeningType::Leader)
            .add_default_application()
            .execute()
            .unwrap();

        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(leader_worker_id)
            .with_origin(RawOrigin::Root);

        slash_stake_fixture.call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StakeSlashed(leader_worker_id));
    });
}

#[test]
fn slash_worker_stake_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        let invalid_worker_id = 22;
        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(invalid_worker_id)
            .with_origin(RawOrigin::None);

        slash_stake_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn slash_leader_stake_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let worker_id = HireLeadFixture::default().hire_lead();

        let slash_stake_fixture =
            SlashWorkerStakeFixture::default_for_worker_id(worker_id).with_origin(RawOrigin::None);

        slash_stake_fixture.call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn slash_worker_stake_fails_with_zero_balance() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_worker_position_with_stake(100);

        let slash_stake_fixture =
            SlashWorkerStakeFixture::default_for_worker_id(worker_id).with_balance(0);

        slash_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::StakeBalanceCannotBeZero.into(),
        ));
    });
}

#[test]
fn slash_worker_stake_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();
        let invalid_worker_id = 11;

        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(invalid_worker_id);

        slash_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::WorkerDoesNotExist.into(),
        ));
    });
}

#[test]
fn slash_worker_stake_fails_with_no_stake_profile() {
    build_test_externalities().execute_with(|| {
        let worker_id = fill_default_worker_position();

        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(worker_id);

        slash_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::NoWorkerStakeProfile.into(),
        ));
    });
}

#[test]
fn slash_worker_stake_fails_with_not_set_lead() {
    build_test_externalities().execute_with(|| {
        let invalid_worker_id = 11;

        let slash_stake_fixture = SlashWorkerStakeFixture::default_for_worker_id(invalid_worker_id);

        slash_stake_fixture.call_and_assert(Err(
            Error::<Test, TestWorkingGroupInstance>::CurrentLeadNotSet.into(),
        ));
    });
}

#[test]
fn get_regular_worker_ids_succeeds() {
    build_test_externalities().execute_with(|| {
        let worker_ids = TestWorkingGroup::get_regular_worker_ids();
        assert_eq!(worker_ids, Vec::<WorkerId<Test>>::new());

        let leader_worker_id = HireLeadFixture::default().hire_lead();

        let worker_id1 = fill_worker_position(None, None, false, OpeningType::Worker, None);
        let worker_id2 = fill_worker_position(None, None, false, OpeningType::Worker, None);

        let mut expected_ids = vec![worker_id1, worker_id2];
        expected_ids.sort();

        let mut worker_ids = TestWorkingGroup::get_regular_worker_ids();
        worker_ids.sort();
        assert_eq!(worker_ids, expected_ids);

        assert!(!expected_ids.contains(&leader_worker_id));

        <crate::WorkerById<Test, TestWorkingGroupInstance>>::remove(worker_id1);
        let worker_ids = TestWorkingGroup::get_regular_worker_ids();
        assert_eq!(worker_ids, vec![worker_id2]);
    });
}

#[test]
fn get_all_worker_ids_succeeds() {
    build_test_externalities().execute_with(|| {
        let worker_ids = TestWorkingGroup::get_all_worker_ids();
        assert_eq!(worker_ids, Vec::<WorkerId<Test>>::new());

        let leader_worker_id = HireLeadFixture::default().hire_lead();

        let worker_id1 = fill_worker_position(None, None, false, OpeningType::Worker, None);
        let worker_id2 = fill_worker_position(None, None, false, OpeningType::Worker, None);

        let mut expected_ids = vec![leader_worker_id, worker_id1, worker_id2];
        expected_ids.sort();

        let mut worker_ids = TestWorkingGroup::get_all_worker_ids();
        worker_ids.sort();
        assert_eq!(worker_ids, expected_ids);

        <crate::WorkerById<Test, TestWorkingGroupInstance>>::remove(worker_id1);
        let worker_ids = TestWorkingGroup::get_all_worker_ids();
        assert_eq!(worker_ids, vec![leader_worker_id, worker_id2]);
    });
}

#[test]
fn set_working_group_mint_capacity_succeeds() {
    build_test_externalities().execute_with(|| {
        let mint_id = <minting::Module<Test>>::add_mint(0, None).unwrap();
        <crate::Mint<Test, TestWorkingGroupInstance>>::put(mint_id);

        let capacity = 15000;
        let result = TestWorkingGroup::set_mint_capacity(RawOrigin::Root.into(), capacity);

        assert_eq!(result, Ok(()));

        let mint = <minting::Module<Test>>::mints(mint_id);
        assert_eq!(mint.capacity(), capacity);
    });
}

#[test]
fn set_working_group_mint_capacity_fails_with_mint_not_found() {
    build_test_externalities().execute_with(|| {
        let capacity = 15000;

        <crate::Mint<Test, TestWorkingGroupInstance>>::put(5); // random mint id
        let result = TestWorkingGroup::set_mint_capacity(RawOrigin::Root.into(), capacity);

        assert_eq!(
            result,
            Err(Error::<Test, TestWorkingGroupInstance>::CannotFindMint.into())
        );
    });
}

#[test]
fn set_working_group_mint_capacity_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        let capacity = 15000;
        let result = TestWorkingGroup::set_mint_capacity(RawOrigin::None.into(), capacity);

        assert_eq!(result, Err(DispatchError::BadOrigin));
    });
}

#[test]
fn ensure_setting_genesis_working_group_mint_succeeds() {
    build_test_externalities().execute_with(|| {
        let mint_id = TestWorkingGroup::mint();

        assert!(minting::Mints::<Test>::contains_key(mint_id));

        let mint = <minting::Module<Test>>::mints(mint_id);
        assert_eq!(mint.capacity(), WORKING_GROUP_MINT_CAPACITY);
    });
}

#[test]
fn ensure_setting_genesis_constraints_succeeds() {
    build_test_externalities().execute_with(|| {
        let default_constraint = common::constraints::InputValidationLengthConstraint::new(
            WORKING_GROUP_CONSTRAINT_MIN,
            WORKING_GROUP_CONSTRAINT_DIFF,
        );
        let opening_text_constraint = TestWorkingGroup::opening_human_readable_text();
        let worker_text_constraint = TestWorkingGroup::application_human_readable_text();
        let worker_exit_text_constraint = TestWorkingGroup::worker_exit_rationale_text();

        assert_eq!(opening_text_constraint, default_constraint);
        assert_eq!(worker_text_constraint, default_constraint);
        assert_eq!(worker_exit_text_constraint, default_constraint);
    });
}

#[test]
fn active_worker_counter_works_successfully() {
    build_test_externalities().execute_with(|| {
        assert_eq!(TestWorkingGroup::active_worker_count(), 0);

        let leader_id = HireLeadFixture::default().hire_lead();
        assert_eq!(TestWorkingGroup::active_worker_count(), 1);

        let worker_id1 = fill_worker_position(
            None,
            None,
            false,
            OpeningType::Worker,
            Some(b"worker1".to_vec()),
        );
        assert_eq!(TestWorkingGroup::active_worker_count(), 2);

        let worker_id2 = fill_worker_position(
            None,
            None,
            false,
            OpeningType::Worker,
            Some(b"worker1".to_vec()),
        );
        assert_eq!(TestWorkingGroup::active_worker_count(), 3);

        TerminateWorkerRoleFixture::default_for_worker_id(worker_id1).call_and_assert(Ok(()));
        assert_eq!(TestWorkingGroup::active_worker_count(), 2);

        TerminateWorkerRoleFixture::default_for_worker_id(worker_id2).call_and_assert(Ok(()));
        assert_eq!(TestWorkingGroup::active_worker_count(), 1);

        TerminateWorkerRoleFixture::default_for_worker_id(leader_id)
            .with_origin(RawOrigin::Root)
            .call_and_assert(Ok(()));
        assert_eq!(TestWorkingGroup::active_worker_count(), 0);
    });
}

#[test]
fn adding_too_much_workers_fails_with_single_application_out_of_limit() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        fill_worker_position(None, None, false, OpeningType::Worker, None);
        fill_worker_position(None, None, false, OpeningType::Worker, None);

        let hiring_workflow = HiringWorkflow::default()
            .disable_setup_environment()
            .add_default_application()
            .expect(Err(
                Error::<Test, TestWorkingGroupInstance>::MaxActiveWorkerNumberExceeded.into(),
            ));

        hiring_workflow.execute()
    });
}

#[test]
fn fill_opening_cannot_hire_more_workers_using_several_applicationst_han_allows_worker_limit() {
    build_test_externalities().execute_with(|| {
        HireLeadFixture::default().hire_lead();

        fill_worker_position(None, None, false, OpeningType::Worker, None);

        let hiring_workflow = HiringWorkflow::default()
            .disable_setup_environment()
            .add_application_with_origin(b"Some1".to_vec(), RawOrigin::Signed(2), 2)
            .add_application_with_origin(b"Some2".to_vec(), RawOrigin::Signed(3), 3)
            .expect(Err(
                Error::<Test, TestWorkingGroupInstance>::MaxActiveWorkerNumberExceeded.into(),
            ));

        hiring_workflow.execute()
    });
}
