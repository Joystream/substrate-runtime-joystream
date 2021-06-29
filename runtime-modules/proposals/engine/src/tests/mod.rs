pub(crate) mod mock;

use crate::*;
use mock::{Event, *};

use codec::Encode;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_support::{StorageDoubleMap, StorageMap, StorageValue};
use frame_system::RawOrigin;
use frame_system::{EventRecord, Phase};
use sp_std::rc::Rc;

pub(crate) fn increase_total_balance_issuance_using_account_id(account_id: u64, balance: u64) {
    let initial_balance = Balances::total_issuance();
    {
        let _ = <Test as stake::Config>::Currency::deposit_creating(&account_id, balance);
    }
    assert_eq!(Balances::total_issuance(), initial_balance + balance);
}

struct ProposalParametersFixture {
    parameters: ProposalParameters<u64, u64>,
}

impl ProposalParametersFixture {
    fn with_required_stake(&self, required_stake: BalanceOf<Test>) -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                required_stake: Some(required_stake),
                ..self.parameters
            },
        }
    }
    fn with_grace_period(&self, grace_period: u64) -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                grace_period,
                ..self.parameters
            },
        }
    }

    fn params(&self) -> ProposalParameters<u64, u64> {
        self.parameters.clone()
    }
}

impl Default for ProposalParametersFixture {
    fn default() -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
            },
        }
    }
}

#[derive(Clone)]
struct DummyProposalFixture {
    parameters: ProposalParameters<u64, u64>,
    account_id: u64,
    proposer_id: u64,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    description: Vec<u8>,
    stake_balance: Option<BalanceOf<Test>>,
}

impl Default for DummyProposalFixture {
    fn default() -> Self {
        let title = b"title".to_vec();
        let description = b"description".to_vec();
        let dummy_proposal =
            mock::proposals::Call::<Test>::dummy_proposal(title.clone(), description.clone());

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
            account_id: 1,
            proposer_id: 1,
            proposal_code: dummy_proposal.encode(),
            title,
            description,
            stake_balance: None,
        }
    }
}

impl DummyProposalFixture {
    fn with_title_and_body(self, title: Vec<u8>, description: Vec<u8>) -> Self {
        DummyProposalFixture {
            title,
            description,
            ..self
        }
    }

    fn with_parameters(self, parameters: ProposalParameters<u64, u64>) -> Self {
        DummyProposalFixture { parameters, ..self }
    }

    fn with_account_id(self, account_id: u64) -> Self {
        DummyProposalFixture { account_id, ..self }
    }

    fn with_stake(self, stake_balance: BalanceOf<Test>) -> Self {
        DummyProposalFixture {
            stake_balance: Some(stake_balance),
            ..self
        }
    }

    fn with_proposal_code(self, proposal_code: Vec<u8>) -> Self {
        DummyProposalFixture {
            proposal_code,
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
    origin: RawOrigin<u64>,
    proposal_id: u32,
    proposer_id: u64,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(1),
            proposer_id: 1,
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        CancelProposalFixture { origin, ..self }
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
struct VetoProposalFixture {
    origin: RawOrigin<u64>,
    proposal_id: u32,
}

impl VetoProposalFixture {
    fn new(proposal_id: u32) -> Self {
        VetoProposalFixture {
            proposal_id,
            origin: RawOrigin::Root,
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        VetoProposalFixture { origin, ..self }
    }

    fn veto_and_assert(self, expected_result: DispatchResult) {
        assert_eq!(
            ProposalsEngine::veto_proposal(self.origin.into(), self.proposal_id,),
            expected_result
        );
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: u64,
    current_voter_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_voter_id: 0,
            current_account_id: 0,
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
            self.current_account_id += 1;
            self.current_voter_id += 1;
        }

        ProposalsEngine::vote(
            frame_system::RawOrigin::Signed(self.current_account_id).into(),
            self.current_voter_id,
            self.proposal_id,
            vote_kind,
        )
    }
}

struct EventFixture;
impl EventFixture {
    fn assert_events(expected_raw_events: Vec<RawEvent<u32, u64, u64, u64, u64>>) {
        let expected_events = expected_raw_events
            .iter()
            .map(|ev| EventRecord {
                phase: Phase::Initialization,
                event: Event::proposals_engine(ev.clone()),
                topics: vec![],
            })
            .collect::<Vec<EventRecord<_, _>>>();

        assert_eq!(System::events(), expected_events);
    }
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <ProposalsEngine as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <ProposalsEngine as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

fn run_to_block_and_finalize(n: u64) {
    run_to_block(n);
    <ProposalsEngine as OnFinalize<u64>>::on_finalize(n);
}

#[test]
fn create_dummy_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();

        dummy_proposal.create_proposal_and_assert(Ok(1));
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
    });
}

#[test]
fn vote_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        assert_eq!(
            ProposalsEngine::vote(
                frame_system::RawOrigin::None.into(),
                1,
                1,
                VoteKind::Approve
            ),
            Err(DispatchError::Other("Bad origin"))
        );
    });
}

