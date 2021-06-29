#![warn(missing_docs)]

use super::{BalanceOf, CurrencyOf, NegativeImbalance};
use crate::Config;
use frame_support::traits::{Currency, ExistenceRequirement, WithdrawReasons};
use sp_std::convert::From;
use sp_std::marker::PhantomData;
use sp_std::rc::Rc;

// Mocking dependencies for testing
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

/// Returns registered stake handler. This is scaffolds for the mocking of the stake module.
pub trait StakeHandlerProvider<T: Config> {
    /// Returns stake logic handler
    fn stakes() -> Rc<dyn StakeHandler<T>>;
}

/// Default implementation of the stake module logic provider. Returns actual implementation
/// dependent on the stake module.
pub struct DefaultStakeHandlerProvider;
impl<T: Config> StakeHandlerProvider<T> for DefaultStakeHandlerProvider {
    /// Returns stake logic handler
    fn stakes() -> Rc<dyn StakeHandler<T>> {
        Rc::new(DefaultStakeHandler {
            marker: PhantomData::<T>::default(),
        })
    }
}

/// Stake logic handler.
#[cfg_attr(test, automock)] // attributes creates mocks in testing environment
pub trait StakeHandler<T: Config> {
    /// Creates a stake. Returns created stake id or an error.
    fn create_stake(&self) -> Result<T::StakeId, &'static str>;

    /// Stake the imbalance
    fn stake(
        &self,
        stake_id: &T::StakeId,
        stake_imbalance: NegativeImbalance<T>,
    ) -> Result<(), &'static str>;

    /// Removes stake
    fn remove_stake(&self, stake_id: T::StakeId) -> Result<(), &'static str>;

    /// Execute unstaking
    fn unstake(&self, stake_id: T::StakeId) -> Result<(), &'static str>;

    /// Slash balance from the existing stake
    fn slash(&self, stake_id: T::StakeId, slash_balance: BalanceOf<T>) -> Result<(), &'static str>;

    /// Withdraw some balance from the source account and create stake imbalance
    fn make_stake_imbalance(
        &self,
        balance: BalanceOf<T>,
        source_account_id: &T::AccountId,
    ) -> Result<NegativeImbalance<T>, &'static str>;
}

/// Default implementation of the stake logic. Uses actual stake module.
/// 'marker' responsible for the 'Config' binding.
pub(crate) struct DefaultStakeHandler<T> {
    pub marker: PhantomData<T>,
}

impl<T: Config> StakeHandler<T> for DefaultStakeHandler<T> {
    /// Creates a stake. Returns created stake id or an error.
    fn create_stake(&self) -> Result<<T as stake::Config>::StakeId, &'static str> {
        Ok(stake::Module::<T>::create_stake())
    }

    /// Stake the imbalance
    fn stake(
        &self,
        stake_id: &<T as stake::Config>::StakeId,
        stake_imbalance: NegativeImbalance<T>,
    ) -> Result<(), &'static str> {
        stake::Module::<T>::stake(&stake_id, stake_imbalance).map_err(WrappedError)?;

        Ok(())
    }

    /// Removes stake
    fn remove_stake(&self, stake_id: <T as stake::Config>::StakeId) -> Result<(), &'static str> {
        stake::Module::<T>::remove_stake(&stake_id).map_err(WrappedError)?;

        Ok(())
    }

    /// Execute unstaking
    fn unstake(&self, stake_id: <T as stake::Config>::StakeId) -> Result<(), &'static str> {
        stake::Module::<T>::initiate_unstaking(&stake_id, None).map_err(WrappedError)?;

        Ok(())
    }

    /// Slash balance from the existing stake
    fn slash(
        &self,
        stake_id: <T as stake::Config>::StakeId,
        slash_balance: BalanceOf<T>,
    ) -> Result<(), &'static str> {
        let _ignored_successful_result =
            stake::Module::<T>::slash_immediate(&stake_id, slash_balance, false)
                .map_err(WrappedError)?;

        Ok(())
    }

    /// Withdraw some balance from the source account and create stake imbalance
    fn make_stake_imbalance(
        &self,
        balance: BalanceOf<T>,
        source_account_id: &T::AccountId,
    ) -> Result<NegativeImbalance<T>, &'static str> {
        CurrencyOf::<T>::withdraw(
            source_account_id,
            balance,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )
        .map_err(<&str>::from)
    }
}

/// Proposal implementation of the stake logic.
/// 'marker' responsible for the 'Config' binding.
pub(crate) struct ProposalStakeManager<T> {
    pub marker: PhantomData<T>,
}

