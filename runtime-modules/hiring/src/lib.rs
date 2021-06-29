//! Hiring substrate module for the Joystream platform
//!
//! Public APIs:
//! - add_opening
//! - ensure_can_add_application
//! - add_application
//! - deactivate_application
//! - cancel_opening
//! - fill_opening
//! - begin_review
//! - begin_acception_application
//! - unstaked
//!
//! Dependency: Joystream stake module

#![allow(clippy::collapsible_match)]
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

// Test dependencies
#[cfg(all(test, not(target_arch = "wasm32")))]
use mockall::predicate::*;
#[cfg(all(test, not(target_arch = "wasm32")))]
use mockall::*;

use codec::Codec;
use frame_support::storage::IterableStorageMap;
use frame_support::traits::{Currency, Imbalance};
use frame_support::{decl_module, decl_storage, ensure, Parameter};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{MaybeSerialize, Member};
use sp_std::cell::RefCell;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::iter::Iterator;
use sp_std::rc::Rc;
use sp_std::vec::Vec;

use stake::{Config as StakeTrait, InitiateUnstakingError, Stake, StakeActionError, StakingError};

mod hiring;
#[macro_use]
mod macroes;
mod mock;
mod test;

pub use hiring::*;

/// Main trait of hiring substrate module
pub trait Config: frame_system::Config + stake::Config + Sized {
    /// OpeningId type
    type OpeningId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// ApplicationId type
    type ApplicationId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Type that will handle various staking events
    type ApplicationDeactivatedHandler: ApplicationDeactivatedHandler<Self>;

    /// Marker type for Stake module handler. Indicates that hiring module uses stake module mock.
    type StakeHandlerProvider: StakeHandlerProvider<Self>;
}

decl_storage! {
    trait Store for Module<T: Config> as Hiring {
        /// Openings.
        pub OpeningById get(fn opening_by_id): map hasher(blake2_128_concat)
            T::OpeningId => Opening<BalanceOf<T>, T::BlockNumber, T::ApplicationId>;

        /// Identifier for next opening to be added.
        pub NextOpeningId get(fn next_opening_id): T::OpeningId;

        /// Applications
        pub ApplicationById get(fn application_by_id): map hasher(blake2_128_concat)
            T::ApplicationId => Application<T::OpeningId, T::BlockNumber, T::StakeId>;

        /// Identifier for next application to be added.
        pub NextApplicationId get(fn next_application_id): T::ApplicationId;

        /// Internal purpose of given stake, i.e. fro what application, and whether for the role or for the application.
        pub ApplicationIdByStakingId get(fn stake_purpose_by_staking_id): map hasher(blake2_128_concat)
            T::StakeId => T::ApplicationId;
    }
}

decl_module! {
    /// Main hiring module definition
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        fn on_finalize(now: T::BlockNumber) {

            //
            // == MUTATION SAFE ==
            //

            // Change opening from WaitingToBegin stage to Active::AcceptingApplications stage
            for (opening_id, opening) in Self::openings_waiting_to_begin_iterator(now) {
                let opening_accepting_applications = opening.clone_with_new_active_opening_stage(
                    hiring::ActiveOpeningStage::AcceptingApplications {
                        started_accepting_applicants_at_block: now
                });

                <OpeningById<T>>::insert(opening_id, opening_accepting_applications);
            }

            // Deactivate opening
            for (opening_id,
                opening,
                (
                    applications_added,
                    started_accepting_applicants_at_block,
                    started_review_period_at_block
                )) in Self::openings_expired_review_period_iterator(now) {

                //
                // Deactivate all applications that are part of this opening
                //

                // Get unstaking periods
                let application_stake_unstaking_period = StakingPolicy::opt_staking_policy_to_review_period_expired_unstaking_period(&opening.application_staking_policy);
                let role_stake_unstaking_period = StakingPolicy::opt_staking_policy_to_review_period_expired_unstaking_period(&opening.role_staking_policy);

                // Get applications
                let applications_map = Self::application_id_iter_to_map(applications_added.iter());

                // Deactivate applications
                Self::initiate_application_deactivations(
                    &applications_map,
                    application_stake_unstaking_period,
                    role_stake_unstaking_period,
                    hiring::ApplicationDeactivationCause::ReviewPeriodExpired
                );

                let deactivated_opening =
                    opening.clone_with_new_active_opening_stage(
                        hiring::ActiveOpeningStage::Deactivated {
                            cause: hiring::OpeningDeactivationCause::ReviewPeriodExpired,
                            deactivated_at_block: now,
                            started_accepting_applicants_at_block,
                            started_review_period_at_block: Some(started_review_period_at_block),
                    });

                <OpeningById<T>>::insert(opening_id, deactivated_opening);
            }
        }
    }
}

/*
 *  ======== Main API implementation ========
 */

// Public API implementation
impl<T: Config> Module<T> {
    /// Add new opening based on given inputs policies.
    /// The new Opening instance has stage WaitingToBegin, and is added to openingsById,
    /// and has identifier equal to nextOpeningId.
    /// The latter is incremented. The used identifier is returned.
    pub fn add_opening(
        activate_at: ActivateOpeningAt<T::BlockNumber>,
        max_review_period_length: T::BlockNumber,
        application_rationing_policy: Option<ApplicationRationingPolicy>,
        application_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        role_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        human_readable_text: Vec<u8>,
    ) -> Result<T::OpeningId, AddOpeningError> {
        let current_block_height = <frame_system::Pallet<T>>::block_number();

        Self::ensure_can_add_opening(
            current_block_height,
            activate_at.clone(),
            T::Currency::minimum_balance(),
            application_rationing_policy.clone(),
            application_staking_policy.clone(),
            role_staking_policy.clone(),
        )?;

        // == MUTATION SAFE ==

        let new_opening = hiring::Opening::new(
            current_block_height,
            activate_at,
            max_review_period_length,
            application_rationing_policy,
            application_staking_policy,
            role_staking_policy,
            human_readable_text,
        );

        // Get Id for new opening
        let new_opening_id = <NextOpeningId<T>>::get();

        // Insert opening in storage
        <OpeningById<T>>::insert(new_opening_id, new_opening);

        // Update NextOpeningId counter
        <NextOpeningId<T>>::mutate(|id| *id += T::OpeningId::one());

        // Return
        Ok(new_opening_id)
    }

