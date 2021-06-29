mod mock;

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::storage::StorageMap;
use frame_support::traits::Currency;
use frame_system::RawOrigin;

use common::working_group::WorkingGroup;
use governance::election_params::ElectionParameters;
use hiring::ActivateOpeningAt;
use proposals_engine::ProposalParameters;
use working_group::OpeningPolicyCommitment;

use crate::proposal_types::ProposalsConfigParameters;
use crate::*;
use crate::{BalanceOf, Error, ProposalDetails};
pub use mock::*;

use strum::IntoEnumIterator;

pub(crate) fn increase_total_balance_issuance(balance: u64) {
    increase_total_balance_issuance_using_account_id(999, balance);
}

pub(crate) fn increase_total_balance_issuance_using_account_id(account_id: u64, balance: u64) {
    let initial_balance = Balances::total_issuance();
    {
        let _ = <Test as stake::Config>::Currency::deposit_creating(&account_id, balance);
    }
    assert_eq!(Balances::total_issuance(), initial_balance + balance);
}

struct ProposalTestFixture<InsufficientRightsCall, EmptyStakeCall, InvalidStakeCall, SuccessfulCall>
where
    InsufficientRightsCall: Fn() -> DispatchResult,
    EmptyStakeCall: Fn() -> DispatchResult,
    InvalidStakeCall: Fn() -> DispatchResult,
    SuccessfulCall: Fn() -> DispatchResult,
{
    insufficient_rights_call: InsufficientRightsCall,
    empty_stake_call: EmptyStakeCall,
    invalid_stake_call: InvalidStakeCall,
    successful_call: SuccessfulCall,
    proposal_parameters: ProposalParameters<u64, u64>,
    proposal_details: ProposalDetails<u64, u64, u64, u64, u64, u64, u64, u64, u64>,
}

impl<InsufficientRightsCall, EmptyStakeCall, InvalidStakeCall, SuccessfulCall>
    ProposalTestFixture<InsufficientRightsCall, EmptyStakeCall, InvalidStakeCall, SuccessfulCall>
where
    InsufficientRightsCall: Fn() -> DispatchResult,
    EmptyStakeCall: Fn() -> DispatchResult,
    InvalidStakeCall: Fn() -> DispatchResult,
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn check_for_invalid_stakes(&self) {
        assert_eq!(
            (self.empty_stake_call)(),
            Err(proposals_engine::Error::<Test>::EmptyStake.into())
        );

        assert_eq!(
            (self.invalid_stake_call)(),
            Err(proposals_engine::Error::<Test>::StakeDiffersFromRequired.into())
        );
    }

    fn check_call_for_insufficient_rights(&self) {
        assert_eq!(
            (self.insufficient_rights_call)(),
            Err(DispatchError::Other("Bad origin"))
        );
    }

    fn check_for_insufficient_balance(&self) {
        assert_eq!(
            (self.successful_call)(),
            Err(proposals_engine::Error::<Test>::InsufficientBalance.into())
        );
    }

    fn check_for_successful_call(&self) {
        let account_id = 1;
        let _imbalance =
            <Test as stake::Config>::Currency::deposit_creating(&account_id, 5_000_000);

        assert_eq!((self.successful_call)(), Ok(()));

        // a discussion was created
        let thread_id = <crate::ThreadIdByProposalId<Test>>::get(1);
        assert_eq!(thread_id, 1);

        let proposal_id = 1;
        let proposal = ProposalsEngine::proposals(proposal_id);
        // check for correct proposal parameters
        assert_eq!(proposal.parameters, self.proposal_parameters);

        // proposal details was set
        let details = <crate::ProposalDetailsByProposalId<Test>>::get(proposal_id);
        assert_eq!(details, self.proposal_details);
    }

    pub fn check_all(&self) {
        self.check_call_for_insufficient_rights();
        self.check_for_invalid_stakes();
        self.check_for_insufficient_balance();
        self.check_for_successful_call();
    }
}

