use crate::{Call, Runtime};
use common::working_group::WorkingGroup;
use proposals_codex::{ProposalDetails, ProposalDetailsOf, ProposalEncoder};
use working_group::OpeningType;

use codec::Encode;
use frame_support::print;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

// The macro binds working group outer-level Call with the provided inner-level working group
// extrinsic call. Outer-call is defined by the provided WorkingGroup param expression.

//Params:
// - $working_group: expression returning the 'common::working_group::WorkingGroup' enum
// - $working_group_instance_call: expression returning the exact working group instance extrinsic call
macro_rules! wrap_working_group_call {
    ($working_group:expr, $working_group_instance_call:expr) => {{
        match $working_group {
            WorkingGroup::Content => {
                Call::ContentDirectoryWorkingGroup($working_group_instance_call)
            }
            WorkingGroup::Storage => Call::StorageWorkingGroup($working_group_instance_call),
            WorkingGroup::Operations => Call::OperationsWorkingGroup($working_group_instance_call),
            WorkingGroup::Gateway => Call::GatewayWorkingGroup($working_group_instance_call),
        }
    }};
}

/// _ProposalEncoder_ implementation. It encodes extrinsics with proposal details parameters
/// using Runtime Call and parity codec.
pub struct ExtrinsicProposalEncoder;
impl ProposalEncoder<Runtime> for ExtrinsicProposalEncoder {
    fn encode_proposal(proposal_details: ProposalDetailsOf<Runtime>) -> Vec<u8> {
        let call = match proposal_details {
            ProposalDetails::Text(text) => {
                Call::ProposalsCodex(proposals_codex::Call::execute_text_proposal(text))
            }
            ProposalDetails::SetElectionParameters(election_parameters) => Call::CouncilElection(
                governance::election::Call::set_election_parameters(election_parameters),
            ),
            ProposalDetails::Spending(balance, destination) => Call::Council(
                governance::council::Call::spend_from_council_mint(balance, destination),
            ),
            ProposalDetails::SetValidatorCount(_new_validator_count) => {
                #[cfg(not(feature = "standalone"))]
                print("Error: Calling SetValidatorCount encoding option in parachain mode.");
                #[cfg(not(feature = "standalone"))]
                return Vec::new();
                #[cfg(feature = "standalone")]
                Call::Staking(pallet_staking::Call::set_validator_count(
                    _new_validator_count,
                ))
            }
            ProposalDetails::RuntimeUpgrade(wasm_code) => Call::ProposalsCodex(
                proposals_codex::Call::execute_runtime_upgrade_proposal(wasm_code),
            ),
            // ********** Deprecated during the Babylon release.
            ProposalDetails::DeprecatedSetLead(_) => {
                print("Error: Calling deprecated SetLead encoding option.");
                return Vec::new();
            }
            // ********** Deprecated during the Babylon release.
            ProposalDetails::DeprecatedSetContentWorkingGroupMintCapacity(_) => {
                print(
                    "Error: Calling deprecated SetContentWorkingGroupMintCapacity encoding option.",
                );
                return Vec::new();
            }
            // ********** Deprecated during the Nicaea release.
            // It is kept only for backward compatibility in the Pioneer. **********
            ProposalDetails::DeprecatedEvictStorageProvider(_) => {
                print("Error: Calling deprecated EvictStorageProvider encoding option.");
                return Vec::new();
            }
            // ********** Deprecated during the Nicaea release.
            // It is kept only for backward compatibility in the Pioneer. **********
            ProposalDetails::DeprecatedSetStorageRoleParameters(_) => {
                print("Error: Calling deprecated SetStorageRoleParameters encoding option.");
                return Vec::new();
            }
            ProposalDetails::AddWorkingGroupLeaderOpening(add_opening_params) => {
                wrap_working_group_call!(
                    add_opening_params.working_group,
                    Wg::create_add_opening_call(add_opening_params)
                )
            }
            ProposalDetails::BeginReviewWorkingGroupLeaderApplications(
                opening_id,
                working_group,
            ) => wrap_working_group_call!(
                working_group,
                Wg::create_begin_review_applications_call(opening_id)
            ),
            ProposalDetails::FillWorkingGroupLeaderOpening(fill_opening_params) => {
                wrap_working_group_call!(
                    fill_opening_params.working_group,
                    Wg::create_fill_opening_call(fill_opening_params)
                )
            }
            ProposalDetails::SetWorkingGroupMintCapacity(mint_balance, working_group) => {
                wrap_working_group_call!(
                    working_group,
                    Wg::create_set_mint_capacity_call(mint_balance)
                )
            }
            ProposalDetails::DecreaseWorkingGroupLeaderStake(
                worker_id,
                decreasing_stake,
                working_group,
            ) => wrap_working_group_call!(
                working_group,
                Wg::create_decrease_stake_call(worker_id, decreasing_stake)
            ),
            ProposalDetails::SlashWorkingGroupLeaderStake(
                worker_id,
                slashing_stake,
                working_group,
            ) => wrap_working_group_call!(
                working_group,
                Wg::create_slash_stake_call(worker_id, slashing_stake,)
            ),
            ProposalDetails::SetWorkingGroupLeaderReward(
                worker_id,
                reward_amount,
                working_group,
            ) => wrap_working_group_call!(
                working_group,
                Wg::create_set_reward_call(worker_id, reward_amount)
            ),
            ProposalDetails::TerminateWorkingGroupLeaderRole(terminate_role_params) => {
                wrap_working_group_call!(
                    terminate_role_params.working_group,
                    Wg::terminate_role_call(terminate_role_params)
                )
            }
        };

        call.encode()
    }
}

