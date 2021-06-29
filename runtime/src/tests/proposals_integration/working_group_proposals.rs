#![allow(unnameable_test_items)]
#![allow(dead_code)]

use super::*;

use frame_system::RawOrigin;

use common::working_group::WorkingGroup;
use hiring::ActivateOpeningAt;
use proposals_codex::AddOpeningParameters;
use working_group::{OpeningPolicyCommitment, RewardPolicy};

use crate::{
    Balance, BlockNumber, ContentDirectoryWorkingGroup, ContentDirectoryWorkingGroupInstance,
    GatewayWorkingGroup, GatewayWorkingGroupInstance, OperationsWorkingGroup,
    OperationsWorkingGroupInstance, StorageWorkingGroup, StorageWorkingGroupInstance,
};
use sp_std::collections::btree_set::BTreeSet;

use crate::primitives::{ActorId, MemberId};
use frame_support::traits;
use strum::IntoEnumIterator;

type WorkingGroupInstance<T, I> = working_group::Module<T, I>;

type Hiring = hiring::Module<Runtime>;

fn add_opening(
    member_id: MemberId,
    account_id: [u8; 32],
    activate_at: hiring::ActivateOpeningAt<BlockNumber>,
    opening_policy_commitment: Option<OpeningPolicyCommitment<BlockNumber, u128>>,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) -> u64 {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let opening_id = match working_group {
        WorkingGroup::Content => {
            let opening_id = ContentDirectoryWorkingGroup::next_opening_id();
            assert!(!<working_group::OpeningById<
                Runtime,
                ContentDirectoryWorkingGroupInstance,
            >>::contains_key(opening_id));
            opening_id
        }
        WorkingGroup::Storage => {
            let opening_id = StorageWorkingGroup::next_opening_id();
            assert!(!<working_group::OpeningById<
                Runtime,
                StorageWorkingGroupInstance,
            >>::contains_key(opening_id));
            opening_id
        }
        WorkingGroup::Operations => {
            let opening_id = OperationsWorkingGroup::next_opening_id();
            assert!(!<working_group::OpeningById<
                Runtime,
                OperationsWorkingGroupInstance,
            >>::contains_key(opening_id));
            opening_id
        }
        WorkingGroup::Gateway => {
            let opening_id = GatewayWorkingGroup::next_opening_id();
            assert!(!<working_group::OpeningById<
                Runtime,
                GatewayWorkingGroupInstance,
            >>::contains_key(opening_id));
            opening_id
        }
    };

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_add_working_group_leader_opening_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id as u64,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(100_000_u32)),
            AddOpeningParameters {
                activate_at: activate_at.clone(),
                commitment: opening_policy_commitment
                    .clone()
                    .unwrap_or(OpeningPolicyCommitment::default()),
                human_readable_text: Vec::new(),
                working_group,
            },
        )
    })
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();

    opening_id
}

fn begin_review_applications(
    member_id: MemberId,
    account_id: [u8; 32],
    opening_id: u64,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_begin_review_working_group_leader_applications_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(25_000_u32)),
            opening_id,
            working_group,
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn fill_opening(
    member_id: MemberId,
    account_id: [u8; 32],
    opening_id: u64,
    successful_application_id: u64,
    reward_policy: Option<RewardPolicy<Balance, BlockNumber>>,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_fill_working_group_leader_opening_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(50_000_u32)),
            proposals_codex::FillOpeningParameters {
                opening_id,
                successful_application_id,
                reward_policy: reward_policy.clone(),
                working_group,
            },
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn get_stake_balance(stake: stake::Stake<BlockNumber, Balance, u64>) -> Balance {
    if let stake::StakingStatus::Staked(stake) = stake.staking_status {
        return stake.staked_amount;
    }

    panic!("Not staked.");
}

