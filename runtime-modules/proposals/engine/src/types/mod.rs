//! Proposals types module for the Joystream platform. Version 2.
//! Provides types for the proposal engine.

#![warn(missing_docs)]

use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Currency;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{PerThing, Perbill};
use sp_std::boxed::Box;
use sp_std::cmp::PartialOrd;
use sp_std::ops::Add;
use sp_std::vec::Vec;

mod proposal_statuses;
mod stakes;

pub use proposal_statuses::{
    ApprovedProposalStatus, FinalizationData, ProposalDecisionStatus, ProposalStatus,
};
pub(crate) use stakes::ProposalStakeManager;
pub use stakes::{DefaultStakeHandlerProvider, StakeHandler, StakeHandlerProvider};

#[cfg(test)]
pub(crate) use stakes::DefaultStakeHandler;

#[cfg(test)]
pub(crate) use stakes::MockStakeHandler;

pub(crate) type Balances<T> = balances::Pallet<T>;

/// Vote kind for the proposal. Sum of all votes defines proposal status.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum VoteKind {
    /// Pass, an alternative or a ranking, for binary, multiple choice
    /// and ranked choice propositions, respectively.
    Approve,

    /// Against proposal.
    Reject,

    /// Reject proposal and slash it stake.
    Slash,

    /// Signals presence, but unwillingness to cast judgment on substance of vote.
    Abstain,
}

impl Default for VoteKind {
    fn default() -> Self {
        VoteKind::Reject
    }
}

/// Proposal parameters required to manage proposal risk.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProposalParameters<BlockNumber, Balance> {
    /// During this period, votes can be accepted
    pub voting_period: BlockNumber,

    /// A pause before execution of the approved proposal. Zero means approved proposal would be
    /// executed immediately.
    pub grace_period: BlockNumber,

    /// Quorum percentage of approving voters required to pass the proposal.
    pub approval_quorum_percentage: u32,

    /// Approval votes percentage threshold to pass the proposal.
    pub approval_threshold_percentage: u32,

    /// Quorum percentage of voters required to slash the proposal.
    pub slashing_quorum_percentage: u32,

    /// Slashing votes percentage threshold to slash the proposal.
    pub slashing_threshold_percentage: u32,

    /// Proposal stake
    pub required_stake: Option<Balance>,
}

/// Contains current voting results
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct VotingResults {
    /// 'Abstain' votes counter
    pub abstentions: u32,

    /// 'Approve' votes counter
    pub approvals: u32,

    /// 'Reject' votes counter
    pub rejections: u32,

    /// 'Slash' votes counter
    pub slashes: u32,
}

impl VotingResults {
    /// Add vote to the related counter
    pub fn add_vote(&mut self, vote: VoteKind) {
        match vote {
            VoteKind::Abstain => self.abstentions += 1,
            VoteKind::Approve => self.approvals += 1,
            VoteKind::Reject => self.rejections += 1,
            VoteKind::Slash => self.slashes += 1,
        }
    }

    /// Calculates number of votes so far
    pub fn votes_number(&self) -> u32 {
        self.abstentions + self.approvals + self.rejections + self.slashes
    }
}

/// Contains created stake id and source account for the stake balance
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct ActiveStake<StakeId, AccountId> {
    /// Created stake id for the proposal
    pub stake_id: StakeId,

    /// Source account of the stake balance. Refund if any will be provided using this account
    pub source_account_id: AccountId,
}

/// 'Proposal' contains information necessary for the proposal frame_system functioning.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Proposal<BlockNumber, ProposerId, Balance, StakeId, AccountId> {
    /// Proposals parameter, characterize different proposal types.
    pub parameters: ProposalParameters<BlockNumber, Balance>,

    /// Identifier of member proposing.
    pub proposer_id: ProposerId,

    /// Proposal description
    pub title: Vec<u8>,

    /// Proposal body
    pub description: Vec<u8>,

    /// When it was created.
    pub created_at: BlockNumber,

    /// Current proposal status
    pub status: ProposalStatus<BlockNumber, StakeId, AccountId>,

    /// Curring voting result for the proposal
    pub voting_results: VotingResults,
}

