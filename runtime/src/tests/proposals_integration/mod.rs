//! Proposals integration tests - with stake, membership, governance modules.

#![cfg(test)]

mod working_group_proposals;

use crate::{BlockNumber, ProposalCancellationFee, Runtime};
use codec::Encode;
use governance::election_params::ElectionParameters;
use membership;
use proposals_engine::{
    ActiveStake, ApprovedProposalStatus, BalanceOf, FinalizationData, Proposal,
    ProposalDecisionStatus, ProposalParameters, ProposalStatus, VoteKind, VotersParameters,
    VotingResults,
};

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_support::{StorageMap, StorageValue};
use frame_system::RawOrigin;
use sp_runtime::AccountId32;

use super::initial_test_ext;

use crate::CouncilManager;

pub type Balances = pallet_balances::Pallet<Runtime>;
pub type System = frame_system::Pallet<Runtime>;
pub type Membership = membership::Pallet<Runtime>;
pub type ProposalsEngine = proposals_engine::Pallet<Runtime>;
pub type Council = governance::council::Pallet<Runtime>;
pub type Election = governance::election::Pallet<Runtime>;
pub type ProposalsCodex = proposals_codex::Pallet<Runtime>;

fn setup_members(count: u8) {
    let authority_account_id = <Runtime as frame_system::Config>::AccountId::default();
    Membership::set_screening_authority(RawOrigin::Root.into(), authority_account_id.clone())
        .unwrap();

    for i in 0..count {
        let account_id: [u8; 32] = [i; 32];
        Membership::add_screened_member(
            RawOrigin::Signed(authority_account_id.clone().into()).into(),
            account_id.clone().into(),
            Some(account_id.to_vec()),
            None,
            None,
            None,
        )
        .unwrap();
    }
}

fn setup_council() {
    let councilor0 = AccountId32::default();
    let councilor1: [u8; 32] = [1; 32];
    let councilor2: [u8; 32] = [2; 32];
    let councilor3: [u8; 32] = [3; 32];
    let councilor4: [u8; 32] = [4; 32];
    let councilor5: [u8; 32] = [5; 32];
    assert!(Council::set_council(
        frame_system::RawOrigin::Root.into(),
        vec![
            councilor0,
            councilor1.into(),
            councilor2.into(),
            councilor3.into(),
            councilor4.into(),
            councilor5.into()
        ]
    )
    .is_ok());
}

pub(crate) fn increase_total_balance_issuance_using_account_id(
    account_id: AccountId32,
    balance: u128,
) {
    type Balances = pallet_balances::Pallet<Runtime>;
    let initial_balance = Balances::total_issuance();
    {
        let _ = <Runtime as stake::Config>::Currency::deposit_creating(&account_id, balance);
    }
    assert_eq!(Balances::total_issuance(), initial_balance + balance);
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: BlockNumber) {
    while System::block_number() < n {
        <System as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        <Election as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        <ProposalsEngine as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
        <Election as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
        <ProposalsEngine as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: AccountId32,
    current_account_id_seed: u8,
    current_voter_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_voter_id: 0,
            current_account_id_seed: 0,
            current_account_id: AccountId32::default(),
            auto_increment_voter_id: true,
        }
    }
    fn vote_and_assert_ok(&mut self, vote_kind: VoteKind) {
        self.vote_and_assert(vote_kind, Ok(()));
    }

    fn vote_and_assert(&mut self, vote_kind: VoteKind, expected_result: DispatchResult) {
        assert_eq!(self.vote(vote_kind.clone()), expected_result);
    }

    fn vote(&mut self, vote_kind: VoteKind) -> DispatchResult {
        if self.auto_increment_voter_id {
            self.current_account_id_seed += 1;
            self.current_voter_id += 1;
            let account_id: [u8; 32] = [self.current_account_id_seed; 32];
            self.current_account_id = account_id.into();
        }

        ProposalsEngine::vote(
            frame_system::RawOrigin::Signed(self.current_account_id.clone()).into(),
            self.current_voter_id,
            self.proposal_id,
            vote_kind,
        )
    }
}

