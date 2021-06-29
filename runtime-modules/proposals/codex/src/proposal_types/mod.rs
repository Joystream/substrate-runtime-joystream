#![warn(missing_docs)]

pub(crate) mod parameters;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

use crate::ElectionParameters;
use common::working_group::WorkingGroup;

/// Encodes proposal using its details information.
pub trait ProposalEncoder<T: crate::Config> {
    /// Encodes proposal using its details information.
    fn encode_proposal(proposal_details: ProposalDetailsOf<T>) -> Vec<u8>;
}

/// _ProposalDetails_ alias for type simplification
pub type ProposalDetailsOf<T> = ProposalDetails<
    crate::BalanceOfMint<T>,
    crate::BalanceOfGovernanceCurrency<T>,
    <T as frame_system::Config>::BlockNumber,
    <T as frame_system::Config>::AccountId,
    working_group::OpeningId<T>,
    working_group::ApplicationId<T>,
    crate::BalanceOf<T>,
    working_group::WorkerId<T>,
    crate::MemberId<T>,
>;

/// Proposal details provide voters the information required for the perceived voting.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum ProposalDetails<
    MintedBalance,
    CurrencyBalance,
    BlockNumber,
    AccountId,
    OpeningId,
    ApplicationId,
    StakeBalance,
    WorkerId,
    MemberId,
> {
    /// The text of the `text` proposal
    Text(Vec<u8>),

    /// The wasm code for the `runtime upgrade` proposal
    RuntimeUpgrade(Vec<u8>),

    /// Election parameters for the `set election parameters` proposal
    SetElectionParameters(ElectionParameters<CurrencyBalance, BlockNumber>),

    /// Balance and destination account for the `spending` proposal
    Spending(MintedBalance, AccountId),

    /// ********** Deprecated during the Babylon release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// New leader memberId and account_id for the `set lead` proposal
    DeprecatedSetLead(Option<(MemberId, AccountId)>),

    /// ********** Deprecated during the Babylon release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// Balance for the `set content working group mint capacity` proposal
    DeprecatedSetContentWorkingGroupMintCapacity(MintedBalance),

    /// ********** Deprecated during the Nicaea release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// AccountId for the `evict storage provider` proposal
    DeprecatedEvictStorageProvider(AccountId),

    /// Validator count for the `set validator count` proposal
    SetValidatorCount(u32),

    /// ********** Deprecated during the Nicaea release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// Role parameters for the `set storage role parameters` proposal
    DeprecatedSetStorageRoleParameters(RoleParameters<CurrencyBalance, BlockNumber>),

    /// Add opening for the working group leader position.
    AddWorkingGroupLeaderOpening(AddOpeningParameters<BlockNumber, CurrencyBalance>),

    /// Begin review applications for the working group leader position.
    BeginReviewWorkingGroupLeaderApplications(OpeningId, WorkingGroup),

    /// Fill opening for the working group leader position.
    FillWorkingGroupLeaderOpening(
        FillOpeningParameters<BlockNumber, MintedBalance, OpeningId, ApplicationId>,
    ),

    /// Set working group mint capacity.
    SetWorkingGroupMintCapacity(MintedBalance, WorkingGroup),

    /// Decrease the working group leader stake.
    DecreaseWorkingGroupLeaderStake(WorkerId, StakeBalance, WorkingGroup),

    /// Slash the working group leader stake.
    SlashWorkingGroupLeaderStake(WorkerId, StakeBalance, WorkingGroup),

    /// Set working group leader reward balance.
    SetWorkingGroupLeaderReward(WorkerId, MintedBalance, WorkingGroup),

    /// Fire the working group leader with possible slashing.
    TerminateWorkingGroupLeaderRole(TerminateRoleParameters<WorkerId>),
}

impl<
        MintedBalance,
        CurrencyBalance,
        BlockNumber,
        AccountId,
        OpeningId,
        ApplicationId,
        StakeBalance,
        WorkerId,
        MemberId,
    > Default
    for ProposalDetails<
        MintedBalance,
        CurrencyBalance,
        BlockNumber,
        AccountId,
        OpeningId,
        ApplicationId,
        StakeBalance,
        WorkerId,
        MemberId,
    >
{
    fn default() -> Self {
        ProposalDetails::Text(b"invalid proposal details".to_vec())
    }
}