#[test]
fn create_text_proposal_common_checks_succeed() {
    initial_test_ext().execute_with(|| {
        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_text_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    b"text".to_vec(),
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_text_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    b"text".to_vec(),
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_text_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    b"text".to_vec(),
                )
            },
            successful_call: || {
                ProposalsCodex::create_text_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(25000u32)),
                    b"text".to_vec(),
                )
            },
            proposal_parameters: crate::proposal_types::parameters::text_proposal::<Test>(),
            proposal_details: ProposalDetails::Text(b"text".to_vec()),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_text_proposal_codex_call_fails_with_incorrect_text_size() {
    initial_test_ext().execute_with(|| {
        let origin = RawOrigin::Signed(1).into();

        let long_text = [0u8; 30000].to_vec();
        assert_eq!(
            ProposalsCodex::create_text_proposal(
                origin,
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                None,
                long_text,
            ),
            Err(Error::<Test>::TextProposalSizeExceeded.into())
        );

        assert_eq!(
            ProposalsCodex::create_text_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                None,
                Vec::new(),
            ),
            Err(Error::<Test>::TextProposalIsEmpty.into())
        );
    });
}

#[test]
fn create_runtime_upgrade_common_checks_succeed() {
    initial_test_ext().execute_with(|| {
        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_runtime_upgrade_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    b"wasm".to_vec(),
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_runtime_upgrade_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    b"wasm".to_vec(),
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_runtime_upgrade_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(500u32)),
                    b"wasm".to_vec(),
                )
            },
            successful_call: || {
                ProposalsCodex::create_runtime_upgrade_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5_000_000_u32)),
                    b"wasm".to_vec(),
                )
            },
            proposal_parameters: crate::proposal_types::parameters::runtime_upgrade_proposal::<Test>(),
            proposal_details: ProposalDetails::RuntimeUpgrade(b"wasm".to_vec()),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_upgrade_runtime_proposal_codex_call_fails_with_incorrect_wasm_size() {
    initial_test_ext().execute_with(|| {
        let origin = RawOrigin::Signed(1).into();

        let long_wasm = [0u8; 30000].to_vec();
        assert_eq!(
            ProposalsCodex::create_runtime_upgrade_proposal(
                origin,
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                None,
                long_wasm,
            ),
            Err(Error::<Test>::RuntimeProposalSizeExceeded.into())
        );

        assert_eq!(
            ProposalsCodex::create_runtime_upgrade_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                None,
                Vec::new(),
            ),
            Err(Error::<Test>::RuntimeProposalIsEmpty.into())
        );
    });
}

#[test]
fn create_set_election_parameters_proposal_common_checks_succeed() {
    initial_test_ext().execute_with(|| {
        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_set_election_parameters_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    get_valid_election_parameters(),
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_set_election_parameters_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    get_valid_election_parameters(),
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_set_election_parameters_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    get_valid_election_parameters(),
                )
            },
            successful_call: || {
                ProposalsCodex::create_set_election_parameters_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(1_000_000_u32)),
                    get_valid_election_parameters(),
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::set_election_parameters_proposal::<Test>(),
            proposal_details: ProposalDetails::SetElectionParameters(
                get_valid_election_parameters(),
            ),
        };
        proposal_fixture.check_all();
    });
}

fn assert_failed_election_parameters_call(
    election_parameters: ElectionParameters<u64, u64>,
    error: DispatchError,
) {
    assert_eq!(
        ProposalsCodex::create_set_election_parameters_proposal(
            RawOrigin::Signed(1).into(),
            1,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Test>>::from(3750u32)),
            election_parameters,
        ),
        Err(error)
    );
}

fn get_valid_election_parameters() -> ElectionParameters<u64, u64> {
    ElectionParameters {
        announcing_period: 14400,
        voting_period: 14400,
        revealing_period: 14400,
        council_size: 6,
        candidacy_limit: 50,
        new_term_duration: 14400,
        min_council_stake: 1,
        min_voting_stake: 1,
    }
}

#[test]
fn create_set_election_parameters_call_fails_with_incorrect_parameters() {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 1500000);

        let mut election_parameters = get_valid_election_parameters();
        election_parameters.council_size = 2;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterCouncilSize.into(),
        );

        election_parameters.council_size = 41;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterCouncilSize.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.candidacy_limit = 22;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterCandidacyLimit.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.candidacy_limit = 222;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterCandidacyLimit.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.min_voting_stake = 0;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterMinVotingStake.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.min_voting_stake = 200000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterMinVotingStake.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.new_term_duration = 0;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterNewTermDuration.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.new_term_duration = 500000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterNewTermDuration.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.min_council_stake = 0;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterMinCouncilStake.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.min_council_stake = 200000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterMinCouncilStake.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.voting_period = 10000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterVotingPeriod.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.voting_period = 50000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterVotingPeriod.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.revealing_period = 10000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterRevealingPeriod.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.revealing_period = 50000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterRevealingPeriod.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.announcing_period = 10000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterAnnouncingPeriod.into(),
        );

        election_parameters = get_valid_election_parameters();
        election_parameters.announcing_period = 50000;
        assert_failed_election_parameters_call(
            election_parameters,
            Error::<Test>::InvalidCouncilElectionParameterAnnouncingPeriod.into(),
        );
    });
}