#[derive(Clone)]
struct DummyProposalFixture {
    parameters: ProposalParameters<u32, u128>,
    account_id: AccountId32,
    proposer_id: u64,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    description: Vec<u8>,
    stake_balance: Option<BalanceOf<Runtime>>,
}

impl Default for DummyProposalFixture {
    fn default() -> Self {
        let title = b"title".to_vec();
        let description = b"description".to_vec();
        let dummy_proposal =
            proposals_codex::Call::<Runtime>::execute_text_proposal(b"text".to_vec());

        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
            },
            account_id: <Runtime as frame_system::Config>::AccountId::default(),
            proposer_id: 0,
            proposal_code: dummy_proposal.encode(),
            title,
            description,
            stake_balance: None,
        }
    }
}

impl DummyProposalFixture {
    fn with_parameters(self, parameters: ProposalParameters<u32, u128>) -> Self {
        DummyProposalFixture { parameters, ..self }
    }

    fn with_account_id(self, account_id: AccountId32) -> Self {
        DummyProposalFixture { account_id, ..self }
    }

    fn with_voting_period(self, voting_period: u32) -> Self {
        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period,
                ..self.parameters
            },
            ..self
        }
    }

    fn with_stake(self, stake_balance: BalanceOf<Runtime>) -> Self {
        DummyProposalFixture {
            stake_balance: Some(stake_balance),
            ..self
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        DummyProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn create_proposal_and_assert(self, result: Result<u32, DispatchError>) -> Option<u32> {
        let proposal_id_result = ProposalsEngine::create_proposal(
            self.account_id,
            self.proposer_id,
            self.parameters,
            self.title,
            self.description,
            self.stake_balance,
            self.proposal_code,
        );
        assert_eq!(proposal_id_result, result);

        proposal_id_result.ok()
    }
}

struct CancelProposalFixture {
    origin: RawOrigin<AccountId32>,
    proposal_id: u32,
    proposer_id: u64,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        let account_id = <Runtime as frame_system::Config>::AccountId::default();
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(account_id),
            proposer_id: 0,
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        CancelProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn cancel_and_assert(self, expected_result: DispatchResult) {
        assert_eq!(
            ProposalsEngine::cancel_proposal(
                self.origin.into(),
                self.proposer_id,
                self.proposal_id
            ),
            expected_result
        );
    }
}

/// Main purpose of this integration test: check balance of the member on proposal finalization (cancellation)
/// It tests StakingEventsHandler integration. Also, membership module is tested during the proposal creation (ActorOriginValidator).
#[test]
fn proposal_cancellation_with_slashes_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = <Runtime as frame_system::Config>::AccountId::default();

        setup_members(2);
        let member_id = 0; // newly created member_id

        let stake_amount = 20000u128;
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 60,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 5,
            required_stake: Some(stake_amount),
        };
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters)
            .with_account_id(account_id.clone())
            .with_stake(stake_amount)
            .with_proposer(member_id);

        let account_top_up = 500000;
        let account_starting_balance =
            <Runtime as stake::Config>::Currency::total_balance(&account_id);

        let _imbalance =
            <Runtime as stake::Config>::Currency::deposit_creating(&account_id, account_top_up);

        assert_eq!(
            <Runtime as stake::Config>::Currency::total_balance(&account_id),
            account_starting_balance + account_top_up
        );

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        assert_eq!(
            <Runtime as stake::Config>::Currency::total_balance(&account_id),
            account_starting_balance + account_top_up - stake_amount
        );

        let mut proposal = ProposalsEngine::proposals(proposal_id);

        let mut expected_proposal = Proposal {
            parameters,
            proposer_id: member_id,
            created_at: 0,
            status: ProposalStatus::Active(Some(ActiveStake {
                stake_id: 0,
                source_account_id: account_id.clone(),
            })),
            title: b"title".to_vec(),
            description: b"description".to_vec(),
            voting_results: VotingResults::default(),
        };

        assert_eq!(proposal, expected_proposal);

        let cancel_proposal_fixture =
            CancelProposalFixture::new(proposal_id).with_proposer(member_id);

        cancel_proposal_fixture.cancel_and_assert(Ok(()));

        proposal = ProposalsEngine::proposals(proposal_id);

        expected_proposal.status = ProposalStatus::Finalized(FinalizationData {
            proposal_status: ProposalDecisionStatus::Canceled,
            finalized_at: 0,
            encoded_unstaking_error_due_to_broken_runtime: None,
            stake_data_after_unstaking_error: None,
        });

        assert_eq!(proposal, expected_proposal);

        let cancellation_fee = ProposalCancellationFee::get() as u128;
        assert_eq!(
            <Runtime as stake::Config>::Currency::total_balance(&account_id),
            account_starting_balance + account_top_up - cancellation_fee
        );
    });
}