fn decrease_stake(
    member_id: u64,
    account_id: [u8; 32],
    leader_worker_id: u64,
    stake_amount: Balance,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_decrease_working_group_leader_stake_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(50_000_u32)),
            leader_worker_id,
            stake_amount,
            working_group,
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn slash_stake(
    member_id: MemberId,
    account_id: [u8; 32],
    leader_worker_id: ActorId,
    stake_amount: Balance,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_slash_working_group_leader_stake_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(50_000_u32)),
            leader_worker_id,
            stake_amount,
            working_group,
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn set_reward(
    member_id: MemberId,
    account_id: [u8; 32],
    leader_worker_id: u64,
    reward_amount: Balance,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_set_working_group_leader_reward_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id as u64,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(50_000_u32)),
            leader_worker_id,
            reward_amount,
            working_group,
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn set_mint_capacity<
    T: working_group::Config<I> + frame_system::Config + minting::Config,
    I: frame_support::traits::Instance,
>(
    member_id: MemberId,
    account_id: [u8; 32],
    mint_capacity: Balance,
    sequence_number: u32, // action sequence number to align with other actions
    setup_environment: bool,
    working_group: WorkingGroup,
) where
    <T as minting::Config>::MintId: From<u64>,
{
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let mint_id_result = <minting::Module<Runtime>>::add_mint(0, None);

    if let Ok(mint_id) = mint_id_result {
        let mint_id: <T as minting::Config>::MintId = mint_id.into();
        <working_group::Mint<T, I>>::put(mint_id);
    }

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_set_working_group_mint_capacity_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(50_000_u32)),
            mint_capacity,
            working_group,
        )
    })
    .with_setup_enviroment(setup_environment)
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

fn terminate_role(
    member_id: MemberId,
    account_id: [u8; 32],
    leader_worker_id: u64,
    slash: bool,
    sequence_number: u32, // action sequence number to align with other actions
    working_group: WorkingGroup,
) {
    let expected_proposal_id = sequence_number;
    let run_to_block = sequence_number * 2;

    let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
        ProposalsCodex::create_terminate_working_group_leader_role_proposal(
            RawOrigin::Signed(account_id.into()).into(),
            member_id,
            b"title".to_vec(),
            b"body".to_vec(),
            Some(<BalanceOf<Runtime>>::from(100_000_u32)),
            proposals_codex::TerminateRoleParameters {
                worker_id: leader_worker_id,
                rationale: Vec::new(),
                slash,
                working_group,
            },
        )
    })
    .disable_setup_enviroment()
    .with_expected_proposal_id(expected_proposal_id)
    .with_run_to_block(run_to_block);

    codex_extrinsic_test_fixture.call_extrinsic_and_assert();
}

#[test]
fn create_add_working_group_leader_opening_proposal_execution_succeeds() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        match group {
            WorkingGroup::Content => {
                run_create_add_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    ContentDirectoryWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Storage => {
                run_create_add_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    StorageWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Operations => {
                run_create_add_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    OperationsWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Gateway => {
                run_create_add_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    GatewayWorkingGroupInstance,
                >(group);
            }
        }
    }
}

fn run_create_add_working_group_leader_opening_proposal_execution_succeeds<
    T: working_group::Config<I> + frame_system::Config + stake::Config,
    I: frame_support::traits::Instance,
>(
    working_group: WorkingGroup,
) where
    <T as membership::Config>::MemberId: From<u64>,
    <T as hiring::Config>::OpeningId: From<u64>,
{
    initial_test_ext().execute_with(|| {
        let member_id: MemberId = 1;
        let account_id: [u8; 32] = [member_id as u8; 32];

        let next_opening_id = WorkingGroupInstance::<T, I>::next_opening_id();

        assert!(!<working_group::OpeningById<T, I>>::contains_key(
            next_opening_id
        ));

        let opening_id: <T as hiring::Config>::OpeningId = add_opening(
            member_id,
            account_id,
            ActivateOpeningAt::CurrentBlock,
            None,
            1,
            working_group,
        )
        .into();

        // Check for expected opening id.
        assert_eq!(opening_id, next_opening_id);

        // Check for the new opening creation.
        assert!(<working_group::OpeningById<T, I>>::contains_key(opening_id));
    });
}

#[test]
fn create_begin_review_working_group_leader_applications_proposal_execution_succeeds() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        match group {
            WorkingGroup::Content => {
                run_create_begin_review_working_group_leader_applications_proposal_execution_succeeds::<
                Runtime,
                ContentDirectoryWorkingGroupInstance,
            >(group);
            }
            WorkingGroup::Storage => {
                run_create_begin_review_working_group_leader_applications_proposal_execution_succeeds::<
                Runtime,
                StorageWorkingGroupInstance,
            >(group);
            }
            WorkingGroup::Operations => {
                run_create_begin_review_working_group_leader_applications_proposal_execution_succeeds::<
                Runtime,
                OperationsWorkingGroupInstance,
            >(group);
            }
            WorkingGroup::Gateway => {
                run_create_begin_review_working_group_leader_applications_proposal_execution_succeeds::<
                Runtime,
                GatewayWorkingGroupInstance,
            >(group);
            }
        }
    }
}