impl<BlockNumber, ProposerId, Balance, StakeId, AccountId>
    Proposal<BlockNumber, ProposerId, Balance, StakeId, AccountId>
where
    BlockNumber: Add<Output = BlockNumber> + PartialOrd + Copy,
    StakeId: Clone,
    AccountId: Clone,
{
    /// Returns whether voting period expired by now
    pub fn is_voting_period_expired(&self, now: BlockNumber) -> bool {
        now >= self.created_at + self.parameters.voting_period
    }

    /// Returns whether grace period expired by now.
    /// Grace period can be expired only if proposal is finalized with Approved status.
    /// Returns false otherwise.
    pub fn is_grace_period_expired(&self, now: BlockNumber) -> bool {
        if let ProposalStatus::Finalized(finalized_status) = self.status.clone() {
            if let ProposalDecisionStatus::Approved(_) = finalized_status.proposal_status {
                return now >= finalized_status.finalized_at + self.parameters.grace_period;
            }
        }

        false
    }

    /// Determines the finalized proposal status using voting results tally for current proposal.
    /// Calculates votes, takes in account voting period expiration.
    /// If voting process is in progress, then decision status is None.
    /// Parameters: current time, total voters number involved (council size).
    /// Returns the proposal finalized status if any.
    pub fn define_proposal_decision_status(
        &self,
        total_voters_count: u32,
        now: BlockNumber,
    ) -> Option<ProposalDecisionStatus> {
        let proposal_status_resolution = ProposalStatusResolution {
            proposal: self,
            approvals: self.voting_results.approvals,
            slashes: self.voting_results.slashes,
            now,
            votes_count: self.voting_results.votes_number(),
            total_voters_count,
        };

        if proposal_status_resolution.is_approval_quorum_reached()
            && proposal_status_resolution.is_approval_threshold_reached()
        {
            Some(ProposalDecisionStatus::Approved(
                ApprovedProposalStatus::PendingExecution,
            ))
        } else if proposal_status_resolution.is_slashing_quorum_reached()
            && proposal_status_resolution.is_slashing_threshold_reached()
        {
            Some(ProposalDecisionStatus::Slashed)
        } else if proposal_status_resolution.is_expired() {
            Some(ProposalDecisionStatus::Expired)
        } else if proposal_status_resolution.is_voting_completed() {
            Some(ProposalDecisionStatus::Rejected)
        } else {
            None
        }
    }

    /// Reset the proposal in Active status. Proposal with other status won't be changed.
    /// Reset proposal operation clears voting results.
    pub fn reset_proposal(&mut self) {
        if let ProposalStatus::Active(_) = self.status.clone() {
            self.voting_results = VotingResults::default();
        }
    }
}

/// Provides data for the voting.
pub trait VotersParameters {
    /// Defines maximum voters count for the proposal
    fn total_voters_count() -> u32;
}

// Calculates quorum, votes threshold, expiration status
struct ProposalStatusResolution<'a, BlockNumber, ProposerId, Balance, StakeId, AccountId> {
    proposal: &'a Proposal<BlockNumber, ProposerId, Balance, StakeId, AccountId>,
    now: BlockNumber,
    votes_count: u32,
    total_voters_count: u32,
    approvals: u32,
    slashes: u32,
}

impl<'a, BlockNumber, ProposerId, Balance, StakeId, AccountId>
    ProposalStatusResolution<'a, BlockNumber, ProposerId, Balance, StakeId, AccountId>