#[test]
fn proposal_reset_succeeds() {
    initial_test_ext().execute_with(|| {
        setup_members(4);
        setup_council();
        // create proposal
        let dummy_proposal = DummyProposalFixture::default().with_voting_period(100);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // create some votes
        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        assert!(<proposals_engine::ActiveProposalIds<Runtime>>::contains_key(proposal_id));

        // check
        let proposal = ProposalsEngine::proposals(proposal_id);
        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 1,
                slashes: 1,
            }
        );

        // Ensure council was elected
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 6);

        let voted_member_id = 2;
        // Check for votes.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, voted_member_id),
            VoteKind::Abstain
        );

        // Check proposals CouncilElected hook just trigger the election hook (empty council).
        //<Runtime as governance::election::Config>::CouncilElected::council_elected(Vec::new(), 10);

        elect_single_councilor();

        let updated_proposal = ProposalsEngine::proposals(proposal_id);

        assert_eq!(
            updated_proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 0,
                slashes: 0,
            }
        );

        // No votes could survive cleaning: should be default value.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, voted_member_id),
            VoteKind::default()
        );

        // Check council CouncilElected hook. It should set current council. And we elected single councilor.
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 1);
    });
}

fn elect_single_councilor() {
    let res = Election::set_election_parameters(
        RawOrigin::Root.into(),
        ElectionParameters {
            announcing_period: 1,
            voting_period: 1,
            revealing_period: 1,
            council_size: 1,
            candidacy_limit: 10,
            new_term_duration: 2000000,
            min_council_stake: 0,
            min_voting_stake: 0,
        },
    );
    assert_eq!(res, Ok(()));

    let res = Election::force_start_election(RawOrigin::Root.into());
    assert_eq!(res, Ok(()));

    let councilor1: [u8; 32] = [1; 32];
    increase_total_balance_issuance_using_account_id(councilor1.clone().into(), 1200000000);

    let res = Election::apply(RawOrigin::Signed(councilor1.into()).into(), 0);
    assert_eq!(res, Ok(()));

    run_to_block(5);
}

struct CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    successful_call: SuccessfulCall,
    member_id: u64,
    setup_environment: bool,
    proposal_id: u32,
    run_to_block: u32,
}

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn default_for_call(call: SuccessfulCall) -> Self {
        Self {
            successful_call: call,
            member_id: 1,
            setup_environment: true,
            proposal_id: 1,
            run_to_block: 2,
        }
    }

    fn disable_setup_enviroment(self) -> Self {
        Self {
            setup_environment: false,
            ..self
        }
    }
    fn with_setup_enviroment(self, setup_environment: bool) -> Self {
        Self {
            setup_environment,
            ..self
        }
    }

    fn with_member_id(self, member_id: u64) -> Self {
        Self { member_id, ..self }
    }

    fn with_expected_proposal_id(self, expected_proposal_id: u32) -> Self {
        Self {
            proposal_id: expected_proposal_id,
            ..self
        }
    }

    fn with_run_to_block(self, run_to_block: u32) -> Self {
        Self {
            run_to_block,
            ..self
        }
    }
}

