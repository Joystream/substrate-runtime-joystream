#![warn(missing_docs)]

use crate::{Config, Instance, Module};
use frame_support::decl_error;

decl_error! {
    /// Discussion module predefined errors
    pub enum Error for Module<T: Config<I>, I: Instance>{
        /// Provided stake balance cannot be zero.
        StakeBalanceCannotBeZero,

        /// Cannot get the worker stake profile.
        NoWorkerStakeProfile,

        /// Current lead is not set.
        CurrentLeadNotSet,

        /// There is leader already, cannot hire another one.
        CannotHireLeaderWhenLeaderExists,

        /// Cannot fill opening with multiple applications.
        CannotHireMultipleLeaders,

        /// Not a lead account.
        IsNotLeadAccount,

        /// Opening text too short.
        OpeningTextTooShort,

        /// Opening text too long.
        OpeningTextTooLong,

        /// Opening does not exist.
        OpeningDoesNotExist,

        /// Insufficient balance to apply.
        InsufficientBalanceToApply,

        /// Unsigned origin.
        MembershipUnsignedOrigin,

        /// Member id is invalid.
        MembershipInvalidMemberId,

        /// Signer does not match controller account.
        ApplyOnWorkerOpeningSignerNotControllerAccount,

        /// Origin must be controller or root account of member.
        OriginIsNeitherMemberControllerOrRoot,

        /// Member already has an active application on the opening.
        MemberHasActiveApplicationOnOpening,

        /// Worker application text too long.
        WorkerApplicationTextTooLong,

        /// Worker application text too short.
        WorkerApplicationTextTooShort,

        /// Insufficient balance to cover stake.
        InsufficientBalanceToCoverStake,

        /// Origin is not applicant.
        OriginIsNotApplicant,

        /// Worker application does not exist.
        WorkerApplicationDoesNotExist,

        /// Successful worker application does not exist.
        SuccessfulWorkerApplicationDoesNotExist,

        /// Reward policy has invalid next payment block number.
        FillOpeningInvalidNextPaymentBlock,

        /// Working group mint does not exist.
        FillOpeningMintDoesNotExist,

        ///Relationship must exist.
        RelationshipMustExist,

        /// Worker exit rationale text is too long.
        WorkerExitRationaleTextTooLong,

        /// Worker exit rationale text is too short.
        WorkerExitRationaleTextTooShort,

        /// Worker storage text is too long.
        WorkerStorageValueTooLong,

        /// Signer is not worker role account.
        SignerIsNotWorkerRoleAccount,

        /// Worker has no recurring reward.
        WorkerHasNoReward,

        /// Worker does not exist.
        WorkerDoesNotExist,

        /// Opening does not exist.
        AcceptWorkerApplicationsOpeningDoesNotExist,

        /// Opening Is Not in Waiting to begin.
        AcceptWorkerApplicationsOpeningIsNotWaitingToBegin,

        /// Opening does not exist.
        BeginWorkerApplicantReviewOpeningDoesNotExist,

        /// Opening Is Not in Waiting.
        BeginWorkerApplicantReviewOpeningOpeningIsNotWaitingToBegin,

        /// OpeningDoesNotExist.
        FullWorkerOpeningOpeningDoesNotExist,

        /// Opening not in review period stage.
        FullWorkerOpeningOpeningNotInReviewPeriodStage,

        /// Application stake unstaking period for successful applicants too short.
        FullWorkerOpeningUnsuccessfulApplicationStakeUnstakingPeriodTooShort,

        /// Application stake unstaking period for failed applicants too short.
        FullWorkerOpeningSuccessfulApplicationStakeUnstakingPeriodTooShort,

        /// Role stake unstaking period for successful applicants too short.
        FullWorkerOpeningSuccessfulRoleStakeUnstakingPeriodTooShort,

        /// Role stake unstaking period for failed applicants too short.
        FullWorkerOpeningUnsuccessfulRoleStakeUnstakingPeriodTooShort,

        /// Application stake unstaking period for successful applicants redundant.
        FullWorkerOpeningSuccessfulApplicationStakeUnstakingPeriodRedundant,

        /// Application stake unstaking period for failed applicants redundant.
        FullWorkerOpeningUnsuccessfulApplicationStakeUnstakingPeriodRedundant,

        /// Role stake unstaking period for successful applicants redundant.
        FullWorkerOpeningSuccessfulRoleStakeUnstakingPeriodRedundant,

        /// Role stake unstaking period for failed applicants redundant.
        FullWorkerOpeningUnsuccessfulRoleStakeUnstakingPeriodRedundant,

        /// Application does not exist.
        FullWorkerOpeningApplicationDoesNotExist,

        /// Application not in active stage.
        FullWorkerOpeningApplicationNotActive,

        /// Applications not for opening.
        FillWorkerOpeningApplicationForWrongOpening,

        /// Application does not exist.
        WithdrawWorkerApplicationApplicationDoesNotExist,

        /// Application is not active.
        WithdrawWorkerApplicationApplicationNotActive,

        /// Opening not accepting applications.
        WithdrawWorkerApplicationOpeningNotAcceptingApplications,

        /// UnstakingPeriodTooShort .... // <== SHOULD REALLY BE TWO SEPARATE, ONE FOR EACH STAKING PURPOSE
        WithdrawWorkerApplicationUnstakingPeriodTooShort,

        /// Redundant unstaking period provided
        WithdrawWorkerApplicationRedundantUnstakingPeriod,

        /// Opening does not activate in the future.
        AddWorkerOpeningActivatesInThePast,

        /// Role stake amount less than minimum currency balance.
        AddWorkerOpeningRoleStakeLessThanMinimum,

        /// Application stake amount less than minimum currency balance.
        AddWorkerOpeningAppliicationStakeLessThanMinimum,

        /// Opening does not exist.
        AddWorkerOpeningOpeningDoesNotExist,

        // <== SHOULD REALLY BE TWO SEPARATE, ONE FOR EACH STAKING PURPOSE
        /// Stake provided when redundant.
        AddWorkerOpeningStakeProvidedWhenRedundant,

        // <== SHOULD REALLY BE TWO SEPARATE, ONE FOR EACH STAKING PURPOSE
        /// Stake missing when required.
        AddWorkerOpeningStakeMissingWhenRequired,

        // <== SHOULD REALLY BE TWO SEPARATE, ONE FOR EACH STAKING PURPOSE
        /// Stake amount too low.
        AddWorkerOpeningStakeAmountTooLow,

        /// Opening is not in accepting applications stage.
        AddWorkerOpeningOpeningNotInAcceptingApplicationStage,

        /// New application was crowded out.
        AddWorkerOpeningNewApplicationWasCrowdedOut,

        /// Application rationing has zero max active applicants.
        AddWorkerOpeningZeroMaxApplicantCount,

        /// Next payment is not in the future.
        RecurringRewardsNextPaymentNotInFuture,

        /// Recipient not found.
        RecurringRewardsRecipientNotFound,

        /// Recipient reward source not found.
        RecurringRewardsRewardSourceNotFound,

        /// Reward relationship not found.
        RecurringRewardsRewardRelationshipNotFound,

        /// Stake not found.
        StakingErrorStakeNotFound,

        /// Unstaking period should be greater than zero.
        StakingErrorUnstakingPeriodShouldBeGreaterThanZero,

        /// Already unstaking.
        StakingErrorAlreadyUnstaking,

        /// Not staked.
        StakingErrorNotStaked,

        /// Cannot unstake while slashes ongoing.
        StakingErrorCannotUnstakeWhileSlashesOngoing,

        /// Insufficient balance in source account.
        StakingErrorInsufficientBalanceInSourceAccount,

        /// Cannot change stake by zero.
        StakingErrorCannotChangeStakeByZero,

        /// Cannot increase stake while unstaking.
        StakingErrorCannotIncreaseStakeWhileUnstaking,

        /// Cannot decrease stake while slashes ongoing.
        StakingErrorCannotDecreaseWhileSlashesOngoing,

        /// Insufficient stake to decrease.
        StakingErrorInsufficientStake,

        /// Slash amount should be greater than zero.
        StakingErrorSlashAmountShouldBeGreaterThanZero,

        /// Cannot find mint in the minting module.
        CannotFindMint,

        /// Require root origin in extrinsics.
        RequireRootOrigin,

        /// Require signed origin in extrinsics.
        RequireSignedOrigin,

        /// Working group size limit exceeded.
        MaxActiveWorkerNumberExceeded,

        /// Add worker opening role stake cannot be zero.
        AddWorkerOpeningRoleStakeCannotBeZero,

        /// Add worker opening application stake cannot be zero.
        AddWorkerOpeningApplicationStakeCannotBeZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// fill_opening_failed_applicant_application_stake_unstaking_period should be non-zero.
        FillOpeningFailedApplicantApplicationStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// fill_opening_failed_applicant_role_stake_unstaking_period should be non-zero.
        FillOpeningFailedApplicantRoleStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// fill_opening_successful_applicant_application_stake_unstaking_period should be non-zero.
        FillOpeningSuccessfulApplicantApplicationStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// exit_role_stake_unstaking_period should be non-zero.
        ExitRoleStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// exit_role_application_stake_unstaking_period should be non-zero.
        ExitRoleApplicationStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// terminate_role_stake_unstaking_period should be non-zero.
        TerminateRoleStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter:
        /// terminate_application_stake_unstaking_period should be non-zero.
        TerminateApplicationStakeUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter (role_staking_policy):
        /// crowded_out_unstaking_period_length should be non-zero.
        RoleStakingPolicyCrowdedOutUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter (role_staking_policy):
        /// review_period_expired_unstaking_period_length should be non-zero.
        RoleStakingPolicyReviewPeriodUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter (application_staking_policy):
        /// crowded_out_unstaking_period_length should be non-zero.
        ApplicationStakingPolicyCrowdedOutUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter (application_staking_policy):
        /// review_period_expired_unstaking_period_length should be non-zero.
        ApplicationStakingPolicyReviewPeriodUnstakingPeriodIsZero,

        /// Invalid OpeningPolicyCommitment parameter (application_rationing_policy):
        /// max_active_applicants should be non-zero.
        ApplicationRationingPolicyMaxActiveApplicantsIsZero,

        /// Minting error: NextAdjustmentInPast
        MintingErrorNextAdjustmentInPast,
    }
}