#[test]
fn create_spending_proposal_common_checks_succeed() {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance(500000);

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_spending_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    20,
                    10,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_spending_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    20,
                    10,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_spending_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    20,
                    10,
                )
            },
            successful_call: || {
                ProposalsCodex::create_spending_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(25000u32)),
                    100,
                    2,
                )
            },
            proposal_parameters: crate::proposal_types::parameters::spending_proposal::<Test>(),
            proposal_details: ProposalDetails::Spending(100, 2),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_spending_proposal_call_fails_with_incorrect_balance() {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(500000, 1);

        assert_eq!(
            ProposalsCodex::create_spending_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(1250u32)),
                0,
                2,
            ),
            Err(Error::<Test>::InvalidSpendingProposalBalance.into())
        );

        assert_eq!(
            ProposalsCodex::create_spending_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(1250u32)),
                50000001,
                2,
            ),
            Err(Error::<Test>::InvalidSpendingProposalBalance.into())
        );
    });
}

#[test]
#[cfg(feature = "standalone")]
fn create_set_validator_count_proposal_common_checks_succeed() {
    initial_test_ext().execute_with(|| {
        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_set_validator_count_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    4,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_set_validator_count_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    4,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_set_validator_count_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    4,
                )
            },
            successful_call: || {
                ProposalsCodex::create_set_validator_count_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(500_000_u32)),
                    4,
                )
            },
            proposal_parameters: crate::proposal_types::parameters::set_validator_count_proposal::<
                Test,
            >(),
            proposal_details: ProposalDetails::SetValidatorCount(4),
        };
        proposal_fixture.check_all();
    });
}

#[test]
#[cfg(feature = "standalone")]
fn create_set_validator_count_proposal_failed_with_invalid_validator_count() {
    initial_test_ext().execute_with(|| {
        staking::MinimumValidatorCount::put(10);

        assert_eq!(
            ProposalsCodex::create_set_validator_count_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(500u32)),
                3,
            ),
            Err(Error::<Test>::InvalidValidatorCount.into())
        );

        assert_eq!(
            ProposalsCodex::create_set_validator_count_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(1001u32)),
                3,
            ),
            Err(Error::<Test>::InvalidValidatorCount.into())
        );
    });
}

