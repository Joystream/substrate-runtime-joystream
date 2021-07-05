//! # Working group module
//! Working group module for the Joystream platform. Version 1.
//! Contains abstract working group workflow.
//!
//! ## Overview
//!
//! The working group module provides working group workflow to use in different modules.
//! It contains extrinsics for the hiring workers, their roles lifecycle and stake management.
//! There is a possibility to hire a special worker - the leader of the working group.
//! Some module operations like 'increase_stake' can be invoked by the worker, others
//! like 'terminate_role' can be invoked by the leader only. The leader himself can be hired and
//! managed only by the council via proposals.
//!
//! Exact working group (eg.: forum working group) should create an instance of the Working group module.
//!
//! ## Supported extrinsics
//! ### Hiring flow
//!
//! - [add_opening](./struct.Module.html#method.add_opening) - Add an opening for a worker/lead role.
//! - [accept_applications](./struct.Module.html#method.accept_applications)- Begin accepting worker/lead applications.
//! - [begin_applicant_review](./struct.Module.html#method.begin_applicant_review) - Begin reviewing worker/lead applications.
//! - [fill_opening](./struct.Module.html#method.fill_opening) - Fill opening for worker/lead.
//! - [withdraw_application](./struct.Module.html#method.withdraw_application) - Withdraw the worker/lead application.
//! - [terminate_application](./struct.Module.html#method.terminate_application) - Terminate the worker/lead application.
//! - [apply_on_opening](./struct.Module.html#method.apply_on_opening) - Apply on a worker/lead opening.
//!
//! ### Roles lifecycle
//!
//! - [update_role_account](./struct.Module.html#method.update_role_account) -  Update the role account of the worker/lead.
//! - [update_reward_account](./struct.Module.html#method.update_reward_account) -  Update the reward account of the worker/lead.
//! - [update_reward_amount](./struct.Module.html#method.update_reward_amount) -  Update the reward amount of the worker/lead.
//! - [leave_role](./struct.Module.html#method.leave_role) - Leave the role by the active worker/lead.
//! - [terminate_role](./struct.Module.html#method.terminate_role) - Terminate the worker/lead role.
//! - [set_mint_capacity](./struct.Module.html#method.set_mint_capacity) -  Sets the capacity to enable working group budget.
//!
//! ### Stakes
//!
//! - [slash_stake](./struct.Module.html#method.slash_stake) - Slashes the worker/lead stake.
//! - [decrease_stake](./struct.Module.html#method.decrease_stake) - Decreases the worker/lead stake and returns the remainder to the worker _role_account_.
//! - [increase_stake](./struct.Module.html#method.increase_stake) - Increases the worker/lead stake.
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

#[cfg(test)]
mod tests;
mod types;
#[macro_use]
mod errors;

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::storage::IterableStorageMap;
use frame_support::traits::{Currency, ExistenceRequirement, Get, Imbalance, WithdrawReasons};
use frame_support::{decl_event, decl_module, decl_storage, ensure, print, StorageValue};
use frame_system::{ensure_root, ensure_signed};
use sp_arithmetic::traits::{Bounded, One, Zero};
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
use sp_std::vec;
use sp_std::vec::Vec;

use crate::types::ExitInitiationOrigin;
use common::constraints::InputValidationLengthConstraint;
use errors::WrappedError;

pub use errors::Error;
pub use types::{
    Application, Opening, OpeningPolicyCommitment, OpeningType, RewardPolicy, RoleStakeProfile,
    Worker,
};

/// Stake identifier in staking module
pub type StakeId<T> = <T as stake::Trait>::StakeId;

/// Member identifier in membership::member module
pub type MemberId<T> = <T as membership::Trait>::MemberId;

/// Workaround for BTreeSet type
pub type ApplicationIdSet<T> = BTreeSet<ApplicationId<T>>;

/// Type for the identifier for an opening for a worker/lead.
pub type OpeningId<T> = <T as hiring::Trait>::OpeningId;

/// Type for the identifier for an application as a worker/lead.
pub type ApplicationId<T> = <T as hiring::Trait>::ApplicationId;

/// Balance type of runtime
pub type BalanceOf<T> =
    <<T as stake::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// Balance type of runtime reward
pub type BalanceOfMint<T> =
    <<T as minting::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// Balance type of runtime
pub type CurrencyOf<T> = <T as stake::Trait>::Currency;

/// Negative imbalance of runtime.
pub type NegativeImbalance<T> = <<T as stake::Trait>::Currency as Currency<
    <T as frame_system::Trait>::AccountId,
>>::NegativeImbalance;

/// Alias for the worker application id to the worker id dictionary
pub type ApplicationIdToWorkerIdMap<T> = BTreeMap<ApplicationId<T>, WorkerId<T>>;

/// Type identifier for worker role, which must be same as membership actor identifier
pub type WorkerId<T> = <T as membership::Trait>::ActorId;

/// Alias for the application id from the hiring module.
pub type HiringApplicationId<T> = <T as hiring::Trait>::ApplicationId;

// Type simplification
type OpeningInfo<T> = (
    OpeningOf<T>,
    hiring::Opening<BalanceOf<T>, <T as frame_system::Trait>::BlockNumber, HiringApplicationId<T>>,
);

// Type simplification
type ApplicationInfo<T> = (ApplicationOf<T>, ApplicationId<T>, OpeningOf<T>);

// Type simplification
type RewardSettings<T> = (
    <T as minting::Trait>::MintId,
    RewardPolicy<BalanceOfMint<T>, <T as frame_system::Trait>::BlockNumber>,
);

// Type simplification
type WorkerOf<T> = Worker<
    <T as frame_system::Trait>::AccountId,
    <T as recurringrewards::Trait>::RewardRelationshipId,
    <T as stake::Trait>::StakeId,
    <T as frame_system::Trait>::BlockNumber,
    MemberId<T>,
>;

// Type simplification
type OpeningOf<T> = Opening<
    <T as hiring::Trait>::OpeningId,
    <T as frame_system::Trait>::BlockNumber,
    BalanceOf<T>,
    ApplicationId<T>,
>;

// Type simplification
type ApplicationOf<T> = Application<
    <T as frame_system::Trait>::AccountId,
    OpeningId<T>,
    MemberId<T>,
    HiringApplicationId<T>,
>;

/// The _Working group_ main _Trait_
pub trait Trait<I: Instance>:
    frame_system::Trait
    + membership::Trait
    + hiring::Trait
    + minting::Trait
    + stake::Trait
    + recurringrewards::Trait
{
    /// _Working group_ event type.
    type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;

    /// Defines max workers number in the working group.
    type MaxWorkerNumberLimit: Get<u32>;
}