/// Parameters for the 'terminate the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct TerminateRoleParameters<WorkerId> {
    /// Leader worker id to fire.
    pub worker_id: WorkerId,

    /// Terminate role rationale.
    pub rationale: Vec<u8>,

    /// Slash the leader stake on terminating.
    pub slash: bool,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// Parameters for the 'fill opening for the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct FillOpeningParameters<BlockNumber, Balance, OpeningId, ApplicationId> {
    /// Finalizing opening id.
    pub opening_id: OpeningId,

    /// Id of the selected application.
    pub successful_application_id: ApplicationId,

    /// Position reward policy.
    pub reward_policy: Option<working_group::RewardPolicy<Balance, BlockNumber>>,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// Parameters for the 'add opening for the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct AddOpeningParameters<BlockNumber, Balance> {
    /// Activate opening at block.
    pub activate_at: hiring::ActivateOpeningAt<BlockNumber>,

    /// Opening conditions.
    pub commitment: working_group::OpeningPolicyCommitment<BlockNumber, Balance>,

    /// Opening description.
    pub human_readable_text: Vec<u8>,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// ********** Deprecated during the Nicaea release.
/// It is kept only for backward compatibility in the Pioneer. **********
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RoleParameters<Balance, BlockNumber> {
    /// Minimum balance required to stake to enter a role.
    pub min_stake: Balance,

    /// Minimum actors to maintain - if role is unstaking
    /// and remaining actors would be less that this value - prevent or punish for unstaking.
    pub min_actors: u32,

    /// The maximum number of spots available to fill for a role.
    pub max_actors: u32,

    /// Fixed amount of tokens paid to actors' primary account.
    pub reward: Balance,

    /// Payouts are made at this block interval.
    pub reward_period: BlockNumber,

    /// Minimum amount of time before being able to unstake.
    pub bonding_period: BlockNumber,

    /// How long tokens remain locked for after unstaking.
    pub unbonding_period: BlockNumber,

    /// Minimum period required to be in service. unbonding before this time is highly penalized
    pub min_service_period: BlockNumber,

    /// "Startup" time allowed for roles that need to sync their infrastructure
    /// with other providers before they are considered in service and punishable for
    /// not delivering required level of service.
    pub startup_grace_period: BlockNumber,

    /// Small fee burned to make a request to enter role.
    pub entry_request_fee: Balance,
}

/// Contains proposal config parameters. Default values are used by migration and genesis config.
#[derive(Copy, Clone)]
pub struct ProposalsConfigParameters {
    /// 'Set validator count' proposal voting period
    pub set_validator_count_proposal_voting_period: u32,

    /// 'Set validator count' proposal grace period
    pub set_validator_count_proposal_grace_period: u32,

    /// 'Runtime upgrade' proposal voting period
    pub runtime_upgrade_proposal_voting_period: u32,

    /// 'Runtime upgrade' proposal grace period
    pub runtime_upgrade_proposal_grace_period: u32,

    /// 'Text' proposal voting period
    pub text_proposal_voting_period: u32,

    /// 'Text' proposal grace period
    pub text_proposal_grace_period: u32,

    /// 'Set election parameters' proposal voting period
    pub set_election_parameters_proposal_voting_period: u32,

    /// 'Set election parameters' proposal grace period
    pub set_election_parameters_proposal_grace_period: u32,

    /// 'Set lead' proposal voting period
    pub set_lead_proposal_voting_period: u32,

    /// 'Set lead' proposal grace period
    pub set_lead_proposal_grace_period: u32,

    /// 'Spending' proposal voting period
    pub spending_proposal_voting_period: u32,

    /// 'Spending' proposal grace period
    pub spending_proposal_grace_period: u32,

    /// 'Add working group opening' proposal voting period
    pub add_working_group_opening_proposal_voting_period: u32,

    /// 'Add working group opening' proposal grace period
    pub add_working_group_opening_proposal_grace_period: u32,

    /// 'Begin review working group leader applications' proposal voting period
    pub begin_review_working_group_leader_applications_proposal_voting_period: u32,

    /// 'Begin review working group leader applications' proposal grace period
    pub begin_review_working_group_leader_applications_proposal_grace_period: u32,

    /// 'Fill working group leader opening' proposal voting period
    pub fill_working_group_leader_opening_proposal_voting_period: u32,

    /// 'Fill working group leader opening' proposal grace period
    pub fill_working_group_leader_opening_proposal_grace_period: u32,

    /// 'Set working group mint capacity' proposal voting period
    pub set_working_group_mint_capacity_proposal_voting_period: u32,

    /// 'Set working group mint capacity' proposal grace period
    pub set_working_group_mint_capacity_proposal_grace_period: u32,

    /// 'Decrease working group leader stake' proposal voting period
    pub decrease_working_group_leader_stake_proposal_voting_period: u32,

    /// 'Decrease working group leader stake' proposal grace period
    pub decrease_working_group_leader_stake_proposal_grace_period: u32,

    /// 'Slash working group leader stake' proposal voting period
    pub slash_working_group_leader_stake_proposal_voting_period: u32,

