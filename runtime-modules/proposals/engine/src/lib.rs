//! # Proposals engine module
//! Proposals `engine` module for the Joystream platform. Version 2.
//! The main component of the proposals frame_system. Provides methods and extrinsics to create and
//! vote for proposals, inspired by Parity **Democracy module**.
//!
//! ## Overview
//! Proposals `engine` module provides an abstract mechanism to work with proposals: creation, voting,
//! execution, canceling, etc. Proposal execution demands serialized _Dispatchable_ proposal code.
//! It could be any _Dispatchable_ + _Parameter_ type, but most likely, it would be serialized (via
//! Parity _codec_ crate) extrisic call. A proposal stage can be described by its [status](./enum.ProposalStatus.html).
//!
//! ## Proposal lifecycle
//! When a proposal passes [checks](./struct.Module.html#method.ensure_create_proposal_parameters_are_valid)
//! for its [parameters](./struct.ProposalParameters.html) - it can be [created](./struct.Module.html#method.create_proposal).
//! The newly created proposal has _Active_ status. The proposal can be voted on or canceled during its
//! _voting period_. Votes can be [different](./enum.VoteKind.html). When the proposal gets enough votes
//! to be slashed or approved or _voting period_ ends - the proposal becomes _Finalized_. If the proposal
//! got approved and _grace period_ passed - the  `engine` module tries to execute the proposal.
//! The final [approved status](./enum.ApprovedProposalStatus.html) of the proposal defines
//! an overall proposal outcome.
//!
//! ### Notes
//!
//! - The proposal can be [vetoed](./struct.Module.html#method.veto_proposal)
//! anytime before the proposal execution by the _sudo_.
//! - When the proposal is created with some stake - refunding on proposal finalization with
//! different statuses should be accomplished from the external handler from the _stake module_
//! (_StakingEventsHandler_). Such a handler should call
//! [refund_proposal_stake](./struct.Module.html#method.refund_proposal_stake) callback function.
//! - If the _council_ got reelected during the proposal _voting period_ the external handler calls
//! [reset_active_proposals](./trait.Module.html#method.reset_active_proposals) function and
//! all voting results get cleared.
//!
//! ### Important abstract types to be implemented
//! Proposals `engine` module has several abstractions to be implemented in order to work correctly.
//! - _VoterOriginValidator_ - ensure valid voter identity. Voters should have permissions to vote:
//! they should be council members.
//! - [VotersParameters](./trait.VotersParameters.html) - defines total voter number, which is
//! the council size
//! - _ProposerOriginValidator_ - ensure valid proposer identity. Proposers should have permissions
//! to create a proposal: they should be members of the Joystream.
//! - [StakeHandlerProvider](./trait.StakeHandlerProvider.html) - defines an interface for the staking.
//!
//! A full list of the abstractions can be found [here](./trait.Config.html).
//!
//! ### Supported extrinsics
//! - [vote](./struct.Module.html#method.vote) - registers a vote for the proposal
//! - [cancel_proposal](./struct.Module.html#method.cancel_proposal) - cancels the proposal (can be canceled only by owner)
//! - [veto_proposal](./struct.Module.html#method.veto_proposal) - vetoes the proposal
//!
//! ### Public API
//! - [create_proposal](./struct.Module.html#method.create_proposal) - creates proposal using provided parameters
//! - [ensure_create_proposal_parameters_are_valid](./struct.Module.html#method.ensure_create_proposal_parameters_are_valid) - ensures that we can create the proposal
//! - [refund_proposal_stake](./struct.Module.html#method.refund_proposal_stake) - a callback for _StakingHandlerEvents_
//! - [reset_active_proposals](./trait.Module.html#method.reset_active_proposals) - resets voting results for active proposals
//!
//! ## Usage
//!
//! ```
//! use frame_support::{decl_module, print};
//! use frame_system::ensure_signed;
//! use codec::Encode;
//! use pallet_proposals_engine::{self as engine, ProposalParameters};
//!
//! pub trait Config: engine::Config + membership::Config {}
//!
//! decl_module! {
//!     pub struct Module<T: Config> for enum Call where origin: T::Origin {
//!         #[weight = 10_000_000]
//!         fn executable_proposal(origin) {
//!             print("executed!");
//!         }
//!
//!         #[weight = 10_000_000]
//!         pub fn create_spending_proposal(
//!             origin,
//!             proposer_id: T::MemberId,
//!         ) {
//!             let account_id = ensure_signed(origin)?;
//!             let parameters = ProposalParameters::default();
//!             let title = b"Spending proposal".to_vec();
//!             let description = b"We need to spend some tokens to support the working group lead."
//!                 .to_vec();
//!             let encoded_proposal_code = <Call<T>>::executable_proposal().encode();
//!
//!             <engine::Module<T>>::ensure_create_proposal_parameters_are_valid(
//!                 &parameters,
//!                 &title,
//!                 &description,
//!                 None,
//!                 &account_id,
//!             )?;
//!             <engine::Module<T>>::create_proposal(
//!                 account_id,
//!                 proposer_id,
//!                 parameters,
//!                 title,
//!                 description,
//!                 None,
//!                 encoded_proposal_code
//!             )?;
//!         }
//!     }
//! }
//! # fn main() {}
//! ```

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