decl_event!(
    /// _Working group_ events
    pub enum Event<T, I>
    where
        WorkerId = WorkerId<T>,
        <T as frame_system::Trait>::AccountId,
        OpeningId = OpeningId<T>,
        ApplicationId = ApplicationId<T>,
        ApplicationIdToWorkerIdMap = ApplicationIdToWorkerIdMap<T>,
        RationaleText = Vec<u8>,
        MintBalanceOf = minting::BalanceOf<T>,
        <T as minting::Trait>::MintId,
    {
        /// Emits on setting the leader.
        /// Params:
        /// - Worker id.
        LeaderSet(WorkerId),

        /// Emits on un-setting the leader.
        /// Params:
        LeaderUnset(),

        /// Emits on terminating the worker.
        /// Params:
        /// - worker id.
        /// - termination rationale text
        TerminatedWorker(WorkerId, RationaleText),

        /// Emits on terminating the leader.
        /// Params:
        /// - leader worker id.
        /// - termination rationale text
        TerminatedLeader(WorkerId, RationaleText),

        /// Emits on exiting the worker.
        /// Params:
        /// - worker id.
        /// - exit rationale text
        WorkerExited(WorkerId, RationaleText),

        /// Emits on updating the role account of the worker.
        /// Params:
        /// - Id of the worker.
        /// - Role account id of the worker.
        WorkerRoleAccountUpdated(WorkerId, AccountId),

        /// Emits on updating the reward account of the worker.
        /// Params:
        /// - Member id of the worker.
        /// - Reward account id of the worker.
        WorkerRewardAccountUpdated(WorkerId, AccountId),

        /// Emits on updating the reward amount of the worker.
        /// Params:
        /// - Id of the worker.
        WorkerRewardAmountUpdated(WorkerId),

        /// Emits on adding new worker opening.
        /// Params:
        /// - Opening id
        OpeningAdded(OpeningId),

        /// Emits on accepting application for the worker opening.
        /// Params:
        /// - Opening id
        AcceptedApplications(OpeningId),

        /// Emits on adding the application for the worker opening.
        /// Params:
        /// - Opening id
        /// - Application id
        AppliedOnOpening(OpeningId, ApplicationId),

        /// Emits on withdrawing the application for the worker/lead opening.
        /// Params:
        /// - Worker application id
        ApplicationWithdrawn(ApplicationId),

        /// Emits on terminating the application for the worker/lead opening.
        /// Params:
        /// - Worker application id
        ApplicationTerminated(ApplicationId),

        /// Emits on beginning the application review for the worker/lead opening.
        /// Params:
        /// - Opening id
        BeganApplicationReview(OpeningId),

        /// Emits on filling the worker opening.
        /// Params:
        /// - Worker opening id
        /// - Worker application id to the worker id dictionary
        OpeningFilled(OpeningId, ApplicationIdToWorkerIdMap),

        /// Emits on increasing the worker/lead stake.
        /// Params:
        /// - worker/lead id.
        StakeIncreased(WorkerId),

        /// Emits on decreasing the worker/lead stake.
        /// Params:
        /// - worker/lead id.
        StakeDecreased(WorkerId),

        /// Emits on slashing the worker/lead stake.
        /// Params:
        /// - worker/lead id.
        StakeSlashed(WorkerId),

        /// Emits on changing working group mint capacity.
        /// Params:
        /// - mint id.
        /// - new mint balance.
        MintCapacityChanged(MintId, MintBalanceOf),
    }
);

decl_storage! {
    trait Store for Module<T: Trait<I>, I: Instance> as WorkingGroup {
        /// The mint currently funding the rewards for this module.
        pub Mint get(fn mint) : <T as minting::Trait>::MintId;

        /// The current lead.
        pub CurrentLead get(fn current_lead) : Option<WorkerId<T>>;

        /// Next identifier value for new worker opening.
        pub NextOpeningId get(fn next_opening_id): OpeningId<T>;

        /// Maps identifier to worker opening.
        pub OpeningById get(fn opening_by_id): map hasher(blake2_128_concat)
            OpeningId<T> => OpeningOf<T>;

        /// Opening human readable text length limits
        pub OpeningHumanReadableText get(fn opening_human_readable_text): InputValidationLengthConstraint;

        /// Maps identifier to worker application on opening.
        pub ApplicationById get(fn application_by_id) : map hasher(blake2_128_concat)
            ApplicationId<T> => ApplicationOf<T>;

        /// Next identifier value for new worker application.
        pub NextApplicationId get(fn next_application_id) : ApplicationId<T>;

        /// Worker application human readable text length limits
        pub WorkerApplicationHumanReadableText get(fn application_human_readable_text) : InputValidationLengthConstraint;

        /// Maps identifier to corresponding worker.
        pub WorkerById get(fn worker_by_id) : map hasher(blake2_128_concat)
            WorkerId<T> => WorkerOf<T>;

        /// Count of active workers.
        pub ActiveWorkerCount get(fn active_worker_count): u32;

        /// Next identifier for new worker.
        pub NextWorkerId get(fn next_worker_id) : WorkerId<T>;

        /// Worker exit rationale text length limits.
        pub WorkerExitRationaleText get(fn worker_exit_rationale_text) : InputValidationLengthConstraint;

        /// Map member id by hiring application id.
        /// Required by StakingEventsHandler callback call to refund the balance on unstaking.
        pub MemberIdByHiringApplicationId get(fn member_id_by_hiring_application_id):
            map hasher(blake2_128_concat) HiringApplicationId<T> =>  MemberId<T>;
    }
        add_extra_genesis {
        config(phantom): sp_std::marker::PhantomData<I>;
        config(working_group_mint_capacity): minting::BalanceOf<T>;
        config(opening_human_readable_text_constraint): InputValidationLengthConstraint;
        config(worker_application_human_readable_text_constraint): InputValidationLengthConstraint;
        config(worker_exit_rationale_text_constraint): InputValidationLengthConstraint;
        build(|config: &GenesisConfig<T, I>| {
            Module::<T, I>::initialize_working_group(
                config.opening_human_readable_text_constraint,
                config.worker_application_human_readable_text_constraint,
                config.worker_exit_rationale_text_constraint,
                config.working_group_mint_capacity)
        });
    }
}