fn run_create_begin_review_working_group_leader_applications_proposal_execution_succeeds<
    T: working_group::Config<I> + frame_system::Config + stake::Config,
    I: frame_support::traits::Instance,
>(
    working_group: WorkingGroup,
) where
    <T as hiring::Config>::OpeningId: From<u64> + Into<u64>,
{
    initial_test_ext().execute_with(|| {
        let member_id: MemberId = 1;
        let account_id: [u8; 32] = [member_id as u8; 32];

        let opening_id = add_opening(
            member_id,
            account_id,
            ActivateOpeningAt::CurrentBlock,
            None,
            1,
            working_group,
        );

        let opening = WorkingGroupInstance::<T, I>::opening_by_id(
            <T as hiring::Config>::OpeningId::from(opening_id),
        );

        let hiring_opening_id: u64 = opening.hiring_opening_id.into();
        let hiring_opening = Hiring::opening_by_id(hiring_opening_id);
        assert_eq!(
            hiring_opening.stage,
            hiring::OpeningStage::Active {
                stage: hiring::ActiveOpeningStage::AcceptingApplications {
                    started_accepting_applicants_at_block: 0
                },
                applications_added: BTreeSet::new(),
                active_application_count: 0,
                unstaking_application_count: 0,
                deactivated_application_count: 0
            }
        );

        begin_review_applications(member_id, account_id, opening_id, 2, working_group);

        let hiring_opening = Hiring::opening_by_id(hiring_opening_id);
        assert_eq!(
            hiring_opening.stage,
            hiring::OpeningStage::Active {
                stage: hiring::ActiveOpeningStage::ReviewPeriod {
                    started_accepting_applicants_at_block: 0,
                    started_review_period_at_block: 2,
                },
                applications_added: BTreeSet::new(),
                active_application_count: 0,
                unstaking_application_count: 0,
                deactivated_application_count: 0
            }
        );
    });
}