#[test]
fn proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::approved(ApprovedProposalStatus::Executed, 0),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
            }
        );

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);
    });
}

#[test]
fn proposal_execution_failed() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();

        let faulty_proposal = mock::proposals::Call::<Test>::faulty_proposal(
            b"title".to_vec(),
            b"description".to_vec(),
        );

        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_proposal_code(faulty_proposal.encode());

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(2);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::approved(
                    ApprovedProposalStatus::failed_execution("ExecutionFailed"),
                    0
                ),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
            }
        )
    });
}

#[test]
fn voting_results_calculation_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 50,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 0,
            required_stake: None,
        };
        let dummy_proposal = DummyProposalFixture::default().with_parameters(parameters);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        run_to_block_and_finalize(2);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 2,
                rejections: 1,
                slashes: 0,
            }
        )
    });
}

#[test]
fn rejected_voting_results_and_remove_proposal_id_from_active_succeeds() {
    initial_test_ext().execute_with(|| {
        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);

        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        assert!(<ActiveProposalIds<Test>>::contains_key(proposal_id));

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        run_to_block_and_finalize(2);

        let proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 2,
                approvals: 0,
                rejections: 2,
                slashes: 0,
            }
        );

        assert_eq!(
            proposal.status,
            ProposalStatus::finalized_successfully(ProposalDecisionStatus::Rejected, 0),
        );
        assert!(!<ActiveProposalIds<Test>>::contains_key(proposal_id));

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);
    });
}

#[test]
fn create_proposal_fails_with_invalid_body_or_title() {
    initial_test_ext().execute_with(|| {
        let mut dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(Vec::new(), b"body".to_vec());
        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::EmptyTitleProvided.into()));

        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(b"title".to_vec(), Vec::new());
        dummy_proposal
            .create_proposal_and_assert(Err(Error::<Test>::EmptyDescriptionProvided.into()));

        let too_long_title = vec![0; 200];
        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(too_long_title, b"body".to_vec());
        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::TitleIsTooLong.into()));

        let too_long_body = vec![0; 11000];
        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(b"title".to_vec(), too_long_body);
        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::DescriptionIsTooLong.into()));
    });
}

#[test]
fn vote_fails_with_expired_voting_period() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        run_to_block_and_finalize(6);

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert(
            VoteKind::Approve,
            Err(Error::<Test>::ProposalFinalized.into()),
        );
    });
}

#[test]
fn vote_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        run_to_block_and_finalize(2);

        let mut vote_generator_to_fail = VoteGenerator::new(proposal_id);
        vote_generator_to_fail.vote_and_assert(
            VoteKind::Approve,
            Err(Error::<Test>::ProposalFinalized.into()),
        );
    });
}

#[test]
fn vote_fails_with_absent_proposal() {
    initial_test_ext().execute_with(|| {
        let mut vote_generator = VoteGenerator::new(2);
        vote_generator.vote_and_assert(
            VoteKind::Approve,
            Err(Error::<Test>::ProposalNotFound.into()),
        );
    });
}

#[test]
fn vote_fails_on_double_voting() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.auto_increment_voter_id = false;

        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Approve, Err(Error::<Test>::AlreadyVoted.into()));
    });
}

#[test]
fn cancel_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Ok(()));

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::finalized_successfully(ProposalDecisionStatus::Canceled, 0),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults::default(),
            }
        )
    });
}

#[test]
fn cancel_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        run_to_block_and_finalize(6);

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Err(Error::<Test>::ProposalFinalized.into()));
    });
}

#[test]
fn cancel_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let cancel_proposal = CancelProposalFixture::new(2);
        cancel_proposal.cancel_and_assert(Err(Error::<Test>::ProposalNotFound.into()));
    });
}

#[test]
fn cancel_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let cancel_proposal = CancelProposalFixture::new(proposal_id)
            .with_origin(RawOrigin::Signed(2))
            .with_proposer(2);
        cancel_proposal.cancel_and_assert(Err(Error::<Test>::NotAuthor.into()));
    });
}