decl_module! {
    /// _Working group_ substrate module.
    pub struct Module<T: Trait<I>, I: Instance> for enum Call where origin: T::Origin {
        /// Default deposit_event() handler
        fn deposit_event() = default;

        /// Predefined errors
        type Error = Error<T, I>;

        /// Exports const -  max simultaneous active worker number.
        const MaxWorkerNumberLimit: u32 = T::MaxWorkerNumberLimit::get();

        // ****************** Roles lifecycle **********************

        /// Update the associated role account of the active worker/lead.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_role_account(
            origin,
            worker_id: WorkerId<T>,
            new_role_account_id: T::AccountId
        ) {
            // Ensuring worker actually exists
            let worker = Self::ensure_worker_exists(&worker_id)?;

            // Ensure that origin is signed by member with given id.
            ensure_on_wrapped_error!(
                membership::Module::<T>::ensure_member_controller_account_signed(origin, &worker.member_id)
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Update role account
            WorkerById::<T, I>::mutate(worker_id, |worker| {
                worker.role_account_id = new_role_account_id.clone()
            });

            // Trigger event
            Self::deposit_event(RawEvent::WorkerRoleAccountUpdated(worker_id, new_role_account_id));
        }

        /// Update the reward account associated with a set reward relationship for the active worker.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_reward_account(
            origin,
            worker_id: WorkerId<T>,
            new_reward_account_id: T::AccountId
        ) {
            // Ensure there is a signer which matches role account of worker corresponding to provided id.
            let worker = Self::ensure_worker_signed(origin, &worker_id)?;

            // Ensure the worker actually has a recurring reward
            let relationship_id = Self::ensure_worker_has_recurring_reward(&worker)?;

            //
            // == MUTATION SAFE ==
            //

            // Update only the reward account.
            ensure_on_wrapped_error!(
                recurringrewards::Module::<T>::set_reward_relationship(
                    relationship_id,
                    Some(new_reward_account_id.clone()), // new_account
                    None, // new_payout
                    None, //new_next_payment_at
                    None) //new_payout_interval
            )?;

            // Trigger event
            Self::deposit_event(RawEvent::WorkerRewardAccountUpdated(worker_id, new_reward_account_id));
        }

        /// Update the reward amount associated with a set reward relationship for the active worker.
        /// Require signed leader origin or the root (to update leader reward amount).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_reward_amount(
            origin,
            worker_id: WorkerId<T>,
            new_amount: BalanceOfMint<T>
        ) {
            // Ensure lead is set and is origin signer or it is the council.
            Self::ensure_origin_for_leader(origin, worker_id)?;

            // Ensuring worker actually exists
            let worker = Self::ensure_worker_exists(&worker_id)?;

            // Ensure the worker actually has a recurring reward
            let relationship_id = Self::ensure_worker_has_recurring_reward(&worker)?;

            //
            // == MUTATION SAFE ==
            //

            // Update only the reward account.
            ensure_on_wrapped_error!(
                recurringrewards::Module::<T>::set_reward_relationship(
                    relationship_id,
                    None, // new_account
                    Some(new_amount), // new_payout
                    None, //new_next_payment_at
                    None) //new_payout_interval
            )?;

            // Trigger event
            Self::deposit_event(RawEvent::WorkerRewardAmountUpdated(worker_id));
        }

        /// Leave the role by the active worker.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn leave_role(
            origin,
            worker_id: WorkerId<T>,
            rationale_text: Vec<u8>
        ) {
            // Ensure there is a signer which matches role account of worker corresponding to provided id.
            let active_worker = Self::ensure_worker_signed(origin, &worker_id)?;

            //
            // == MUTATION SAFE ==
            //

            Self::deactivate_worker(
                &worker_id,
                &active_worker,
                &ExitInitiationOrigin::Worker,
                &rationale_text
            )?;
        }

        /// Terminate the active worker by the lead.
        /// Require signed leader origin or the root (to terminate the leader role).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn terminate_role(
            origin,
            worker_id: WorkerId<T>,
            rationale_text: Vec<u8>,
            slash_stake: bool,
        ) {
            let (cloned_origin1, cloned_origin2) = common::origin::double_origin::<T>(origin);

            // Ensure lead is set or it is the council terminating the leader.
            let exit_origin = Self::ensure_origin_for_leader(cloned_origin1, worker_id)?;

            // Ensuring worker actually exists.
            let worker = Self::ensure_worker_exists(&worker_id)?;

            // Ensure rationale text is valid.
            Self::ensure_worker_exit_rationale_text_is_valid(&rationale_text)?;

            //
            // == MUTATION SAFE ==
            //

            if slash_stake {
                Self::slash_stake(cloned_origin2, worker_id, BalanceOf::<T>::max_value())?;
            }

            Self::deactivate_worker(
                &worker_id,
                &worker,
                &exit_origin,
                &rationale_text
            )?;
        }

        // ****************** Hiring flow **********************

        /// Add an opening for a worker role.
        /// Require signed leader origin or the root (to add opening for the leader position).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_opening(
            origin,
            activate_at: hiring::ActivateOpeningAt<T::BlockNumber>,
            commitment: OpeningPolicyCommitment<T::BlockNumber, BalanceOf<T>>,
            human_readable_text: Vec<u8>,
            opening_type: OpeningType,
        ){
            Self::ensure_origin_for_opening_type(origin, opening_type)?;

            Self::ensure_opening_human_readable_text_is_valid(&human_readable_text)?;

            Self::ensure_opening_policy_commitment_is_valid(&commitment)?;


            // Add opening
            // NB: This call can in principle fail, because the staking policies
            // may not respect the minimum currency requirement.

            let policy_commitment = commitment.clone();

            //
            // == MUTATION SAFE ==
            //

            let opening_id = ensure_on_wrapped_error!(
                hiring::Module::<T>::add_opening(
                    activate_at,
                    commitment.max_review_period_length,
                    commitment.application_rationing_policy,
                    commitment.application_staking_policy,
                    commitment.role_staking_policy,
                    human_readable_text,
            ))?;

            let new_opening_id = NextOpeningId::<T, I>::get();

            // Create and add worker opening.
            let new_opening_by_id = Opening::<OpeningId<T>, T::BlockNumber, BalanceOf<T>, ApplicationId<T>> {
                hiring_opening_id: opening_id,
                applications: BTreeSet::new(),
                policy_commitment,
                opening_type,
            };

            OpeningById::<T, I>::insert(new_opening_id, new_opening_by_id);

            // Update NextOpeningId
            NextOpeningId::<T, I>::mutate(|id| *id += <OpeningId<T> as One>::one());

            // Trigger event
            Self::deposit_event(RawEvent::OpeningAdded(new_opening_id));
        }

        /// Begin accepting worker applications to an opening that is active.
        /// Require signed leader origin or the root (to accept applications for the leader position).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_applications(origin, opening_id: OpeningId<T>)  {
            // Ensure opening exists in this working group
            // NB: Even though call to hiring module will have implicit check for
            // existence of opening as well, this check is to make sure that the opening is for
            // this working group, not something else.
            let (opening, _opening) = Self::ensure_opening_exists(&opening_id)?;

            Self::ensure_origin_for_opening_type(origin, opening.opening_type)?;

            // Attempt to begin accepting applications
            // NB: Combined ensure check and mutation in hiring module

            //
            // == MUTATION SAFE ==
            //

            ensure_on_wrapped_error!(
                hiring::Module::<T>::begin_accepting_applications(opening.hiring_opening_id)
            )?;


            // Trigger event
            Self::deposit_event(RawEvent::AcceptedApplications(opening_id));
        }

        /// Apply on a worker opening.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn apply_on_opening(
            origin,
            member_id: T::MemberId,
            opening_id: OpeningId<T>,
            role_account_id: T::AccountId,
            opt_role_stake_balance: Option<BalanceOf<T>>,
            opt_application_stake_balance: Option<BalanceOf<T>>,
            human_readable_text: Vec<u8>
        ) {
            // Ensure origin which will server as the source account for staked funds is signed
            let source_account = ensure_signed(origin)?;

            // In absence of a more general key delegation frame_system which allows an account with some funds to
            // grant another account permission to stake from its funds, the origin of this call must have the funds
            // and cannot specify another arbitrary account as the source account.
            // Ensure the source_account is either the controller or root account of member with given id
            ensure!(
                membership::Module::<T>::ensure_member_controller_account(&source_account, &member_id).is_ok() ||
                membership::Module::<T>::ensure_member_root_account(&source_account, &member_id).is_ok(),
                Error::<T, I>::OriginIsNeitherMemberControllerOrRoot
            );

            // Ensure worker opening exists
            let (opening, _opening) = Self::ensure_opening_exists(&opening_id)?;

            // Ensure that there is sufficient balance to cover stake proposed
            Self::ensure_can_make_stake_imbalance(
                vec![&opt_role_stake_balance, &opt_application_stake_balance],
                &source_account
            )
            .map_err(|_| Error::<T, I>::InsufficientBalanceToApply)?;

            // Ensure application text is valid
            Self::ensure_application_text_is_valid(&human_readable_text)?;

            // Ensure application can actually be added
            ensure_on_wrapped_error!(
                hiring::Module::<T>::ensure_can_add_application(
                    opening.hiring_opening_id,
                    opt_role_stake_balance,
                    opt_application_stake_balance)
            )?;

            // Ensure member does not have an active application to this opening
            Self::ensure_member_has_no_active_application_on_opening(
                opening.applications,
                member_id
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Make imbalances for staking
            let opt_role_stake_imbalance = Self::make_stake_opt_imbalance(&opt_role_stake_balance, &source_account);
            let opt_application_stake_imbalance = Self::make_stake_opt_imbalance(&opt_application_stake_balance, &source_account);

            // Call hiring module to add application
            let add_application = ensure_on_wrapped_error!(
                    hiring::Module::<T>::add_application(
                    opening.hiring_opening_id,
                    opt_role_stake_imbalance,
                    opt_application_stake_imbalance,
                    human_readable_text
                )
            )?;

            let hiring_application_id = add_application.application_id_added;

            // Save member id to refund the stakes. This piece of date should outlive the 'worker'.
            <MemberIdByHiringApplicationId<T, I>>::insert(hiring_application_id, member_id);

            // Get id of new worker/lead application
            let new_application_id = NextApplicationId::<T, I>::get();

            // Make worker/lead application
            let application = Application::new(&role_account_id, &opening_id, &member_id, &hiring_application_id);

            // Store application
            ApplicationById::<T, I>::insert(new_application_id, application);

            // Update next application identifier value
            NextApplicationId::<T, I>::mutate(|id| *id += <ApplicationId<T> as One>::one());

            // Add application to set of application in worker opening
            OpeningById::<T, I>::mutate(opening_id, |opening| {
                opening.applications.insert(new_application_id);
            });

            // Trigger event
            Self::deposit_event(RawEvent::AppliedOnOpening(opening_id, new_application_id));
        }

        /// Withdraw the worker application. Can be done by the worker itself only.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn withdraw_application(
            origin,
            application_id: ApplicationId<T>
        ) {
            // Ensuring worker application actually exists
            let (application, _, opening) = Self::ensure_application_exists(&application_id)?;

            // Ensure that it is signed
            let signer_account = ensure_signed(origin)?;

            // Ensure that signer is applicant role account
            ensure!(
                signer_account == application.role_account_id,
                Error::<T, I>::OriginIsNotApplicant
            );

            //
            // == MUTATION SAFE ==
            //

            // Attempt to deactivate application
            // NB: Combined ensure check and mutation in hiring module
            ensure_on_wrapped_error!(
                hiring::Module::<T>::deactive_application(
                    application.hiring_application_id,
                    opening.policy_commitment.exit_role_application_stake_unstaking_period,
                    opening.policy_commitment.exit_role_stake_unstaking_period
                )
            )?;

            // Trigger event
            Self::deposit_event(RawEvent::ApplicationWithdrawn(application_id));
        }

        /// Terminate the worker application. Can be done by the lead only.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn terminate_application(
            origin,
            application_id: ApplicationId<T>
        ) {

            // Ensure lead is set and is origin signer
            Self::ensure_origin_is_active_leader(origin)?;

            // Ensuring worker application actually exists
            let (application, _, opening) = Self::ensure_application_exists(&application_id)?;

            // Attempt to deactivate application.
            // NB: Combined ensure check and mutation in hiring module.
            ensure_on_wrapped_error!(
                hiring::Module::<T>::deactive_application(
                    application.hiring_application_id,
                    opening.policy_commitment.terminate_application_stake_unstaking_period,
                    opening.policy_commitment.terminate_role_stake_unstaking_period
                )
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Trigger event
            Self::deposit_event(RawEvent::ApplicationTerminated(application_id));
        }

        /// Begin reviewing, and therefore not accepting new applications.
        /// Require signed leader origin or the root (to begin review applications for the leader position).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn begin_applicant_review(origin, opening_id: OpeningId<T>) {
            // Ensure opening exists
            // NB: Even though call to hiring modul will have implicit check for
            // existence of opening as well, this check is to make sure that the opening is for
            // this working group, not something else.
            let (opening, _opening) = Self::ensure_opening_exists(&opening_id)?;

            Self::ensure_origin_for_opening_type(origin, opening.opening_type)?;

            //
            // == MUTATION SAFE ==
            //

            // Attempt to begin review of applications.
            // NB: Combined ensure check and mutation in hiring module.
            ensure_on_wrapped_error!(
                hiring::Module::<T>::begin_review(opening.hiring_opening_id)
                )?;

            // Trigger event
            Self::deposit_event(RawEvent::BeganApplicationReview(opening_id));
        }

        /// Fill opening for worker/lead.
        /// Require signed leader origin or the root (to fill opening for the leader position).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn fill_opening(
            origin,
            opening_id: OpeningId<T>,
            successful_application_ids: ApplicationIdSet<T>,
            reward_policy: Option<RewardPolicy<minting::BalanceOf<T>, T::BlockNumber>>
        ) {
            // Ensure worker opening exists
            let (opening, _) = Self::ensure_opening_exists(&opening_id)?;

            Self::ensure_origin_for_opening_type(origin, opening.opening_type)?;

            let potential_worker_number =
                Self::active_worker_count() + (successful_application_ids.len() as u32);

            ensure!(
                potential_worker_number <= T::MaxWorkerNumberLimit::get(),
                Error::<T, I>::MaxActiveWorkerNumberExceeded
            );

            // Cannot hire a lead when another leader exists.
            if matches!(opening.opening_type, OpeningType::Leader) {
                ensure!(!<CurrentLead<T,I>>::exists(), Error::<T, I>::CannotHireLeaderWhenLeaderExists);
            }

            // Ensure a mint exists if lead is providing a reward for positions being filled
            let create_reward_settings = if let Some(policy) = reward_policy {

                // A reward will need to be created so ensure our configured mint exists
                let mint_id = Self::mint();

                // Make sure valid parameters are selected for next payment at block number
                ensure!(policy.next_payment_at_block > <frame_system::Module<T>>::block_number(),
                    Error::<T, I>::FillOpeningInvalidNextPaymentBlock);

                // The verified reward settings to use
                Some((mint_id, policy))
            } else {
                None
            };

            // Make iterator over successful worker application
            let successful_iter = successful_application_ids
                                    .iter()
                                    // recover worker application from id
                                    .map(|application_id| { Self::ensure_application_exists(application_id)})
                                    // remove Err cases, i.e. non-existing applications
                                    .filter_map(|result| result.ok());

            // Count number of successful workers provided
            let num_provided_successful_application_ids = successful_application_ids.len();

            // Ensure all worker applications exist
            let number_of_successful_applications = successful_iter
                                                    .clone()
                                                    .count();

            ensure!(
                number_of_successful_applications == num_provided_successful_application_ids,
                Error::<T, I>::SuccessfulWorkerApplicationDoesNotExist
            );

            // Attempt to fill opening
            let successful_application_ids = successful_iter
                                            .clone()
                                            .map(|(successful_application, _, _)| successful_application.hiring_application_id)
                                            .collect::<BTreeSet<_>>();

            // Check for a single application for a leader.
            if matches!(opening.opening_type, OpeningType::Leader) {
                ensure!(successful_application_ids.len() == 1, Error::<T, I>::CannotHireMultipleLeaders);
            }

            // NB: Combined ensure check and mutation in hiring module
            ensure_on_wrapped_error!(
                hiring::Module::<T>::fill_opening(
                    opening.hiring_opening_id,
                    successful_application_ids,
                    opening.policy_commitment.fill_opening_successful_applicant_application_stake_unstaking_period,
                    opening.policy_commitment.fill_opening_failed_applicant_application_stake_unstaking_period,
                    opening.policy_commitment.fill_opening_failed_applicant_role_stake_unstaking_period
                )
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Process successful applications
            let application_id_to_worker_id = Self::fulfill_successful_applications(
                &opening,
                create_reward_settings,
                successful_iter.collect()
            );

            // Trigger event
            Self::deposit_event(RawEvent::OpeningFilled(opening_id, application_id_to_worker_id));
        }

        // ****************** Stakes **********************

        /// Slashes the worker stake, demands a leader origin. No limits, no actions on zero stake.
        /// If slashing balance greater than the existing stake - stake is slashed to zero.
        /// Require signed leader origin or the root (to slash the leader stake).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn slash_stake(origin, worker_id: WorkerId<T>, balance: BalanceOf<T>) {
            // Ensure lead is set or it is the council terminating the leader.
            Self::ensure_origin_for_leader(origin, worker_id)?;

            // Ensuring worker actually exists.
            let worker = Self::ensure_worker_exists(&worker_id)?;

            ensure!(balance != <BalanceOf<T>>::zero(), Error::<T, I>::StakeBalanceCannotBeZero);

            let stake_profile = worker.role_stake_profile.ok_or(Error::<T, I>::NoWorkerStakeProfile)?;

            //
            // == MUTATION SAFE ==
            //

            // This external module call both checks and mutates the state.
            ensure_on_wrapped_error!(
                <stake::Module<T>>::slash_immediate(
                    &stake_profile.stake_id,
                    balance,
                    false
                )
            )?;

            Self::deposit_event(RawEvent::StakeSlashed(worker_id));
        }

        /// Decreases the worker/lead stake and returns the remainder to the worker role_account_id.
        /// Can be decreased to zero, no actions on zero stake.
        /// Require signed leader origin or the root (to decrease the leader stake).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn decrease_stake(origin, worker_id: WorkerId<T>, balance: BalanceOf<T>) {
            // Ensure lead is set or it is the council terminating the leader.
            Self::ensure_origin_for_leader(origin, worker_id)?;

            let worker = Self::ensure_worker_exists(&worker_id)?;

            ensure!(balance != <BalanceOf<T>>::zero(), Error::<T, I>::StakeBalanceCannotBeZero);

            let stake_profile = worker.role_stake_profile.ok_or(Error::<T, I>::NoWorkerStakeProfile)?;

            //
            // == MUTATION SAFE ==
            //

            // This external module call both checks and mutates the state.
            ensure_on_wrapped_error!(
                <stake::Module<T>>::decrease_stake_to_account(
                    &stake_profile.stake_id,
                    &worker.role_account_id,
                    balance
                )
            )?;

            Self::deposit_event(RawEvent::StakeDecreased(worker_id));
        }

        /// Increases the worker/lead stake, demands a worker origin. Transfers tokens from the worker
        /// role_account_id to the stake. No limits on the stake.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn increase_stake(origin, worker_id: WorkerId<T>, balance: BalanceOf<T>) {
            // Checks worker origin, worker existence
            let worker = Self::ensure_worker_signed(origin, &worker_id)?;

            ensure!(balance != <BalanceOf<T>>::zero(), Error::<T, I>::StakeBalanceCannotBeZero);

            let stake_profile = worker.role_stake_profile.ok_or(Error::<T, I>::NoWorkerStakeProfile)?;

            //
            // == MUTATION SAFE ==
            //

            // This external module call both checks and mutates the state.
            ensure_on_wrapped_error!(
                <stake::Module<T>>::increase_stake_from_account(
                    &stake_profile.stake_id,
                    &worker.role_account_id,
                    balance
                )
            )?;

            Self::deposit_event(RawEvent::StakeIncreased(worker_id));
        }

        /// Sets the capacity to enable working group budget. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_mint_capacity(
            origin,
            new_capacity: minting::BalanceOf<T>
        ) {
            ensure_root(origin)?;

            let mint_id = Self::mint();

            // Technically this is a bug-check and should not be here.
            ensure!(<minting::Mints<T>>::contains_key(mint_id), Error::<T, I>::CannotFindMint);

            // Mint must exist - it is set at genesis or migration.
            let mint = <minting::Module<T>>::mints(mint_id);

            let current_capacity = mint.capacity();

            //
            // == MUTATION SAFE ==
            //

            if new_capacity != current_capacity {
                ensure_on_wrapped_error!(
                    <minting::Module<T>>::set_mint_capacity(mint_id, new_capacity)
                )?;

                Self::deposit_event(RawEvent::MintCapacityChanged(mint_id, new_capacity));
            }
        }
    }
}