    /// Cancels opening with given identifier, using provided unstaking periods for
    /// application and role, as necesary.
    pub fn cancel_opening(
        opening_id: T::OpeningId,
        application_stake_unstaking_period: Option<T::BlockNumber>,
        role_stake_unstaking_period: Option<T::BlockNumber>,
    ) -> Result<OpeningCancelled, CancelOpeningError> {
        // Ensure that the opening exists
        let opening =
            ensure_opening_exists!(T, opening_id, CancelOpeningError::OpeningDoesNotExist)?;

        // Opening is in stage Active.{AcceptingApplications or ReviewPeriod}

        let (
            active_stage,
            applications_added,
            active_application_count,
            unstaking_application_count,
            deactivated_application_count,
        ) = ensure_opening_is_active!(
            opening.stage,
            CancelOpeningError::OpeningNotInCancellableStage
        )?;

        //
        let current_block_height = <frame_system::Pallet<T>>::block_number(); // move later!
        let new_active_stage = active_stage.new_stage_on_cancelling(current_block_height)?;

        // Ensure unstaking periods are OK.
        ensure_opt_unstaking_period_is_ok!(
            application_stake_unstaking_period,
            opening.application_staking_policy,
            CancelOpeningError::UnstakingPeriodTooShort(StakePurpose::Application),
            CancelOpeningError::RedundantUnstakingPeriodProvided(StakePurpose::Application)
        )?;

        ensure_opt_unstaking_period_is_ok!(
            role_stake_unstaking_period,
            opening.role_staking_policy,
            CancelOpeningError::UnstakingPeriodTooShort(StakePurpose::Role),
            CancelOpeningError::RedundantUnstakingPeriodProvided(StakePurpose::Role)
        )?;

        //
        // == MUTATION SAFE ==
        //

        // Create and store new cancelled opening
        let new_opening = Opening {
            stage: hiring::OpeningStage::Active {
                stage: new_active_stage,
                applications_added: applications_added.clone(),
                active_application_count,
                unstaking_application_count,
                deactivated_application_count,
            },
            ..opening
        };

        OpeningById::<T>::insert(opening_id, new_opening);

        // Map with applications
        let applications_map = Self::application_id_iter_to_map(applications_added.iter());

        // Initiate deactivation of all active applications
        let net_result = Self::initiate_application_deactivations(
            &applications_map,
            application_stake_unstaking_period,
            role_stake_unstaking_period,
            hiring::ApplicationDeactivationCause::OpeningCancelled,
        );

        // Return
        Ok(OpeningCancelled {
            number_of_unstaking_applications: net_result.number_of_unstaking_applications,
            number_of_deactivated_applications: net_result.number_of_deactivated_applications,
        })
    }

    /// Transit opening to the accepting application stage.
    /// Applies when given opening is in WaitingToBegin stage.
    /// The stage is updated to Active stage with AcceptingApplications substage
    pub fn begin_accepting_applications(
        opening_id: T::OpeningId,
    ) -> Result<(), BeginAcceptingApplicationsError> {
        // Ensure that the opening exists
        let opening = ensure_opening_exists!(
            T,
            opening_id,
            BeginAcceptingApplicationsError::OpeningDoesNotExist
        )?;

        // Ensure that it is the waiting to begin stage
        opening.stage.ensure_opening_stage_is_waiting_to_begin(
            BeginAcceptingApplicationsError::OpeningIsNotInWaitingToBeginStage,
        )?;

        //
        // == MUTATION SAFE ==
        //

        let current_block_height = <frame_system::Pallet<T>>::block_number();

        // Update state of opening
        let new_opening = opening.clone_with_new_active_opening_stage(
            hiring::ActiveOpeningStage::AcceptingApplications {
                started_accepting_applicants_at_block: current_block_height,
            },
        );

        // Write back opening
        <OpeningById<T>>::insert(opening_id, new_opening);

        // DONE
        Ok(())
    }

    /// Transit opening to the begin review period stage.
    /// Applies when given opening is in Active stage and AcceptingApplications substage.
    /// The stage is updated to Active stage and ReviewPeriod substage
    pub fn begin_review(opening_id: T::OpeningId) -> Result<(), BeginReviewError> {
        // Ensure that the opening exists
        let opening = ensure_opening_exists!(T, opening_id, BeginReviewError::OpeningDoesNotExist)?;

        // Opening is accepting applications
        let (active_stage, _, _, _, _) = ensure_opening_is_active!(
            opening.stage,
            BeginReviewError::OpeningNotInAcceptingApplicationsStage
        )?;

        let started_accepting_applicants_at_block = ensure_active_opening_is_accepting_applications!(
            active_stage,
            BeginReviewError::OpeningNotInAcceptingApplicationsStage
        )?;

        //
        // == MUTATION SAFE ==
        //

        let current_block_height = <frame_system::Pallet<T>>::block_number();

        let new_opening =
            opening.clone_with_new_active_opening_stage(hiring::ActiveOpeningStage::ReviewPeriod {
                started_accepting_applicants_at_block,
                started_review_period_at_block: current_block_height,
            });

        // Update to new opening
        <OpeningById<T>>::insert(opening_id, new_opening);

        Ok(())
    }