#[test]
fn create_fill_working_group_leader_opening_proposal_execution_succeeds() {
    // This uses strum crate for enum iteration
    for group in WorkingGroup::iter() {
        match group {
            WorkingGroup::Content => {
                run_create_fill_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    ContentDirectoryWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Storage => {
                run_create_fill_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    StorageWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Operations => {
                run_create_fill_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    OperationsWorkingGroupInstance,
                >(group);
            }
            WorkingGroup::Gateway => {
                run_create_fill_working_group_leader_opening_proposal_execution_succeeds::<
                    Runtime,
                    GatewayWorkingGroupInstance,
                >(group);
            }
        }
    }

    fn run_create_fill_working_group_leader_opening_proposal_execution_succeeds<
        T: working_group::Config<I> + frame_system::Config + stake::Config,
        I: frame_support::traits::Instance,
    >(
        working_group: WorkingGroup,
    ) where
        <T as frame_system::Config>::AccountId: From<[u8; 32]>,
        <T as membership::Config>::MemberId: From<u64>,
        <T as hiring::Config>::OpeningId: From<u64>,
    {
        initial_test_ext().execute_with(|| {
            let member_id: MemberId = 1;
            let account_id: [u8; 32] = [member_id as u8; 32];

            let opening_id = add_opening(
                member_id,
                account_id,
                ActivateOpeningAt::CurrentBlock,
                None,
                1,
                working_group,
            );

            let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                RawOrigin::Signed(account_id.into()).into(),
                member_id.into(),
                opening_id.into(),
                account_id.into(),
                None,
                None,
                Vec::new(),
            );

            assert_eq!(apply_result, Ok(()));

            let expected_application_id = 0;

            begin_review_applications(member_id, account_id, opening_id, 2, working_group);

            let lead = WorkingGroupInstance::<T, I>::current_lead();
            assert!(lead.is_none());

            fill_opening(
                member_id,
                account_id,
                opening_id,
                expected_application_id,
                None,
                3,
                working_group,
            );

            let lead = WorkingGroupInstance::<T, I>::current_lead();
            assert!(lead.is_some());
        });
    }

    #[test]
    fn create_decrease_group_leader_stake_proposal_execution_succeeds() {
        // This uses strum crate for enum iteration
        for group in WorkingGroup::iter() {
            match group {
                WorkingGroup::Content => {
                    run_create_decrease_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        ContentDirectoryWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Storage => {
                    run_create_decrease_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        StorageWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Operations => {
                    run_create_decrease_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        OperationsWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Gateway => {
                    run_create_decrease_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        GatewayWorkingGroupInstance,
                    >(group);
                }
            }
        }
    }

    fn run_create_decrease_group_leader_stake_proposal_execution_succeeds<
        T: working_group::Config<I> + frame_system::Config + stake::Config,
        I: frame_support::traits::Instance,
    >(
        working_group: WorkingGroup,
    ) where
        <T as frame_system::Config>::AccountId: From<[u8; 32]>,
        <T as hiring::Config>::OpeningId: From<u64>,
        <T as membership::Config>::MemberId: From<u64>,
        <T as membership::Config>::ActorId: Into<u64>,
        <<T as stake::Config>::Currency as traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        initial_test_ext().execute_with(|| {
            let member_id: MemberId = 1;
            let account_id: [u8; 32] = [member_id as u8; 32];
            let stake_amount: Balance = 100;

            let opening_policy_commitment = OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    amount: 100,
                    amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                    crowded_out_unstaking_period_length: None,
                    review_period_expired_unstaking_period_length: None,
                }),
                ..OpeningPolicyCommitment::default()
            };

            let opening_id = add_opening(
                member_id,
                account_id,
                ActivateOpeningAt::CurrentBlock,
                Some(opening_policy_commitment),
                1,
                working_group,
            );

            let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                RawOrigin::Signed(account_id.into()).into(),
                member_id.into(),
                opening_id.into(),
                account_id.into(),
                Some(stake_amount.into()),
                None,
                Vec::new(),
            );

            assert_eq!(apply_result, Ok(()));

            let expected_application_id = 0;

            begin_review_applications(member_id, account_id, opening_id, 2, working_group);

            let lead = WorkingGroupInstance::<T, I>::current_lead();
            assert!(lead.is_none());

            fill_opening(
                member_id,
                account_id,
                opening_id,
                expected_application_id,
                None,
                3,
                working_group,
            );

            let leader_worker_id = WorkingGroupInstance::<T, I>::current_lead().unwrap();

            let stake_id = 1;
            let old_balance = Balances::free_balance(&account_id.into());
            let old_stake = <stake::Module<Runtime>>::stakes(stake_id);

            assert_eq!(get_stake_balance(old_stake), stake_amount);

            let decreasing_stake_amount = 30;
            decrease_stake(
                member_id,
                account_id,
                leader_worker_id.into(),
                decreasing_stake_amount,
                4,
                working_group,
            );

            let new_balance = Balances::free_balance(&account_id.into());
            let new_stake = <stake::Module<Runtime>>::stakes(stake_id);

            assert_eq!(
                get_stake_balance(new_stake),
                stake_amount - decreasing_stake_amount
            );
            assert_eq!(new_balance, old_balance + decreasing_stake_amount);
        });
    }

    #[test]
    fn create_slash_group_leader_stake_proposal_execution_succeeds() {
        // This uses strum crate for enum iteration
        for group in WorkingGroup::iter() {
            match group {
                WorkingGroup::Content => {
                    run_create_slash_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        ContentDirectoryWorkingGroupInstance,
                    >(group)
                }
                WorkingGroup::Storage => {
                    run_create_slash_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        StorageWorkingGroupInstance,
                    >(group)
                }
                WorkingGroup::Operations => {
                    run_create_slash_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        OperationsWorkingGroupInstance,
                    >(group)
                }
                WorkingGroup::Gateway => {
                    run_create_slash_group_leader_stake_proposal_execution_succeeds::<
                        Runtime,
                        GatewayWorkingGroupInstance,
                    >(group)
                }
            }
        }
    }

    fn run_create_slash_group_leader_stake_proposal_execution_succeeds<
        T: working_group::Config<I> + frame_system::Config + stake::Config,
        I: frame_support::traits::Instance,
    >(
        working_group: WorkingGroup,
    ) where
        <T as frame_system::Config>::AccountId: From<[u8; 32]>,
        <T as hiring::Config>::OpeningId: From<u64>,
        <T as membership::Config>::MemberId: From<u64>,
        <T as membership::Config>::ActorId: Into<u64>,
        <<T as stake::Config>::Currency as traits::Currency<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        initial_test_ext().execute_with(|| {
            let member_id: MemberId = 1;
            let account_id: [u8; 32] = [member_id as u8; 32];
            let stake_amount: Balance = 100;

            let opening_policy_commitment = OpeningPolicyCommitment {
                role_staking_policy: Some(hiring::StakingPolicy {
                    amount: 100,
                    amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                    crowded_out_unstaking_period_length: None,
                    review_period_expired_unstaking_period_length: None,
                }),
                ..OpeningPolicyCommitment::default()
            };

            let opening_id = add_opening(
                member_id,
                account_id,
                ActivateOpeningAt::CurrentBlock,
                Some(opening_policy_commitment),
                1,
                working_group,
            );

            let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                RawOrigin::Signed(account_id.into()).into(),
                member_id.into(),
                opening_id.into(),
                account_id.into(),
                Some(stake_amount.into()),
                None,
                Vec::new(),
            );

            assert_eq!(apply_result, Ok(()));

            let expected_application_id = 0;

            begin_review_applications(member_id, account_id, opening_id, 2, working_group);

            let lead = WorkingGroupInstance::<T, I>::current_lead();

            assert!(lead.is_none());

            fill_opening(
                member_id,
                account_id,
                opening_id,
                expected_application_id,
                None,
                3,
                working_group,
            );

            let leader_worker_id = WorkingGroupInstance::<T, I>::current_lead().unwrap();

            let stake_id = 1;
            let old_balance = Balances::free_balance(&account_id.into());
            let old_stake = <stake::Module<Runtime>>::stakes(stake_id);

            assert_eq!(get_stake_balance(old_stake), stake_amount);

            let slashing_stake_amount = 30;
            slash_stake(
                member_id,
                account_id,
                leader_worker_id.into(),
                slashing_stake_amount,
                4,
                working_group,
            );

            let new_balance = Balances::free_balance(&account_id.into());
            let new_stake = <stake::Module<Runtime>>::stakes(stake_id);

            assert_eq!(
                get_stake_balance(new_stake),
                stake_amount as u128 - slashing_stake_amount
            );
            assert_eq!(new_balance, old_balance);
        });
    }

    #[test]
    fn create_set_working_group_mint_capacity_proposal_execution_succeeds() {
        // This uses strum crate for enum iteration
        for group in WorkingGroup::iter() {
            match group {
                WorkingGroup::Content => {
                    run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                        Runtime,
                        ContentDirectoryWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Storage => {
                    run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                        Runtime,
                        StorageWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Operations => {
                    run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                        Runtime,
                        OperationsWorkingGroupInstance,
                    >(group);
                }
                WorkingGroup::Gateway => {
                    run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                        Runtime,
                        GatewayWorkingGroupInstance,
                    >(group);
                }
            }
        }

        fn run_create_set_working_group_mint_capacity_proposal_execution_succeeds<
            T: working_group::Config<I> + frame_system::Config + minting::Config,
            I: frame_support::traits::Instance,
        >(
            working_group: WorkingGroup,
        ) where
            <T as frame_system::Config>::AccountId: From<[u8; 32]>,
            <T as membership::Config>::MemberId: From<u64>,
            <T as minting::Config>::MintId: From<u64>,
            <<T as minting::Config>::Currency as traits::Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance: From<u128>,
        {
            initial_test_ext().execute_with(|| {
                let member_id: MemberId = 1;
                let account_id: [u8; 32] = [member_id as u8; 32];

                assert_eq!(WorkingGroupInstance::<T, I>::mint(), 0.into());

                let mint_capacity = 999999;
                set_mint_capacity::<T, I>(
                    member_id,
                    account_id,
                    mint_capacity,
                    1,
                    true,
                    working_group,
                );

                let mint_id = WorkingGroupInstance::<T, I>::mint();
                let mint = <minting::Module<T>>::mints(mint_id);

                assert_eq!(mint.capacity(), mint_capacity.into());
            });
        }

        #[test]
        fn create_set_group_leader_reward_proposal_execution_succeeds() {
            // This uses strum crate for enum iteration
            for group in WorkingGroup::iter() {
                match group {
                    WorkingGroup::Content => {
                        run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                            Runtime,
                            ContentDirectoryWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Storage => {
                        run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                            Runtime,
                            StorageWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Operations => {
                        run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                            Runtime,
                            OperationsWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Gateway => {
                        run_create_set_working_group_mint_capacity_proposal_execution_succeeds::<
                            Runtime,
                            GatewayWorkingGroupInstance,
                        >(group);
                    }
                }
            }
        }

        fn run_create_set_group_leader_reward_proposal_execution_succeeds<
            T: working_group::Config<I> + frame_system::Config + minting::Config,
            I: frame_support::traits::Instance,
        >(
            working_group: WorkingGroup,
        ) where
            <T as frame_system::Config>::AccountId: From<[u8; 32]>,
            <T as membership::Config>::MemberId: From<u64>,
            <T as membership::Config>::ActorId: Into<u64>,
            <T as minting::Config>::MintId: From<u64>,
            <T as hiring::Config>::OpeningId: From<u64>,
            <<T as minting::Config>::Currency as traits::Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance: From<u128>,
        {
            initial_test_ext().execute_with(|| {
                let member_id: MemberId = 1;
                let account_id: [u8; 32] = [member_id as u8; 32];
                let stake_amount = 100u32;

                let opening_policy_commitment = OpeningPolicyCommitment {
                    role_staking_policy: Some(hiring::StakingPolicy {
                        amount: 100,
                        amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                        crowded_out_unstaking_period_length: None,
                        review_period_expired_unstaking_period_length: None,
                    }),
                    ..OpeningPolicyCommitment::default()
                };

                let opening_id = add_opening(
                    member_id,
                    account_id,
                    ActivateOpeningAt::CurrentBlock,
                    Some(opening_policy_commitment),
                    1,
                    working_group,
                );

                let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                    RawOrigin::Signed(account_id.into()).into(),
                    member_id.into(),
                    opening_id.into(),
                    account_id.into(),
                    Some(stake_amount.into()),
                    None,
                    Vec::new(),
                );

                assert_eq!(apply_result, Ok(()));

                let expected_application_id = 0;

                begin_review_applications(member_id, account_id, opening_id, 2, working_group);

                let lead = WorkingGroupInstance::<T, I>::current_lead();
                assert!(lead.is_none());

                let old_reward_amount = 100;
                let reward_policy = Some(RewardPolicy {
                    amount_per_payout: old_reward_amount,
                    next_payment_at_block: 9999,
                    payout_interval: None,
                });

                set_mint_capacity::<T, I>(member_id, account_id, 999999, 3, false, working_group);

                fill_opening(
                    member_id,
                    account_id,
                    opening_id,
                    expected_application_id,
                    reward_policy,
                    4,
                    working_group,
                );

                let leader_worker_id = WorkingGroupInstance::<T, I>::current_lead().unwrap();

                let worker = WorkingGroupInstance::<T, I>::worker_by_id(leader_worker_id);
                let relationship_id = worker.reward_relationship.unwrap();

                let relationship =
                    recurring_rewards::RewardRelationships::<T>::get(relationship_id);
                assert_eq!(relationship.amount_per_payout, old_reward_amount.into());

                let new_reward_amount = 999;
                set_reward(
                    member_id,
                    account_id,
                    leader_worker_id.into(),
                    new_reward_amount,
                    5,
                    working_group,
                );

                let relationship =
                    recurring_rewards::RewardRelationships::<T>::get(relationship_id);
                assert_eq!(relationship.amount_per_payout, new_reward_amount.into());
            });
        }

        #[test]
        fn create_terminate_group_leader_role_proposal_execution_succeeds() {
            // This uses strum crate for enum iteration
            for group in WorkingGroup::iter() {
                match group {
                    WorkingGroup::Content => {
                        run_create_terminate_group_leader_role_proposal_execution_succeeds::<
                            Runtime,
                            ContentDirectoryWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Storage => {
                        run_create_terminate_group_leader_role_proposal_execution_succeeds::<
                            Runtime,
                            StorageWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Operations => {
                        run_create_terminate_group_leader_role_proposal_execution_succeeds::<
                            Runtime,
                            OperationsWorkingGroupInstance,
                        >(group);
                    }
                    WorkingGroup::Gateway => {
                        run_create_terminate_group_leader_role_proposal_execution_succeeds::<
                            Runtime,
                            GatewayWorkingGroupInstance,
                        >(group);
                    }
                }
            }
        }

        fn run_create_terminate_group_leader_role_proposal_execution_succeeds<
            T: working_group::Config<I> + frame_system::Config + minting::Config,
            I: frame_support::traits::Instance,
        >(
            working_group: WorkingGroup,
        ) where
            <T as frame_system::Config>::AccountId: From<[u8; 32]>,
            <T as membership::Config>::MemberId: From<u64>,
            <T as membership::Config>::ActorId: Into<u64>,
            <T as minting::Config>::MintId: From<u64>,
            <T as hiring::Config>::OpeningId: From<u64>,
            <<T as stake::Config>::Currency as traits::Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance: From<u128>,
        {
            initial_test_ext().execute_with(|| {
                let member_id: MemberId = 1;
                let account_id: [u8; 32] = [0; 32];
                let stake_amount = 100_u128;

                let opening_policy_commitment = OpeningPolicyCommitment {
                    role_staking_policy: Some(hiring::StakingPolicy {
                        amount: 100,
                        amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                        crowded_out_unstaking_period_length: None,
                        review_period_expired_unstaking_period_length: None,
                    }),
                    ..OpeningPolicyCommitment::default()
                };

                let opening_id = add_opening(
                    member_id.into(),
                    account_id,
                    ActivateOpeningAt::CurrentBlock,
                    Some(opening_policy_commitment),
                    1,
                    working_group,
                );

                let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                    RawOrigin::Signed(account_id.into()).into(),
                    member_id.into(),
                    opening_id.into(),
                    account_id.into(),
                    Some(stake_amount.into()),
                    None,
                    Vec::new(),
                );

                assert_eq!(apply_result, Ok(()));

                let expected_application_id = 0;

                begin_review_applications(member_id, account_id, opening_id, 2, working_group);

                let lead = WorkingGroupInstance::<T, I>::current_lead();
                assert!(lead.is_none());

                let old_reward_amount = 100;
                let reward_policy = Some(RewardPolicy {
                    amount_per_payout: old_reward_amount,
                    next_payment_at_block: 9999,
                    payout_interval: None,
                });

                set_mint_capacity::<T, I>(member_id, account_id, 999999, 3, false, working_group);

                fill_opening(
                    member_id,
                    account_id,
                    opening_id,
                    expected_application_id,
                    reward_policy,
                    4,
                    working_group,
                );

                let stake_id = 1;
                let old_balance = Balances::free_balance(&account_id.into());
                let old_stake = <stake::Module<Runtime>>::stakes(stake_id);

                assert_eq!(get_stake_balance(old_stake), stake_amount);

                let leader_worker_id = WorkingGroupInstance::<T, I>::current_lead().unwrap();

                terminate_role(
                    member_id,
                    account_id,
                    leader_worker_id.into(),
                    false,
                    5,
                    working_group,
                );

                assert!(WorkingGroupInstance::<T, I>::current_lead().is_none());

                let new_balance = Balances::free_balance(&account_id.into());
                let new_stake = <stake::Module<Runtime>>::stakes(stake_id);

                assert_eq!(new_stake.staking_status, stake::StakingStatus::NotStaked);
                assert_eq!(new_balance, old_balance + stake_amount);
            });
        }

        #[test]
        fn create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds() {
            // This uses strum crate for enum iteration
            for group in WorkingGroup::iter() {
                match group {
                    WorkingGroup::Content => {
                        run_create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds::<Runtime, ContentDirectoryWorkingGroupInstance>(group);
                    }
                    WorkingGroup::Storage => {
                        run_create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds::<Runtime, StorageWorkingGroupInstance>(group);
                    }
                    WorkingGroup::Operations => {
                        run_create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds::<Runtime, OperationsWorkingGroupInstance>(group);
                    }
                    WorkingGroup::Gateway => {
                        run_create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds::<Runtime, GatewayWorkingGroupInstance>(group);
                    }
                }
            }
        }

        fn run_create_terminate_group_leader_role_proposal_with_slashing_execution_succeeds<
            T: working_group::Config<I> + frame_system::Config + minting::Config,
            I: frame_support::traits::Instance,
        >(
            working_group: WorkingGroup,
        ) where
            <T as frame_system::Config>::AccountId: From<[u8; 32]>,
            <T as membership::Config>::MemberId: From<u64>,
            <T as membership::Config>::ActorId: Into<u64>,
            <T as minting::Config>::MintId: From<u64>,
            <T as hiring::Config>::OpeningId: From<u64>,
            <<T as stake::Config>::Currency as traits::Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance: From<u128>,
        {
            initial_test_ext().execute_with(|| {
                let member_id: MemberId = 1;
                let account_id: [u8; 32] = [0; 32];
                let stake_amount = 100_u128;

                let opening_policy_commitment = OpeningPolicyCommitment {
                    role_staking_policy: Some(hiring::StakingPolicy {
                        amount: 100,
                        amount_mode: hiring::StakingAmountLimitMode::AtLeast,
                        crowded_out_unstaking_period_length: None,
                        review_period_expired_unstaking_period_length: None,
                    }),
                    ..OpeningPolicyCommitment::default()
                };

                let opening_id = add_opening(
                    member_id,
                    account_id,
                    ActivateOpeningAt::CurrentBlock,
                    Some(opening_policy_commitment),
                    1,
                    working_group,
                );

                let apply_result = WorkingGroupInstance::<T, I>::apply_on_opening(
                    RawOrigin::Signed(account_id.into()).into(),
                    member_id.into(),
                    opening_id.into(),
                    account_id.into(),
                    Some(stake_amount.into()),
                    None,
                    Vec::new(),
                );

                assert_eq!(apply_result, Ok(()));

                let expected_application_id = 0;

                begin_review_applications(
                    member_id,
                    account_id,
                    opening_id.into(),
                    2,
                    working_group,
                );

                let lead = WorkingGroupInstance::<T, I>::current_lead();
                assert!(lead.is_none());

                let old_reward_amount = 100;
                let reward_policy = Some(RewardPolicy {
                    amount_per_payout: old_reward_amount,
                    next_payment_at_block: 9999,
                    payout_interval: None,
                });

                set_mint_capacity::<T, I>(member_id, account_id, 999999, 3, false, working_group);

                fill_opening(
                    member_id,
                    account_id,
                    opening_id,
                    expected_application_id,
                    reward_policy,
                    4,
                    working_group,
                );

                let stake_id = 1;
                let old_balance = Balances::free_balance(&account_id.into());
                let old_stake = <stake::Module<Runtime>>::stakes(stake_id);

                assert_eq!(get_stake_balance(old_stake), stake_amount);

                let leader_worker_id = WorkingGroupInstance::<T, I>::current_lead().unwrap();

                terminate_role(
                    member_id,
                    account_id,
                    leader_worker_id.into(),
                    true,
                    5,
                    working_group,
                );

                assert!(WorkingGroupInstance::<T, I>::current_lead().is_none());

                let new_balance = Balances::free_balance(&account_id.into());
                let new_stake = <stake::Module<Runtime>>::stakes(stake_id);

                assert_eq!(new_stake.staking_status, stake::StakingStatus::NotStaked);
                assert_eq!(new_balance, old_balance);
            });
        }
    }
}