use crate::types::ApprovedProposalData;
use types::FinalizedProposalData;
use types::ProposalStakeManager;
pub use types::{
    ActiveStake, ApprovedProposalStatus, FinalizationData, Proposal, ProposalDecisionStatus,
    ProposalParameters, ProposalStatus, VotingResults,
};
pub use types::{BalanceOf, CurrencyOf, NegativeImbalance};
pub use types::{DefaultStakeHandlerProvider, StakeHandler, StakeHandlerProvider};
pub use types::{ProposalCodeDecoder, ProposalExecutable};
pub use types::{VoteKind, VotersParameters};

pub(crate) mod types;

#[cfg(test)]
mod tests;

use codec::Decode;
use frame_support::dispatch::{DispatchError, DispatchResult, UnfilteredDispatchable};
use frame_support::storage::IterableStorageMap;
use frame_support::traits::{Currency, Get};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, print, Parameter, StorageDoubleMap,
};
use frame_system::{ensure_root, RawOrigin};
use sp_arithmetic::traits::{SaturatedConversion, Zero};
use sp_std::vec::Vec;

use common::origin::ActorOriginValidator;

type MemberId<T> = <T as membership::Config>::MemberId;

/// Proposals engine trait.
pub trait Config:
    frame_system::Config
    + pallet_timestamp::Config
    + stake::Config
    + membership::Config
    + balances::Config
{
    /// Engine event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// Validates proposer id and origin combination
    type ProposerOriginValidator: ActorOriginValidator<
        Self::Origin,
        MemberId<Self>,
        Self::AccountId,
    >;

    /// Validates voter id and origin combination
    type VoterOriginValidator: ActorOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;

    /// Provides data for voting. Defines maximum voters count for the proposal.
    type TotalVotersCounter: VotersParameters;

    /// Proposal Id type
    type ProposalId: From<u32> + Parameter + Default + Copy;

    /// Provides stake logic implementation. Can be used to mock stake logic.
    type StakeHandlerProvider: StakeHandlerProvider<Self>;

    /// The fee is applied when cancel the proposal. A fee would be slashed (burned).
    type CancellationFee: Get<BalanceOf<Self>>;

    /// The fee is applied when the proposal gets rejected. A fee would be slashed (burned).
    type RejectionFee: Get<BalanceOf<Self>>;

    /// Defines max allowed proposal title length.
    type TitleMaxLength: Get<u32>;

    /// Defines max allowed proposal description length.
    type DescriptionMaxLength: Get<u32>;

    /// Defines max simultaneous active proposals number.
    type MaxActiveProposalLimit: Get<u32>;

    /// Proposals executable code. Can be instantiated by external module Call enum members.
    type DispatchableCallCode: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + Default;
}

decl_event!(
    /// Proposals engine events
    pub enum Event<T>
    where
        <T as Config>::ProposalId,
        MemberId = MemberId<T>,
        <T as frame_system::Config>::BlockNumber,
        <T as frame_system::Config>::AccountId,
        <T as stake::Config>::StakeId,
    {
        /// Emits on proposal creation.
        /// Params:
        /// - Member id of a proposer.
        /// - Id of a newly created proposal after it was saved in storage.
        ProposalCreated(MemberId, ProposalId),

        /// Emits on proposal status change.
        /// Params:
        /// - Id of a updated proposal.
        /// - New proposal status
        ProposalStatusUpdated(ProposalId, ProposalStatus<BlockNumber, StakeId, AccountId>),

        /// Emits on voting for the proposal
        /// Params:
        /// - Voter - member id of a voter.
        /// - Id of a proposal.
        /// - Kind of vote.
        Voted(MemberId, ProposalId, VoteKind),
    }
);