#[test]
fn veto_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);

        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::finalized_successfully(ProposalDecisionStatus::Vetoed, 0),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults::default(),
            }
        );

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);
    });
}

#[test]
fn veto_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        run_to_block_and_finalize(6);

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Err(Error::<Test>::ProposalFinalized.into()));
    });
}

#[test]
fn veto_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let veto_proposal = VetoProposalFixture::new(2);
        veto_proposal.veto_and_assert(Err(Error::<Test>::ProposalNotFound.into()));
    });
}

#[test]
fn veto_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let veto_proposal = VetoProposalFixture::new(proposal_id).with_origin(RawOrigin::Signed(2));
        veto_proposal.veto_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn create_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        // Events start only from 1 first block. No events on block zero.
        run_to_block_and_finalize(1);

        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal.create_proposal_and_assert(Ok(1));

        EventFixture::assert_events(vec![RawEvent::ProposalCreated(1, 1)]);
    });
}

#[test]
fn veto_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        // Events start only from 1 first block. No events on block zero.
        run_to_block_and_finalize(1);

        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::ProposalStatusUpdated(
                1,
                ProposalStatus::finalized_successfully(ProposalDecisionStatus::Vetoed, 1),
            ),
        ]);
    });
}

#[test]
fn cancel_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        // Events start only from 1 first block. No events on block zero.
        run_to_block_and_finalize(1);

        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Ok(()));

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::ProposalStatusUpdated(
                1,
                ProposalStatus::Finalized(FinalizationData {
                    proposal_status: ProposalDecisionStatus::Canceled,
                    encoded_unstaking_error_due_to_broken_runtime: None,
                    stake_data_after_unstaking_error: None,
                    finalized_at: 1,
                }),
            ),
        ]);
    });
}

#[test]
fn vote_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        // Events start only from 1 first block. No events on block zero.
        run_to_block_and_finalize(1);

        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::Voted(1, 1, VoteKind::Approve),
        ]);
    });
}

#[test]
fn create_proposal_and_expire_it() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        run_to_block_and_finalize(8);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::finalized_successfully(ProposalDecisionStatus::Expired, 3),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults::default(),
            }
        )
    });
}

#[test]
fn proposal_execution_postponed_because_of_grace_period() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default().with_grace_period(3);
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);
        run_to_block_and_finalize(2);

        // check internal cache for proposal_id presence
        assert!(<PendingExecutionProposalIds<Test>>::contains_key(
            proposal_id
        ));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::approved(ApprovedProposalStatus::PendingExecution, 0),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
            }
        );
    });
}

#[test]
fn proposal_execution_vetoed_successfully_during_the_grace_period() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default().with_grace_period(3);
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);
        run_to_block_and_finalize(2);

        // check internal cache for proposal_id presence
        assert!(<PendingExecutionProposalIds<Test>>::contains_key(
            proposal_id
        ));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::approved(ApprovedProposalStatus::PendingExecution, 0),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
            }
        );

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::finalized_successfully(ProposalDecisionStatus::Vetoed, 2),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
            }
        );

        // check internal cache for proposal_id absence
        assert!(!<PendingExecutionProposalIds<Test>>::contains_key(
            proposal_id
        ));
    });
}

#[test]
fn proposal_execution_succeeds_after_the_grace_period() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default().with_grace_period(2);
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);

        // check internal cache for proposal_id presence
        assert!(<PendingExecutionProposalIds<Test>>::contains_key(
            proposal_id
        ));

        let mut proposal = <crate::Proposals<Test>>::get(proposal_id);

        let mut expected_proposal = Proposal {
            parameters: parameters_fixture.params(),
            proposer_id: 1,
            created_at: 0,
            status: ProposalStatus::approved(ApprovedProposalStatus::PendingExecution, 0),
            title: b"title".to_vec(),
            description: b"description".to_vec(),
            voting_results: VotingResults {
                abstentions: 0,
                approvals: 4,
                rejections: 0,
                slashes: 0,
            },
        };

        assert_eq!(proposal, expected_proposal);

        run_to_block_and_finalize(2);

        proposal = <crate::Proposals<Test>>::get(proposal_id);

        expected_proposal.status = ProposalStatus::approved(ApprovedProposalStatus::Executed, 0);

        assert_eq!(proposal, expected_proposal);

        // check internal cache for proposal_id absence
        assert!(!<PendingExecutionProposalIds<Test>>::contains_key(
            proposal_id
        ));
    });
}