const NUMBER_OF_MEMBERS_TO_SETUP_IN_CODEX_PROPOSAL_FIXTURE: u8 = 15;

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn call_extrinsic_and_assert(&self) {
        let account_id: [u8; 32] = [self.member_id as u8; 32];

        if self.setup_environment {
            setup_members(NUMBER_OF_MEMBERS_TO_SETUP_IN_CODEX_PROPOSAL_FIXTURE);
            setup_council();

            increase_total_balance_issuance_using_account_id(account_id.clone().into(), 1000000);
        }

        assert_eq!((self.successful_call)(), Ok(()));

        let mut vote_generator = VoteGenerator::new(self.proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block(self.run_to_block);

        let proposal = ProposalsEngine::proposals(self.proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                status: ProposalStatus::approved(
                    ApprovedProposalStatus::Executed,
                    self.run_to_block - 2
                ),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 5,
                    rejections: 0,
                    slashes: 0,
                },
                ..proposal
            }
        );
    }
}

#[test]
fn text_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            ProposalsCodex::create_text_proposal(
                RawOrigin::Signed(account_id.into()).into(),
                member_id as u64,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Runtime>>::from(25000u32)),
                b"text".to_vec(),
            )
        })
        .with_member_id(member_id as u64);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();
    });
}

#[test]
fn spending_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];
        let new_balance = <BalanceOf<Runtime>>::from(5555u32);

        // account id outside range of generated member accounts when environement is setup
        // to ensure it doesn't get any endowed balance
        let target_account_id: [u8; 32] =
            [NUMBER_OF_MEMBERS_TO_SETUP_IN_CODEX_PROPOSAL_FIXTURE + 1; 32];

        assert!(Council::set_council_mint_capacity(RawOrigin::Root.into(), new_balance).is_ok());

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            ProposalsCodex::create_spending_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                member_id as u64,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Runtime>>::from(25_000_u32)),
                new_balance,
                target_account_id.clone().into(),
            )
        })
        .with_member_id(member_id as u64);

        let converted_account_id: AccountId32 = target_account_id.clone().into();

        assert_eq!(Balances::free_balance(converted_account_id.clone()), 0);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        assert_eq!(Balances::free_balance(converted_account_id), new_balance);
    });
}

#[test]
fn set_election_parameters_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 1;
        let account_id: [u8; 32] = [member_id; 32];

        let election_parameters = ElectionParameters {
            announcing_period: 14400,
            voting_period: 14400,
            revealing_period: 14400,
            council_size: 6,
            candidacy_limit: 50,
            new_term_duration: 14400,
            min_council_stake: 1,
            min_voting_stake: 1,
        };
        assert_eq!(Election::announcing_period(), 0);

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            ProposalsCodex::create_set_election_parameters_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                member_id as u64,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Runtime>>::from(1_000_000_u32)),
                election_parameters,
            )
        });
        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        assert_eq!(Election::announcing_period(), 14400);
    });
}

#[test]
#[cfg(feature = "standalone")]
fn set_validator_count_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 1;
        let account_id: [u8; 32] = [member_id; 32];

        let new_validator_count = 8;
        assert_eq!(<pallet_staking::ValidatorCount>::get(), 0);

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            ProposalsCodex::create_set_validator_count_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                member_id as u64,
                b"title".to_vec(),
                b"body".to_vec(),
                Some(<BalanceOf<Runtime>>::from(500_000_u32)),
                new_validator_count,
            )
        });
        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        assert_eq!(<pallet_staking::ValidatorCount>::get(), new_validator_count);
    });
}