    /// Fill an opening, identified with `opening_id`, currently in the review period.
    /// Applies when given opening is in ReviewPeriod stage.
    /// Given list of applications are deactivated to under the Hired,
    /// all other active applicants are NotHired.
    /// Separately for each group,
    /// unstaking periods for any applicable application and/or role stake must be provided.
    pub fn fill_opening(
        opening_id: T::OpeningId,
        successful_applications: BTreeSet<T::ApplicationId>,
        opt_successful_applicant_application_stake_unstaking_period: Option<T::BlockNumber>,
        opt_failed_applicant_application_stake_unstaking_period: Option<T::BlockNumber>,
        /* this parameter does not make sense? opt_successful_applicant_role_stake_unstaking_period: Option<T::BlockNumber>, */
        opt_failed_applicant_role_stake_unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), FillOpeningError<T>> {
        // Ensure that the opening exists
        let opening = ensure_opening_exists!(T, opening_id, FillOpeningError::OpeningDoesNotExist)?;

        let (active_stage, applications_added, _, _, _) = ensure_opening_is_active!(
            opening.stage,
            FillOpeningError::OpeningNotInReviewPeriodStage
        )?;

        // Ensure opening is in review period
        let (started_accepting_applicants_at_block, started_review_period_at_block) = active_stage
            .ensure_active_opening_is_in_review_period(
                FillOpeningError::OpeningNotInReviewPeriodStage,
            )?;

        //
        // Ensure that all unstaking periods are neither too short (0) nor redundant.
        //

        ensure_opt_unstaking_period_is_ok!(
            opt_successful_applicant_application_stake_unstaking_period,
            opening.application_staking_policy,
            FillOpeningError::UnstakingPeriodTooShort(
                StakePurpose::Application,
                ApplicationOutcomeInFilledOpening::Success
            ),
            FillOpeningError::RedundantUnstakingPeriodProvided(
                StakePurpose::Application,
                ApplicationOutcomeInFilledOpening::Success
            )
        )?;

        ensure_opt_unstaking_period_is_ok!(
            opt_failed_applicant_application_stake_unstaking_period,
            opening.application_staking_policy,
            FillOpeningError::UnstakingPeriodTooShort(
                StakePurpose::Application,
                ApplicationOutcomeInFilledOpening::Failure
            ),
            FillOpeningError::RedundantUnstakingPeriodProvided(
                StakePurpose::Application,
                ApplicationOutcomeInFilledOpening::Failure
            )
        )?;

        ensure_opt_unstaking_period_is_ok!(
            opt_failed_applicant_role_stake_unstaking_period,
            opening.role_staking_policy,
            FillOpeningError::UnstakingPeriodTooShort(
                StakePurpose::Role,
                ApplicationOutcomeInFilledOpening::Failure
            ),
            FillOpeningError::RedundantUnstakingPeriodProvided(
                StakePurpose::Role,
                ApplicationOutcomeInFilledOpening::Failure
            )
        )?;

        // Ensure that all successful applications actually exist
        for application_id in &successful_applications {
            ensure_application_exists!(
                T,
                *application_id,
                FillOpeningError::ApplicationDoesNotExist(*application_id)
            )?;
        }

        let successful_applications_map =
            Self::application_id_iter_to_map(successful_applications.iter());

        // Ensure that all successful applications are actually active and associated with the opening
        for (application_id, application) in &successful_applications_map {
            ensure_eq!(
                application.stage,
                hiring::ApplicationStage::Active,
                FillOpeningError::ApplicationNotInActiveStage(*application_id,)
            );

            ensure_eq!(
                application.opening_id,
                opening_id,
                FillOpeningError::ApplicationForWrongOpening(*application_id)
            );
        }

        //
        // == MUTATION SAFE ==
        //

        // Deactivate all successful applications, with cause being hired
        Self::initiate_application_deactivations(
            &successful_applications_map,
            opt_successful_applicant_application_stake_unstaking_period,
            None,
            hiring::ApplicationDeactivationCause::Hired,
        );

        // Deactivate all unsuccessful applications, with cause being not being hired.

        // First get all failed applications by their id.
        let failed_applications_map = Self::application_id_iter_to_map(
            applications_added.difference(&successful_applications),
        );

        // Deactivate all successful applications, with cause being not hired
        Self::initiate_application_deactivations(
            &failed_applications_map,
            opt_failed_applicant_application_stake_unstaking_period,
            opt_failed_applicant_role_stake_unstaking_period,
            hiring::ApplicationDeactivationCause::NotHired,
        );

        // Grab current block height
        let current_block_height = <frame_system::Pallet<T>>::block_number();
        // Get opening with updated counters
        let opening_needed_for_data = <OpeningById<T>>::get(opening_id);

        // Deactivate opening
        let new_opening = opening_needed_for_data.clone_with_new_active_opening_stage(
            hiring::ActiveOpeningStage::Deactivated {
                cause: OpeningDeactivationCause::Filled,
                deactivated_at_block: current_block_height,
                started_accepting_applicants_at_block,
                started_review_period_at_block: Some(started_review_period_at_block),
            },
        );

        // Write back new opening
        <OpeningById<T>>::insert(opening_id, new_opening);

        // DONE
        Ok(())
    }

    /// Adds a new application on the given opening, and begins staking for
    /// the role, the application or both possibly.
    pub fn ensure_can_add_application(
        opening_id: T::OpeningId,
        opt_role_stake_balance: Option<BalanceOf<T>>,
        opt_application_stake_balance: Option<BalanceOf<T>>,
    ) -> Result<DestructuredApplicationCanBeAddedEvaluation<T>, AddApplicationError> {
        // Ensure that the opening exists
        let opening =
            ensure_opening_exists!(T, opening_id, AddApplicationError::OpeningDoesNotExist)?;

        // Ensure that proposed stakes match the policy of the opening.
        let opt_role_stake_balance = ensure_stake_balance_matches_staking_policy!(
            &opt_role_stake_balance,
            &opening.role_staking_policy,
            AddApplicationError::StakeMissingWhenRequired(StakePurpose::Role),
            AddApplicationError::StakeProvidedWhenRedundant(StakePurpose::Role),
            AddApplicationError::StakeAmountTooLow(StakePurpose::Role)
        )?;

        let opt_application_stake_balance = ensure_stake_balance_matches_staking_policy!(
            &opt_application_stake_balance,
            &opening.application_staking_policy,
            AddApplicationError::StakeMissingWhenRequired(StakePurpose::Application),
            AddApplicationError::StakeProvidedWhenRedundant(StakePurpose::Application),
            AddApplicationError::StakeAmountTooLow(StakePurpose::Application)
        )?;

        // Opening is accepting applications

        let (
            active_stage,
            applications_added,
            active_application_count,
            unstaking_application_count,
            deactivated_application_count,
        ) = ensure_opening_is_active!(
            opening.stage,
            AddApplicationError::OpeningNotInAcceptingApplicationsStage
        )?;

        active_stage.ensure_active_opening_is_accepting_applications(
            AddApplicationError::OpeningNotInAcceptingApplicationsStage,
        )?;

        // Ensure that the new application would actually make it
        let would_get_added_success = ensure_application_would_get_added!(
            &opening.application_rationing_policy,
            &applications_added,
            &opt_role_stake_balance,
            &opt_application_stake_balance,
            AddApplicationError::NewApplicationWasCrowdedOut
        )?;

        Ok(DestructuredApplicationCanBeAddedEvaluation {
            opening,
            active_stage,
            applications_added,
            active_application_count,
            unstaking_application_count,
            deactivated_application_count,
            would_get_added_success,
        })
    }

