// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::storage::IterableStorageMap;
use frame_support::traits::{Currency, ExistenceRequirement, Get, Imbalance, WithdrawReasons};
use frame_support::{decl_module, decl_storage, ensure, PalletId, Parameter};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{AccountIdConversion, MaybeSerialize, Member};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::prelude::*;

mod errors;
pub use errors::*;
mod macroes;
mod mock;
mod tests;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalance<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub trait Config: frame_system::Config + Sized {
    /// The currency that is managed by the module
    type Currency: Currency<Self::AccountId>;

    /// ModuleId for computing deterministic AccountId for the module
    type StakePoolId: Get<[u8; 8]>;

    /// Type that will handle various staking events
    type StakingEventsHandler: StakingEventsHandler<Self>;

    /// The type used as a stake identifier.
    type StakeId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// The type used as slash identifier.
    type SlashId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + Ord; //required to be a key in BTreeMap
}

pub trait StakingEventsHandler<T: Config> {
    /// Handler for unstaking event.
    /// The handler is informed of the amount that was unstaked, and the value removed from stake is passed as a negative imbalance.
    /// The handler is responsible to consume part or all of the value (for example by moving it into an account). The remainder
    /// of the value that is not consumed should be returned as a negative imbalance.
    fn unstaked(
        id: &T::StakeId,
        unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T>;

    /// Handler for slashing event.
    /// NB: actually_slashed can be less than amount of the slash itself if the
    /// claim amount on the stake cannot cover it fully.
    /// The SlashId is optional, as slashing may not be associated with a slashing that was initiated, but was an immediate slashing.
    /// For Immediate slashes, the stake may have transitioned to NotStaked so handler should not assume the state
    /// is still in staked status.
    fn slashed(
        id: &T::StakeId,
        slash_id: Option<T::SlashId>,
        slashed_amount: BalanceOf<T>,
        remaining_stake: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T>;
}

/// Default implementation just destroys the unstaked or slashed value
impl<T: Config> StakingEventsHandler<T> for () {
    fn unstaked(
        _id: &T::StakeId,
        _unstaked_amount: BalanceOf<T>,
        _remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        NegativeImbalance::<T>::zero()
    }

    fn slashed(
        _id: &T::StakeId,
        _slash_id: Option<T::SlashId>,
        _slahed_amount: BalanceOf<T>,
        _remaining_stake: BalanceOf<T>,
        _remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        NegativeImbalance::<T>::zero()
    }
}

/// Helper implementation so we can chain multiple handlers by grouping handlers in tuple pairs.
/// For example for three handlers, A, B and C we can set the StakingEventHandler type on the trait to:
/// type StakingEventHandler = ((A, B), C)
/// Individual handlers are expected consume in full or in part the negative imbalance and return any unconsumed value.
/// The unconsumed value is then passed to the next handler in the chain.
impl<T: Config, X: StakingEventsHandler<T>, Y: StakingEventsHandler<T>> StakingEventsHandler<T>
    for (X, Y)
{
    fn unstaked(
        id: &T::StakeId,
        unstaked_amount: BalanceOf<T>,
        imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        let unused_imbalance = X::unstaked(id, unstaked_amount, imbalance);
        Y::unstaked(id, unstaked_amount, unused_imbalance)
    }

    fn slashed(
        id: &T::StakeId,
        slash_id: Option<T::SlashId>,
        slashed_amount: BalanceOf<T>,
        remaining_stake: BalanceOf<T>,
        imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        let unused_imbalance = X::slashed(id, slash_id, slashed_amount, remaining_stake, imbalance);
        Y::slashed(
            id,
            slash_id,
            slashed_amount,
            remaining_stake,
            unused_imbalance,
        )
    }
}

#[derive(Encode, Decode, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Slash<BlockNumber, Balance> {
    /// The block where slashing was initiated.
    pub started_at_block: BlockNumber,

    /// Whether slashing is in active, or conversley paused state.
    /// Blocks are only counted towards slashing execution delay when active.
    pub is_active: bool,

    /// The number blocks which must be finalised while in the active period before the slashing can be executed
    pub blocks_remaining_in_active_period_for_slashing: BlockNumber,

    /// Amount to slash
    pub slash_amount: Balance,
}

#[derive(Encode, Decode, Debug, Default, Eq, PartialEq)]
pub struct UnstakingState<BlockNumber> {
    /// The block where the unstaking was initiated
    pub started_at_block: BlockNumber,

    /// Whether unstaking is in active, or conversely paused state.
    /// Blocks are only counted towards unstaking period when active.
    pub is_active: bool,