// Working group calls container. It helps to instantiate proper working group instance for calls.
struct Wg<T, I> {
    phantom_module: PhantomData<T>,
    phantom_instance: PhantomData<I>,
}

impl<T, I> Wg<T, I>
where
    T: working_group::Config<I>,
    I: frame_support::traits::Instance,
{
    // Generic call constructor for the add working group opening.
    fn create_add_opening_call(
        add_opening_params: proposals_codex::AddOpeningParameters<
            T::BlockNumber,
            working_group::BalanceOf<T>,
        >,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::add_opening(
            add_opening_params.activate_at,
            add_opening_params.commitment,
            add_opening_params.human_readable_text,
            OpeningType::Leader,
        )
    }

    // Generic call constructor for the begin review working group applications.
    fn create_begin_review_applications_call(
        opening_id: working_group::OpeningId<T>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::begin_applicant_review(opening_id)
    }

    // Generic call constructor for the add working group opening.
    fn create_fill_opening_call(
        fill_opening_params: proposals_codex::FillOpeningParameters<
            T::BlockNumber,
            minting::BalanceOf<T>,
            working_group::OpeningId<T>,
            working_group::ApplicationId<T>,
        >,
    ) -> working_group::Call<T, I> {
        let mut successful_application_ids = BTreeSet::new();
        successful_application_ids.insert(fill_opening_params.successful_application_id);

        working_group::Call::<T, I>::fill_opening(
            fill_opening_params.opening_id,
            successful_application_ids,
            fill_opening_params.reward_policy,
        )
    }

    // Generic call constructor for the working group 'set mit capacity'.
    fn create_set_mint_capacity_call(
        mint_balance: working_group::BalanceOfMint<T>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::set_mint_capacity(mint_balance)
    }

    // Generic call constructor for the working group 'decrease stake'.
    fn create_decrease_stake_call(
        worker_id: working_group::WorkerId<T>,
        decreasing_stake: working_group::BalanceOf<T>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::decrease_stake(worker_id, decreasing_stake)
    }

    // Generic call constructor for the working group 'slash stake'.
    fn create_slash_stake_call(
        worker_id: working_group::WorkerId<T>,
        slashing_stake: working_group::BalanceOf<T>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::slash_stake(worker_id, slashing_stake)
    }

    // Generic call constructor for the working group 'update reward amount'.
    fn create_set_reward_call(
        worker_id: working_group::WorkerId<T>,
        reward_amount: working_group::BalanceOfMint<T>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::update_reward_amount(worker_id, reward_amount)
    }

    // Generic call constructor for the working group 'terminate role'.
    fn terminate_role_call(
        terminate_role_params: proposals_codex::TerminateRoleParameters<working_group::WorkerId<T>>,
    ) -> working_group::Call<T, I> {
        working_group::Call::<T, I>::terminate_role(
            terminate_role_params.worker_id,
            terminate_role_params.rationale,
            terminate_role_params.slash,
        )
    }
}
