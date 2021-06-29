use codec::{Decode, Encode};
use frame_support::ensure;
use sp_arithmetic::traits::{BaseArithmetic, Zero};

#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq)]
pub enum AdjustCapacityBy<Balance: Zero> {
    /// Set capacity of mint to specific value
    Setting(Balance),
    /// Add to the capacity of the mint
    Adding(Balance),
    /// Reduce capacity of the mint
    Reducing(Balance),
}

#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq)]
pub struct AdjustOnInterval<Balance: Zero, BlockNumber> {
    pub block_interval: BlockNumber,
    pub adjustment_type: AdjustCapacityBy<Balance>,
}

#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq)]
pub struct NextAdjustment<Balance: Zero, BlockNumber> {
    pub adjustment: AdjustOnInterval<Balance, BlockNumber>,
    pub at_block: BlockNumber,
}

#[derive(Encode, Decode, Default, Copy, Clone)]
// Note we don't use TokenMint<T: Config> it breaks the Default derivation macro with error T doesn't impl Default
// Which requires manually implementing Default trait.
// We want Default trait on TokenMint so we can use it as value in StorageMap without needing to wrap it in an Option
pub struct Mint<Balance, BlockNumber>
where
    Balance: Copy + BaseArithmetic + Zero,
    BlockNumber: Copy + BaseArithmetic,
{
    capacity: Balance,

    // Whether there is an upcoming block where an adjustment to the mint will be made
    // When this is not set, the mint is effectively paused.
    next_adjustment: Option<NextAdjustment<Balance, BlockNumber>>,

    created_at: BlockNumber,

    total_minted: Balance,
}

#[derive(PartialEq, Eq, Debug)]
pub enum MintingError {
    NotEnoughCapacity,
}

impl<Balance, BlockNumber> Mint<Balance, BlockNumber>
where
    Balance: Copy + BaseArithmetic + Zero,
    BlockNumber: Copy + BaseArithmetic,
{
    pub fn new(
        initial_capacity: Balance,
        next_adjustment: Option<NextAdjustment<Balance, BlockNumber>>,
        now: BlockNumber,
    ) -> Self {
        Mint {
            capacity: initial_capacity,
            created_at: now,
            total_minted: Zero::zero(),
            next_adjustment,
        }
    }

    pub fn mint_tokens(&mut self, requested_amount: Balance) -> Result<(), MintingError> {
        ensure!(
            self.capacity >= requested_amount,
            MintingError::NotEnoughCapacity
        );
        self.capacity -= requested_amount;
        self.total_minted += requested_amount;
        Ok(())
    }

    pub fn set_capacity(&mut self, new_capacity: Balance) {
        self.capacity = new_capacity;
    }

    pub fn capacity(&self) -> Balance {
        self.capacity
    }

    pub fn can_mint(&self, amount: Balance) -> bool {
        self.capacity >= amount
    }

    pub fn created_at(&self) -> BlockNumber {
        self.created_at
    }

    pub fn total_minted(&self) -> Balance {
        self.total_minted
    }

    pub fn transfer_capacity_to(
        &mut self,
        destination: &mut Self,
        capacity_to_transfer: Balance,
    ) -> Result<(), MintingError> {
        ensure!(
            self.capacity >= capacity_to_transfer,
            MintingError::NotEnoughCapacity
        );
        self.capacity -= capacity_to_transfer;
        destination.capacity += capacity_to_transfer;
        Ok(())
    }

    pub fn next_adjustment(&self) -> Option<NextAdjustment<Balance, BlockNumber>> {
        self.next_adjustment
    }

    pub fn maybe_do_capacity_adjustment(&mut self, now: BlockNumber) -> bool {
        self.next_adjustment.map_or(false, |next_adjustment| {
            if now != next_adjustment.at_block {
                false
            } else {
                // update mint capacity
                self.capacity = Self::adjusted_capacity(
                    self.capacity,
                    next_adjustment.adjustment.adjustment_type,
                );

                // set next adjustment
                self.next_adjustment = Some(NextAdjustment {
                    adjustment: next_adjustment.adjustment,
                    at_block: now + next_adjustment.adjustment.block_interval,
                });

                true
            }
        })
    }

    fn adjusted_capacity(capacity: Balance, adjustment_type: AdjustCapacityBy<Balance>) -> Balance {
        match adjustment_type {
            AdjustCapacityBy::Adding(amount) => capacity + amount,
            AdjustCapacityBy::Setting(amount) => amount,
            AdjustCapacityBy::Reducing(amount) => {
                if amount > capacity {
                    Zero::zero()
                } else {
                    capacity - amount
                }
            }
        }
    }
}