    /// Adds a new application on the given opening, and begins staking for
    /// the role, the application or both possibly.
    pub fn add_application(
        opening_id: T::OpeningId,
        opt_role_stake_imbalance: Option<NegativeImbalance<T>>,
        opt_application_stake_imbalance: Option<NegativeImbalance<T>>,
        human_readable_text: Vec<u8>,
    ) -> Result<ApplicationAdded<T::ApplicationId>, AddApplicationError> {
        let opt_role_stake_balance = Self::create_stake_balance(&opt_role_stake_imbalance);
        let opt_application_stake_balance =
            Self::create_stake_balance(&opt_application_stake_imbalance);

        let can_be_added_destructured = Self::ensure_can_add_application(
            opening_id,
            opt_role_stake_balance,
            opt_application_stake_balance,
        )?;

        //
        // == MUTATION SAFE ==
        //

        // If required, deactive another application that was crowded out.
        if let ApplicationAddedSuccess::CrowdsOutExistingApplication(
            id_of_croweded_out_application,
        ) = can_be_added_destructured.would_get_added_success
        {
            // Get relevant unstaking periods
            let opt_application_stake_unstaking_period =
                hiring::StakingPolicy::opt_staking_policy_to_crowded_out_unstaking_period(
                    &can_be_added_destructured.opening.application_staking_policy,
                );
            let opt_role_stake_unstaking_period =
                hiring::StakingPolicy::opt_staking_policy_to_crowded_out_unstaking_period(
                    &can_be_added_destructured.opening.role_staking_policy,
                );

            // Fetch application
            let crowded_out_application = <ApplicationById<T>>::get(id_of_croweded_out_application);

            // Initiate actual deactivation
            //
            // MUST not have been ignored, is runtime invariant, false means code is broken.
            // But should we do panic in runtime? Is there safer way?
            let deactivation_result = Self::try_to_initiate_application_deactivation(
                &crowded_out_application,
                id_of_croweded_out_application,
                opt_application_stake_unstaking_period,
                opt_role_stake_unstaking_period,
                hiring::ApplicationDeactivationCause::CrowdedOut,
            );

            assert_ne!(
                deactivation_result,
                ApplicationDeactivationInitiationResult::Ignored
            );
        }

        // Get Id for this new application
        let new_application_id = <NextApplicationId<T>>::get();

        // Possibly initiate staking
        let active_role_staking_id =
            Self::infallible_opt_stake_initiation(opt_role_stake_imbalance, &new_application_id);
        let active_application_staking_id = Self::infallible_opt_stake_initiation(
            opt_application_stake_imbalance,
            &new_application_id,
        );

        // Grab current block height
        let current_block_height = <frame_system::Pallet<T>>::block_number();

        // Compute index for this new application
        let application_index_in_opening =
            can_be_added_destructured.calculate_total_application_count();

        // Create a new application
        let new_application = hiring::Application {
            opening_id,
            application_index_in_opening,
            add_to_opening_in_block: current_block_height,
            active_role_staking_id,
            active_application_staking_id,
            // Stage of new application
            stage: hiring::ApplicationStage::Active,
            human_readable_text,
        };

        // Insert into main application map
        <ApplicationById<T>>::insert(new_application_id, new_application);

        // Update next application id
        <NextApplicationId<T>>::mutate(|id| *id += One::one());

        // Update counter on opening
        // Should reload after possible deactivation in try_to_initiate_application_deactivation
        let opening_needed_for_data = <OpeningById<T>>::get(opening_id);
        let new_active_stage = opening_needed_for_data
            .stage
            .clone_with_added_active_application(new_application_id);

        <OpeningById<T>>::mutate(opening_id, |opening| {
            opening.stage = new_active_stage;
        });

        let application_id_crowded_out = can_be_added_destructured
            .would_get_added_success
            .crowded_out_application_id();

        // DONE
        Ok(ApplicationAdded {
            application_id_added: new_application_id,
            application_id_crowded_out,
        })
    }

    /// Deactive an active application.
    /// Does currently not support slashing
    pub fn deactive_application(
        application_id: T::ApplicationId,
        application_stake_unstaking_period: Option<T::BlockNumber>,
        role_stake_unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), DeactivateApplicationError> {
        // Check that application id is valid, and if so,
        // grab corresponding application and opening.
        let (application, opening) = ensure_application_exists!(
            T,
            application_id,
            DeactivateApplicationError::ApplicationDoesNotExist,
            auto_fetch_opening
        )?;

        // Application is active
        ensure_eq!(
            application.stage,
            hiring::ApplicationStage::Active,
            DeactivateApplicationError::ApplicationNotActive
        );

        // Opening is accepting applications
        let (active_stage, ..) = ensure_opening_is_active!(
            opening.stage,
            DeactivateApplicationError::OpeningNotAcceptingApplications
        )?;

        active_stage.ensure_active_opening_is_accepting_applications(
            DeactivateApplicationError::OpeningNotAcceptingApplications,
        )?;

        // Ensure unstaking periods are OK.
        ensure_opt_unstaking_period_is_ok!(
            application_stake_unstaking_period,
            opening.application_staking_policy,
            DeactivateApplicationError::UnstakingPeriodTooShort(StakePurpose::Application),
            DeactivateApplicationError::RedundantUnstakingPeriodProvided(StakePurpose::Application)
        )?;

        ensure_opt_unstaking_period_is_ok!(
            role_stake_unstaking_period,
            opening.role_staking_policy,
            DeactivateApplicationError::UnstakingPeriodTooShort(StakePurpose::Role),
            DeactivateApplicationError::RedundantUnstakingPeriodProvided(StakePurpose::Role)
        )?;

        //
        // == MUTATION SAFE ==
        //

        // Deactive application
        let result = Self::try_to_initiate_application_deactivation(
            &application,
            application_id,
            application_stake_unstaking_period,
            role_stake_unstaking_period,
            hiring::ApplicationDeactivationCause::External,
        );

        assert_ne!(result, ApplicationDeactivationInitiationResult::Ignored);

        // DONE
        Ok(())
    }