// ****************** Ensures **********************

impl<T: Trait<I>, I: Instance> Module<T, I> {
    fn ensure_opening_policy_commitment_is_valid(
        policy_commitment: &OpeningPolicyCommitment<T::BlockNumber, BalanceOf<T>>,
    ) -> Result<(), Error<T, I>> {
        // Helper function. Ensures that unstaking period is None or non-zero.
        fn check_unstaking_period<BlockNumber: PartialEq + Zero, Error>(
            unstaking_period: Option<BlockNumber>,
            error: Error,
        ) -> Result<(), Error> {
            if let Some(unstaking_period) = unstaking_period {
                ensure!(unstaking_period != Zero::zero(), error);
            }
            Ok(())
        }

        // Helper function. Ensures that unstaking period is None or non-zero in the staking_policy.
        fn check_staking_policy<Balance, BlockNumber: PartialEq + Zero, Error>(
            staking_policy: Option<hiring::StakingPolicy<Balance, BlockNumber>>,
            crowded_out_unstaking_period_error: Error,
            review_period_unstaking_period_error: Error,
        ) -> Result<(), Error> {
            if let Some(staking_policy) = staking_policy {
                check_unstaking_period(
                    staking_policy.crowded_out_unstaking_period_length,
                    crowded_out_unstaking_period_error,
                )?;

                check_unstaking_period(
                    staking_policy.review_period_expired_unstaking_period_length,
                    review_period_unstaking_period_error,
                )?;
            }

            Ok(())
        }

        // Check all fill_opening unstaking periods.
        check_unstaking_period(
            policy_commitment.fill_opening_failed_applicant_role_stake_unstaking_period,
            Error::<T, I>::FillOpeningFailedApplicantRoleStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.fill_opening_failed_applicant_application_stake_unstaking_period,
            Error::<T, I>::FillOpeningFailedApplicantApplicationStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.fill_opening_successful_applicant_application_stake_unstaking_period,
            Error::<T, I>::FillOpeningSuccessfulApplicantApplicationStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.exit_role_stake_unstaking_period,
            Error::<T, I>::ExitRoleStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.exit_role_application_stake_unstaking_period,
            Error::<T, I>::ExitRoleApplicationStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.terminate_role_stake_unstaking_period,
            Error::<T, I>::TerminateRoleStakeUnstakingPeriodIsZero,
        )?;

        check_unstaking_period(
            policy_commitment.terminate_application_stake_unstaking_period,
            Error::<T, I>::TerminateApplicationStakeUnstakingPeriodIsZero,
        )?;

        check_staking_policy(
            policy_commitment.role_staking_policy.clone(),
            Error::<T, I>::RoleStakingPolicyCrowdedOutUnstakingPeriodIsZero,
            Error::<T, I>::RoleStakingPolicyReviewPeriodUnstakingPeriodIsZero,
        )?;

        check_staking_policy(
            policy_commitment.application_staking_policy.clone(),
            Error::<T, I>::ApplicationStakingPolicyCrowdedOutUnstakingPeriodIsZero,
            Error::<T, I>::ApplicationStakingPolicyReviewPeriodUnstakingPeriodIsZero,
        )?;

        if let Some(application_rationing_policy) =
            policy_commitment.application_rationing_policy.clone()
        {
            ensure!(
                application_rationing_policy.max_active_applicants > 0,
                Error::<T, I>::ApplicationRationingPolicyMaxActiveApplicantsIsZero
            );
        }

        Ok(())
    }