where
    BlockNumber: Add<Output = BlockNumber> + PartialOrd + Copy,
    StakeId: Clone,
    AccountId: Clone,
{
    // Proposal has been expired and quorum not reached.
    pub fn is_expired(&self) -> bool {
        self.proposal.is_voting_period_expired(self.now)
    }

    // Approval quorum reached for the proposal. Compares predefined parameter with actual
    // votes sum divided by total possible votes count.
    pub fn is_approval_quorum_reached(&self) -> bool {
        let actual_votes_fraction: Perbill =
            PerThing::from_rational(self.votes_count, self.total_voters_count);
        let approval_quorum_fraction =
            Perbill::from_percent(self.proposal.parameters.approval_quorum_percentage);

        actual_votes_fraction.deconstruct() >= approval_quorum_fraction.deconstruct()
    }

    // Slashing quorum reached for the proposal. Compares predefined parameter with actual
    // votes sum divided by total possible votes count.
    pub fn is_slashing_quorum_reached(&self) -> bool {
        let actual_votes_fraction: Perbill =
            PerThing::from_rational(self.votes_count, self.total_voters_count);
        let slashing_quorum_fraction =
            Perbill::from_percent(self.proposal.parameters.slashing_quorum_percentage);

        actual_votes_fraction.deconstruct() >= slashing_quorum_fraction.deconstruct()
    }

    // Approval threshold reached for the proposal. Compares predefined parameter with 'approve'
    // votes sum divided by actual votes count.
    pub fn is_approval_threshold_reached(&self) -> bool {
        let approval_votes_fraction: Perbill =
            PerThing::from_rational(self.approvals, self.votes_count);
        let required_threshold_fraction =
            Perbill::from_percent(self.proposal.parameters.approval_threshold_percentage);

        approval_votes_fraction.deconstruct() >= required_threshold_fraction.deconstruct()
    }

    // Slashing threshold reached for the proposal. Compares predefined parameter with 'approve'
    // votes sum divided by actual votes count.
    pub fn is_slashing_threshold_reached(&self) -> bool {
        let slashing_votes_fraction: Perbill =
            PerThing::from_rational(self.slashes, self.votes_count);
        let required_threshold_fraction =
            Perbill::from_percent(self.proposal.parameters.slashing_threshold_percentage);

        slashing_votes_fraction.deconstruct() >= required_threshold_fraction.deconstruct()
    }

    // All voters had voted
    pub fn is_voting_completed(&self) -> bool {
        self.votes_count == self.total_voters_count
    }
}

/// Proposal executable code wrapper
pub trait ProposalExecutable {
    /// Executes proposal code
    fn execute(&self) -> DispatchResult;
}

/// Proposal code binary converter
pub trait ProposalCodeDecoder<T: frame_system::Config> {
    /// Converts proposal code binary to executable representation
    fn decode_proposal(
        proposal_type: u32,
        proposal_code: Vec<u8>,
    ) -> Result<Box<dyn ProposalExecutable>, &'static str>;
}

/// Balance alias
pub type BalanceOf<T> =
    <<T as stake::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Balance alias for staking
pub type NegativeImbalance<T> = <<T as stake::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Balance type of runtime
pub type CurrencyOf<T> = <T as stake::Config>::Currency;

/// Data container for the finalized proposal results
pub(crate) struct FinalizedProposalData<
    ProposalId,
    BlockNumber,
    ProposerId,
    Balance,
    StakeId,
    AccountId,
> {
    /// Proposal id
    pub proposal_id: ProposalId,

    /// Proposal to be finalized
    pub proposal: Proposal<BlockNumber, ProposerId, Balance, StakeId, AccountId>,

    /// Proposal finalization status
    pub status: ProposalDecisionStatus,

    /// Proposal finalization block number
    pub finalized_at: BlockNumber,
}

/// Data container for the approved proposal results
pub(crate) struct ApprovedProposalData<
    ProposalId,
    BlockNumber,
    ProposerId,
    Balance,
    StakeId,
    AccountId,
> {
    /// Proposal id
    pub proposal_id: ProposalId,

    /// Proposal to be finalized
    pub proposal: Proposal<BlockNumber, ProposerId, Balance, StakeId, AccountId>,

    /// Proposal finalisation status data
    pub finalisation_status_data: FinalizationData<BlockNumber, StakeId, AccountId>,
}

#[cfg(test)]
mod tests {
    use crate::types::ProposalStatusResolution;
    use crate::*;

    // Alias introduced for simplicity of changing Proposal exact types.
    type ProposalObject = Proposal<u64, u64, u64, u64, u64>;

    #[test]
    fn proposal_voting_period_expired() {
        let mut proposal = ProposalObject::default();

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;

        assert!(proposal.is_voting_period_expired(4));
    }

    #[test]
    fn proposal_voting_period_not_expired() {
        let mut proposal = ProposalObject::default();

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;

        assert!(!proposal.is_voting_period_expired(3));
    }