decl_error! {
    /// Engine module predefined errors
    pub enum Error for Module<T: Config>{
        /// Proposal cannot have an empty title"
        EmptyTitleProvided,

        /// Proposal cannot have an empty body
        EmptyDescriptionProvided,

        /// Title is too long
        TitleIsTooLong,

        /// Description is too long
        DescriptionIsTooLong,

        /// The proposal does not exist
        ProposalNotFound,

        /// Proposal is finalized already
        ProposalFinalized,

        /// The proposal have been already voted on
        AlreadyVoted,

        /// Not an author
        NotAuthor,

        /// Max active proposals number exceeded
        MaxActiveProposalNumberExceeded,

        /// Stake cannot be empty with this proposal
        EmptyStake,

        /// Stake should be empty for this proposal
        StakeShouldBeEmpty,

        /// Stake differs from the proposal requirements
        StakeDiffersFromRequired,

        /// Approval threshold cannot be zero
        InvalidParameterApprovalThreshold,

        /// Slashing threshold cannot be zero
        InvalidParameterSlashingThreshold,

        /// Require root origin in extrinsics
        RequireRootOrigin,

        /// Insufficient balance for operation.
        InsufficientBalance,
    }
}

// Storage for the proposals engine module
decl_storage! {
    pub trait Store for Module<T: Config> as ProposalEngine{
        /// Map proposal by its id.
        pub Proposals get(fn proposals): map hasher(blake2_128_concat)
            T::ProposalId => ProposalOf<T>;

        /// Count of all proposals that have been created.
        pub ProposalCount get(fn proposal_count): u32;

        /// Map proposal executable code by proposal id.
        pub DispatchableCallCode get(fn proposal_codes): map hasher(blake2_128_concat)
            T::ProposalId =>  Vec<u8>;

        /// Count of active proposals.
        pub ActiveProposalCount get(fn active_proposal_count): u32;

        /// Ids of proposals that are open for voting (have not been finalized yet).
        pub ActiveProposalIds get(fn active_proposal_ids): map hasher(blake2_128_concat)
            T::ProposalId=> ();

        /// Ids of proposals that were approved and theirs grace period was not expired.
        pub PendingExecutionProposalIds get(fn pending_proposal_ids): map hasher(blake2_128_concat)
            T::ProposalId=> ();

        /// Double map for preventing duplicate votes. Should be cleaned after usage.
        pub VoteExistsByProposalByVoter get(fn vote_by_proposal_by_voter):
            double_map hasher(blake2_128_concat)  T::ProposalId, hasher(blake2_128_concat) MemberId<T> => VoteKind;

        /// Map proposal id by stake id. Required by StakingEventsHandler callback call
        pub StakesProposals get(fn stakes_proposals): map hasher(blake2_128_concat)
            T::StakeId =>  T::ProposalId;
    }
}

