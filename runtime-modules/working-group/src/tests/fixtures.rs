use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::storage::{StorageMap, StorageValue};
use frame_system::{EventRecord, Phase, RawOrigin};
use std::collections::BTreeSet;

use super::mock::{
    Balances, Event, Membership, System, Test, TestWorkingGroup, TestWorkingGroupInstance,
};
use crate::tests::fill_worker_position;
use crate::types::{
    Application, Opening, OpeningPolicyCommitment, OpeningType, RewardPolicy, RoleStakeProfile,
    Worker,
};
use crate::RawEvent;
use common::constraints::InputValidationLengthConstraint;

pub struct IncreaseWorkerStakeFixture {
    origin: RawOrigin<u64>,
    worker_id: u64,
    balance: u64,
    account_id: u64,
}

impl IncreaseWorkerStakeFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        let account_id = 1;
        Self {
            origin: RawOrigin::Signed(1),
            worker_id,
            balance: 10,
            account_id,
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_balance(self, balance: u64) -> Self {
        Self { balance, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let stake_id = 0;
        let old_stake = <stake::Module<Test>>::stakes(stake_id);
        let old_balance = Balances::free_balance(&self.account_id);
        let actual_result = TestWorkingGroup::increase_stake(
            self.origin.clone().into(),
            self.worker_id,
            self.balance,
        );

        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            let new_stake = <stake::Module<Test>>::stakes(stake_id);

            // stake increased
            assert_eq!(
                get_stake_balance(new_stake),
                get_stake_balance(old_stake) + self.balance
            );

            let new_balance = Balances::free_balance(&self.account_id);

            // worker balance decreased
            assert_eq!(new_balance, old_balance - self.balance,);
        }
    }
}

pub struct TerminateWorkerRoleFixture {
    worker_id: u64,
    origin: RawOrigin<u64>,
    text: Vec<u8>,
    constraint: InputValidationLengthConstraint,
    slash_stake: bool,
}

impl TerminateWorkerRoleFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        Self {
            worker_id,
            origin: RawOrigin::Signed(1),
            text: b"rationale_text".to_vec(),
            constraint: InputValidationLengthConstraint {
                min: 1,
                max_min_diff: 20,
            },
            slash_stake: false,
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_text(self, text: Vec<u8>) -> Self {
        Self { text, ..self }
    }

    pub fn with_slashing(self) -> Self {
        Self {
            slash_stake: true,
            ..self
        }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        <crate::WorkerExitRationaleText<TestWorkingGroupInstance>>::put(self.constraint.clone());

        let actual_result = TestWorkingGroup::terminate_role(
            self.origin.clone().into(),
            self.worker_id,
            self.text.clone(),
            self.slash_stake,
        );
        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            if actual_result.is_ok() {
                assert!(
                    !<crate::WorkerById<Test, TestWorkingGroupInstance>>::contains_key(
                        self.worker_id
                    )
                );
            }
        }
    }
}

pub(crate) struct LeaveWorkerRoleFixture {
    worker_id: u64,
    origin: RawOrigin<u64>,
}

impl LeaveWorkerRoleFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        Self {
            worker_id,
            origin: RawOrigin::Signed(1),
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let rationale_text = b"rationale_text".to_vec();
        let actual_result = TestWorkingGroup::leave_role(
            self.origin.clone().into(),
            self.worker_id,
            rationale_text.clone(),
        );
        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            assert!(
                !<crate::WorkerById<Test, TestWorkingGroupInstance>>::contains_key(self.worker_id)
            );
        }
    }
}

pub struct UpdateWorkerRewardAmountFixture {
    worker_id: u64,
    amount: u64,
    origin: RawOrigin<u64>,
}

impl UpdateWorkerRewardAmountFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        let lead_account_id = get_current_lead_account_id();

        Self {
            worker_id,
            amount: 120,
            origin: RawOrigin::Signed(lead_account_id),
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::update_reward_amount(
            self.origin.clone().into(),
            self.worker_id,
            self.amount,
        );

        assert_eq!(actual_result.clone(), expected_result);

        if actual_result.is_ok() {
            let worker = TestWorkingGroup::worker_by_id(self.worker_id);
            let relationship_id = worker.reward_relationship.unwrap();

            let relationship = recurringrewards::RewardRelationships::<Test>::get(relationship_id);

            assert_eq!(relationship.amount_per_payout, self.amount);
        }
    }
}
pub struct UpdateWorkerRewardAccountFixture {
    worker_id: u64,
    new_reward_account_id: u64,
    origin: RawOrigin<u64>,
}

impl UpdateWorkerRewardAccountFixture {
    pub fn default_with_ids(worker_id: u64, new_reward_account_id: u64) -> Self {
        Self {
            worker_id,
            new_reward_account_id,
            origin: RawOrigin::Signed(1),
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::update_reward_account(
            self.origin.clone().into(),
            self.worker_id,
            self.new_reward_account_id,
        );

        assert_eq!(actual_result.clone(), expected_result);

        if actual_result.is_ok() {
            let worker = TestWorkingGroup::worker_by_id(self.worker_id);
            let relationship_id = worker.reward_relationship.unwrap();

            let relationship = recurringrewards::RewardRelationships::<Test>::get(relationship_id);

            assert_eq!(relationship.account, self.new_reward_account_id);
        }
    }
}

pub struct UpdateWorkerRoleAccountFixture {
    worker_id: u64,
    new_role_account_id: u64,
    origin: RawOrigin<u64>,
}

impl UpdateWorkerRoleAccountFixture {
    pub fn default_with_ids(worker_id: u64, new_role_account_id: u64) -> Self {
        Self {
            worker_id,
            new_role_account_id,
            origin: RawOrigin::Signed(1),
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::update_role_account(
            self.origin.clone().into(),
            self.worker_id,
            self.new_role_account_id,
        );
        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            let worker = TestWorkingGroup::worker_by_id(self.worker_id);

            assert_eq!(worker.role_account_id, self.new_role_account_id);
        }
    }
}

pub struct UpdateWorkerStorageFixture {
    worker_id: u64,
    storage_field: Vec<u8>,
    origin: RawOrigin<u64>,
}

impl UpdateWorkerStorageFixture {
    pub fn default_with_storage_field(worker_id: u64, storage_field: Vec<u8>) -> Self {
        Self {
            worker_id,
            storage_field,
            origin: RawOrigin::Signed(1),
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::update_role_storage(
            self.origin.clone().into(),
            self.worker_id,
            self.storage_field.clone(),
        );
        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            let storage = TestWorkingGroup::worker_storage(self.worker_id);

            assert_eq!(storage, self.storage_field);
        }
    }
}

pub fn set_mint_id(mint_id: u64) {
    <crate::Mint<Test, TestWorkingGroupInstance>>::put(mint_id);
}

pub fn create_mint() -> u64 {
    <minting::Module<Test>>::add_mint(100, None).unwrap()
}

pub struct FillWorkerOpeningFixture {
    origin: RawOrigin<u64>,
    opening_id: u64,
    successful_application_ids: BTreeSet<u64>,
    role_account_id: u64,
    reward_policy: Option<RewardPolicy<u64, u64>>,
}

impl FillWorkerOpeningFixture {
    pub fn default_for_ids(opening_id: u64, application_ids: Vec<u64>) -> Self {
        let application_ids: BTreeSet<u64> = application_ids.iter().map(|x| *x).collect();

        Self {
            origin: RawOrigin::Signed(1),
            opening_id,
            successful_application_ids: application_ids,
            role_account_id: 1,
            reward_policy: None,
        }
    }

    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_reward_policy(self, reward_policy: RewardPolicy<u64, u64>) -> Self {
        Self {
            reward_policy: Some(reward_policy),
            ..self
        }
    }