#[test]
fn create_proposal_fails_on_exceeding_max_active_proposals_count() {
    initial_test_ext().execute_with(|| {
        for idx in 1..101 {
            let dummy_proposal = DummyProposalFixture::default();
            dummy_proposal.create_proposal_and_assert(Ok(idx));
            // internal active proposal counter check
            assert_eq!(<ActiveProposalCount>::get(), idx);
        }

        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal
            .create_proposal_and_assert(Err(Error::<Test>::MaxActiveProposalNumberExceeded.into()));
        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 100);
    });
}

#[test]
fn voting_internal_cache_exists_after_proposal_finalization() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal.create_proposal_and_assert(Ok(1));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        // cache exists
        assert!(<crate::VoteExistsByProposalByVoter<Test>>::contains_key(
            proposal_id,
            1
        ));

        run_to_block_and_finalize(2);

        // cache still exists and is not cleared
        assert!(<crate::VoteExistsByProposalByVoter<Test>>::contains_key(
            proposal_id,
            1
        ));
    });
}

#[test]
fn create_dummy_proposal_succeeds_with_stake() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let required_stake = 200;
        let parameters_fixture =
            ProposalParametersFixture::default().with_required_stake(required_stake);

        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_account_id(account_id)
            .with_stake(200);

        let _imbalance = <Test as stake::Config>::Currency::deposit_creating(&account_id, 500);

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 0,
                status: ProposalStatus::Active(Some(ActiveStake {
                    stake_id: 0, // valid stake_id
                    source_account_id: 1
                })),
                title: b"title".to_vec(),
                description: b"description".to_vec(),
                voting_results: VotingResults::default(),
            }
        )
    });
}

#[test]
fn create_dummy_proposal_fail_with_stake_on_empty_account() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let required_stake = 200;
        let parameters_fixture =
            ProposalParametersFixture::default().with_required_stake(required_stake);
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_account_id(account_id)
            .with_stake(required_stake);

        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::InsufficientBalance.into()));
    });
}

#[test]
fn create_proposal_fais_with_invalid_stake_parameters() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();

        let mut dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_stake(200);

        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::StakeShouldBeEmpty.into()));

        let parameters_fixture_stake_200 = parameters_fixture.with_required_stake(200);
        dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture_stake_200.params());

        dummy_proposal.create_proposal_and_assert(Err(Error::<Test>::EmptyStake.into()));

        let parameters_fixture_stake_300 = parameters_fixture.with_required_stake(300);
        dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture_stake_300.params())
            .with_stake(200);

        dummy_proposal
            .create_proposal_and_assert(Err(Error::<Test>::StakeDiffersFromRequired.into()));
    });
}

#[test]
fn finalize_expired_proposal_and_check_stake_removing_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let stake_amount = 200;
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
            .with_account_id(account_id)
            .with_stake(stake_amount);

        let account_balance = 500;
        let _imbalance =
            <Test as stake::Config>::Currency::deposit_creating(&account_id, account_balance);

        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance
        );

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance - stake_amount
        );

        let mut proposal = <crate::Proposals<Test>>::get(proposal_id);

        let mut expected_proposal = Proposal {
            parameters,
            proposer_id: 1,
            created_at: 0,
            status: ProposalStatus::Active(Some(ActiveStake {
                stake_id: 0,
                source_account_id: 1,
            })),
            title: b"title".to_vec(),
            description: b"description".to_vec(),
            voting_results: VotingResults::default(),
        };

        assert_eq!(proposal, expected_proposal);

        run_to_block_and_finalize(5);

        proposal = <crate::Proposals<Test>>::get(proposal_id);

        expected_proposal.status = ProposalStatus::Finalized(FinalizationData {
            proposal_status: ProposalDecisionStatus::Expired,
            finalized_at: 3,
            encoded_unstaking_error_due_to_broken_runtime: None,
            stake_data_after_unstaking_error: None,
        });

        assert_eq!(proposal, expected_proposal);

        let rejection_fee = RejectionFee::get();
        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance - rejection_fee
        );
    });
}