decl_module! {
    /// 'Proposal engine' substrate module
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Emits an event. Default substrate implementation.
        fn deposit_event() = default;

        /// Exports const - the fee is applied when cancel the proposal. A fee would be slashed (burned).
        const CancellationFee: BalanceOf<T> = T::CancellationFee::get();

        /// Exports const -  the fee is applied when the proposal gets rejected. A fee would be slashed (burned).
        const RejectionFee: BalanceOf<T> = T::RejectionFee::get();

        /// Exports const -  max allowed proposal title length.
        const TitleMaxLength: u32 = T::TitleMaxLength::get();

        /// Exports const -  max allowed proposal description length.
        const DescriptionMaxLength: u32 = T::DescriptionMaxLength::get();

        /// Exports const -  max simultaneous active proposals number.
        const MaxActiveProposalLimit: u32 = T::MaxActiveProposalLimit::get();

        /// Vote extrinsic. Conditions:  origin must allow votes.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn vote(origin, voter_id: MemberId<T>, proposal_id: T::ProposalId, vote: VoteKind)  {
            T::VoterOriginValidator::ensure_actor_origin(
                origin,
                voter_id,
            )?;

            ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::ProposalNotFound);
            let mut proposal = Self::proposals(proposal_id);

            ensure!(matches!(proposal.status, ProposalStatus::Active{..}), Error::<T>::ProposalFinalized);

            let did_not_vote_before = !<VoteExistsByProposalByVoter<T>>::contains_key(
                proposal_id,
                voter_id,
            );

            ensure!(did_not_vote_before, Error::<T>::AlreadyVoted);

            proposal.voting_results.add_vote(vote.clone());

            // mutation

            <Proposals<T>>::insert(proposal_id, proposal);
            <VoteExistsByProposalByVoter<T>>::insert(proposal_id, voter_id, vote.clone());
            Self::deposit_event(RawEvent::Voted(voter_id, proposal_id, vote));
        }

        /// Cancel a proposal by its original proposer.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_proposal(origin, proposer_id: MemberId<T>, proposal_id: T::ProposalId) {
            T::ProposerOriginValidator::ensure_actor_origin(
                origin,
                proposer_id,
            )?;

            ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::ProposalNotFound);
            let proposal = Self::proposals(proposal_id);

            ensure!(proposer_id == proposal.proposer_id, Error::<T>::NotAuthor);
            ensure!(matches!(proposal.status, ProposalStatus::Active{..}), Error::<T>::ProposalFinalized);

            // mutation

            Self::finalize_proposal(proposal_id, ProposalDecisionStatus::Canceled);
        }

        /// Veto a proposal. Must be root.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn veto_proposal(origin, proposal_id: T::ProposalId) {
            ensure_root(origin)?;

            ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::ProposalNotFound);
            let proposal = Self::proposals(proposal_id);

            // mutation

            if <PendingExecutionProposalIds<T>>::contains_key(proposal_id) {
                Self::veto_pending_execution_proposal(proposal_id, proposal);
            } else {
                ensure!(matches!(proposal.status, ProposalStatus::Active{..}), Error::<T>::ProposalFinalized);
                Self::finalize_proposal(proposal_id, ProposalDecisionStatus::Vetoed);
            }
        }

        /// Block finalization. Perform voting period check, vote result tally, approved proposals
        /// grace period checks, and proposal execution.
        fn on_finalize(_n: T::BlockNumber) {
            let finalized_proposals = Self::get_finalized_proposals();

            // mutation

            // Check vote results. Approved proposals with zero grace period will be
            // transitioned to the PendingExecution status.
            for  proposal_data in finalized_proposals {
                <Proposals<T>>::insert(proposal_data.proposal_id, proposal_data.proposal);
                Self::finalize_proposal(proposal_data.proposal_id, proposal_data.status);
            }

            let executable_proposals =
                Self::get_approved_proposal_with_expired_grace_period();

            // Execute approved proposals with expired grace period
            for approved_proosal in executable_proposals {
                Self::execute_proposal(approved_proosal);
            }
        }
    }
}

impl<T: Config> Module<T> {
    /// Create proposal. Requires 'proposal origin' membership.
    pub fn create_proposal(
        account_id: T::AccountId,
        proposer_id: MemberId<T>,
        parameters: ProposalParameters<T::BlockNumber, types::BalanceOf<T>>,
        title: Vec<u8>,
        description: Vec<u8>,
        stake_balance: Option<types::BalanceOf<T>>,
        encoded_dispatchable_call_code: Vec<u8>,
    ) -> Result<T::ProposalId, DispatchError> {
        Self::ensure_create_proposal_parameters_are_valid(
            &parameters,
            &title,
            &description,
            stake_balance,
            &account_id,
        )?;

        // checks passed
        // mutation

        let next_proposal_count_value = Self::proposal_count() + 1;
        let new_proposal_id = next_proposal_count_value;
        let proposal_id = T::ProposalId::from(new_proposal_id);

        // Check stake_balance for value and create stake if value exists, else take None
        // If create_stake() returns error - return error from extrinsic
        let stake_id_result = stake_balance
            .map(|stake_amount| {
                ProposalStakeManager::<T>::create_stake(stake_amount, account_id.clone())
            })
            .transpose()?;

        let mut stake_data = None;
        if let Some(stake_id) = stake_id_result {
            stake_data = Some(ActiveStake {
                stake_id,
                source_account_id: account_id,
            });

            <StakesProposals<T>>::insert(stake_id, proposal_id);
        }

        let new_proposal = Proposal {
            created_at: Self::current_block(),
            parameters,
            title,
            description,
            proposer_id,
            status: ProposalStatus::Active(stake_data),
            voting_results: VotingResults::default(),
        };

        <Proposals<T>>::insert(proposal_id, new_proposal);
        <DispatchableCallCode<T>>::insert(proposal_id, encoded_dispatchable_call_code);
        <ActiveProposalIds<T>>::insert(proposal_id, ());
        ProposalCount::put(next_proposal_count_value);
        Self::increase_active_proposal_counter();

        Self::deposit_event(RawEvent::ProposalCreated(proposer_id, proposal_id));

        Ok(proposal_id)
    }