    /// 'Slash working group leader stake' proposal grace period
    pub slash_working_group_leader_stake_proposal_grace_period: u32,

    /// 'Set working group leader reward' proposal voting period
    pub set_working_group_leader_reward_proposal_voting_period: u32,

    /// 'Set working group leader reward' proposal grace period
    pub set_working_group_leader_reward_proposal_grace_period: u32,

    /// 'Terminate working group leader role' proposal voting period
    pub terminate_working_group_leader_role_proposal_voting_period: u32,

    /// 'Terminate working group leader role' proposal grace period
    pub terminate_working_group_leader_role_proposal_grace_period: u32,
}

impl Default for ProposalsConfigParameters {
    fn default() -> Self {
        ProposalsConfigParameters {
            set_validator_count_proposal_voting_period: 43200u32,
            set_validator_count_proposal_grace_period: 0u32,
            runtime_upgrade_proposal_voting_period: 72000u32,
            runtime_upgrade_proposal_grace_period: 28800u32,
            text_proposal_voting_period: 72000u32,
            text_proposal_grace_period: 0u32,
            set_election_parameters_proposal_voting_period: 72000u32,
            set_election_parameters_proposal_grace_period: 201_601_u32,
            set_lead_proposal_voting_period: 43200u32,
            set_lead_proposal_grace_period: 0u32,
            spending_proposal_voting_period: 72000u32,
            spending_proposal_grace_period: 0u32,
            add_working_group_opening_proposal_voting_period: 72000u32,
            add_working_group_opening_proposal_grace_period: 0u32,
            begin_review_working_group_leader_applications_proposal_voting_period: 43200u32,
            begin_review_working_group_leader_applications_proposal_grace_period: 14400u32,
            fill_working_group_leader_opening_proposal_voting_period: 43200u32,
            fill_working_group_leader_opening_proposal_grace_period: 0u32,
            set_working_group_mint_capacity_proposal_voting_period: 43200u32,
            set_working_group_mint_capacity_proposal_grace_period: 0u32,
            decrease_working_group_leader_stake_proposal_voting_period: 43200u32,
            decrease_working_group_leader_stake_proposal_grace_period: 0u32,
            slash_working_group_leader_stake_proposal_voting_period: 43200u32,
            slash_working_group_leader_stake_proposal_grace_period: 0u32,
            set_working_group_leader_reward_proposal_voting_period: 43200u32,
            set_working_group_leader_reward_proposal_grace_period: 0u32,
            terminate_working_group_leader_role_proposal_voting_period: 72200u32,
            terminate_working_group_leader_role_proposal_grace_period: 0u32,
        }
    }
}

impl ProposalsConfigParameters {
    /// Set same voting_period for all proposals. For proposals
    /// that by default have 0 grace period remain with 0 grace period.
    /// All remaining proposals get assigned grace_period.
    pub fn with_grace_and_voting_periods(grace_period: u32, voting_period: u32) -> Self {
        ProposalsConfigParameters {
            set_validator_count_proposal_voting_period: voting_period,
            set_validator_count_proposal_grace_period: 0,
            runtime_upgrade_proposal_voting_period: voting_period,
            runtime_upgrade_proposal_grace_period: grace_period,
            text_proposal_voting_period: voting_period,
            text_proposal_grace_period: 0,
            set_election_parameters_proposal_voting_period: voting_period,
            set_election_parameters_proposal_grace_period: grace_period,
            set_lead_proposal_voting_period: voting_period,
            set_lead_proposal_grace_period: 0,
            spending_proposal_voting_period: voting_period,
            spending_proposal_grace_period: 0,
            add_working_group_opening_proposal_voting_period: voting_period,
            add_working_group_opening_proposal_grace_period: 0,
            begin_review_working_group_leader_applications_proposal_voting_period: voting_period,
            begin_review_working_group_leader_applications_proposal_grace_period: grace_period,
            fill_working_group_leader_opening_proposal_voting_period: voting_period,
            fill_working_group_leader_opening_proposal_grace_period: 0,
            set_working_group_mint_capacity_proposal_voting_period: voting_period,
            set_working_group_mint_capacity_proposal_grace_period: 0,
            decrease_working_group_leader_stake_proposal_voting_period: voting_period,
            decrease_working_group_leader_stake_proposal_grace_period: 0,
            slash_working_group_leader_stake_proposal_voting_period: voting_period,
            slash_working_group_leader_stake_proposal_grace_period: 0,
            set_working_group_leader_reward_proposal_voting_period: voting_period,
            set_working_group_leader_reward_proposal_grace_period: 0,
            terminate_working_group_leader_role_proposal_voting_period: voting_period,
            terminate_working_group_leader_role_proposal_grace_period: 0,
        }
    }
}