/// Error wrapper for external module error conversions.
pub struct WrappedError<E> {
    /// Generic error.
    pub error: E,
}

/// Helps with conversion of other modules errors.
#[macro_export]
macro_rules! ensure_on_wrapped_error {
    ($call:expr) => {{
        { $call }
            .map_err(|err| crate::WrappedError { error: err })
            .map_err(|err| {
                let e: Error<T, I> = err.into();

                e
            })
    }};
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<hiring::BeginAcceptingApplicationsError>> for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::BeginAcceptingApplicationsError>) -> Self {
        match wrapper.error {
            hiring::BeginAcceptingApplicationsError::OpeningDoesNotExist => {
                Error::AcceptWorkerApplicationsOpeningDoesNotExist
            }
            hiring::BeginAcceptingApplicationsError::OpeningIsNotInWaitingToBeginStage => {
                Error::AcceptWorkerApplicationsOpeningIsNotWaitingToBegin
            }
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<hiring::AddOpeningError>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::AddOpeningError>) -> Self {
        match wrapper.error {
            hiring::AddOpeningError::OpeningMustActivateInTheFuture => {
                Error::AddWorkerOpeningActivatesInThePast
            }
            hiring::AddOpeningError::StakeAmountLessThanMinimumStakeBalance(purpose) => {
                match purpose {
                    hiring::StakePurpose::Role => Error::AddWorkerOpeningRoleStakeLessThanMinimum,
                    hiring::StakePurpose::Application => {
                        Error::AddWorkerOpeningAppliicationStakeLessThanMinimum
                    }
                }
            }
            hiring::AddOpeningError::ApplicationRationingZeroMaxApplicants => {
                Error::AddWorkerOpeningZeroMaxApplicantCount
            }
            hiring::AddOpeningError::StakeAmountCannotBeZero(purpose) => match purpose {
                hiring::StakePurpose::Role => Error::AddWorkerOpeningRoleStakeCannotBeZero,
                hiring::StakePurpose::Application => {
                    Error::AddWorkerOpeningApplicationStakeCannotBeZero
                }
            },
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<hiring::BeginReviewError>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::BeginReviewError>) -> Self {
        match wrapper.error {
            hiring::BeginReviewError::OpeningDoesNotExist => {
                Error::BeginWorkerApplicantReviewOpeningDoesNotExist
            }
            hiring::BeginReviewError::OpeningNotInAcceptingApplicationsStage => {
                Error::BeginWorkerApplicantReviewOpeningOpeningIsNotWaitingToBegin
            }
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<hiring::FillOpeningError<T>>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::FillOpeningError<T>>) -> Self {
        match wrapper.error {
            hiring::FillOpeningError::<T>::OpeningDoesNotExist => {
                Error::FullWorkerOpeningOpeningDoesNotExist
            }
            hiring::FillOpeningError::<T>::OpeningNotInReviewPeriodStage => {
                Error::FullWorkerOpeningOpeningNotInReviewPeriodStage
            }
            hiring::FillOpeningError::<T>::UnstakingPeriodTooShort(
                stake_purpose,
                outcome_in_filled_opening,
            ) => match stake_purpose {
                hiring::StakePurpose::Application => match outcome_in_filled_opening {
                    hiring::ApplicationOutcomeInFilledOpening::Success => {
                        Error::FullWorkerOpeningSuccessfulApplicationStakeUnstakingPeriodTooShort
                    }
                    hiring::ApplicationOutcomeInFilledOpening::Failure => {
                        Error::FullWorkerOpeningUnsuccessfulApplicationStakeUnstakingPeriodTooShort
                    }
                },
                hiring::StakePurpose::Role => match outcome_in_filled_opening {
                    hiring::ApplicationOutcomeInFilledOpening::Success => {
                        Error::FullWorkerOpeningSuccessfulRoleStakeUnstakingPeriodTooShort
                    }
                    hiring::ApplicationOutcomeInFilledOpening::Failure => {
                        Error::FullWorkerOpeningUnsuccessfulRoleStakeUnstakingPeriodTooShort
                    }
                },
            },
            hiring::FillOpeningError::<T>::RedundantUnstakingPeriodProvided(
                stake_purpose,
                outcome_in_filled_opening,
            ) => match stake_purpose {
                hiring::StakePurpose::Application => match outcome_in_filled_opening {
                    hiring::ApplicationOutcomeInFilledOpening::Success => {
                        Error::FullWorkerOpeningSuccessfulApplicationStakeUnstakingPeriodRedundant
                    }
                    hiring::ApplicationOutcomeInFilledOpening::Failure => {
                        Error::FullWorkerOpeningUnsuccessfulApplicationStakeUnstakingPeriodRedundant
                    }
                },
                hiring::StakePurpose::Role => match outcome_in_filled_opening {
                    hiring::ApplicationOutcomeInFilledOpening::Success => {
                        Error::FullWorkerOpeningSuccessfulRoleStakeUnstakingPeriodRedundant
                    }
                    hiring::ApplicationOutcomeInFilledOpening::Failure => {
                        Error::FullWorkerOpeningUnsuccessfulRoleStakeUnstakingPeriodRedundant
                    }
                },
            },
            hiring::FillOpeningError::<T>::ApplicationDoesNotExist(_application_id) => {
                Error::FullWorkerOpeningApplicationDoesNotExist
            }
            hiring::FillOpeningError::<T>::ApplicationNotInActiveStage(_application_id) => {
                Error::FullWorkerOpeningApplicationNotActive
            }
            hiring::FillOpeningError::<T>::ApplicationForWrongOpening(_application_id) => {
                Error::FillWorkerOpeningApplicationForWrongOpening
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<hiring::DeactivateApplicationError>> for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::DeactivateApplicationError>) -> Self {
        match wrapper.error {
            hiring::DeactivateApplicationError::ApplicationDoesNotExist => {
                Error::WithdrawWorkerApplicationApplicationDoesNotExist
            }
            hiring::DeactivateApplicationError::ApplicationNotActive => {
                Error::WithdrawWorkerApplicationApplicationNotActive
            }
            hiring::DeactivateApplicationError::OpeningNotAcceptingApplications => {
                Error::WithdrawWorkerApplicationOpeningNotAcceptingApplications
            }
            hiring::DeactivateApplicationError::UnstakingPeriodTooShort(_stake_purpose) => {
                Error::WithdrawWorkerApplicationUnstakingPeriodTooShort
            }
            hiring::DeactivateApplicationError::RedundantUnstakingPeriodProvided(
                _stake_purpose,
            ) => Error::WithdrawWorkerApplicationRedundantUnstakingPeriod,
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<hiring::AddApplicationError>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<hiring::AddApplicationError>) -> Self {
        match wrapper.error {
            hiring::AddApplicationError::OpeningDoesNotExist => {
                Error::AddWorkerOpeningOpeningDoesNotExist
            }
            hiring::AddApplicationError::StakeProvidedWhenRedundant(_stake_purpose) => {
                Error::AddWorkerOpeningStakeProvidedWhenRedundant
            }
            hiring::AddApplicationError::StakeMissingWhenRequired(_stake_purpose) => {
                Error::AddWorkerOpeningStakeMissingWhenRequired
            }
            hiring::AddApplicationError::StakeAmountTooLow(_stake_purpose) => {
                Error::AddWorkerOpeningStakeAmountTooLow
            }
            hiring::AddApplicationError::OpeningNotInAcceptingApplicationsStage => {
                Error::AddWorkerOpeningOpeningNotInAcceptingApplicationStage
            }
            hiring::AddApplicationError::NewApplicationWasCrowdedOut => {
                Error::AddWorkerOpeningNewApplicationWasCrowdedOut
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<membership::MemberControllerAccountDidNotSign>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<membership::MemberControllerAccountDidNotSign>) -> Self {
        match wrapper.error {
            membership::MemberControllerAccountDidNotSign::UnsignedOrigin => {
                Error::MembershipUnsignedOrigin
            }
            membership::MemberControllerAccountDidNotSign::MemberIdInvalid => {
                Error::MembershipInvalidMemberId
            }
            membership::MemberControllerAccountDidNotSign::SignerControllerAccountMismatch => {
                Error::ApplyOnWorkerOpeningSignerNotControllerAccount
            }
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<recurringrewards::RewardsError>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<recurringrewards::RewardsError>) -> Self {
        match wrapper.error {
            recurringrewards::RewardsError::NextPaymentNotInFuture => {
                Error::RecurringRewardsNextPaymentNotInFuture
            }
            recurringrewards::RewardsError::RecipientNotFound => {
                Error::RecurringRewardsRecipientNotFound
            }
            recurringrewards::RewardsError::RewardSourceNotFound => {
                Error::RecurringRewardsRewardSourceNotFound
            }
            recurringrewards::RewardsError::RewardRelationshipNotFound => {
                Error::RecurringRewardsRewardRelationshipNotFound
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<stake::StakeActionError<stake::InitiateUnstakingError>>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::InitiateUnstakingError>>) -> Self {
        match wrapper.error {
            stake::StakeActionError::StakeNotFound => Error::StakingErrorStakeNotFound,
            stake::StakeActionError::Error(initiate_unstaking_error) => {
                match initiate_unstaking_error {
                    stake::InitiateUnstakingError::UnstakingPeriodShouldBeGreaterThanZero => {
                        Error::StakingErrorUnstakingPeriodShouldBeGreaterThanZero
                    }
                    stake::InitiateUnstakingError::UnstakingError(unstaking_error) => {
                        match unstaking_error {
                            stake::UnstakingError::AlreadyUnstaking => {
                                Error::StakingErrorAlreadyUnstaking
                            }
                            stake::UnstakingError::NotStaked => Error::StakingErrorNotStaked,
                            stake::UnstakingError::CannotUnstakeWhileSlashesOngoing => {
                                Error::StakingErrorCannotUnstakeWhileSlashesOngoing
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<
        WrappedError<stake::StakeActionError<stake::IncreasingStakeFromAccountError>>,
    > for Error<T, I>
{
    fn from(
        wrapper: WrappedError<stake::StakeActionError<stake::IncreasingStakeFromAccountError>>,
    ) -> Self {
        match wrapper.error {
            stake::StakeActionError::StakeNotFound => Error::StakingErrorStakeNotFound,
            stake::StakeActionError::Error(increase_stake_error_from_account) => {
                match increase_stake_error_from_account {
                    stake::IncreasingStakeFromAccountError::InsufficientBalanceInSourceAccount => {
                        Error::StakingErrorInsufficientBalanceInSourceAccount
                    }
                    stake::IncreasingStakeFromAccountError::IncreasingStakeError(
                        increasing_stake_error,
                    ) => match increasing_stake_error {
                        stake::IncreasingStakeError::NotStaked => Error::StakingErrorNotStaked,
                        stake::IncreasingStakeError::CannotChangeStakeByZero => {
                            Error::StakingErrorCannotChangeStakeByZero
                        }
                        stake::IncreasingStakeError::CannotIncreaseStakeWhileUnstaking => {
                            Error::StakingErrorCannotIncreaseStakeWhileUnstaking
                        }
                    },
                }
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<stake::StakeActionError<stake::IncreasingStakeError>>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::IncreasingStakeError>>) -> Self {
        match wrapper.error {
            stake::StakeActionError::StakeNotFound => Error::StakingErrorStakeNotFound,
            stake::StakeActionError::Error(increasing_stake_error) => {
                match increasing_stake_error {
                    stake::IncreasingStakeError::NotStaked => Error::StakingErrorNotStaked,
                    stake::IncreasingStakeError::CannotChangeStakeByZero => {
                        Error::StakingErrorCannotChangeStakeByZero
                    }
                    stake::IncreasingStakeError::CannotIncreaseStakeWhileUnstaking => {
                        Error::StakingErrorCannotIncreaseStakeWhileUnstaking
                    }
                }
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<stake::StakeActionError<stake::DecreasingStakeError>>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::DecreasingStakeError>>) -> Self {
        match wrapper.error {
            stake::StakeActionError::StakeNotFound => Error::StakingErrorStakeNotFound,
            stake::StakeActionError::Error(decreasing_stake_error) => {
                match decreasing_stake_error {
                    stake::DecreasingStakeError::NotStaked => Error::StakingErrorNotStaked,
                    stake::DecreasingStakeError::CannotChangeStakeByZero => {
                        Error::StakingErrorCannotChangeStakeByZero
                    }
                    stake::DecreasingStakeError::CannotDecreaseStakeWhileUnstaking => {
                        Error::StakingErrorCannotIncreaseStakeWhileUnstaking
                    }
                    stake::DecreasingStakeError::CannotDecreaseStakeWhileOngoingSlahes => {
                        Error::StakingErrorCannotDecreaseWhileSlashesOngoing
                    }
                    stake::DecreasingStakeError::InsufficientStake => {
                        Error::StakingErrorInsufficientStake
                    }
                }
            }
        }
    }
}

impl<T: Config<I>, I: Instance>
    sp_std::convert::From<WrappedError<stake::StakeActionError<stake::ImmediateSlashingError>>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::ImmediateSlashingError>>) -> Self {
        match wrapper.error {
            stake::StakeActionError::StakeNotFound => Error::StakingErrorStakeNotFound,
            stake::StakeActionError::Error(slashing_error) => match slashing_error {
                stake::ImmediateSlashingError::NotStaked => Error::StakingErrorNotStaked,
                stake::ImmediateSlashingError::SlashAmountShouldBeGreaterThanZero => {
                    Error::StakingErrorSlashAmountShouldBeGreaterThanZero
                }
            },
        }
    }
}

impl<T: Config<I>, I: Instance> sp_std::convert::From<WrappedError<minting::GeneralError>>
    for Error<T, I>
{
    fn from(wrapper: WrappedError<minting::GeneralError>) -> Self {
        match wrapper.error {
            minting::GeneralError::MintNotFound => Error::CannotFindMint,
            minting::GeneralError::NextAdjustmentInPast => Error::MintingErrorNextAdjustmentInPast,
        }
    }
}