#[test]
fn proposal_cancellation_with_slashes_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let stake_amount = 200;
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
            .with_stake(stake_amount);

        let account_balance = 500;
        let _imbalance =
            <Test as stake::Config>::Currency::deposit_creating(&account_id, account_balance);

        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance
        );

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance - stake_amount
        );

        let mut proposal = <crate::Proposals<Test>>::get(proposal_id);

        let mut expected_proposal = Proposal {
            parameters,
            proposer_id: 1,
            created_at: 0,
            status: ProposalStatus::Active(Some(ActiveStake {
                stake_id: 0,
                source_account_id: 1,
            })),
            title: b"title".to_vec(),
            description: b"description".to_vec(),
            voting_results: VotingResults::default(),
        };

        assert_eq!(proposal, expected_proposal);

        let cancel_proposal_fixture = CancelProposalFixture::new(proposal_id);

        cancel_proposal_fixture.cancel_and_assert(Ok(()));

        proposal = <crate::Proposals<Test>>::get(proposal_id);

        expected_proposal.status = ProposalStatus::Finalized(FinalizationData {
            proposal_status: ProposalDecisionStatus::Canceled,
            finalized_at: 0,
            encoded_unstaking_error_due_to_broken_runtime: None,
            stake_data_after_unstaking_error: None,
        });

        assert_eq!(proposal, expected_proposal);

        let cancellation_fee = CancellationFee::get();
        assert_eq!(
            <Test as stake::Config>::Currency::total_balance(&account_id),
            account_balance - cancellation_fee
        );
    });
}

#[test]
fn finalize_proposal_using_stake_mocks_succeeds() {
    handle_mock(|| {
        initial_test_ext().execute_with(|| {
            let mock = {
                let mut mock = crate::types::MockStakeHandler::<Test>::new();
                mock.expect_create_stake().times(1).returning(|| Ok(1));

                mock.expect_make_stake_imbalance()
                    .times(1)
                    .returning(|_, _| Ok(crate::types::NegativeImbalance::<Test>::new(200)));

                mock.expect_stake().times(1).returning(|_, _| Ok(()));

                mock.expect_remove_stake().times(1).returning(|_| Ok(()));

                mock.expect_unstake().times(1).returning(|_| Ok(()));

                mock.expect_slash().times(1).returning(|_, _| Ok(()));

                Rc::new(mock)
            };
            set_stake_handler_impl(mock.clone());

            let account_id = 1;

            increase_total_balance_issuance_using_account_id(account_id, 1000000);

            let stake_amount = 200;
            let parameters_fixture =
                ProposalParametersFixture::default().with_required_stake(stake_amount);
            let dummy_proposal = DummyProposalFixture::default()
                .with_parameters(parameters_fixture.params())
                .with_account_id(account_id)
                .with_stake(stake_amount);

            let _proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

            run_to_block_and_finalize(5);
        });
    });
}

#[test]
fn proposal_slashing_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        assert!(<ActiveProposalIds<Test>>::contains_key(proposal_id));

        run_to_block_and_finalize(2);

        let proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 1,
                slashes: 3,
            }
        );

        assert_eq!(
            proposal.status,
            ProposalStatus::Finalized(FinalizationData {
                proposal_status: ProposalDecisionStatus::Slashed,
                encoded_unstaking_error_due_to_broken_runtime: None,
                finalized_at: 0,
                stake_data_after_unstaking_error: None,
            }),
        );
        assert!(!<ActiveProposalIds<Test>>::contains_key(proposal_id));
    });
}

#[test]
fn finalize_proposal_using_stake_mocks_failed() {
    handle_mock(|| {
        initial_test_ext().execute_with(|| {
            let mock = {
                let mut mock = crate::types::MockStakeHandler::<Test>::new();
                mock.expect_create_stake().times(1).returning(|| Ok(1));

                mock.expect_remove_stake()
                    .times(1)
                    .returning(|_| Err("Cannot remove stake"));

                mock.expect_make_stake_imbalance()
                    .times(1)
                    .returning(|_, _| Ok(crate::types::NegativeImbalance::<Test>::new(200)));

                mock.expect_stake().times(1).returning(|_, _| Ok(()));

                mock.expect_unstake().times(1).returning(|_| Ok(()));

                mock.expect_slash().times(1).returning(|_, _| Ok(()));

                Rc::new(mock)
            };
            set_stake_handler_impl(mock.clone());

            let account_id = 1;

            increase_total_balance_issuance_using_account_id(account_id, 1000000);

            let stake_amount = 200;
            let parameters_fixture =
                ProposalParametersFixture::default().with_required_stake(stake_amount);
            let dummy_proposal = DummyProposalFixture::default()
                .with_parameters(parameters_fixture.params())
                .with_account_id(account_id)
                .with_stake(stake_amount);

            let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

            run_to_block_and_finalize(5);

            let proposal = <Proposals<Test>>::get(proposal_id);
            assert_eq!(
                proposal,
                Proposal {
                    parameters: parameters_fixture.params(),
                    proposer_id: 1,
                    created_at: 0,
                    status: ProposalStatus::finalized(
                        ProposalDecisionStatus::Expired,
                        Some("Cannot remove stake"),
                        Some(ActiveStake {
                            stake_id: 1,
                            source_account_id: 1
                        }),
                        3,
                    ),
                    title: b"title".to_vec(),
                    description: b"description".to_vec(),
                    voting_results: VotingResults::default(),
                }
            );
        });
    });
}

