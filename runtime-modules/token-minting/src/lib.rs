// Clippy linter warning
#![allow(clippy::type_complexity)]
// disable it because of possible frontend API break
// TODO: remove post-Constaninople
#![allow(clippy::collapsible_match)]
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::storage::IterableStorageMap;
use frame_support::traits::Currency;
use frame_support::{decl_module, decl_storage, ensure, Parameter};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{MaybeSerialize, Member};

mod mint;
mod mock;
mod tests;

pub use mint::*;

pub trait Config: frame_system::Config {
    /// The currency to mint.
    type Currency: Currency<Self::AccountId>;

    /// The type used as a mint identifier.
    type MintId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;
}

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(PartialEq, Eq, Debug)]
pub enum GeneralError {
    MintNotFound,
    NextAdjustmentInPast,
}

/// Errors that can arise from attempt to mint and transfer tokens from a mint to
/// an account.
#[derive(PartialEq, Eq, Debug)]
pub enum TransferError {
    MintNotFound,
    NotEnoughCapacity,
}

/// Errors that can arise from attempt to transfer capacity between mints.
#[derive(PartialEq, Eq, Debug)]
pub enum CapacityTransferError {
    SourceMintNotFound,
    DestinationMintNotFound,
    NotEnoughCapacity,
}

impl From<MintingError> for CapacityTransferError {
    fn from(err: MintingError) -> CapacityTransferError {
        match err {
            MintingError::NotEnoughCapacity => CapacityTransferError::NotEnoughCapacity,
        }
    }
}

impl From<MintingError> for TransferError {
    fn from(err: MintingError) -> TransferError {
        match err {
            MintingError::NotEnoughCapacity => TransferError::NotEnoughCapacity,
        }
    }
}

impl From<GeneralError> for &'static str {
    fn from(err: GeneralError) -> &'static str {
        match err {
            GeneralError::MintNotFound => "MintNotFound",
            GeneralError::NextAdjustmentInPast => "NextAdjustmentInPast",
        }
    }
}

impl From<TransferError> for &'static str {
    fn from(err: TransferError) -> &'static str {
        match err {
            TransferError::MintNotFound => "MintNotFound",
            TransferError::NotEnoughCapacity => "NotEnoughCapacity",
        }
    }
}

#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Adjustment<Balance: Zero, BlockNumber> {
    // First adjustment will be after AdjustOnInterval.block_interval
    Interval(AdjustOnInterval<Balance, BlockNumber>),
    // First Adjustment will be at absolute blocknumber
    IntervalAfterFirstAdjustmentAbsolute(AdjustOnInterval<Balance, BlockNumber>, BlockNumber),
    // First Adjustment will be after a specified number of blocks
    IntervalAfterFirstAdjustmentRelative(AdjustOnInterval<Balance, BlockNumber>, BlockNumber),
}

decl_storage! {
    trait Store for Module<T: Config> as TokenMint {
        /// Mints
        pub Mints get(fn mints) : map hasher(blake2_128_concat) T::MintId => Mint<BalanceOf<T>, T::BlockNumber>;

        /// The number of mints created.
        pub MintsCreated get(fn mints_created): T::MintId;
    }
}
// pub Account: map hasher(blake2_128_concat) T::AccountId => AccountData<T::Balance>;
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn on_finalize(now: T::BlockNumber) {
            Self::update_mints(now);
        }
    }
}

impl<T: Config> Module<T> {
    fn update_mints(now: T::BlockNumber) {
        // Are we reading value from storage twice?
        for (mint_id, ref mut mint) in <Mints<T>>::iter() {
            if mint.maybe_do_capacity_adjustment(now) {
                <Mints<T>>::insert(&mint_id, mint);
            }
        }
    }