#[test]
fn set_default_proposal_parameters_succeeded() {
    initial_test_ext().execute_with(|| {
        let p = ProposalsConfigParameters::default();

        // nothing is set
        assert_eq!(<SetValidatorCountProposalVotingPeriod<Test>>::get(), 0);

        ProposalsCodex::set_config_values(p);

        assert_eq!(
            <SetValidatorCountProposalVotingPeriod<Test>>::get(),
            p.set_validator_count_proposal_voting_period as u64
        );
        assert_eq!(
            <SetValidatorCountProposalGracePeriod<Test>>::get(),
            p.set_validator_count_proposal_grace_period as u64
        );
        assert_eq!(
            <RuntimeUpgradeProposalVotingPeriod<Test>>::get(),
            p.runtime_upgrade_proposal_voting_period as u64
        );
        assert_eq!(
            <RuntimeUpgradeProposalGracePeriod<Test>>::get(),
            p.runtime_upgrade_proposal_grace_period as u64
        );
        assert_eq!(
            <TextProposalVotingPeriod<Test>>::get(),
            p.text_proposal_voting_period as u64
        );
        assert_eq!(
            <TextProposalGracePeriod<Test>>::get(),
            p.text_proposal_grace_period as u64
        );
        assert_eq!(
            <SetElectionParametersProposalVotingPeriod<Test>>::get(),
            p.set_election_parameters_proposal_voting_period as u64
        );
        assert_eq!(
            <SetElectionParametersProposalGracePeriod<Test>>::get(),
            p.set_election_parameters_proposal_grace_period as u64
        );
        assert_eq!(
            <SpendingProposalVotingPeriod<Test>>::get(),
            p.spending_proposal_voting_period as u64
        );
        assert_eq!(
            <SpendingProposalGracePeriod<Test>>::get(),
            p.spending_proposal_grace_period as u64
        );
        assert_eq!(
            <AddWorkingGroupOpeningProposalVotingPeriod<Test>>::get(),
            p.add_working_group_opening_proposal_voting_period as u64
        );
        assert_eq!(
            <AddWorkingGroupOpeningProposalGracePeriod<Test>>::get(),
            p.add_working_group_opening_proposal_grace_period as u64
        );
        assert_eq!(
            <BeginReviewWorkingGroupLeaderApplicationsProposalVotingPeriod<Test>>::get(),
            p.begin_review_working_group_leader_applications_proposal_voting_period as u64
        );
        assert_eq!(
            <BeginReviewWorkingGroupLeaderApplicationsProposalGracePeriod<Test>>::get(),
            p.begin_review_working_group_leader_applications_proposal_grace_period as u64
        );
        assert_eq!(
            <FillWorkingGroupLeaderOpeningProposalVotingPeriod<Test>>::get(),
            p.fill_working_group_leader_opening_proposal_voting_period as u64
        );
        assert_eq!(
            <FillWorkingGroupLeaderOpeningProposalGracePeriod<Test>>::get(),
            p.fill_working_group_leader_opening_proposal_grace_period as u64
        );
        assert_eq!(
            <SetWorkingGroupMintCapacityProposalVotingPeriod<Test>>::get(),
            p.set_working_group_mint_capacity_proposal_voting_period as u64
        );
        assert_eq!(
            <SetWorkingGroupMintCapacityProposalGracePeriod<Test>>::get(),
            p.set_working_group_mint_capacity_proposal_grace_period as u64
        );
        assert_eq!(
            <DecreaseWorkingGroupLeaderStakeProposalVotingPeriod<Test>>::get(),
            p.decrease_working_group_leader_stake_proposal_voting_period as u64
        );
        assert_eq!(
            <DecreaseWorkingGroupLeaderStakeProposalGracePeriod<Test>>::get(),
            p.decrease_working_group_leader_stake_proposal_grace_period as u64
        );
        assert_eq!(
            <SlashWorkingGroupLeaderStakeProposalVotingPeriod<Test>>::get(),
            p.slash_working_group_leader_stake_proposal_voting_period as u64
        );
        assert_eq!(
            <SlashWorkingGroupLeaderStakeProposalGracePeriod<Test>>::get(),
            p.slash_working_group_leader_stake_proposal_grace_period as u64
        );
        assert_eq!(
            <SetWorkingGroupLeaderRewardProposalVotingPeriod<Test>>::get(),
            p.set_working_group_leader_reward_proposal_voting_period as u64
        );
        assert_eq!(
            <SetWorkingGroupLeaderRewardProposalGracePeriod<Test>>::get(),
            p.set_working_group_leader_reward_proposal_grace_period as u64
        );
        assert_eq!(
            <TerminateWorkingGroupLeaderRoleProposalVotingPeriod<Test>>::get(),
            p.terminate_working_group_leader_role_proposal_voting_period as u64
        );
        assert_eq!(
            <TerminateWorkingGroupLeaderRoleProposalGracePeriod<Test>>::get(),
            p.terminate_working_group_leader_role_proposal_grace_period as u64
        );
    });
}

#[test]
fn create_add_working_group_leader_opening_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_add_working_group_leader_opening_proposal_common_checks_succeed(group);
    }
}

fn run_create_add_working_group_leader_opening_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        let add_opening_parameters = AddOpeningParameters {
            activate_at: ActivateOpeningAt::CurrentBlock,
            commitment: OpeningPolicyCommitment::default(),
            human_readable_text: b"some text".to_vec(),
            working_group,
        };

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_add_working_group_leader_opening_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    add_opening_parameters.clone(),
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_add_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    add_opening_parameters.clone(),
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_add_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    add_opening_parameters.clone(),
                )
            },
            successful_call: || {
                ProposalsCodex::create_add_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(100_000_u32)),
                    add_opening_parameters.clone(),
                )
            },
            proposal_parameters: crate::proposal_types::parameters::add_working_group_leader_opening_proposal::<
                Test,
            >(),
            proposal_details: ProposalDetails::AddWorkingGroupLeaderOpening(add_opening_parameters.clone()),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_begin_review_working_group_leader_applications_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_begin_review_working_group_leader_applications_proposal_common_checks_succeed(
            group,
        );
    }
}