    /// The stake, with the given id, was unstaked.
    pub fn unstaked(stake_id: T::StakeId) -> UnstakedResult {
        // Ignore unstaked
        if !<ApplicationIdByStakingId<T>>::contains_key(stake_id) {
            return UnstakedResult::StakeIdNonExistent;
        }

        // Get application
        let application_id = <ApplicationIdByStakingId<T>>::get(stake_id);

        assert!(<ApplicationById<T>>::contains_key(application_id));

        let application = <ApplicationById<T>>::get(application_id);

        // Make sure that we are actually unstaking, ignore otherwise.
        let (deactivation_initiated, cause) = if let ApplicationStage::Unstaking {
            deactivation_initiated,
            cause,
        } = application.stage
        {
            (deactivation_initiated, cause)
        } else {
            return UnstakedResult::ApplicationIsNotUnstaking;
        };

        //
        // == MUTATION SAFE ==
        //

        // Drop stake from stake to application map
        <ApplicationIdByStakingId<T>>::remove(stake_id);
        let current_block_height = <frame_system::Pallet<T>>::block_number();

        // New application computed
        let mut new_application = application.clone();
        let is_now_done_unstaking = new_application.unstake_application(
            current_block_height,
            deactivation_initiated,
            cause,
            stake_id,
        );

        // Update to new application
        <ApplicationById<T>>::insert(&application_id, new_application);

        // If the application is now finished compeleting any pending unstaking process,
        // then we need to update the opening counters, and make the deactivation callback.
        if is_now_done_unstaking {
            // Update Opening
            // We know the stage MUST be active, hence mutate is certain.
            <OpeningById<T>>::mutate(application.opening_id, |opening| {
                opening.change_opening_stage_after_application_unstaked();
            });

            // Call handler
            T::ApplicationDeactivatedHandler::deactivated(&application_id, cause);
            return UnstakedResult::Unstaked;
        }

        UnstakedResult::UnstakingInProgress
    }
}

/*
 *  === Application Deactivated Handler  ======
 */

/// Handles application deactivation with a cause
pub trait ApplicationDeactivatedHandler<T: Config> {
    /// An application, with the given id, was fully deactivated, with the
    /// given cause, and was put in the inactive state.
    fn deactivated(application_id: &T::ApplicationId, cause: hiring::ApplicationDeactivationCause);
}

/// Helper implementation so we can provide multiple handlers by grouping handlers in tuple pairs.
/// For example for three handlers, A, B and C we can set the StakingEventHandler type on the trait to:
/// type StakingEventHandler = ((A, B), C)
impl<T: Config> ApplicationDeactivatedHandler<T> for () {
    fn deactivated(
        _application_id: &T::ApplicationId,
        _cause: hiring::ApplicationDeactivationCause,
    ) {
    }
}

/*
 *  ======== API types bound to the Config ========
 */

/// Error due to attempting to fill an opening.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum FillOpeningError<T: Config> {
    /// Opening does not exist
    OpeningDoesNotExist,

    /// Opening is not in review period
    OpeningNotInReviewPeriodStage,

    /// Provided unstaking period is too short
    UnstakingPeriodTooShort(StakePurpose, ApplicationOutcomeInFilledOpening),

    /// Provided redundant unstaking period
    RedundantUnstakingPeriodProvided(StakePurpose, ApplicationOutcomeInFilledOpening),

    /// Application does not exist
    ApplicationDoesNotExist(T::ApplicationId),

    /// Application is not in active stage
    ApplicationNotInActiveStage(T::ApplicationId),

    /// Application is not for the opening
    ApplicationForWrongOpening(T::ApplicationId),
}

/// Product of ensure_can_add_application()
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DestructuredApplicationCanBeAddedEvaluation<T: Config> {
    /// Opening object
    pub opening: Opening<BalanceOf<T>, T::BlockNumber, T::ApplicationId>,

    /// Active opening stage
    pub active_stage: ActiveOpeningStage<T::BlockNumber>,

    /// Collection of added applicaiton ids
    pub applications_added: BTreeSet<T::ApplicationId>,

    /// Active applications counter
    pub active_application_count: u32,

    /// Unstaking applications counter
    pub unstaking_application_count: u32,

    /// Deactivated applications counter
    pub deactivated_application_count: u32,

    /// Prospects of application adding
    pub would_get_added_success: ApplicationAddedSuccess<T>,
}

impl<T: Config> DestructuredApplicationCanBeAddedEvaluation<T> {
    pub(crate) fn calculate_total_application_count(&self) -> u32 {
        // TODO: fix so that `number_of_appliations_ever_added` can be invoked.
        // cant do this due to bad design of stage => opening.stage.number_of_appliations_ever_added();
        self.active_application_count
            + self.unstaking_application_count
            + self.deactivated_application_count
    }
}

/// Adding application result
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ApplicationAddedSuccess<T: Config> {
    /// Application was added without side-effects
    Unconditionally,

    /// Application has crowded out existing application
    CrowdsOutExistingApplication(T::ApplicationId),
}

impl<T: Config> ApplicationAddedSuccess<T> {
    pub(crate) fn crowded_out_application_id(&self) -> Option<T::ApplicationId> {
        if let ApplicationAddedSuccess::CrowdsOutExistingApplication(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}

/// Prospects of application. Whether it would be added to the opening.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ApplicationWouldGetAddedEvaluation<T: Config> {
    /// Negative prospects
    No,

    /// Positive prospects
    Yes(ApplicationAddedSuccess<T>),
}

/// Balance alias
pub type BalanceOf<T> =
    <<T as stake::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Balance alias for staking
pub type NegativeImbalance<T> = <<T as stake::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/*
 *  ======== ======== ======== ======== =======
 *  ======== PRIVATE TYPES AND METHODS ========
 *  ======== ======== ======== ======== =======
 */

#[derive(PartialEq, Debug, Clone)]
struct ApplicationsDeactivationsInitiationResult {
    number_of_unstaking_applications: u32,
    number_of_deactivated_applications: u32,
}

type ApplicationBTreeMap<T> = BTreeMap<
    <T as Config>::ApplicationId,
    hiring::Application<
        <T as Config>::OpeningId,
        <T as frame_system::Config>::BlockNumber,
        <T as stake::Config>::StakeId,
    >,
>;

#[derive(PartialEq, Debug, Clone)]
enum ApplicationDeactivationInitiationResult {
    Ignored, // <= is there a case for kicking this out, making sure that initiation cannot happen when it may fail?
    Unstaking,
    Deactivated,
}

// Opening and application iterators
impl<T: Config> Module<T> {
    // Iterate through ApplicationById map
    fn application_id_iter_to_map<'a>(
        application_id_iter: impl Iterator<Item = &'a T::ApplicationId>,
    ) -> ApplicationBTreeMap<T> {
        application_id_iter
            .map(|application_id| {
                let application = <ApplicationById<T>>::get(application_id);

                (*application_id, application)
            })
            .collect::<BTreeMap<_, _>>()
    }