    #[test]
    fn proposal_grace_period_expired() {
        let mut proposal = ProposalObject::default();

        proposal.parameters.grace_period = 3;
        proposal.status = ProposalStatus::finalized_successfully(
            ProposalDecisionStatus::Approved(ApprovedProposalStatus::PendingExecution),
            0,
        );

        assert!(proposal.is_grace_period_expired(4));
    }

    #[test]
    fn proposal_grace_period_auto_expired() {
        let mut proposal = ProposalObject::default();

        proposal.parameters.grace_period = 0;
        proposal.status = ProposalStatus::finalized_successfully(
            ProposalDecisionStatus::Approved(ApprovedProposalStatus::PendingExecution),
            0,
        );

        assert!(proposal.is_grace_period_expired(1));
    }

    #[test]
    fn proposal_grace_period_not_expired() {
        let mut proposal = ProposalObject::default();

        proposal.parameters.grace_period = 3;

        assert!(!proposal.is_grace_period_expired(3));
    }

    #[test]
    fn proposal_grace_period_not_expired_because_of_not_approved_proposal() {
        let mut proposal = ProposalObject::default();

        proposal.parameters.grace_period = 3;

        assert!(!proposal.is_grace_period_expired(3));
    }

    #[test]
    fn define_proposal_decision_status_returns_expired() {
        let mut proposal = ProposalObject::default();
        let now = 5;
        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 80;
        proposal.parameters.approval_threshold_percentage = 40;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 50;

        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Approve);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 2,
                rejections: 1,
                slashes: 0,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(5, now);
        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Expired)
        );
    }

    #[test]
    fn define_proposal_decision_status_returns_approved() {
        let now = 2;
        let mut proposal = ProposalObject::default();
        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 60;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 50;

        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Approve);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 3,
                rejections: 1,
                slashes: 0,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(5, now);
        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Approved(
                ApprovedProposalStatus::PendingExecution
            ))
        );
    }

    #[test]
    fn define_proposal_decision_status_returns_rejected() {
        let mut proposal = ProposalObject::default();
        let now = 2;

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 50;
        proposal.parameters.approval_threshold_percentage = 51;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 50;

        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Abstain);
        proposal.voting_results.add_vote(VoteKind::Approve);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 1,
                rejections: 2,
                slashes: 0,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(4, now);
        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Rejected)
        );
    }

    #[test]
    fn define_proposal_decision_status_returns_slashed() {
        let mut proposal = ProposalObject::default();
        let now = 2;

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 50;
        proposal.parameters.approval_threshold_percentage = 50;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 50;

        proposal.voting_results.add_vote(VoteKind::Slash);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Abstain);
        proposal.voting_results.add_vote(VoteKind::Slash);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 1,
                slashes: 2,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(4, now);
        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Slashed)
        );
    }

    #[test]
    fn define_proposal_decision_status_returns_none() {
        let mut proposal = ProposalObject::default();
        let now = 2;

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 60;
        proposal.parameters.slashing_quorum_percentage = 50;

        proposal.voting_results.add_vote(VoteKind::Abstain);
        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 0,
                slashes: 0,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(5, now);
        assert_eq!(expected_proposal_status, None);
    }

    #[test]
    fn define_proposal_decision_status_returns_approved_before_slashing_before_rejection() {
        let mut proposal = ProposalObject::default();
        let now = 2;

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 50;
        proposal.parameters.approval_threshold_percentage = 30;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 30;

        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Slash);
        proposal.voting_results.add_vote(VoteKind::Slash);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 2,
                rejections: 2,
                slashes: 2,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(6, now);

        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Approved(
                ApprovedProposalStatus::PendingExecution
            ))
        );
    }

    #[test]
    fn define_proposal_decision_status_returns_slashed_before_rejection() {
        let mut proposal = ProposalObject::default();
        let now = 2;

        proposal.created_at = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.approval_quorum_percentage = 50;
        proposal.parameters.approval_threshold_percentage = 30;
        proposal.parameters.slashing_quorum_percentage = 50;
        proposal.parameters.slashing_threshold_percentage = 30;

        proposal.voting_results.add_vote(VoteKind::Abstain);
        proposal.voting_results.add_vote(VoteKind::Approve);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Reject);
        proposal.voting_results.add_vote(VoteKind::Slash);
        proposal.voting_results.add_vote(VoteKind::Slash);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 1,
                rejections: 2,
                slashes: 2,
            }
        );

        let expected_proposal_status = proposal.define_proposal_decision_status(6, now);

        assert_eq!(
            expected_proposal_status,
            Some(ProposalDecisionStatus::Slashed)
        );
    }

    #[test]
    fn proposal_status_resolution_approval_quorum_works_correctly() {
        let no_approval_quorum_proposal: Proposal<u64, u64, u64, u64, u64> = Proposal {
            parameters: ProposalParameters {
                approval_quorum_percentage: 63,
                slashing_threshold_percentage: 63,
                ..ProposalParameters::default()
            },
            ..Proposal::default()
        };
        let no_approval_proposal_status_resolution = ProposalStatusResolution {
            proposal: &no_approval_quorum_proposal,
            now: 20,
            votes_count: 314,
            total_voters_count: 500,
            approvals: 3,
            slashes: 3,
        };

        assert!(!no_approval_proposal_status_resolution.is_approval_quorum_reached());

        let approval_quorum_proposal_status_resolution = ProposalStatusResolution {
            votes_count: 315,
            ..no_approval_proposal_status_resolution
        };

        assert!(approval_quorum_proposal_status_resolution.is_approval_quorum_reached());
    }

    #[test]
    fn proposal_status_resolution_slashing_quorum_works_correctly() {
        let no_slashing_quorum_proposal: Proposal<u64, u64, u64, u64, u64> = Proposal {
            parameters: ProposalParameters {
                approval_quorum_percentage: 63,
                slashing_quorum_percentage: 63,
                ..ProposalParameters::default()
            },
            ..Proposal::default()
        };
        let no_slashing_proposal_status_resolution = ProposalStatusResolution {
            proposal: &no_slashing_quorum_proposal,
            now: 20,
            votes_count: 314,
            total_voters_count: 500,
            approvals: 3,
            slashes: 3,
        };

        assert!(!no_slashing_proposal_status_resolution.is_slashing_quorum_reached());

        let slashing_quorum_proposal_status_resolution = ProposalStatusResolution {
            votes_count: 315,
            ..no_slashing_proposal_status_resolution
        };

        assert!(slashing_quorum_proposal_status_resolution.is_slashing_quorum_reached());
    }

    #[test]
    fn proposal_status_resolution_approval_threshold_works_correctly() {
        let no_approval_threshold_proposal: Proposal<u64, u64, u64, u64, u64> = Proposal {
            parameters: ProposalParameters {
                slashing_threshold_percentage: 63,
                approval_threshold_percentage: 63,
                ..ProposalParameters::default()
            },
            ..Proposal::default()
        };
        let no_approval_proposal_status_resolution = ProposalStatusResolution {
            proposal: &no_approval_threshold_proposal,
            now: 20,
            votes_count: 500,
            total_voters_count: 600,
            approvals: 314,
            slashes: 3,
        };

        assert!(!no_approval_proposal_status_resolution.is_approval_threshold_reached());

        let approval_threshold_proposal_status_resolution = ProposalStatusResolution {
            approvals: 315,
            ..no_approval_proposal_status_resolution
        };

        assert!(approval_threshold_proposal_status_resolution.is_approval_threshold_reached());
    }

    #[test]
    fn proposal_status_resolution_slashing_threshold_works_correctly() {
        let no_slashing_threshold_proposal: Proposal<u64, u64, u64, u64, u64> = Proposal {
            parameters: ProposalParameters {
                slashing_threshold_percentage: 63,
                approval_threshold_percentage: 63,
                ..ProposalParameters::default()
            },
            ..Proposal::default()
        };
        let no_slashing_proposal_status_resolution = ProposalStatusResolution {
            proposal: &no_slashing_threshold_proposal,
            now: 20,
            votes_count: 500,
            total_voters_count: 600,
            approvals: 3,
            slashes: 314,
        };

        assert!(!no_slashing_proposal_status_resolution.is_slashing_threshold_reached());

        let slashing_threshold_proposal_status_resolution = ProposalStatusResolution {
            slashes: 315,
            ..no_slashing_proposal_status_resolution
        };

        assert!(slashing_threshold_proposal_status_resolution.is_slashing_threshold_reached());
    }
}