    pub fn call(&self) -> Result<u64, DispatchError> {
        let saved_worker_next_id = TestWorkingGroup::next_worker_id();
        TestWorkingGroup::fill_opening(
            self.origin.clone().into(),
            self.opening_id,
            self.successful_application_ids.clone(),
            self.reward_policy.clone(),
        )?;

        Ok(saved_worker_next_id)
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) -> u64 {
        let saved_worker_next_id = TestWorkingGroup::next_worker_id();
        let actual_result = self.call().map(|_| ());
        assert_eq!(actual_result.clone(), expected_result);

        if actual_result.is_ok() {
            assert_eq!(TestWorkingGroup::next_worker_id(), saved_worker_next_id + 1);
            let worker_id = saved_worker_next_id;

            let opening = TestWorkingGroup::opening_by_id(self.opening_id);

            let role_stake_profile = if opening
                .policy_commitment
                .application_staking_policy
                .is_some()
                || opening.policy_commitment.role_staking_policy.is_some()
            {
                let stake_id = 0;
                Some(RoleStakeProfile::new(
                    &stake_id,
                    &opening
                        .policy_commitment
                        .terminate_role_stake_unstaking_period,
                    &opening.policy_commitment.exit_role_stake_unstaking_period,
                ))
            } else {
                None
            };
            let reward_relationship = self.reward_policy.clone().map(|_| 0);

            let expected_worker = Worker {
                member_id: 1,
                role_account_id: self.role_account_id,
                reward_relationship,
                role_stake_profile,
            };

            let actual_worker = TestWorkingGroup::worker_by_id(worker_id);

            assert_eq!(actual_worker, expected_worker);
        }

        saved_worker_next_id
    }
}

pub struct BeginReviewWorkerApplicationsFixture {
    origin: RawOrigin<u64>,
    opening_id: u64,
}

impl BeginReviewWorkerApplicationsFixture {
    pub fn default_for_opening_id(opening_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            opening_id,
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }
    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result =
            TestWorkingGroup::begin_applicant_review(self.origin.clone().into(), self.opening_id);
        assert_eq!(actual_result, expected_result);
    }
}

pub struct TerminateApplicationFixture {
    origin: RawOrigin<u64>,
    worker_application_id: u64,
}

impl TerminateApplicationFixture {
    pub fn with_signer(self, account_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(account_id),
            ..self
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }
    pub fn default_for_application_id(application_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            worker_application_id: application_id,
        }
    }
    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::terminate_application(
            self.origin.clone().into(),
            self.worker_application_id,
        );
        assert_eq!(actual_result.clone(), expected_result);
    }
}
pub struct WithdrawApplicationFixture {
    origin: RawOrigin<u64>,
    worker_application_id: u64,
}

impl WithdrawApplicationFixture {
    pub fn with_signer(self, account_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(account_id),
            ..self
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }
    pub fn default_for_application_id(application_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            worker_application_id: application_id,
        }
    }
    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result = TestWorkingGroup::withdraw_application(
            self.origin.clone().into(),
            self.worker_application_id,
        );
        assert_eq!(actual_result.clone(), expected_result);
    }
}

pub fn increase_total_balance_issuance_using_account_id(account_id: u64, balance: u64) {
    let _ =
        <Balances as frame_support::traits::Currency<u64>>::deposit_creating(&account_id, balance);
}

pub fn get_balance(account_id: u64) -> u64 {
    <super::mock::Balances as frame_support::traits::Currency<u64>>::total_balance(&account_id)
}

pub fn setup_members(count: u8) {
    let authority_account_id = 1;
    Membership::set_screening_authority(RawOrigin::Root.into(), authority_account_id).unwrap();

    for i in 0..count {
        let account_id: u64 = i as u64;
        let handle: [u8; 20] = [i; 20];
        Membership::add_screened_member(
            RawOrigin::Signed(authority_account_id).into(),
            account_id,
            Some(handle.to_vec()),
            None,
            None,
            None,
        )
        .unwrap();
    }
}