    /// Performs all checks for the proposal creation:
    /// - title, body lengths
    /// - max active proposal
    /// - provided parameters: approval_threshold_percentage and slashing_threshold_percentage > 0
    /// - provided stake balance and parameters.required_stake are valid
    pub fn ensure_create_proposal_parameters_are_valid(
        parameters: &ProposalParameters<T::BlockNumber, types::BalanceOf<T>>,
        title: &[u8],
        description: &[u8],
        stake_balance: Option<types::BalanceOf<T>>,
        stake_account_id: &T::AccountId,
    ) -> DispatchResult {
        ensure!(!title.is_empty(), Error::<T>::EmptyTitleProvided);
        ensure!(
            title.len() as u32 <= T::TitleMaxLength::get(),
            Error::<T>::TitleIsTooLong
        );

        ensure!(
            !description.is_empty(),
            Error::<T>::EmptyDescriptionProvided
        );
        ensure!(
            description.len() as u32 <= T::DescriptionMaxLength::get(),
            Error::<T>::DescriptionIsTooLong
        );

        ensure!(
            (Self::active_proposal_count()) < T::MaxActiveProposalLimit::get(),
            Error::<T>::MaxActiveProposalNumberExceeded
        );

        ensure!(
            parameters.approval_threshold_percentage > 0,
            Error::<T>::InvalidParameterApprovalThreshold
        );

        ensure!(
            parameters.slashing_threshold_percentage > 0,
            Error::<T>::InvalidParameterSlashingThreshold
        );

        // check stake parameters
        if let Some(required_stake) = parameters.required_stake {
            if let Some(staked_balance) = stake_balance {
                ensure!(
                    required_stake == staked_balance,
                    Error::<T>::StakeDiffersFromRequired
                );
            } else {
                return Err(Error::<T>::EmptyStake.into());
            }

            ensure!(
                types::Balances::<T>::usable_balance(stake_account_id).saturated_into::<u128>()
                    >= required_stake.saturated_into::<u128>(),
                Error::<T>::InsufficientBalance
            );
        }

        if stake_balance.is_some() && parameters.required_stake.is_none() {
            return Err(Error::<T>::StakeShouldBeEmpty.into());
        }

        Ok(())
    }

    /// Callback from StakingEventsHandler. Refunds unstaked imbalance back to the source account.
    /// There can be a lot of invariant breaks in the scope of this proposal.
    /// Such situations are handled by adding error messages to the log.
    pub fn refund_proposal_stake(stake_id: T::StakeId, imbalance: NegativeImbalance<T>) {
        if <StakesProposals<T>>::contains_key(stake_id) {
            let proposal_id = Self::stakes_proposals(stake_id);

            if <Proposals<T>>::contains_key(proposal_id) {
                let proposal = Self::proposals(proposal_id);

                if let ProposalStatus::Active(active_stake_result) = proposal.status {
                    if let Some(active_stake) = active_stake_result {
                        let refunding_result = CurrencyOf::<T>::resolve_into_existing(
                            &active_stake.source_account_id,
                            imbalance,
                        );

                        if refunding_result.is_err() {
                            print("Broken invariant: cannot refund");
                        }
                    }
                } else {
                    print("Broken invariant: proposal status is not Active");
                }
            } else {
                print("Broken invariant: proposal doesn't exist");
            }
        } else {
            print("Broken invariant: stake doesn't exist");
        }
    }

    /// Resets voting results for active proposals.
    /// Possible application includes new council elections.
    pub fn reset_active_proposals() {
        <ActiveProposalIds<T>>::iter().for_each(|(proposal_id, _)| {
            <Proposals<T>>::mutate(proposal_id, |proposal| {
                proposal.reset_proposal();
                <VoteExistsByProposalByVoter<T>>::remove_prefix(&proposal_id);
            });
        });
    }
}

impl<T: Config> Module<T> {
    // Wrapper-function over frame_system::block_number()
    fn current_block() -> T::BlockNumber {
        <frame_system::Pallet<T>>::block_number()
    }