    // Compute iterator of openings waiting to begin
    fn openings_waiting_to_begin_iterator(
        now: T::BlockNumber,
    ) -> impl Iterator<
        Item = (
            T::OpeningId,
            Opening<BalanceOf<T>, T::BlockNumber, T::ApplicationId>,
        ),
    > {
        <OpeningById<T>>::iter().filter_map(move |(opening_id, opening)| {
            if let hiring::OpeningStage::WaitingToBegin { begins_at_block } = opening.stage {
                if begins_at_block == now {
                    Some((opening_id, opening))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    // Compute iterator of openings in expired review period
    fn openings_expired_review_period_iterator(
        now: T::BlockNumber,
    ) -> impl Iterator<
        Item = (
            T::OpeningId,
            Opening<BalanceOf<T>, T::BlockNumber, T::ApplicationId>,
            (BTreeSet<T::ApplicationId>, T::BlockNumber, T::BlockNumber),
        ),
    > {
        <OpeningById<T>>::iter().filter_map(move |(opening_id, opening)| {
            if let hiring::OpeningStage::Active {
                ref stage,
                ref applications_added,
                ..
            } = opening.stage
            {
                if let hiring::ActiveOpeningStage::ReviewPeriod {
                    ref started_accepting_applicants_at_block,
                    ref started_review_period_at_block,
                } = stage
                {
                    if now == opening.max_review_period_length + *started_review_period_at_block {
                        Some((
                            opening_id,
                            opening.clone(),
                            (
                                applications_added.clone(),
                                *started_accepting_applicants_at_block,
                                *started_review_period_at_block,
                            ),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

// Application deactivation logic methods.
impl<T: Config> Module<T> {
    fn initiate_application_deactivations(
        applications: &ApplicationBTreeMap<T>,
        application_stake_unstaking_period: Option<T::BlockNumber>,
        role_stake_unstaking_period: Option<T::BlockNumber>,
        cause: ApplicationDeactivationCause,
    ) -> ApplicationsDeactivationsInitiationResult {
        // Update stage on active applications, and collect result

        applications
            .iter()
            .map(
                |(application_id, application)| -> ApplicationDeactivationInitiationResult {
                    // Initiate deactivations!
                    Self::try_to_initiate_application_deactivation(
                        application,
                        *application_id,
                        application_stake_unstaking_period,
                        role_stake_unstaking_period,
                        cause,
                    )
                },
            )
            .fold(
                // Initiatial reducer value
                ApplicationsDeactivationsInitiationResult {
                    number_of_unstaking_applications: 0,
                    number_of_deactivated_applications: 0,
                },
                |acc, deactivation_result| {
                    // Update accumulator counters based on what actually happened
                    match deactivation_result {
                        ApplicationDeactivationInitiationResult::Ignored => acc,

                        ApplicationDeactivationInitiationResult::Unstaking => {
                            ApplicationsDeactivationsInitiationResult {
                                number_of_unstaking_applications: 1 + acc
                                    .number_of_unstaking_applications,
                                number_of_deactivated_applications: acc
                                    .number_of_deactivated_applications,
                            }
                        }

                        ApplicationDeactivationInitiationResult::Deactivated => {
                            ApplicationsDeactivationsInitiationResult {
                                number_of_unstaking_applications: acc
                                    .number_of_unstaking_applications,
                                number_of_deactivated_applications: 1 + acc
                                    .number_of_deactivated_applications,
                            }
                        }
                    }
                },
            )
    }

    /// Initiates
    fn try_to_initiate_application_deactivation(
        application: &Application<T::OpeningId, T::BlockNumber, T::StakeId>,
        application_id: T::ApplicationId,
        application_stake_unstaking_period: Option<T::BlockNumber>,
        role_stake_unstaking_period: Option<T::BlockNumber>,
        cause: hiring::ApplicationDeactivationCause,
    ) -> ApplicationDeactivationInitiationResult {
        match application.stage {
            ApplicationStage::Active => {
                // Initiate unstaking of any active application stake
                let application_was_unstaked = Self::opt_infallible_unstake(
                    application.active_application_staking_id,
                    application_stake_unstaking_period,
                );

                // Only unstake role stake for a non successful result ie. not Hired
                let role_was_unstaked = cause != hiring::ApplicationDeactivationCause::Hired
                    && Self::opt_infallible_unstake(
                        application.active_role_staking_id,
                        role_stake_unstaking_period,
                    );

                // Capture if any unstaking occured at all
                let was_unstaked = application_was_unstaked || role_was_unstaked;

                // Grab current block height
                let current_block_height = <frame_system::Pallet<T>>::block_number();

                /*
                 * TODO:
                 * There should be a single transformation based on
                 * was_unstaked which renders a new value for `application.stage`
                 * and `opening.stage`, which guarantees to only produces new values
                 * for given variant values, but the state machine types are currently
                 * not well organised to support this.
                 *
                 * Likewise the construction of hiring::OpeningStage::Active below
                 * is a wreck because of this.
                 *
                 * Issue: https://github.com/Joystream/joystream/issues/36#issuecomment-539567407
                 */

                // Figure out new stage for the application
                let new_application_stage = if was_unstaked {
                    ApplicationStage::Unstaking {
                        deactivation_initiated: current_block_height,
                        cause,
                    }
                } else {
                    ApplicationStage::Inactive {
                        deactivation_initiated: current_block_height,
                        deactivated: current_block_height,
                        cause,
                    }
                };

                // Update the application stage
                <ApplicationById<T>>::mutate(application_id, |application| {
                    application.stage = new_application_stage;
                });

                // Update counters on opening
                <OpeningById<T>>::mutate(application.opening_id, |opening| {
                    // NB: This ugly byref destructuring is same issue as pointed out multiple times now.
                    if let hiring::OpeningStage::Active {
                        ref stage,
                        ref applications_added,
                        ref active_application_count,
                        ref unstaking_application_count,
                        ref deactivated_application_count,
                    } = opening.stage
                    {
                        assert!(*active_application_count > 0);

                        let new_active_application_count = active_application_count - 1;

                        let new_unstaking_application_count =
                            unstaking_application_count + if was_unstaked { 1 } else { 0 };

                        let new_deactivated_application_count =
                            deactivated_application_count + if was_unstaked { 0 } else { 1 };

                        opening.stage = hiring::OpeningStage::Active {
                            stage: stage.clone(),
                            applications_added: applications_added.clone(),
                            active_application_count: new_active_application_count,
                            unstaking_application_count: new_unstaking_application_count,
                            deactivated_application_count: new_deactivated_application_count,
                        };
                    } else {
                        panic!("opening stage must be 'Active'");
                    }
                });

                // Call handler(s)
                if was_unstaked {
                    T::ApplicationDeactivatedHandler::deactivated(&application_id, cause);
                }

                // Return conclusion
                if was_unstaked {
                    ApplicationDeactivationInitiationResult::Unstaking
                } else {
                    ApplicationDeactivationInitiationResult::Deactivated
                }
            }
            _ => ApplicationDeactivationInitiationResult::Ignored,
        }
    }

    /// Tries to unstake, based on a stake id which, if set, MUST
    /// be ready to be unstaked, with an optional unstaking period.
    ///
    /// Returns whether unstaking was actually initiated.
    fn opt_infallible_unstake(
        opt_stake_id: Option<T::StakeId>,
        opt_unstaking_period: Option<T::BlockNumber>,
    ) -> bool {
        if let Some(stake_id) = opt_stake_id {
            // `initiate_unstaking` MUST hold, is runtime invariant, false means code is broken.
            // But should we do panic in runtime? Is there safer way?

            assert!(T::StakeHandlerProvider::staking()
                .initiate_unstaking(&stake_id, opt_unstaking_period)
                .is_ok());
        }

        opt_stake_id.is_some()
    }
}

// Stake initiation
impl<T: Config> Module<T> {
    fn infallible_opt_stake_initiation(
        opt_imbalance: Option<NegativeImbalance<T>>,
        application_id: &T::ApplicationId,
    ) -> Option<T::StakeId> {
        opt_imbalance.map(|imbalance| {
            Self::infallible_stake_initiation_on_application(imbalance, application_id)
        })
    }

    fn infallible_stake_initiation_on_application(
        imbalance: NegativeImbalance<T>,
        application_id: &T::ApplicationId,
    ) -> T::StakeId {
        // Create stake
        let new_stake_id = T::StakeHandlerProvider::staking().create_stake();

        // Keep track of this stake id to process unstaking callbacks that may
        // be invoked later.
        // NB: We purposefully update state to reflect mapping _before_ initiating staking below
        // in order to be safe from race conditions arising out of third party code executing in callback of staking module.

        // MUST never already be a key for new stake, false means code is broken.
        // But should we do panic in runtime? Is there safer way?
        assert!(!<ApplicationIdByStakingId<T>>::contains_key(new_stake_id));

        <ApplicationIdByStakingId<T>>::insert(new_stake_id, application_id);

        // Initiate staking
        //
        // MUST work, is runtime invariant, false means code is broken.
        // But should we do panic in runtime? Is there safer way?
        assert_eq!(
            T::StakeHandlerProvider::staking().stake(&new_stake_id, imbalance),
            Ok(())
        );

        new_stake_id
    }
}

// Conditions for adding application
impl<T: Config> Module<T> {
    /// Evaluates prospects for a new application
    ///
    pub(crate) fn would_application_get_added(
        possible_opening_application_rationing_policy: &Option<ApplicationRationingPolicy>,
        opening_applicants: &BTreeSet<T::ApplicationId>,
        opt_role_stake_balance: &Option<BalanceOf<T>>,
        opt_application_stake_balance: &Option<BalanceOf<T>>,
    ) -> ApplicationWouldGetAddedEvaluation<T> {
        // Check whether any rationing policy is set at all, if not
        // then there is no rationing, and any application can get added.
        let application_rationing_policy = if let Some(application_rationing_policy) =
            possible_opening_application_rationing_policy
        {
            application_rationing_policy
        } else {
            return ApplicationWouldGetAddedEvaluation::Yes(
                ApplicationAddedSuccess::Unconditionally,
            );
        };

        // Map with applications
        let applications_map = Self::application_id_iter_to_map(opening_applicants.iter());

        let active_applications_with_stake_iter =
            applications_map
                .iter()
                .filter_map(|(application_id, application)| {
                    if application.stage == hiring::ApplicationStage::Active {
                        let total_stake =
                            Self::get_opt_stake_amount(application.active_role_staking_id)
                                + Self::get_opt_stake_amount(
                                    application.active_application_staking_id,
                                );

                        Some((application_id, application, total_stake))
                    } else {
                        None
                    }
                });

        // Compute number of active applications
        let number_of_active_applications = active_applications_with_stake_iter.clone().count();

        // Check whether the current number of _active_ applicants is either at or above the maximum
        // limit, if not, then we can add at least one additional application,
        // otherwise we must evaluate whether this new application would specifically get added.
        if (number_of_active_applications as u32)
            < application_rationing_policy.max_active_applicants
        {
            return ApplicationWouldGetAddedEvaluation::Yes(
                ApplicationAddedSuccess::Unconditionally,
            );
        }

        // Here we try to figure out if the new application
        // has sufficient stake to crowd out one of the already
        // active applicants.

        // The total stake of new application
        let total_stake_of_new_application = opt_role_stake_balance.unwrap_or_default()
            + opt_application_stake_balance.unwrap_or_default();

        // The total stake of all current active applications
        let opt_min_item = active_applications_with_stake_iter
            .clone()
            .min_by_key(|(_, _, total_stake)| *total_stake);

        if let Some((application_id, _, lowest_active_total_stake)) = opt_min_item {
            // Finally we compare the two and come up with a final evaluation
            if total_stake_of_new_application <= lowest_active_total_stake {
                ApplicationWouldGetAddedEvaluation::No // stake too low!
            } else {
                ApplicationWouldGetAddedEvaluation::Yes(
                    ApplicationAddedSuccess::CrowdsOutExistingApplication(*application_id),
                )
            }
        } else {
            panic!("`number_of_active_applications` (length of `active_applications_iter`) == 0")
        }
    }

    fn get_opt_stake_amount(stake_id: Option<T::StakeId>) -> BalanceOf<T> {
        stake_id.map_or(<BalanceOf<T> as Zero>::zero(), |stake_id| {
            // INVARIANT: stake MUST exist in the staking module
            assert!(T::StakeHandlerProvider::staking().stake_exists(stake_id));

            let stake = T::StakeHandlerProvider::staking().get_stake(stake_id);

            match stake.staking_status {
                // INVARIANT: stake MUST be in the staked state.
                stake::StakingStatus::Staked(staked_state) => staked_state.staked_amount,
                _ => panic!("stake MUST be in the staked state."),
            }
        })
    }

    pub(crate) fn create_stake_balance(
        opt_stake_imbalance: &Option<NegativeImbalance<T>>,
    ) -> Option<BalanceOf<T>> {
        opt_stake_imbalance
            .as_ref()
            .map(|imbalance| imbalance.peek())
    }

    /// Performs all necessary check before adding an opening
    pub(crate) fn ensure_can_add_opening(
        current_block_height: T::BlockNumber,
        activate_at: ActivateOpeningAt<T::BlockNumber>,
        minimum_stake_balance: BalanceOf<T>,
        application_rationing_policy: Option<ApplicationRationingPolicy>,
        application_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        role_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
    ) -> Result<(), AddOpeningError> {
        // Check that exact activation is actually in the future
        ensure!(
            match activate_at {
                ActivateOpeningAt::ExactBlock(block_number) => block_number > current_block_height,
                _ => true,
            },
            AddOpeningError::OpeningMustActivateInTheFuture
        );

        if let Some(app_rationing_policy) = application_rationing_policy {
            ensure!(
                app_rationing_policy.max_active_applicants > 0,
                AddOpeningError::ApplicationRationingZeroMaxApplicants
            );
        }

        // Check that staking amounts clear minimum balance required.
        Self::ensure_amount_valid_in_opt_staking_policy(
            application_staking_policy,
            minimum_stake_balance,
            StakePurpose::Application,
        )?;

        // Check that staking amounts clear minimum balance required.
        Self::ensure_amount_valid_in_opt_staking_policy(
            role_staking_policy,
            minimum_stake_balance,
            StakePurpose::Role,
        )?;

        Ok(())
    }

    /// Ensures that optional staking policy prescribes value that clears minimum balance requirement
    pub(crate) fn ensure_amount_valid_in_opt_staking_policy(
        opt_staking_policy: Option<StakingPolicy<BalanceOf<T>, T::BlockNumber>>,
        minimum_stake_balance: BalanceOf<T>,
        stake_purpose: StakePurpose,
    ) -> Result<(), AddOpeningError> {
        if let Some(ref staking_policy) = opt_staking_policy {
            ensure!(
                staking_policy.amount > Zero::zero(),
                AddOpeningError::StakeAmountCannotBeZero(stake_purpose)
            );

            ensure!(
                staking_policy.amount >= minimum_stake_balance,
                AddOpeningError::StakeAmountLessThanMinimumStakeBalance(stake_purpose)
            );
        }

        Ok(())
    }
}

/*
 *  === Stake module wrappers  ======
 */

/// Defines stake module interface
#[cfg_attr(all(test, not(target_arch = "wasm32")), automock)]
pub trait StakeHandler<T: StakeTrait> {
    /// Adds a new Stake which is NotStaked, created at given block, into stakes map.
    fn create_stake(&self) -> T::StakeId;

    /// to the module's account, and the corresponding staked_balance is set to this amount in the new Staked state.
    /// On error, as the negative imbalance is not returned to the caller, it is the caller's responsibility to return the funds
    /// back to the source (by creating a new positive imbalance)
    fn stake(
        &self,
        new_stake_id: &T::StakeId,
        imbalance: NegativeImbalance<T>,
    ) -> Result<(), StakeActionError<stake::StakingError>>;

    /// Checks whether stake exists by its id
    fn stake_exists(&self, stake_id: T::StakeId) -> bool;

    /// Acquires stake by id
    fn get_stake(&self, stake_id: T::StakeId) -> Stake<T::BlockNumber, BalanceOf<T>, T::SlashId>;

    /// Initiate unstaking of a Staked stake.
    fn initiate_unstaking(
        &self,
        stake_id: &T::StakeId,
        unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), StakeActionError<InitiateUnstakingError>>;
}

/// Allows to provide different StakeHandler implementation. Useful for mocks.
pub trait StakeHandlerProvider<T: Config> {
    /// Returns StakeHandler. Mock entry point for stake module.
    fn staking() -> Rc<RefCell<dyn StakeHandler<T>>>;
}

impl<T: Config> StakeHandlerProvider<T> for Module<T> {
    /// Returns StakeHandler. Mock entry point for stake module.
    fn staking() -> Rc<RefCell<dyn StakeHandler<T>>> {
        Rc::new(RefCell::new(HiringStakeHandler {}))
    }
}

/// Default stake module logic implementation
pub struct HiringStakeHandler;
impl<T: Config> StakeHandler<T> for HiringStakeHandler {
    fn create_stake(&self) -> T::StakeId {
        <stake::Module<T>>::create_stake()
    }

    fn stake(
        &self,
        new_stake_id: &T::StakeId,
        imbalance: NegativeImbalance<T>,
    ) -> Result<(), StakeActionError<StakingError>> {
        <stake::Module<T>>::stake(new_stake_id, imbalance)
    }

    fn stake_exists(&self, stake_id: T::StakeId) -> bool {
        <stake::Stakes<T>>::contains_key(stake_id)
    }

    fn get_stake(&self, stake_id: T::StakeId) -> Stake<T::BlockNumber, BalanceOf<T>, T::SlashId> {
        <stake::Stakes<T>>::get(stake_id)
    }

    fn initiate_unstaking(
        &self,
        stake_id: &T::StakeId,
        unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), StakeActionError<InitiateUnstakingError>> {
        <stake::Module<T>>::initiate_unstaking(&stake_id, unstaking_period)
    }
}

// Proxy implementation of StakeHandler trait to simplify calls via staking() method
// Allows to get rid of borrow() calls,
// eg.: T::StakeHandlerProvider::staking().get_stake(stake_id);
// instead of T::StakeHandlerProvider::staking().borrow().get_stake(stake_id);
impl<T: Config> StakeHandler<T> for Rc<RefCell<dyn StakeHandler<T>>> {
    fn create_stake(&self) -> T::StakeId {
        self.borrow().create_stake()
    }

    fn stake(
        &self,
        new_stake_id: &T::StakeId,
        imbalance: NegativeImbalance<T>,
    ) -> Result<(), StakeActionError<StakingError>> {
        self.borrow().stake(new_stake_id, imbalance)
    }

    fn stake_exists(&self, stake_id: T::StakeId) -> bool {
        self.borrow().stake_exists(stake_id)
    }

    fn get_stake(&self, stake_id: T::StakeId) -> Stake<T::BlockNumber, BalanceOf<T>, T::SlashId> {
        self.borrow().get_stake(stake_id)
    }

    fn initiate_unstaking(
        &self,
        stake_id: &T::StakeId,
        unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), StakeActionError<InitiateUnstakingError>> {
        self.borrow().initiate_unstaking(stake_id, unstaking_period)
    }
}