    /// The number blocks which must be finalised while in the active period before the unstaking is finished
    pub blocks_remaining_in_active_period_for_unstaking: BlockNumber,
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum StakedStatus<BlockNumber> {
    /// Baseline staking status, nothing is happening.
    Normal,

    /// Unstaking is under way.
    Unstaking(UnstakingState<BlockNumber>),
}

impl<BlockNumber> Default for StakedStatus<BlockNumber> {
    fn default() -> Self {
        StakedStatus::Normal
    }
}

#[derive(Encode, Decode, Debug, Default, Eq, PartialEq)]
pub struct StakedState<BlockNumber, Balance, SlashId: Ord> {
    /// Total amount of funds at stake.
    pub staked_amount: Balance,

    /// Status of the staking.
    pub staked_status: StakedStatus<BlockNumber>,

    /// SlashId to use for next Slash that is initiated.
    /// Will be incremented by one after adding a new Slash.
    pub next_slash_id: SlashId,

    /// All ongoing slashing.
    pub ongoing_slashes: BTreeMap<SlashId, Slash<BlockNumber, Balance>>,
}

impl<BlockNumber, Balance, SlashId> StakedState<BlockNumber, Balance, SlashId>
where
    BlockNumber: BaseArithmetic + Copy,
    Balance: BaseArithmetic + Copy,
    SlashId: Ord + Copy,
{
    /// Iterates over all ongoing slashes and decrements blocks_remaining_in_active_period_for_slashing of active slashes (advancing the timer).
    /// Returns true if there was at least one slashe that was active and had its timer advanced.
    fn advance_slashing_timer(&mut self) -> bool {
        let mut did_advance_timers = false;

        for (_slash_id, slash) in self.ongoing_slashes.iter_mut() {
            if slash.is_active
                && slash.blocks_remaining_in_active_period_for_slashing > Zero::zero()
            {
                slash.blocks_remaining_in_active_period_for_slashing -= One::one();
                did_advance_timers = true;
            }
        }

        did_advance_timers
    }

    /// Returns pair of slash_id and slashes that should be executed
    fn get_slashes_to_finalize(&mut self) -> Vec<(SlashId, Slash<BlockNumber, Balance>)> {
        let slashes_to_finalize = self
            .ongoing_slashes
            .iter()
            .filter(|(_, slash)| {
                slash.blocks_remaining_in_active_period_for_slashing == Zero::zero()
            })
            .map(|(slash_id, _)| *slash_id)
            .collect::<Vec<_>>();

        // remove and return the slashes
        slashes_to_finalize
            .iter()
            .map(|slash_id| {
                // assert!(self.ongoing_slashes.contains_key(slash_id))
                (*slash_id, self.ongoing_slashes.remove(slash_id).unwrap())
            })
            .collect()
    }

    /// Executes a Slash. If remaining at stake drops below the minimum_balance, it will slash the entire staked amount.
    /// Returns the actual slashed amount.
    fn apply_slash(&mut self, slash_amount: Balance, minimum_balance: Balance) -> Balance {
        // calculate how much to slash
        let mut slash_amount = if slash_amount > self.staked_amount {
            self.staked_amount
        } else {
            slash_amount
        };

        // apply the slashing
        self.staked_amount -= slash_amount;

        // don't leave less than minimum_balance at stake
        if self.staked_amount < minimum_balance {
            slash_amount += self.staked_amount;
            self.staked_amount = Zero::zero();
        }

        slash_amount
    }

    /// For all slahes that should be executed, will apply the Slash to the staked amount, and drop it from the ongoing slashes map.
    /// Returns a vector of the executed slashes outcome: (SlashId, Slashed Amount, Remaining Staked Amount)
    fn finalize_slashes(&mut self, minimum_balance: Balance) -> Vec<(SlashId, Balance, Balance)> {
        let mut finalized_slashes: Vec<(SlashId, Balance, Balance)> = Vec::new();

        for (slash_id, slash) in self.get_slashes_to_finalize().iter() {
            // apply the slashing and get back actual amount slashed
            let slashed_amount = self.apply_slash(slash.slash_amount, minimum_balance);

            finalized_slashes.push((*slash_id, slashed_amount, self.staked_amount));
        }

        finalized_slashes
    }
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum StakingStatus<BlockNumber, Balance, SlashId: Ord> {
    NotStaked,

    Staked(StakedState<BlockNumber, Balance, SlashId>),
}

impl<BlockNumber, Balance, SlashId: Ord> Default for StakingStatus<BlockNumber, Balance, SlashId> {
    fn default() -> Self {
        StakingStatus::NotStaked
    }
}

#[derive(Encode, Decode, Default, Debug, Eq, PartialEq)]
pub struct Stake<BlockNumber, Balance, SlashId: Ord> {
    /// When role was created
    pub created: BlockNumber,

    /// Status of any possible ongoing staking
    pub staking_status: StakingStatus<BlockNumber, Balance, SlashId>,
}

impl<BlockNumber, Balance, SlashId> Stake<BlockNumber, Balance, SlashId>
where
    BlockNumber: Copy + BaseArithmetic + Zero,
    Balance: Copy + BaseArithmetic,
    SlashId: Copy + Ord + Zero + One,
{
    fn new(created_at: BlockNumber) -> Self {
        Self {
            created: created_at,
            staking_status: StakingStatus::NotStaked,
        }
    }

    fn is_not_staked(&self) -> bool {
        self.staking_status == StakingStatus::NotStaked
    }

    /// If staking status is Staked and not currently Unstaking it will increase the staked amount by value.
    /// On success returns new total staked value.
    /// Increasing stake by zero is an error.
    fn increase_stake(&mut self, value: Balance) -> Result<Balance, IncreasingStakeError> {
        ensure!(
            value > Zero::zero(),
            IncreasingStakeError::CannotChangeStakeByZero
        );
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => match staked_state.staked_status {
                StakedStatus::Normal => {
                    staked_state.staked_amount += value;
                    Ok(staked_state.staked_amount)
                }
                _ => Err(IncreasingStakeError::CannotIncreaseStakeWhileUnstaking),
            },
            _ => Err(IncreasingStakeError::NotStaked),
        }
    }

    /// If staking status is Staked and not currently Unstaking, and no ongoing slashes exist, it will decrease the amount at stake
    /// by provided value. If remaining at stake drops below the minimum_balance it will decrease the stake to zero.
    /// On success returns (the actual amount of stake decreased, the remaining amount at stake).
    /// Decreasing stake by zero is an error.
    fn decrease_stake(
        &mut self,
        value: Balance,
        minimum_balance: Balance,
    ) -> Result<(Balance, Balance), DecreasingStakeError> {
        // maybe StakeDecrease
        ensure!(
            value > Zero::zero(),
            DecreasingStakeError::CannotChangeStakeByZero
        );
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => match staked_state.staked_status {
                StakedStatus::Normal => {
                    // prevent decreasing stake if there are any ongoing slashes (irrespective if active or not)
                    if !staked_state.ongoing_slashes.is_empty() {
                        return Err(DecreasingStakeError::CannotDecreaseStakeWhileOngoingSlahes);
                    }

                    if value > staked_state.staked_amount {
                        return Err(DecreasingStakeError::InsufficientStake);
                    }

                    let stake_to_reduce = if staked_state.staked_amount - value < minimum_balance {
                        // If staked amount would drop below minimum balance, deduct the entire stake
                        staked_state.staked_amount
                    } else {
                        value
                    };

                    staked_state.staked_amount -= stake_to_reduce;

                    Ok((stake_to_reduce, staked_state.staked_amount))
                }
                _ => Err(DecreasingStakeError::CannotDecreaseStakeWhileUnstaking),
            },
            _ => Err(DecreasingStakeError::NotStaked),
        }
    }