    fn ensure_origin_for_opening_type(
        origin: T::Origin,
        opening_type: OpeningType,
    ) -> DispatchResult {
        match opening_type {
            OpeningType::Worker => {
                // Ensure lead is set and is origin signer.
                Self::ensure_origin_is_active_leader(origin)
            }
            OpeningType::Leader => {
                // Council proposal.
                ensure_root(origin).map_err(|err| err.into())
            }
        }
    }

    fn ensure_origin_for_leader(
        origin: T::Origin,
        worker_id: WorkerId<T>,
    ) -> Result<ExitInitiationOrigin, DispatchError> {
        let leader_worker_id = Self::ensure_lead_is_set()?;

        let (worker_opening_type, exit_origin) = if leader_worker_id == worker_id {
            (OpeningType::Leader, ExitInitiationOrigin::Sudo)
        } else {
            (OpeningType::Worker, ExitInitiationOrigin::Lead)
        };

        Self::ensure_origin_for_opening_type(origin, worker_opening_type)?;

        Ok(exit_origin)
    }

    fn ensure_lead_is_set() -> Result<WorkerId<T>, Error<T, I>> {
        let leader_worker_id = Self::current_lead();

        if let Some(leader_worker_id) = leader_worker_id {
            Ok(leader_worker_id)
        } else {
            Err(Error::<T, I>::CurrentLeadNotSet)
        }
    }