pub struct ApplyOnWorkerOpeningFixture {
    origin: RawOrigin<u64>,
    member_id: u64,
    worker_opening_id: u64,
    role_account_id: u64,
    opt_role_stake_balance: Option<u64>,
    opt_application_stake_balance: Option<u64>,
    human_readable_text: Vec<u8>,
}

impl ApplyOnWorkerOpeningFixture {
    pub fn with_text(self, text: Vec<u8>) -> Self {
        Self {
            human_readable_text: text,
            ..self
        }
    }

    pub fn with_origin(self, origin: RawOrigin<u64>, member_id: u64) -> Self {
        Self {
            origin,
            member_id,
            ..self
        }
    }

    pub fn with_role_stake(self, stake: Option<u64>) -> Self {
        Self {
            opt_role_stake_balance: stake,
            ..self
        }
    }

    pub fn with_application_stake(self, stake: u64) -> Self {
        Self {
            opt_application_stake_balance: Some(stake),
            ..self
        }
    }

    pub fn default_for_opening_id(opening_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            member_id: 1,
            worker_opening_id: opening_id,
            role_account_id: 1,
            opt_role_stake_balance: None,
            opt_application_stake_balance: None,
            human_readable_text: b"human_text".to_vec(),
        }
    }

    pub fn call(&self) -> Result<u64, DispatchError> {
        let saved_application_next_id = TestWorkingGroup::next_application_id();
        TestWorkingGroup::apply_on_opening(
            self.origin.clone().into(),
            self.member_id,
            self.worker_opening_id,
            self.role_account_id,
            self.opt_role_stake_balance,
            self.opt_application_stake_balance,
            self.human_readable_text.clone(),
        )?;

        Ok(saved_application_next_id)
    }
    pub fn call_and_assert(&self, expected_result: DispatchResult) -> u64 {
        let saved_application_next_id = TestWorkingGroup::next_application_id();

        let actual_result = self.call().map(|_| ());
        assert_eq!(actual_result.clone(), expected_result);

        if actual_result.is_ok() {
            assert_eq!(
                TestWorkingGroup::next_application_id(),
                saved_application_next_id + 1
            );
            let application_id = saved_application_next_id;

            let actual_application = TestWorkingGroup::application_by_id(application_id);

            let expected_application = Application {
                role_account_id: self.role_account_id,
                opening_id: self.worker_opening_id,
                member_id: self.member_id,
                hiring_application_id: application_id,
            };

            assert_eq!(actual_application, expected_application);

            let current_opening = TestWorkingGroup::opening_by_id(self.worker_opening_id);
            assert!(current_opening.applications.contains(&application_id));
        }

        saved_application_next_id
    }
}

pub struct AcceptWorkerApplicationsFixture {
    origin: RawOrigin<u64>,
    opening_id: u64,
}

impl AcceptWorkerApplicationsFixture {
    pub fn default_for_opening_id(opening_id: u64) -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            opening_id,
        }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let actual_result =
            TestWorkingGroup::accept_applications(self.origin.clone().into(), self.opening_id);
        assert_eq!(actual_result, expected_result);
    }
}

pub struct SetLeadFixture {
    pub member_id: u64,
    pub role_account_id: u64,
    pub worker_id: u64,
}
impl Default for SetLeadFixture {
    fn default() -> Self {
        SetLeadFixture {
            member_id: 1,
            role_account_id: 1,
            worker_id: 1,
        }
    }
}

impl SetLeadFixture {
    pub fn unset_lead() {
        TestWorkingGroup::unset_lead();
    }