#[test]
fn create_proposal_fails_with_invalid_threshold_parameters() {
    initial_test_ext().execute_with(|| {
        let mut parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 0,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 5,
            required_stake: None,
        };

        let mut dummy_proposal = DummyProposalFixture::default().with_parameters(parameters);

        dummy_proposal.create_proposal_and_assert(Err(
            Error::<Test>::InvalidParameterApprovalThreshold.into(),
        ));

        parameters.approval_threshold_percentage = 60;
        parameters.slashing_threshold_percentage = 0;
        dummy_proposal = DummyProposalFixture::default().with_parameters(parameters);

        dummy_proposal.create_proposal_and_assert(Err(
            Error::<Test>::InvalidParameterSlashingThreshold.into(),
        ));
    });
}

#[test]
fn proposal_reset_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        assert!(<ActiveProposalIds<Test>>::contains_key(proposal_id));
        assert_eq!(
            <VoteExistsByProposalByVoter<Test>>::get(&proposal_id, &2),
            VoteKind::Abstain
        );

        run_to_block_and_finalize(2);

        let proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 1,
                slashes: 1,
            }
        );

        ProposalsEngine::reset_active_proposals();

        let updated_proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            updated_proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 0,
                slashes: 0,
            }
        );

        // whole double map prefix was removed (should return default value)
        assert_eq!(
            <VoteExistsByProposalByVoter<Test>>::get(&proposal_id, &2),
            VoteKind::default()
        );
    });
}

#[test]
fn proposal_counters_are_valid() {
    initial_test_ext().execute_with(|| {
        let mut dummy_proposal = DummyProposalFixture::default();
        let _ = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        dummy_proposal = DummyProposalFixture::default();
        let _ = dummy_proposal.create_proposal_and_assert(Ok(2)).unwrap();

        dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(3)).unwrap();

        assert_eq!(ActiveProposalCount::get(), 3);
        assert_eq!(ProposalCount::get(), 3);

        let cancel_proposal_fixture = CancelProposalFixture::new(proposal_id);
        cancel_proposal_fixture.cancel_and_assert(Ok(()));

        assert_eq!(ActiveProposalCount::get(), 2);
        assert_eq!(ProposalCount::get(), 3);
    });
}

#[test]
fn proposal_stake_cache_is_valid() {
    initial_test_ext().execute_with(|| {
        increase_total_balance_issuance_using_account_id(1, 50000);

        let stake = 250u32;
        let parameters = ProposalParametersFixture::default().with_required_stake(stake.into());
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters.params())
            .with_stake(stake as u64);

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        let expected_stake_id = 0;
        assert_eq!(
            <StakesProposals<Test>>::get(&expected_stake_id),
            proposal_id
        );
    });
}

#[test]
fn slash_balance_is_calculated_correctly() {
    initial_test_ext().execute_with(|| {
        let vetoed_slash_balance = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Vetoed,
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(vetoed_slash_balance, 0);

        let approved_slash_balance = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Approved(ApprovedProposalStatus::Executed),
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(approved_slash_balance, 0);

        let rejection_fee = <Test as crate::Config>::RejectionFee::get();

        let rejected_slash_balance = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Rejected,
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(rejected_slash_balance, rejection_fee);

        let expired_slash_balance = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Expired,
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(expired_slash_balance, rejection_fee);

        let cancellation_fee = <Test as crate::Config>::CancellationFee::get();

        let cancellation_slash_balance = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Canceled,
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(cancellation_slash_balance, cancellation_fee);

        let slash_balance_with_no_stake = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Slashed,
            &ProposalParametersFixture::default().params(),
        );

        assert_eq!(slash_balance_with_no_stake, 0);

        let stake = 256;
        let slash_balance_with_stake = ProposalsEngine::calculate_slash_balance(
            &ProposalDecisionStatus::Slashed,
            &ProposalParametersFixture::default()
                .with_required_stake(stake)
                .params(),
        );

        assert_eq!(slash_balance_with_stake, stake);
    });
}