    // Enumerates through active proposals. Tally Voting results.
    // Returns proposals with finalized status and id
    fn get_finalized_proposals() -> Vec<FinalizedProposal<T>> {
        // Enumerate active proposals id and gather finalization data.
        // Skip proposals with unfinished voting.
        <ActiveProposalIds<T>>::iter()
            .filter_map(|(proposal_id, _)| {
                // load current proposal
                let proposal = Self::proposals(proposal_id);

                // Calculates votes, takes in account voting period expiration.
                // If voting process is in progress, then decision status is None.
                let decision_status = proposal.define_proposal_decision_status(
                    T::TotalVotersCounter::total_voters_count(),
                    Self::current_block(),
                );

                // map to FinalizedProposalData if decision for the proposal is made or return None
                decision_status.map(|status| FinalizedProposalData {
                    proposal_id,
                    proposal,
                    status,
                    finalized_at: Self::current_block(),
                })
            })
            .collect() // compose output vector
    }

    // Veto approved proposal during its grace period. Saves a new proposal status and removes
    // proposal id from the 'PendingExecutionProposalIds'
    fn veto_pending_execution_proposal(proposal_id: T::ProposalId, proposal: ProposalOf<T>) {
        <PendingExecutionProposalIds<T>>::remove(proposal_id);

        let vetoed_proposal_status = ProposalStatus::finalized(
            ProposalDecisionStatus::Vetoed,
            None,
            None,
            Self::current_block(),
        );

        <Proposals<T>>::insert(
            proposal_id,
            Proposal {
                status: vetoed_proposal_status,
                ..proposal
            },
        );
    }

    // Executes approved proposal code
    fn execute_proposal(approved_proposal: ApprovedProposal<T>) {
        let proposal_code = Self::proposal_codes(approved_proposal.proposal_id);

        let proposal_code_result = T::DispatchableCallCode::decode(&mut &proposal_code[..]);

        let approved_proposal_status = match proposal_code_result {
            Ok(proposal_code) => {
                if let Err(dispatch_error) =
                    proposal_code.dispatch_bypass_filter(T::Origin::from(RawOrigin::Root))
                {
                    ApprovedProposalStatus::failed_execution(Self::parse_dispatch_error(
                        dispatch_error.error,
                    ))
                } else {
                    ApprovedProposalStatus::Executed
                }
            }
            Err(_) => {
                ApprovedProposalStatus::failed_execution("Error during proposal decoding occured.")
            }
        };

        let proposal_execution_status = approved_proposal
            .finalisation_status_data
            .create_approved_proposal_status(approved_proposal_status);

        let mut proposal = approved_proposal.proposal;
        proposal.status = proposal_execution_status.clone();
        <Proposals<T>>::insert(approved_proposal.proposal_id, proposal);

        Self::deposit_event(RawEvent::ProposalStatusUpdated(
            approved_proposal.proposal_id,
            proposal_execution_status,
        ));

        <PendingExecutionProposalIds<T>>::remove(&approved_proposal.proposal_id);
    }

    // Performs all actions on proposal finalization:
    // - clean active proposal cache
    // - update proposal status fields (status, finalized_at)
    // - add to pending execution proposal cache if approved
    // - slash and unstake proposal stake if stake exists
    // - decrease active proposal counter
    // - fire an event
    // It prints an error message in case of an attempt to finalize the non-active proposal.
    fn finalize_proposal(proposal_id: T::ProposalId, decision_status: ProposalDecisionStatus) {
        Self::decrease_active_proposal_counter();
        <ActiveProposalIds<T>>::remove(&proposal_id.clone());

        let mut proposal = Self::proposals(proposal_id);

        if let ProposalStatus::Active(active_stake) = proposal.status.clone() {
            if let ProposalDecisionStatus::Approved { .. } = decision_status {
                <PendingExecutionProposalIds<T>>::insert(proposal_id, ());
            }

            // deal with stakes if necessary
            let slash_balance =
                Self::calculate_slash_balance(&decision_status, &proposal.parameters);
            let slash_and_unstake_result =
                Self::slash_and_unstake(active_stake.clone(), slash_balance);

            // create finalized proposal status with error if any
            let new_proposal_status = ProposalStatus::finalized(
                decision_status,
                slash_and_unstake_result.err(),
                active_stake,
                Self::current_block(),
            );

            proposal.status = new_proposal_status.clone();
            <Proposals<T>>::insert(proposal_id, proposal);

            Self::deposit_event(RawEvent::ProposalStatusUpdated(
                proposal_id,
                new_proposal_status,
            ));
        } else {
            print("Broken invariant: proposal cannot be non-active during the finalisation");
        }
    }