    fn start_staking(
        &mut self,
        value: Balance,
        minimum_balance: Balance,
    ) -> Result<(), StakingError> {
        ensure!(value > Zero::zero(), StakingError::CannotStakeZero);
        ensure!(
            value >= minimum_balance,
            StakingError::CannotStakeLessThanMinimumBalance
        );
        if self.is_not_staked() {
            self.staking_status = StakingStatus::Staked(StakedState {
                staked_amount: value,
                next_slash_id: Zero::zero(),
                ongoing_slashes: BTreeMap::new(),
                staked_status: StakedStatus::Normal,
            });
            Ok(())
        } else {
            Err(StakingError::AlreadyStaked)
        }
    }

    fn slash_immediate(
        &mut self,
        slash_amount: Balance,
        minimum_balance: Balance,
    ) -> Result<(Balance, Balance), ImmediateSlashingError> {
        ensure!(
            slash_amount > Zero::zero(),
            ImmediateSlashingError::SlashAmountShouldBeGreaterThanZero
        );

        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                // irrespective of wether we are unstaking or not, slash!

                let actually_slashed = staked_state.apply_slash(slash_amount, minimum_balance);

                let remaining_stake = staked_state.staked_amount;

                Ok((actually_slashed, remaining_stake))
            }
            // can't slash if not staked
            _ => Err(ImmediateSlashingError::NotStaked),
        }
    }

    fn initiate_slashing(
        &mut self,
        slash_amount: Balance,
        slash_period: BlockNumber,
        now: BlockNumber,
    ) -> Result<SlashId, InitiateSlashingError> {
        ensure!(
            slash_period > Zero::zero(),
            InitiateSlashingError::SlashPeriodShouldBeGreaterThanZero
        );
        ensure!(
            slash_amount > Zero::zero(),
            InitiateSlashingError::SlashAmountShouldBeGreaterThanZero
        );

        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                let slash_id = staked_state.next_slash_id;
                staked_state.next_slash_id = slash_id + One::one();

                staked_state.ongoing_slashes.insert(
                    slash_id,
                    Slash {
                        is_active: true,
                        blocks_remaining_in_active_period_for_slashing: slash_period,
                        slash_amount,
                        started_at_block: now,
                    },
                );

                // pause Unstaking if unstaking is active
                if let StakedStatus::Unstaking(ref mut unstaking_state) = staked_state.staked_status
                {
                    unstaking_state.is_active = false;
                }

                Ok(slash_id)
            }
            _ => Err(InitiateSlashingError::NotStaked),
        }
    }

    fn pause_slashing(&mut self, slash_id: &SlashId) -> Result<(), PauseSlashingError> {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                match staked_state.ongoing_slashes.get_mut(slash_id) {
                    Some(ref mut slash) => {
                        if slash.is_active {
                            slash.is_active = false;
                            Ok(())
                        } else {
                            Err(PauseSlashingError::AlreadyPaused)
                        }
                    }
                    _ => Err(PauseSlashingError::SlashNotFound),
                }
            }
            _ => Err(PauseSlashingError::NotStaked),
        }
    }

    fn resume_slashing(&mut self, slash_id: &SlashId) -> Result<(), ResumeSlashingError> {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                match staked_state.ongoing_slashes.get_mut(slash_id) {
                    Some(ref mut slash) => {
                        if slash.is_active {
                            Err(ResumeSlashingError::NotPaused)
                        } else {
                            slash.is_active = true;
                            Ok(())
                        }
                    }
                    _ => Err(ResumeSlashingError::SlashNotFound),
                }
            }
            _ => Err(ResumeSlashingError::NotStaked),
        }
    }

    fn cancel_slashing(&mut self, slash_id: &SlashId) -> Result<(), CancelSlashingError> {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                if staked_state.ongoing_slashes.remove(slash_id).is_none() {
                    return Err(CancelSlashingError::SlashNotFound);
                }

                // unpause unstaking on last ongoing slash cancelled
                if staked_state.ongoing_slashes.is_empty() {
                    if let StakedStatus::Unstaking(ref mut unstaking_state) =
                        staked_state.staked_status
                    {
                        unstaking_state.is_active = true;
                    }
                }

                Ok(())
            }
            _ => Err(CancelSlashingError::NotStaked),
        }
    }

    fn unstake(&mut self) -> Result<Balance, UnstakingError> {
        let staked_amount = match self.staking_status {
            StakingStatus::Staked(ref staked_state) => {
                // prevent unstaking if there are any ongonig slashes (irrespective if active or not)
                if !staked_state.ongoing_slashes.is_empty() {
                    return Err(UnstakingError::CannotUnstakeWhileSlashesOngoing);
                }
                if StakedStatus::Normal != staked_state.staked_status {
                    return Err(UnstakingError::AlreadyUnstaking);
                }
                Ok(staked_state.staked_amount)
            }
            _ => Err(UnstakingError::NotStaked),
        }?;

        self.staking_status = StakingStatus::NotStaked;
        Ok(staked_amount)
    }

    fn initiate_unstaking(
        &mut self,
        unstaking_period: BlockNumber,
        now: BlockNumber,
    ) -> Result<(), InitiateUnstakingError> {
        ensure!(
            unstaking_period > Zero::zero(),
            InitiateUnstakingError::UnstakingPeriodShouldBeGreaterThanZero
        );
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                // prevent unstaking if there are any ongonig slashes (irrespective if active or not)
                if !staked_state.ongoing_slashes.is_empty() {
                    return Err(InitiateUnstakingError::UnstakingError(
                        UnstakingError::CannotUnstakeWhileSlashesOngoing,
                    ));
                }

                if StakedStatus::Normal != staked_state.staked_status {
                    return Err(InitiateUnstakingError::UnstakingError(
                        UnstakingError::AlreadyUnstaking,
                    ));
                }

                staked_state.staked_status = StakedStatus::Unstaking(UnstakingState {
                    started_at_block: now,
                    is_active: true,
                    blocks_remaining_in_active_period_for_unstaking: unstaking_period,
                });

                Ok(())
            }
            _ => Err(InitiateUnstakingError::UnstakingError(
                UnstakingError::NotStaked,
            )),
        }
    }

    fn pause_unstaking(&mut self) -> Result<(), PauseUnstakingError> {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => match staked_state.staked_status {
                StakedStatus::Unstaking(ref mut unstaking_state) => {
                    if unstaking_state.is_active {
                        unstaking_state.is_active = false;
                        Ok(())
                    } else {
                        Err(PauseUnstakingError::AlreadyPaused)
                    }
                }
                _ => Err(PauseUnstakingError::NotUnstaking),
            },
            _ => Err(PauseUnstakingError::NotStaked),
        }
    }

    fn resume_unstaking(&mut self) -> Result<(), ResumeUnstakingError> {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => match staked_state.staked_status {
                StakedStatus::Unstaking(ref mut unstaking_state) => {
                    if !unstaking_state.is_active {
                        unstaking_state.is_active = true;
                        Ok(())
                    } else {
                        Err(ResumeUnstakingError::NotPaused)
                    }
                }
                _ => Err(ResumeUnstakingError::NotUnstaking),
            },
            _ => Err(ResumeUnstakingError::NotStaked),
        }
    }

    fn finalize_slashing(
        &mut self,
        minimum_balance: Balance,
    ) -> (bool, Vec<(SlashId, Balance, Balance)>) {
        match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => {
                // tick the slashing timer
                let did_update = staked_state.advance_slashing_timer();

                // finalize and apply slashes
                let slashed = staked_state.finalize_slashes(minimum_balance);

                (did_update, slashed)
            }
            _ => (false, Vec::new()),
        }
    }

    fn finalize_unstaking(&mut self) -> (bool, Option<Balance>) {
        let (did_update, unstaked) = match self.staking_status {
            StakingStatus::Staked(ref mut staked_state) => match staked_state.staked_status {
                StakedStatus::Unstaking(ref mut unstaking_state) => {
                    // if all slashes were processed and there are no more active slashes
                    // resume unstaking
                    if staked_state.ongoing_slashes.is_empty() {
                        unstaking_state.is_active = true;
                    }

                    // tick the unstaking timer
                    if unstaking_state.is_active
                        && unstaking_state.blocks_remaining_in_active_period_for_unstaking
                            > Zero::zero()
                    {
                        // tick the unstaking timer
                        unstaking_state.blocks_remaining_in_active_period_for_unstaking -=
                            One::one();
                    }

                    // finalize unstaking
                    if unstaking_state.blocks_remaining_in_active_period_for_unstaking
                        == Zero::zero()
                    {
                        (true, Some(staked_state.staked_amount))
                    } else {
                        (unstaking_state.is_active, None)
                    }
                }
                _ => (false, None),
            },
            _ => (false, None),
        };

        // if unstaking was finalized transition to NotStaked state
        if unstaked.is_some() {
            self.staking_status = StakingStatus::NotStaked;
        }

        (did_update, unstaked)
    }

    fn finalize_slashing_and_unstaking(
        &mut self,
        minimum_balance: Balance,
    ) -> (bool, Vec<(SlashId, Balance, Balance)>, Option<Balance>) {
        let (did_update_slashing_timers, slashed) = self.finalize_slashing(minimum_balance);

        let (did_update_unstaking_timer, unstaked) = self.finalize_unstaking();

        (
            did_update_slashing_timers || did_update_unstaking_timer,
            slashed,
            unstaked,
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SlashImmediateOutcome<Balance, NegativeImbalance> {
    pub caused_unstake: bool,
    pub actually_slashed: Balance,
    pub remaining_stake: Balance,
    pub remaining_imbalance: NegativeImbalance,
}

decl_storage! {
    trait Store for Module<T: Config> as StakePool {
        /// Maps identifiers to a stake.
        pub Stakes get(fn stakes): map hasher(blake2_128_concat)
            T::StakeId => Stake<T::BlockNumber, BalanceOf<T>, T::SlashId>;

        /// Identifier value for next stake, and count of total stakes created (not necessarily the number of current
        /// stakes in the Stakes map as stakes can be removed.)
        pub StakesCreated get(fn stakes_created): T::StakeId;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn on_finalize(_now: T::BlockNumber) {
            Self::finalize_slashing_and_unstaking();
        }
    }
}

impl<T: Config> Module<T> {
    /// The account ID of theis module which holds all the staked balance. (referred to as the stake pool)
    ///
    /// This actually does computation. If you need to keep using it, then make sure you cache the
    /// value and only call this once. Is it deterministic?
    pub fn stake_pool_account_id() -> T::AccountId {
        PalletId(T::StakePoolId::get()).into_account()
    }

    pub fn stake_pool_balance() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::stake_pool_account_id())
    }

    /// Adds a new Stake which is NotStaked, created at given block, into stakes map.
    pub fn create_stake() -> T::StakeId {
        let stake_id = Self::stakes_created();
        <StakesCreated<T>>::put(stake_id + One::one());

        <Stakes<T>>::insert(
            &stake_id,
            Stake::new(<frame_system::Pallet<T>>::block_number()),
        );

        stake_id
    }

    /// Given that stake with id exists in stakes and is NotStaked, remove from stakes.
    pub fn remove_stake(stake_id: &T::StakeId) -> Result<(), StakeActionError<StakingError>> {
        let stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        ensure!(
            stake.is_not_staked(),
            StakeActionError::Error(StakingError::AlreadyStaked)
        );

        <Stakes<T>>::remove(stake_id);

        Ok(())
    }

    /// Dry run to see if staking can be initiated for the specified stake id. This should
    /// be called before stake() to make sure staking is possible before withdrawing funds.
    pub fn ensure_can_stake(
        stake_id: &T::StakeId,
        value: BalanceOf<T>,
    ) -> Result<(), StakeActionError<StakingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake
            .start_staking(value, T::Currency::minimum_balance())
            .err()
            .map_or(Ok(()), |err| Err(StakeActionError::Error(err)))
    }

    /// Provided the stake exists and is in state NotStaked the value is transferred
    /// to the module's account, and the corresponding staked_balance is set to this amount in the new Staked state.
    /// On error, as the negative imbalance is not returned to the caller, it is the caller's responsibility to return the funds
    /// back to the source (by creating a new positive imbalance)
    pub fn stake(
        stake_id: &T::StakeId,
        imbalance: NegativeImbalance<T>,
    ) -> Result<(), StakeActionError<StakingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        let value = imbalance.peek();

        stake.start_staking(value, T::Currency::minimum_balance())?;

        <Stakes<T>>::insert(stake_id, stake);

        Self::deposit_funds_into_stake_pool(imbalance);

        Ok(())
    }

    pub fn stake_from_account(
        stake_id: &T::StakeId,
        source_account_id: &T::AccountId,
        value: BalanceOf<T>,
    ) -> Result<(), StakeActionError<StakingFromAccountError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.start_staking(value, T::Currency::minimum_balance())?;

        // Its important to only do the transfer as the last step to ensure starting staking was possible.
        Self::transfer_funds_from_account_into_stake_pool(source_account_id, value)?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(())
    }

    /// Moves funds from specified account into the module's account
    fn transfer_funds_from_account_into_stake_pool(
        source: &T::AccountId,
        value: BalanceOf<T>,
    ) -> Result<(), TransferFromAccountError> {
        // We don't use T::Currency::transfer() to prevent fees being incurred.
        let negative_imbalance = T::Currency::withdraw(
            source,
            value,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )
        .map_err(|_err| TransferFromAccountError::InsufficientBalance)?;

        Self::deposit_funds_into_stake_pool(negative_imbalance);
        Ok(())
    }

    fn deposit_funds_into_stake_pool(imbalance: NegativeImbalance<T>) {
        // move the negative imbalance into the stake pool
        T::Currency::resolve_creating(&Self::stake_pool_account_id(), imbalance);
    }

    /// Moves funds from the module's account into specified account. Should never fail if used internally.
    /// Will panic! if value exceeds balance in the pool.
    fn transfer_funds_from_pool_into_account(destination: &T::AccountId, value: BalanceOf<T>) {
        let imbalance = Self::withdraw_funds_from_stake_pool(value);
        T::Currency::resolve_creating(destination, imbalance);
    }

    /// Withdraws value from the pool and returns a NegativeImbalance.
    /// As long as it is only called internally when executing slashes and unstaking, it
    /// should never fail as the pool balance is always in sync with total amount at stake.
    fn withdraw_funds_from_stake_pool(value: BalanceOf<T>) -> NegativeImbalance<T> {
        // We don't use T::Currency::transfer() to prevent fees being incurred.
        T::Currency::withdraw(
            &Self::stake_pool_account_id(),
            value,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )
        .expect("pool had less than expected funds!")
    }

    /// Dry run to see if the state of stake allows for increasing stake. This should be called
    /// to make sure increasing stake is possible before withdrawing funds.
    pub fn ensure_can_increase_stake(
        stake_id: &T::StakeId,
        value: BalanceOf<T>,
    ) -> Result<(), StakeActionError<IncreasingStakeError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake
            .increase_stake(value)
            .err()
            .map_or(Ok(()), |err| Err(StakeActionError::Error(err)))
    }

    /// Provided the stake exists and is in state Staked.Normal, then the amount is transferred to the module's account,
    /// and the corresponding staked_amount is increased by the value. New value of staked_amount is returned.
    /// Caller should call check ensure_can_increase_stake() prior to avoid getting back an error. On error, as the negative imbalance
    /// is not returned to the caller, it is the caller's responsibility to return the funds back to the source (by creating a new positive imbalance)
    pub fn increase_stake(
        stake_id: &T::StakeId,
        imbalance: NegativeImbalance<T>,
    ) -> Result<BalanceOf<T>, StakeActionError<IncreasingStakeError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        let total_staked_amount = stake.increase_stake(imbalance.peek())?;
        <Stakes<T>>::insert(stake_id, stake);

        Self::deposit_funds_into_stake_pool(imbalance);

        Ok(total_staked_amount)
    }

    /// Provided the stake exists and is in state Staked.Normal, and the given source account covers the amount,
    /// then the amount is transferred to the module's account, and the corresponding staked_amount is increased
    /// by the amount. New value of staked_amount is returned.
    pub fn increase_stake_from_account(
        stake_id: &T::StakeId,
        source_account_id: &T::AccountId,
        value: BalanceOf<T>,
    ) -> Result<BalanceOf<T>, StakeActionError<IncreasingStakeFromAccountError>> {
        let mut stake = ensure_stake_exists!(
            T,
            stake_id,
            <StakeActionError<IncreasingStakeFromAccountError>>::StakeNotFound
        )?;

        let total_staked_amount = stake.increase_stake(value)?;

        Self::transfer_funds_from_account_into_stake_pool(&source_account_id, value)?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(total_staked_amount)
    }

    pub fn ensure_can_decrease_stake(
        stake_id: &T::StakeId,
        value: BalanceOf<T>,
    ) -> Result<(), StakeActionError<DecreasingStakeError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake
            .decrease_stake(value, T::Currency::minimum_balance())
            .err()
            .map_or(Ok(()), |err| Err(StakeActionError::Error(err)))
    }

    pub fn decrease_stake(
        stake_id: &T::StakeId,
        value: BalanceOf<T>,
    ) -> Result<(BalanceOf<T>, NegativeImbalance<T>), StakeActionError<DecreasingStakeError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        let (deduct_from_pool, staked_amount) =
            stake.decrease_stake(value, T::Currency::minimum_balance())?;

        <Stakes<T>>::insert(stake_id, stake);

        let imbalance = Self::withdraw_funds_from_stake_pool(deduct_from_pool);

        Ok((staked_amount, imbalance))
    }

    /// Provided the stake exists and is in state Staked.Normal, and the given stake holds at least the value,
    /// then the value is transferred from the module's account to the destination_account, and the corresponding
    /// staked_amount is decreased by the value. New value of staked_amount is returned.
    pub fn decrease_stake_to_account(
        stake_id: &T::StakeId,
        destination_account_id: &T::AccountId,
        value: BalanceOf<T>,
    ) -> Result<BalanceOf<T>, StakeActionError<DecreasingStakeError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        let (deduct_from_pool, staked_amount) =
            stake.decrease_stake(value, T::Currency::minimum_balance())?;

        <Stakes<T>>::insert(stake_id, stake);

        Self::transfer_funds_from_pool_into_account(&destination_account_id, deduct_from_pool);

        Ok(staked_amount)
    }

    /// Slashes a stake with immediate effect, returns the outcome of the slashing.
    /// Can optionally specify if slashing can result in immediate unstaking if staked amount
    /// after slashing goes to zero.
    pub fn slash_immediate(
        stake_id: &T::StakeId,
        slash_amount: BalanceOf<T>,
        unstake_on_zero_staked: bool,
    ) -> Result<
        SlashImmediateOutcome<BalanceOf<T>, NegativeImbalance<T>>,
        StakeActionError<ImmediateSlashingError>,
    > {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        // Get amount at stake before slashing to be used in unstaked event trigger
        let staked_amount_before_slash = ensure_staked_amount!(
            stake,
            StakeActionError::Error(ImmediateSlashingError::NotStaked)
        )?;

        let (actually_slashed, remaining_stake) =
            stake.slash_immediate(slash_amount, T::Currency::minimum_balance())?;

        let caused_unstake = unstake_on_zero_staked && remaining_stake == BalanceOf::<T>::zero();

        if caused_unstake {
            stake.staking_status = StakingStatus::NotStaked;
        }

        // Update state before calling handlers!
        <Stakes<T>>::insert(stake_id, stake);

        // Remove the slashed amount from the pool
        let slashed_imbalance = Self::withdraw_funds_from_stake_pool(actually_slashed);

        // Notify slashing event handler before unstaked handler.
        let remaining_imbalance_after_slash_handler = T::StakingEventsHandler::slashed(
            stake_id,
            None,
            actually_slashed,
            remaining_stake,
            slashed_imbalance,
        );

        let remaining_imbalance = if caused_unstake {
            // Notify unstaked handler with any remaining unused imbalance
            // from the slashing event handler
            T::StakingEventsHandler::unstaked(
                &stake_id,
                staked_amount_before_slash,
                remaining_imbalance_after_slash_handler,
            )
        } else {
            remaining_imbalance_after_slash_handler
        };

        Ok(SlashImmediateOutcome {
            caused_unstake,
            actually_slashed,
            remaining_stake,
            remaining_imbalance,
        })
    }

    /// Initiate a new slashing of a staked stake. Slashing begins at next block.
    pub fn initiate_slashing(
        stake_id: &T::StakeId,
        slash_amount: BalanceOf<T>,
        slash_period: T::BlockNumber,
    ) -> Result<T::SlashId, StakeActionError<InitiateSlashingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        let slash_id = stake.initiate_slashing(
            slash_amount,
            slash_period,
            <frame_system::Pallet<T>>::block_number(),
        )?;

        <Stakes<T>>::insert(stake_id, stake);
        Ok(slash_id)
    }

    /// Pause an ongoing slashing
    pub fn pause_slashing(
        stake_id: &T::StakeId,
        slash_id: &T::SlashId,
    ) -> Result<(), StakeActionError<PauseSlashingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.pause_slashing(slash_id)?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(())
    }

    /// Resume a currently paused ongoing slashing.
    pub fn resume_slashing(
        stake_id: &T::StakeId,
        slash_id: &T::SlashId,
    ) -> Result<(), StakeActionError<ResumeSlashingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.resume_slashing(slash_id)?;

        <Stakes<T>>::insert(stake_id, stake);
        Ok(())
    }

    /// Cancel an ongoing slashing (regardless of whether its active or paused).
    pub fn cancel_slashing(
        stake_id: &T::StakeId,
        slash_id: &T::SlashId,
    ) -> Result<(), StakeActionError<CancelSlashingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.cancel_slashing(slash_id)?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(())
    }

    /// Initiate unstaking of a Staked stake.
    pub fn initiate_unstaking(
        stake_id: &T::StakeId,
        unstaking_period: Option<T::BlockNumber>,
    ) -> Result<(), StakeActionError<InitiateUnstakingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        if let Some(unstaking_period) = unstaking_period {
            stake
                .initiate_unstaking(unstaking_period, <frame_system::Pallet<T>>::block_number())?;
            <Stakes<T>>::insert(stake_id, stake);
        } else {
            let staked_amount = stake.unstake()?;
            <Stakes<T>>::insert(stake_id, stake);

            let imbalance = Self::withdraw_funds_from_stake_pool(staked_amount);
            let _ = T::StakingEventsHandler::unstaked(stake_id, staked_amount, imbalance);
        }

        Ok(())
    }

    /// Pause an ongoing Unstaking.
    pub fn pause_unstaking(
        stake_id: &T::StakeId,
    ) -> Result<(), StakeActionError<PauseUnstakingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.pause_unstaking()?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(())
    }

    /// Resume a currently paused ongoing unstaking.
    pub fn resume_unstaking(
        stake_id: &T::StakeId,
    ) -> Result<(), StakeActionError<ResumeUnstakingError>> {
        let mut stake = ensure_stake_exists!(T, stake_id, StakeActionError::StakeNotFound)?;

        stake.resume_unstaking()?;

        <Stakes<T>>::insert(stake_id, stake);

        Ok(())
    }

    /// Handle timers for finalizing unstaking and slashing.
    /// Finalised unstaking results in the staked_balance in the given stake to removed from the pool, the corresponding
    /// imbalance is provided to the unstaked() hook in the StakingEventsHandler.
    /// Finalised slashing results in the staked_balance in the given stake being correspondingly reduced, and the imbalance
    /// is provided to the slashed() hook in the StakingEventsHandler.
    fn finalize_slashing_and_unstaking() {
        for (stake_id, ref mut stake) in <Stakes<T>>::iter() {
            let (updated, slashed, unstaked) =
                stake.finalize_slashing_and_unstaking(T::Currency::minimum_balance());

            // update the state before making external calls to StakingEventsHandler
            if updated {
                <Stakes<T>>::insert(stake_id, stake)
            }

            for (slash_id, slashed_amount, staked_amount) in slashed.into_iter() {
                // remove the slashed amount from the pool
                let imbalance = Self::withdraw_funds_from_stake_pool(slashed_amount);

                let _ = T::StakingEventsHandler::slashed(
                    &stake_id,
                    Some(slash_id),
                    slashed_amount,
                    staked_amount,
                    imbalance,
                );
            }

            if let Some(staked_amount) = unstaked {
                // remove the unstaked amount from the pool
                let imbalance = Self::withdraw_funds_from_stake_pool(staked_amount);

                let _ = T::StakingEventsHandler::unstaked(&stake_id, staked_amount, imbalance);
            }
        }
    }
}