    pub fn set_lead(self) {
        TestWorkingGroup::set_lead(self.worker_id);
    }
    pub fn set_lead_with_ids(member_id: u64, role_account_id: u64, worker_id: u64) {
        Self {
            member_id,
            role_account_id,
            worker_id,
        }
        .set_lead();
    }
}

pub struct HireLeadFixture {
    setup_environment: bool,
    stake: Option<u64>,
    reward_policy: Option<RewardPolicy<u64, u64>>,
}

impl Default for HireLeadFixture {
    fn default() -> Self {
        Self {
            setup_environment: true,
            stake: None,
            reward_policy: None,
        }
    }
}
impl HireLeadFixture {
    pub fn with_stake(self, stake: u64) -> Self {
        Self {
            stake: Some(stake),
            ..self
        }
    }
    pub fn with_reward_policy(self, reward_policy: RewardPolicy<u64, u64>) -> Self {
        Self {
            reward_policy: Some(reward_policy),
            ..self
        }
    }

    pub fn hire_lead(self) -> u64 {
        fill_worker_position(
            self.reward_policy,
            self.stake,
            self.setup_environment,
            OpeningType::Leader,
            Some(b"leader".to_vec()),
        )
    }
}

pub fn get_worker_by_id(worker_id: u64) -> Worker<u64, u64, u64, u64, u64> {
    TestWorkingGroup::worker_by_id(worker_id)
}

pub struct AddWorkerOpeningFixture {
    origin: RawOrigin<u64>,
    activate_at: hiring::ActivateOpeningAt<u64>,
    commitment: OpeningPolicyCommitment<u64, u64>,
    human_readable_text: Vec<u8>,
    opening_type: OpeningType,
}

impl Default for AddWorkerOpeningFixture {
    fn default() -> Self {
        Self {
            origin: RawOrigin::Signed(1),
            activate_at: hiring::ActivateOpeningAt::CurrentBlock,
            commitment: <OpeningPolicyCommitment<u64, u64>>::default(),
            human_readable_text: b"human_text".to_vec(),
            opening_type: OpeningType::Worker,
        }
    }
}

impl AddWorkerOpeningFixture {
    pub fn with_policy_commitment(
        self,
        policy_commitment: OpeningPolicyCommitment<u64, u64>,
    ) -> Self {
        Self {
            commitment: policy_commitment,
            ..self
        }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) -> u64 {
        let saved_opening_next_id = TestWorkingGroup::next_opening_id();
        let actual_result = self.call().map(|_| ());

        assert_eq!(actual_result.clone(), expected_result);

        if actual_result.is_ok() {
            assert_eq!(
                TestWorkingGroup::next_opening_id(),
                saved_opening_next_id + 1
            );
            let opening_id = saved_opening_next_id;

            let actual_opening = TestWorkingGroup::opening_by_id(opening_id);

            let expected_opening = Opening::<u64, u64, u64, u64> {
                hiring_opening_id: opening_id,
                applications: BTreeSet::new(),
                policy_commitment: self.commitment.clone(),
                opening_type: self.opening_type,
            };

            assert_eq!(actual_opening, expected_opening);
        }

        saved_opening_next_id
    }

    pub fn call(&self) -> Result<u64, DispatchError> {
        let saved_opening_next_id = TestWorkingGroup::next_opening_id();
        TestWorkingGroup::add_opening(
            self.origin.clone().into(),
            self.activate_at.clone(),
            self.commitment.clone(),
            self.human_readable_text.clone(),
            self.opening_type,
        )?;

        Ok(saved_opening_next_id)
    }

    pub fn with_text(self, text: Vec<u8>) -> Self {
        Self {
            human_readable_text: text,
            ..self
        }
    }

    pub fn with_opening_type(self, opening_type: OpeningType) -> Self {
        Self {
            opening_type,
            ..self
        }
    }

    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_activate_at(self, activate_at: hiring::ActivateOpeningAt<u64>) -> Self {
        Self {
            activate_at,
            ..self
        }
    }
}