    /// Adds a new mint with given settings to mints, and returns new MintId.
    pub fn add_mint(
        initial_capacity: BalanceOf<T>,
        adjustment: Option<Adjustment<BalanceOf<T>, T::BlockNumber>>,
    ) -> Result<T::MintId, GeneralError> {
        let now = <frame_system::Pallet<T>>::block_number();

        // Ensure the next adjustment if set, is in the future
        if let Some(adjustment) = adjustment {
            if let Adjustment::IntervalAfterFirstAdjustmentAbsolute(_, first_adjustment_in) =
                adjustment
            {
                ensure!(
                    first_adjustment_in > now,
                    GeneralError::NextAdjustmentInPast
                );
            }
        }

        // Determine next adjutment
        let next_adjustment = adjustment.map(|adjustment| match adjustment {
            Adjustment::Interval(adjust_on_interval) => NextAdjustment {
                at_block: now + adjust_on_interval.block_interval,
                adjustment: adjust_on_interval,
            },
            Adjustment::IntervalAfterFirstAdjustmentAbsolute(
                adjust_on_interval,
                first_adjustment_at,
            ) => NextAdjustment {
                adjustment: adjust_on_interval,
                at_block: first_adjustment_at,
            },
            Adjustment::IntervalAfterFirstAdjustmentRelative(
                adjust_on_interval,
                first_adjustment_after,
            ) => NextAdjustment {
                adjustment: adjust_on_interval,
                at_block: now + first_adjustment_after,
            },
        });

        // get next mint_id and increment total number of mints created
        let mint_id = Self::mints_created();
        <MintsCreated<T>>::put(mint_id + One::one());

        <Mints<T>>::insert(mint_id, Mint::new(initial_capacity, next_adjustment, now));

        Ok(mint_id)
    }

    /// Removes a mint. Passing a non existent mint has no side effects.
    pub fn remove_mint(mint_id: T::MintId) {
        <Mints<T>>::remove(&mint_id);
    }

    /// Tries to transfer exact requested amount from mint to a recipient account id.
    /// Returns error if amount exceeds mint capacity or the specified mint doesn't exist.
    /// Transfering amount of zero has no side effects. Return nothing on success.
    pub fn transfer_tokens(
        mint_id: T::MintId,
        requested_amount: BalanceOf<T>,
        recipient: &T::AccountId,
    ) -> Result<(), TransferError> {
        if requested_amount == Zero::zero() {
            return Ok(());
        }

        ensure!(
            <Mints<T>>::contains_key(&mint_id),
            TransferError::MintNotFound
        );

        let mut mint = Self::mints(&mint_id);

        // Try minting
        mint.mint_tokens(requested_amount)?;

        <Mints<T>>::insert(&mint_id, mint);

        // Deposit into recipient account
        T::Currency::deposit_creating(recipient, requested_amount);

        Ok(())
    }

    /// Provided mint exists, sets its capacity to specied value, return error otherwise.
    pub fn set_mint_capacity(
        mint_id: T::MintId,
        capacity: BalanceOf<T>,
    ) -> Result<(), GeneralError> {
        ensure!(
            <Mints<T>>::contains_key(&mint_id),
            GeneralError::MintNotFound
        );

        <Mints<T>>::mutate(&mint_id, |mint| {
            mint.set_capacity(capacity);
        });

        Ok(())
    }

    /// Provided source and destination mints exist, will attempt to transfer capacity from the source mint
    /// to the destination mint. Will return errors on non-existence of
    /// mints or capacity_to_transfer exceeds the source mint's capacity.
    pub fn transfer_capacity(
        source: T::MintId,
        destination: T::MintId,
        capacity_to_transfer: BalanceOf<T>,
    ) -> Result<(), CapacityTransferError> {
        ensure!(
            <Mints<T>>::contains_key(&source),
            CapacityTransferError::SourceMintNotFound
        );
        ensure!(
            <Mints<T>>::contains_key(&destination),
            CapacityTransferError::DestinationMintNotFound
        );

        <Mints<T>>::mutate(&source, |source_mint| {
            <Mints<T>>::mutate(&destination, |destination_mint| {
                source_mint.transfer_capacity_to(destination_mint, capacity_to_transfer)
            })
        })?;

        Ok(())
    }

    /// Returns a mint's capacity if it exists, error otherwise.
    pub fn get_mint_capacity(mint_id: T::MintId) -> Result<BalanceOf<T>, GeneralError> {
        ensure!(
            <Mints<T>>::contains_key(&mint_id),
            GeneralError::MintNotFound
        );
        let mint = Self::mints(&mint_id);

        Ok(mint.capacity())
    }

    /// Returns a mint's adjustment policy if it exists, error otherwise.
    pub fn get_mint_next_adjustment(
        mint_id: T::MintId,
    ) -> Result<Option<NextAdjustment<BalanceOf<T>, T::BlockNumber>>, GeneralError> {
        ensure!(
            <Mints<T>>::contains_key(&mint_id),
            GeneralError::MintNotFound
        );

        let mint = Self::mints(&mint_id);

        Ok(mint.next_adjustment())
    }

    /// Returns true if a mint exists.
    pub fn mint_exists(mint_id: T::MintId) -> bool {
        <Mints<T>>::contains_key(&mint_id)
    }
}