fn run_create_begin_review_working_group_leader_applications_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        let opening_id = 1; // random opening id.

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_begin_review_working_group_leader_applications_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    opening_id,
                    working_group
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_begin_review_working_group_leader_applications_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    opening_id,
                    working_group
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_begin_review_working_group_leader_applications_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    opening_id,
                    working_group
                )
            },
            successful_call: || {
                ProposalsCodex::create_begin_review_working_group_leader_applications_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(25000u32)),
                    opening_id,
                    working_group
                )
            },
            proposal_parameters: crate::proposal_types::parameters::begin_review_working_group_leader_applications_proposal::<
                Test,
            >(),
            proposal_details: ProposalDetails::BeginReviewWorkingGroupLeaderApplications(opening_id,
                working_group),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_fill_working_group_leader_opening_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_fill_working_group_leader_opening_proposal_common_checks_succeed(group);
    }
}

fn run_create_fill_working_group_leader_opening_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        let opening_id = 1; // random opening id.

        let fill_opening_parameters = FillOpeningParameters {
            opening_id,
            successful_application_id: 1,
            reward_policy: None,
            working_group,
        };

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_fill_working_group_leader_opening_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    fill_opening_parameters.clone()
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_fill_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    fill_opening_parameters.clone()
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_fill_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    fill_opening_parameters.clone()
                )
            },
            successful_call: || {
                ProposalsCodex::create_fill_working_group_leader_opening_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    fill_opening_parameters.clone()
                )
            },
            proposal_parameters: crate::proposal_types::parameters::fill_working_group_leader_opening_proposal::<
                Test,
            >(),
            proposal_details: ProposalDetails::FillWorkingGroupLeaderOpening(fill_opening_parameters.clone()),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_working_group_mint_capacity_proposal_fails_with_invalid_parameters() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_working_group_mint_capacity_proposal_fails_with_invalid_parameters(group);
    }
}

fn run_create_working_group_mint_capacity_proposal_fails_with_invalid_parameters(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 500000);

        assert_eq!(
            ProposalsCodex::create_set_working_group_mint_capacity_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(50000u32)),
                (crate::WORKING_GROUP_MINT_CAPACITY_MAX_VALUE + 1) as u64,
                working_group,
            ),
            Err(Error::<Test>::InvalidWorkingGroupMintCapacity.into())
        );
    });
}

#[test]
fn create_set_working_group_mint_capacity_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_set_working_group_mint_capacity_proposal_common_checks_succeed(group);
    }
}

fn run_create_set_working_group_mint_capacity_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance(500000);

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_set_working_group_mint_capacity_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    working_group,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_set_working_group_mint_capacity_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    working_group,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_set_working_group_mint_capacity_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    0,
                    working_group,
                )
            },
            successful_call: || {
                ProposalsCodex::create_set_working_group_mint_capacity_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    10,
                    working_group,
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::set_working_group_mint_capacity_proposal::<Test>(
                ),
            proposal_details: ProposalDetails::SetWorkingGroupMintCapacity(10, working_group),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_decrease_working_group_leader_stake_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_decrease_working_group_leader_stake_proposal_common_checks_succeed(group);
    }
}

fn run_create_decrease_working_group_leader_stake_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance(500000);

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    0,
                    10,
                    working_group,
                )
            },
            successful_call: || {
                ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    10,
                    10,
                    working_group,
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::decrease_working_group_leader_stake_proposal::<
                    Test,
                >(),
            proposal_details: ProposalDetails::DecreaseWorkingGroupLeaderStake(
                10,
                10,
                working_group,
            ),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_slash_working_group_leader_stake_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_slash_working_group_leader_stake_proposal_common_checks_succeed(group);
    }
}