    // Checks that provided lead account id belongs to the current working group leader
    fn ensure_is_lead_account(lead_account_id: T::AccountId) -> DispatchResult {
        let leader_worker_id = Self::ensure_lead_is_set()?;

        let leader = Self::worker_by_id(leader_worker_id);

        if leader.role_account_id != lead_account_id {
            return Err(Error::<T, I>::IsNotLeadAccount.into());
        }

        Ok(())
    }

    fn ensure_opening_human_readable_text_is_valid(text: &[u8]) -> DispatchResult {
        <OpeningHumanReadableText<I>>::get().ensure_valid(
            text.len(),
            Error::<T, I>::OpeningTextTooShort.into(),
            Error::<T, I>::OpeningTextTooLong.into(),
        )
    }

    /// Ensures origin is signed by the leader.
    pub fn ensure_origin_is_active_leader(origin: T::Origin) -> DispatchResult {
        // Ensure is signed
        let signer = ensure_signed(origin)?;

        Self::ensure_is_lead_account(signer)
    }

    fn ensure_opening_exists(opening_id: &OpeningId<T>) -> Result<OpeningInfo<T>, Error<T, I>> {
        ensure!(
            OpeningById::<T, I>::contains_key(opening_id),
            Error::<T, I>::OpeningDoesNotExist
        );

        let opening = OpeningById::<T, I>::get(opening_id);

        let hiring_opening = hiring::OpeningById::<T>::get(opening.hiring_opening_id);

        Ok((opening, hiring_opening))
    }

    fn ensure_member_has_no_active_application_on_opening(
        applications: ApplicationIdSet<T>,
        member_id: T::MemberId,
    ) -> Result<(), Error<T, I>> {
        for application_id in applications {
            let application = ApplicationById::<T, I>::get(application_id);
            // Look for application by the member for the opening
            if application.member_id != member_id {
                continue;
            }
            // Get application details
            let application = <hiring::ApplicationById<T>>::get(application.hiring_application_id);
            // Return error if application is in active stage
            if application.stage == hiring::ApplicationStage::Active {
                return Err(Error::<T, I>::MemberHasActiveApplicationOnOpening);
            }
        }
        // Member does not have any active applications to the opening
        Ok(())
    }