impl<T: Config> ProposalStakeManager<T> {
    /// Creates a stake using stake balance and source account.
    /// Returns created stake id or an error.
    pub fn create_stake(
        stake_balance: BalanceOf<T>,
        source_account_id: T::AccountId,
    ) -> Result<T::StakeId, &'static str> {
        let stake_id = T::StakeHandlerProvider::stakes().create_stake()?;

        let stake_imbalance = T::StakeHandlerProvider::stakes()
            .make_stake_imbalance(stake_balance, &source_account_id)?;

        T::StakeHandlerProvider::stakes().stake(&stake_id, stake_imbalance)?;

        Ok(stake_id)
    }

    /// Execute unstaking and removes the stake
    pub fn remove_stake(stake_id: T::StakeId) -> Result<(), &'static str> {
        T::StakeHandlerProvider::stakes().unstake(stake_id)?;

        T::StakeHandlerProvider::stakes().remove_stake(stake_id)?;

        Ok(())
    }

    /// Slash balance from the existing stake
    pub fn slash(stake_id: T::StakeId, slash_balance: BalanceOf<T>) -> Result<(), &'static str> {
        T::StakeHandlerProvider::stakes().slash(stake_id, slash_balance)
    }
}

// 'New type' pattern for the error
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
struct WrappedError<E>(E);

// error conversion for the Wrapped StakeActionError with the inner InitiateUnstakingError
impl From<WrappedError<stake::StakeActionError<stake::InitiateUnstakingError>>> for &str {
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::InitiateUnstakingError>>) -> Self {
        {
            match wrapper.0 {
                stake::StakeActionError::StakeNotFound => "StakeNotFound",
                stake::StakeActionError::Error(err) => match err {
                    stake::InitiateUnstakingError::UnstakingPeriodShouldBeGreaterThanZero => {
                        "UnstakingPeriodShouldBeGreaterThanZero"
                    }
                    stake::InitiateUnstakingError::UnstakingError(e) => match e {
                        stake::UnstakingError::NotStaked => "NotStaked",
                        stake::UnstakingError::AlreadyUnstaking => "AlreadyUnstaking",
                        stake::UnstakingError::CannotUnstakeWhileSlashesOngoing => {
                            "CannotUnstakeWhileSlashesOngoing"
                        }
                    },
                },
            }
        }
    }
}

// error conversion for the Wrapped StakeActionError with the inner StakingError
impl From<WrappedError<stake::StakeActionError<stake::StakingError>>> for &str {
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::StakingError>>) -> Self {
        {
            match wrapper.0 {
                stake::StakeActionError::StakeNotFound => "StakeNotFound",
                stake::StakeActionError::Error(err) => match err {
                    stake::StakingError::CannotStakeZero => "CannotStakeZero",
                    stake::StakingError::CannotStakeLessThanMinimumBalance => {
                        "CannotStakeLessThanMinimumBalance"
                    }
                    stake::StakingError::AlreadyStaked => "AlreadyStaked",
                },
            }
        }
    }
}

// error conversion for the Wrapped StakeActionError with the inner InitiateSlashingError
impl From<WrappedError<stake::StakeActionError<stake::InitiateSlashingError>>> for &str {
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::InitiateSlashingError>>) -> Self {
        {
            match wrapper.0 {
                stake::StakeActionError::StakeNotFound => "StakeNotFound",
                stake::StakeActionError::Error(err) => match err {
                    stake::InitiateSlashingError::NotStaked => "NotStaked",
                    stake::InitiateSlashingError::SlashPeriodShouldBeGreaterThanZero => {
                        "SlashPeriodShouldBeGreaterThanZero"
                    }
                    stake::InitiateSlashingError::SlashAmountShouldBeGreaterThanZero => {
                        "SlashAmountShouldBeGreaterThanZero"
                    }
                },
            }
        }
    }
}

// error conversion for the Wrapped StakeActionError with the inner ImmediateSlashingError
impl From<WrappedError<stake::StakeActionError<stake::ImmediateSlashingError>>> for &str {
    fn from(wrapper: WrappedError<stake::StakeActionError<stake::ImmediateSlashingError>>) -> Self {
        {
            match wrapper.0 {
                stake::StakeActionError::StakeNotFound => "StakeNotFound",
                stake::StakeActionError::Error(err) => match err {
                    stake::ImmediateSlashingError::NotStaked => "NotStaked",
                    stake::ImmediateSlashingError::SlashAmountShouldBeGreaterThanZero => {
                        "SlashAmountShouldBeGreaterThanZero"
                    }
                },
            }
        }
    }
}