pub struct EventFixture;
impl EventFixture {
    pub fn assert_last_crate_event(
        expected_raw_event: RawEvent<
            u64,
            u64,
            u64,
            u64,
            std::collections::BTreeMap<u64, u64>,
            Vec<u8>,
            u64,
            u64,
            TestWorkingGroupInstance,
        >,
    ) {
        let converted_event = Event::working_group_Instance1(expected_raw_event);

        Self::assert_last_global_event(converted_event)
    }

    pub fn assert_last_global_event(expected_event: Event) {
        let expected_event = EventRecord {
            phase: Phase::Initialization,
            event: expected_event,
            topics: vec![],
        };

        assert_eq!(System::events().pop().unwrap(), expected_event);
    }
}

pub struct DecreaseWorkerStakeFixture {
    origin: RawOrigin<u64>,
    worker_id: u64,
    balance: u64,
    account_id: u64,
}

impl DecreaseWorkerStakeFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        let account_id = 1;

        let lead_account_id = get_current_lead_account_id();

        Self {
            origin: RawOrigin::Signed(lead_account_id),
            worker_id,
            balance: 10,
            account_id,
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_balance(self, balance: u64) -> Self {
        Self { balance, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let stake_id = 0;
        let old_balance = Balances::free_balance(&self.account_id);
        let old_stake = <stake::Module<Test>>::stakes(stake_id);
        let actual_result = TestWorkingGroup::decrease_stake(
            self.origin.clone().into(),
            self.worker_id,
            self.balance,
        );

        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            let new_stake = <stake::Module<Test>>::stakes(stake_id);

            // stake decreased
            assert_eq!(
                get_stake_balance(new_stake),
                get_stake_balance(old_stake) - self.balance
            );

            let new_balance = Balances::free_balance(&self.account_id);

            // worker balance increased
            assert_eq!(new_balance, old_balance + self.balance,);
        }
    }
}

pub(crate) fn get_stake_balance(stake: stake::Stake<u64, u64, u64>) -> u64 {
    if let stake::StakingStatus::Staked(stake) = stake.staking_status {
        return stake.staked_amount;
    }

    panic!("Not staked.");
}

fn get_current_lead_account_id() -> u64 {
    let leader_worker_id = TestWorkingGroup::current_lead();

    if let Some(leader_worker_id) = leader_worker_id {
        let leader = TestWorkingGroup::worker_by_id(leader_worker_id);
        leader.role_account_id
    } else {
        0 // return invalid lead_account_id for testing
    }
}

pub struct SlashWorkerStakeFixture {
    origin: RawOrigin<u64>,
    worker_id: u64,
    balance: u64,
    account_id: u64,
}

impl SlashWorkerStakeFixture {
    pub fn default_for_worker_id(worker_id: u64) -> Self {
        let account_id = 1;

        let lead_account_id = get_current_lead_account_id();

        Self {
            origin: RawOrigin::Signed(lead_account_id),
            worker_id,
            balance: 10,
            account_id,
        }
    }
    pub fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        Self { origin, ..self }
    }

    pub fn with_balance(self, balance: u64) -> Self {
        Self { balance, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let stake_id = 0;
        let old_balance = Balances::free_balance(&self.account_id);
        let old_stake = <stake::Module<Test>>::stakes(stake_id);
        let actual_result =
            TestWorkingGroup::slash_stake(self.origin.clone().into(), self.worker_id, self.balance);

        assert_eq!(actual_result, expected_result);

        if actual_result.is_ok() {
            let new_stake = <stake::Module<Test>>::stakes(stake_id);

            // stake decreased
            assert_eq!(
                get_stake_balance(new_stake),
                get_stake_balance(old_stake) - self.balance
            );

            let new_balance = Balances::free_balance(&self.account_id);

            // worker balance unchanged
            assert_eq!(new_balance, old_balance,);
        }
    }
}
