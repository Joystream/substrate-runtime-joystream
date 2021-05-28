mod application;
mod opening;
mod staking_policy;

pub use application::*;
pub use opening::*;
pub use staking_policy::*;

/// Stake purpose enumeration.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum StakePurpose {
    /// Stake for a role
    Role,

    /// Stake for an application
    Application,
}

/// Informational result of the unstaked() method. Can be ignored.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UnstakedResult {
    /// Non-existentent stake id provided
    StakeIdNonExistent,

    /// Application is not in 'Unstaking' state
    ApplicationIsNotUnstaking,

    /// Fully unstaked
    Unstaked,

    /// Unstaking in progress
    UnstakingInProgress,
}

/// Error of the add_application() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AddApplicationError {
    /// Opening does not exist
    OpeningDoesNotExist,

    /// Provided redundant stake
    StakeProvidedWhenRedundant(StakePurpose),

    /// Required stake was not provided
    StakeMissingWhenRequired(StakePurpose),

    /// Provided stake amount is too low
    StakeAmountTooLow(StakePurpose),

    /// Opening is not in acception application active stage
    OpeningNotInAcceptingApplicationsStage,

    /// Newly created application was crowded out
    NewApplicationWasCrowdedOut,
}

/// Successful result of the add_opening() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ApplicationAdded<ApplicationId> {
    /// Added application id
    pub application_id_added: ApplicationId,

    /// Possible id of the crowded out application
    pub application_id_crowded_out: Option<ApplicationId>,
}

/// Error of the deactivate_application() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeactivateApplicationError {
    /// Application does not exist
    ApplicationDoesNotExist,

    /// Application is not in active stage
    ApplicationNotActive,

    /// Opening is not in accepting application stage
    OpeningNotAcceptingApplications,

    /// Provided unstaking period is too short
    UnstakingPeriodTooShort(StakePurpose),

    /// Provided redundant unstaking period
    RedundantUnstakingPeriodProvided(StakePurpose),
}

/// Error of the cancel_opening() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum CancelOpeningError {
    /// Provided unstaking period is too short
    UnstakingPeriodTooShort(StakePurpose),

    /// Provided redundant unstaking period
    RedundantUnstakingPeriodProvided(StakePurpose),

    /// Opening does not exist
    OpeningDoesNotExist,

    /// Opening cannot be cancelled
    OpeningNotInCancellableStage,
}

/// NB:
/// `OpeningCancelled` does not have the ideal form.
/// <https://github.com/Joystream/substrate-hiring-module/issues/10>
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OpeningCancelled {
    /// Number of unstaking application because of canceled opening
    pub number_of_unstaking_applications: u32,

    /// Number of deactivaed application because of canceled opening
    pub number_of_deactivated_applications: u32,
}

/// Error of the begin_review() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BeginReviewError {
    /// Opening does not exist
    OpeningDoesNotExist,

    /// Opening is not in acception application active stage
    OpeningNotInAcceptingApplicationsStage,
}

/// Error of the begin_accepting_application() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BeginAcceptingApplicationsError {
    /// Opening does not exist
    OpeningDoesNotExist,

    /// Opening is not in waiting to begin stage
    OpeningIsNotInWaitingToBeginStage,
}

/// The possible outcome for an application in an opening which is being filled.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ApplicationOutcomeInFilledOpening {
    /// Application is successful
    Success,

    /// Application failed
    Failure,
}

/// Error of the add_opening() API method.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AddOpeningError {
    /// Indicates that opening with provided parameters must be activated in the future
    OpeningMustActivateInTheFuture,

    /// It is not possible to stake less than the minimum balance defined in the
    /// `Currency` module.
    StakeAmountLessThanMinimumStakeBalance(StakePurpose),

    /// It is not possible to provide application rationing policy with zero
    /// 'max_active_applicants' parameter.
    ApplicationRationingZeroMaxApplicants,

    /// It is not possible to stake zero.
    StakeAmountCannotBeZero(StakePurpose),
}