fn run_create_slash_working_group_leader_stake_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance(500000);

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_slash_working_group_leader_stake_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_slash_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_slash_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    0,
                    10,
                    working_group,
                )
            },
            successful_call: || {
                ProposalsCodex::create_slash_working_group_leader_stake_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    10,
                    10,
                    working_group,
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::slash_working_group_leader_stake_proposal::<
                    Test,
                >(),
            proposal_details: ProposalDetails::SlashWorkingGroupLeaderStake(
                10,
                10,
                working_group,
            ),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn slash_stake_with_zero_staking_balance_fails() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_slash_stake_with_zero_staking_balance_fails(group);
    }
}

fn run_slash_stake_with_zero_staking_balance_fails(working_group: WorkingGroup) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 500000);

        let lead_account_id = 20;
        <governance::council::Module<Test>>::set_council(
            RawOrigin::Root.into(),
            vec![lead_account_id],
        )
        .unwrap();

        assert_eq!(
            ProposalsCodex::create_slash_working_group_leader_stake_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(50000u32)),
                10,
                0,
                working_group,
            ),
            Err(Error::<Test>::SlashingStakeIsZero.into())
        );
    });
}

#[test]
fn decrease_stake_with_zero_staking_balance_fails() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_decrease_stake_with_zero_staking_balance_fails(group);
    }
}

fn run_decrease_stake_with_zero_staking_balance_fails(working_group: WorkingGroup) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 500000);

        let lead_account_id = 20;
        <governance::council::Module<Test>>::set_council(
            RawOrigin::Root.into(),
            vec![lead_account_id],
        )
        .unwrap();

        assert_eq!(
            ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
                RawOrigin::Signed(1).into(),
                1,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Test>>::from(50000u32)),
                10,
                0,
                working_group,
            ),
            Err(Error::<Test>::DecreasingStakeIsZero.into())
        );
    });
}

#[test]
fn create_set_working_group_leader_reward_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_set_working_group_leader_reward_proposal_common_checks_succeed(group);
    }
}

fn run_create_set_working_group_leader_reward_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_set_working_group_leader_reward_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_set_working_group_leader_reward_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    0,
                    10,
                    working_group,
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_set_working_group_leader_reward_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    0,
                    10,
                    working_group,
                )
            },
            successful_call: || {
                ProposalsCodex::create_set_working_group_leader_reward_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(50000u32)),
                    10,
                    10,
                    working_group,
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::set_working_group_leader_reward_proposal::<Test>(
                ),
            proposal_details: ProposalDetails::SetWorkingGroupLeaderReward(10, 10, working_group),
        };
        proposal_fixture.check_all();
    });
}

#[test]
fn create_terminate_working_group_leader_role_proposal_common_checks_succeed() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        run_create_terminate_working_group_leader_role_proposal_common_checks_succeed(group);
    }
}

fn run_create_terminate_working_group_leader_role_proposal_common_checks_succeed(
    working_group: WorkingGroup,
) {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance(500000);

        let terminate_role_parameters = TerminateRoleParameters {
            worker_id: 10,
            rationale: Vec::new(),
            slash: false,
            working_group,
        };

        let proposal_fixture = ProposalTestFixture {
            insufficient_rights_call: || {
                ProposalsCodex::create_terminate_working_group_leader_role_proposal(
                    RawOrigin::None.into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    terminate_role_parameters.clone(),
                )
            },
            empty_stake_call: || {
                ProposalsCodex::create_terminate_working_group_leader_role_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    None,
                    terminate_role_parameters.clone(),
                )
            },
            invalid_stake_call: || {
                ProposalsCodex::create_terminate_working_group_leader_role_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(5000u32)),
                    terminate_role_parameters.clone(),
                )
            },
            successful_call: || {
                ProposalsCodex::create_terminate_working_group_leader_role_proposal(
                    RawOrigin::Signed(1).into(),
                    1,
                    b"title".to_vec(),
                    b"body".to_vec(),
                    Some(<BalanceOf<Test>>::from(100_000_u32)),
                    terminate_role_parameters.clone(),
                )
            },
            proposal_parameters:
                crate::proposal_types::parameters::terminate_working_group_leader_role_proposal::<
                    Test,
                >(),
            proposal_details: ProposalDetails::TerminateWorkingGroupLeaderRole(
                terminate_role_parameters.clone(),
            ),
        };
        proposal_fixture.check_all();
    });
}