    fn ensure_application_text_is_valid(text: &[u8]) -> DispatchResult {
        <WorkerApplicationHumanReadableText<I>>::get().ensure_valid(
            text.len(),
            Error::<T, I>::WorkerApplicationTextTooShort.into(),
            Error::<T, I>::WorkerApplicationTextTooLong.into(),
        )
    }

    // CRITICAL:
    // https://github.com/Joystream/substrate-runtime-joystream/issues/92
    // This assumes that ensure_can_withdraw can be done
    // for a sum of balance that later will be actually withdrawn
    // using individual terms in that sum.
    // This needs to be fully checked across all possibly scenarios
    // of actual balance, minimum balance limit, reservation, vesting and locking.
    fn ensure_can_make_stake_imbalance(
        opt_balances: Vec<&Option<BalanceOf<T>>>,
        source_account: &T::AccountId,
    ) -> DispatchResult {
        let zero_balance = <BalanceOf<T> as Zero>::zero();

        // Total amount to be staked
        let total_amount = opt_balances.iter().fold(zero_balance, |sum, opt_balance| {
            sum + if let Some(balance) = opt_balance {
                *balance
            } else {
                zero_balance
            }
        });

        if total_amount > zero_balance {
            // Ensure that
            if CurrencyOf::<T>::free_balance(source_account) < total_amount {
                Err(Error::<T, I>::InsufficientBalanceToCoverStake.into())
            } else {
                let new_balance = CurrencyOf::<T>::free_balance(source_account) - total_amount;

                CurrencyOf::<T>::ensure_can_withdraw(
                    source_account,
                    total_amount,
                    WithdrawReasons::all(),
                    new_balance,
                )
            }
        } else {
            Ok(())
        }
    }

    fn ensure_application_exists(
        application_id: &ApplicationId<T>,
    ) -> Result<ApplicationInfo<T>, Error<T, I>> {
        ensure!(
            ApplicationById::<T, I>::contains_key(application_id),
            Error::<T, I>::WorkerApplicationDoesNotExist
        );

        let application = ApplicationById::<T, I>::get(application_id);

        let opening = OpeningById::<T, I>::get(application.opening_id);

        Ok((application, *application_id, opening))
    }

    /// Ensures the origin contains signed account that belongs to existing worker.
    pub fn ensure_worker_signed(
        origin: T::Origin,
        worker_id: &WorkerId<T>,
    ) -> Result<WorkerOf<T>, DispatchError> {
        // Ensure that it is signed
        let signer_account = ensure_signed(origin)?;

        // Ensure that id corresponds to active worker
        let worker = Self::ensure_worker_exists(&worker_id)?;

        // Ensure that signer is actually role account of worker
        ensure!(
            signer_account == worker.role_account_id,
            Error::<T, I>::SignerIsNotWorkerRoleAccount
        );

        Ok(worker)
    }

    /// Ensures worker under given id already exists
    pub fn ensure_worker_exists(worker_id: &WorkerId<T>) -> Result<WorkerOf<T>, Error<T, I>> {
        ensure!(
            WorkerById::<T, I>::contains_key(worker_id),
            Error::<T, I>::WorkerDoesNotExist
        );

        let worker = WorkerById::<T, I>::get(worker_id);

        Ok(worker)
    }

    fn ensure_worker_has_recurring_reward(
        worker: &WorkerOf<T>,
    ) -> Result<T::RewardRelationshipId, Error<T, I>> {
        if let Some(relationship_id) = worker.reward_relationship {
            Ok(relationship_id)
        } else {
            Err(Error::<T, I>::WorkerHasNoReward)
        }
    }

    fn ensure_worker_exit_rationale_text_is_valid(text: &[u8]) -> DispatchResult {
        Self::worker_exit_rationale_text().ensure_valid(
            text.len(),
            Error::<T, I>::WorkerExitRationaleTextTooShort.into(),
            Error::<T, I>::WorkerExitRationaleTextTooLong.into(),
        )
    }
}

/// Creates default text constraint.
pub fn default_text_constraint() -> InputValidationLengthConstraint {
    InputValidationLengthConstraint::new(1, 1024)
}