    // Slashes the stake and perform unstake only in case of existing stake
    fn slash_and_unstake(
        current_stake_data: Option<ActiveStake<T::StakeId, T::AccountId>>,
        slash_balance: BalanceOf<T>,
    ) -> Result<(), &'static str> {
        // only if stake exists
        if let Some(stake_data) = current_stake_data {
            if !slash_balance.is_zero() {
                ProposalStakeManager::<T>::slash(stake_data.stake_id, slash_balance)?;
            }

            ProposalStakeManager::<T>::remove_stake(stake_data.stake_id)?;
        }

        Ok(())
    }

    // Calculates required slash based on finalization ProposalDecisionStatus and proposal parameters.
    // Method visibility allows testing.
    pub(crate) fn calculate_slash_balance(
        decision_status: &ProposalDecisionStatus,
        proposal_parameters: &ProposalParameters<T::BlockNumber, types::BalanceOf<T>>,
    ) -> types::BalanceOf<T> {
        match decision_status {
            ProposalDecisionStatus::Rejected | ProposalDecisionStatus::Expired => {
                T::RejectionFee::get()
            }
            ProposalDecisionStatus::Approved { .. } | ProposalDecisionStatus::Vetoed => {
                BalanceOf::<T>::zero()
            }
            ProposalDecisionStatus::Canceled => T::CancellationFee::get(),
            ProposalDecisionStatus::Slashed => proposal_parameters
                .required_stake
                .unwrap_or_else(BalanceOf::<T>::zero), // stake if set or zero
        }
    }

    // Enumerates approved proposals and checks their grace period expiration
    fn get_approved_proposal_with_expired_grace_period() -> Vec<ApprovedProposal<T>> {
        <PendingExecutionProposalIds<T>>::iter()
            .filter_map(|(proposal_id, _)| {
                let proposal = Self::proposals(proposal_id);

                if proposal.is_grace_period_expired(Self::current_block()) {
                    // this should be true, because it was tested inside is_grace_period_expired()
                    if let ProposalStatus::Finalized(finalisation_data) = proposal.status.clone() {
                        Some(ApprovedProposalData {
                            proposal_id,
                            proposal,
                            finalisation_status_data: finalisation_data,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    // Increases active proposal counter.
    fn increase_active_proposal_counter() {
        let next_active_proposal_count_value = Self::active_proposal_count() + 1;
        ActiveProposalCount::put(next_active_proposal_count_value);
    }

    // Decreases active proposal counter down to zero. Decreasing below zero has no effect.
    fn decrease_active_proposal_counter() {
        let current_active_proposal_counter = Self::active_proposal_count();

        if current_active_proposal_counter > 0 {
            let next_active_proposal_count_value = current_active_proposal_counter - 1;
            ActiveProposalCount::put(next_active_proposal_count_value);
        };
    }

    // Parse dispatchable execution result.
    fn parse_dispatch_error(error: DispatchError) -> &'static str {
        match error {
            DispatchError::Other(msg) => msg,
            DispatchError::Module {
                index: _,
                error: _,
                message: msg,
            } => msg.unwrap_or("Dispatch error."),
            _ => error.into(),
        }
    }
}

// Simplification of the 'FinalizedProposalData' type
type FinalizedProposal<T> = FinalizedProposalData<
    <T as Config>::ProposalId,
    <T as frame_system::Config>::BlockNumber,
    MemberId<T>,
    types::BalanceOf<T>,
    <T as stake::Config>::StakeId,
    <T as frame_system::Config>::AccountId,
>;

// Simplification of the 'ApprovedProposalData' type
type ApprovedProposal<T> = ApprovedProposalData<
    <T as Config>::ProposalId,
    <T as frame_system::Config>::BlockNumber,
    MemberId<T>,
    types::BalanceOf<T>,
    <T as stake::Config>::StakeId,
    <T as frame_system::Config>::AccountId,
>;

// Simplification of the 'Proposal' type
type ProposalOf<T> = Proposal<
    <T as frame_system::Config>::BlockNumber,
    MemberId<T>,
    types::BalanceOf<T>,
    <T as stake::Config>::StakeId,
    <T as frame_system::Config>::AccountId,
>;