impl<T: Trait<I>, I: Instance> Module<T, I> {
    /// Callback from StakingEventsHandler. Refunds unstaked imbalance back to the source account.
    pub fn refund_working_group_stake(
        stake_id: StakeId<T>,
        imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(stake_id) {
            print("Working group broken invariant: no stake id in the hiring module.");
            return imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(stake_id);

        if !MemberIdByHiringApplicationId::<T, I>::contains_key(hiring_application_id) {
            // Stake is not related to the working group module.
            return imbalance;
        }

        let member_id = Module::<T, I>::member_id_by_hiring_application_id(hiring_application_id);

        if membership::MembershipById::<T>::contains_key(member_id) {
            let member_profile = membership::MembershipById::<T>::get(member_id);
            let refunding_result = CurrencyOf::<T>::resolve_into_existing(
                &member_profile.controller_account,
                imbalance,
            );

            if refunding_result.is_err() {
                print("Working group broken invariant: cannot refund.");
                // cannot return imbalance here, because of possible double spending.
                return <NegativeImbalance<T>>::zero();
            }
        } else {
            print("Working group broken invariant: no member profile.");
            return imbalance;
        }

        <NegativeImbalance<T>>::zero()
    }

    /// Returns all existing worker id list excluding the current leader worker id.
    pub fn get_regular_worker_ids() -> Vec<WorkerId<T>> {
        let lead_worker_id = Self::current_lead();

        <WorkerById<T, I>>::iter()
            .filter_map(|(worker_id, _)| {
                // Filter the leader worker id if the leader is set.
                lead_worker_id
                    .clone()
                    .map_or(Some(worker_id), |lead_worker_id| {
                        if worker_id == lead_worker_id {
                            None
                        } else {
                            Some(worker_id)
                        }
                    })
            })
            .collect()
    }

    /// Returns all existing worker id list.
    pub fn get_all_worker_ids() -> Vec<WorkerId<T>> {
        <WorkerById<T, I>>::iter()
            .map(|(worker_id, _)| worker_id)
            .collect()
    }

    fn make_stake_opt_imbalance(
        opt_balance: &Option<BalanceOf<T>>,
        source_account: &T::AccountId,
    ) -> Option<NegativeImbalance<T>> {
        if let Some(balance) = opt_balance {
            let withdraw_result = CurrencyOf::<T>::withdraw(
                source_account,
                *balance,
                WithdrawReasons::all(),
                ExistenceRequirement::AllowDeath,
            );

            assert!(withdraw_result.is_ok());

            withdraw_result.ok()
        } else {
            None
        }
    }

    fn deactivate_worker(
        worker_id: &WorkerId<T>,
        worker: &WorkerOf<T>,
        exit_initiation_origin: &ExitInitiationOrigin,
        rationale_text: &[u8],
    ) -> Result<(), Error<T, I>> {
        // Stop any possible recurring rewards

        if let Some(reward_relationship_id) = worker.reward_relationship {
            // Attempt to deactivate
            recurringrewards::Module::<T>::try_to_deactivate_relationship(reward_relationship_id)
                .map_err(|_| Error::<T, I>::RelationshipMustExist)?;
        }; // else: Did not deactivate, there was no reward relationship!

        // Unstake if stake profile exists
        if let Some(ref stake_profile) = worker.role_stake_profile {
            // Determine unstaking period based on who initiated deactivation
            let unstaking_period = match exit_initiation_origin {
                ExitInitiationOrigin::Lead => stake_profile.termination_unstaking_period,
                ExitInitiationOrigin::Sudo => stake_profile.termination_unstaking_period,
                ExitInitiationOrigin::Worker => stake_profile.exit_unstaking_period,
            };

            // Unstake
            ensure_on_wrapped_error!(stake::Module::<T>::initiate_unstaking(
                &stake_profile.stake_id,
                unstaking_period
            ))?;
        }

        // Unset lead if the leader is leaving.
        let leader_worker_id = <CurrentLead<T, I>>::get();
        if let Some(leader_worker_id) = leader_worker_id {
            if leader_worker_id == *worker_id {
                Self::unset_lead();
            }
        }

        // Remove the worker from the storage.
        WorkerById::<T, I>::remove(worker_id);
        Self::decrease_active_worker_counter();

        // Trigger the event
        let event = match exit_initiation_origin {
            ExitInitiationOrigin::Lead => {
                RawEvent::TerminatedWorker(*worker_id, rationale_text.to_vec())
            }
            ExitInitiationOrigin::Worker => {
                RawEvent::WorkerExited(*worker_id, rationale_text.to_vec())
            }
            ExitInitiationOrigin::Sudo => {
                RawEvent::TerminatedLeader(*worker_id, rationale_text.to_vec())
            }
        };

        Self::deposit_event(event);

        Ok(())
    }

    /// Initialize working group constraints and mint.
    pub fn initialize_working_group(
        opening_human_readable_text_constraint: InputValidationLengthConstraint,
        worker_application_human_readable_text_constraint: InputValidationLengthConstraint,
        worker_exit_rationale_text_constraint: InputValidationLengthConstraint,
        working_group_mint_capacity: minting::BalanceOf<T>,
    ) {
        // Create a mint.
        let mint_id_result = <minting::Module<T>>::add_mint(working_group_mint_capacity, None);

        if let Ok(mint_id) = mint_id_result {
            <Mint<T, I>>::put(mint_id);
        } else {
            panic!("Failed to create a mint for the working group");
        }

        // Create constraints
        <OpeningHumanReadableText<I>>::put(opening_human_readable_text_constraint);
        <WorkerApplicationHumanReadableText<I>>::put(
            worker_application_human_readable_text_constraint,
        );
        <WorkerExitRationaleText<I>>::put(worker_exit_rationale_text_constraint);
    }

    // Set worker id as a leader id.
    pub(crate) fn set_lead(worker_id: WorkerId<T>) {
        // Update current lead
        <CurrentLead<T, I>>::put(worker_id);

        // Trigger an event
        Self::deposit_event(RawEvent::LeaderSet(worker_id));
    }

    // Evict the currently set lead.
    pub(crate) fn unset_lead() {
        if Self::ensure_lead_is_set().is_ok() {
            // Update current lead
            <CurrentLead<T, I>>::kill();

            Self::deposit_event(RawEvent::LeaderUnset());
        }
    }

    // Processes successful application during the fill_opening().
    fn fulfill_successful_applications(
        opening: &OpeningOf<T>,
        reward_settings: Option<RewardSettings<T>>,
        successful_applications_info: Vec<ApplicationInfo<T>>,
    ) -> BTreeMap<ApplicationId<T>, WorkerId<T>> {
        let mut application_id_to_worker_id = BTreeMap::new();

        successful_applications_info
            .iter()
            .for_each(|(successful_application, id, _)| {
                // Create a reward relationship.
                let reward_relationship = if let Some((mint_id, checked_policy)) =
                    reward_settings.clone()
                {
                    // Create a new recipient for the new relationship.
                    let recipient = <recurringrewards::Module<T>>::add_recipient();

                    // Member must exist, since it was checked that it can enter the role.
                    let member_profile =
                        <membership::Module<T>>::membership(successful_application.member_id);

                    // Rewards are deposited in the member's root account.
                    let reward_destination_account = member_profile.root_account;

                    // Values have been checked so this should not fail!
                    let relationship_id = <recurringrewards::Module<T>>::add_reward_relationship(
                        mint_id,
                        recipient,
                        reward_destination_account,
                        checked_policy.amount_per_payout,
                        checked_policy.next_payment_at_block,
                        checked_policy.payout_interval,
                    )
                    .expect("Failed to create reward relationship!");

                    Some(relationship_id)
                } else {
                    None
                };

                // Get possible stake for role
                let application =
                    hiring::ApplicationById::<T>::get(successful_application.hiring_application_id);

                // Staking profile for worker
                let stake_profile = application.active_role_staking_id.as_ref().map(|stake_id| {
                    RoleStakeProfile::new(
                        stake_id,
                        &opening
                            .policy_commitment
                            .terminate_role_stake_unstaking_period,
                        &opening.policy_commitment.exit_role_stake_unstaking_period,
                    )
                });

                // Get worker id
                let new_worker_id = <NextWorkerId<T, I>>::get();

                // Construct worker
                let worker = Worker::new(
                    &successful_application.member_id,
                    &successful_application.role_account_id,
                    &reward_relationship,
                    &stake_profile,
                );

                // Store a worker
                <WorkerById<T, I>>::insert(new_worker_id, worker);
                Self::increase_active_worker_counter();

                // Update next worker id
                <NextWorkerId<T, I>>::mutate(|id| *id += <WorkerId<T> as One>::one());

                application_id_to_worker_id.insert(*id, new_worker_id);

                // Sets a leader on successful opening when opening is for leader.
                if matches!(opening.opening_type, OpeningType::Leader) {
                    Self::set_lead(new_worker_id);
                }
            });

        application_id_to_worker_id
    }

    // Increases active worker counter (saturating).
    fn increase_active_worker_counter() {
        let next_active_worker_count_value = Self::active_worker_count().saturating_add(1);
        <ActiveWorkerCount<I>>::put(next_active_worker_count_value);
    }

    // Decreases active worker counter (saturating).
    fn decrease_active_worker_counter() {
        let next_active_worker_count_value = Self::active_worker_count().saturating_sub(1);
        <ActiveWorkerCount<I>>::put(next_active_worker_count_value);
    }
}
