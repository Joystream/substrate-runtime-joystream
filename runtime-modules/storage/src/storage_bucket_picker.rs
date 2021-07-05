#![warn(missing_docs)]

use frame_support::traits::{Get, Randomness};
use sp_arithmetic::traits::{One, Zero};
use sp_runtime::SaturatedConversion;
use sp_std::cell::RefCell;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::marker::PhantomData;

use crate::{DynamicBagType, Module, Trait};

// Generates storage bucket IDs to assign to a new dynamic bag.
pub(crate) struct StorageBucketPicker<T> {
    trait_marker: PhantomData<T>,
}

impl<T: Trait> StorageBucketPicker<T> {
    // Selects storage bucket ID sets to assign to the storage bucket.
    // At first, it tries to generate random bucket IDs. If acquired random IDs number is not enough
    // it tries to get additional IDs starting from zero up to the total number of the possible IDs.
    // The function filters deleted buckets and disabled buckets (accepting_new_bags == false)
    // Total number of possible IDs is limited by the dynamic bag settings.
    // Returns an accumulated bucket ID set or an empty set.
    pub(crate) fn pick_storage_buckets(bag_type: DynamicBagType) -> BTreeSet<T::StorageBucketId> {
        let creation_policy = Module::<T>::get_dynamic_bag_creation_policy(bag_type);

        if creation_policy.no_storage_buckets_required() {
            return BTreeSet::new();
        }

        let required_bucket_num = creation_policy.number_of_storage_buckets as usize;

        // Storage IDs accumulator.
        let bucket_ids_cell = RefCell::new(BTreeSet::new());

        RandomStorageBucketIdIterator::<T>::new()
            .chain(SequentialStorageBucketIdIterator::<T>::new())
            .filter(Self::check_storage_bucket_is_valid_for_bag_assigning)
            .filter(|bucket_id| {
                let bucket_ids = bucket_ids_cell.borrow();

                // Skips the iteration on existing ID.
                !bucket_ids.contains(bucket_id)
            })
            .take(required_bucket_num)
            .for_each(|bucket_id| {
                let mut bucket_ids = bucket_ids_cell.borrow_mut();

                bucket_ids.insert(bucket_id);
            });

        bucket_ids_cell.into_inner()
    }

    // Verifies storage bucket ID (non-deleted and accepting new bags).
    pub(crate) fn check_storage_bucket_is_valid_for_bag_assigning(
        bucket_id: &T::StorageBucketId,
    ) -> bool {
        // Check bucket for existence (return false if not). Check `accepting_new_bags`.
        Module::<T>::ensure_storage_bucket_exists(bucket_id)
            .ok()
            .map(|bucket| bucket.accepting_new_bags)
            .unwrap_or(false)
    }
}

// Iterator for random storage bucket IDs. It uses Substrate Randomness trait
// (and possibly randomness_collective_flip pallet for implementation).
// Its maximum iterations are bounded.
pub(crate) struct RandomStorageBucketIdIterator<T: Trait> {
    // Trait marker.
    trait_marker: PhantomData<T>,

    // Current Iterator step number.
    current_iteration: u64,

    // Maximum allowed iteration number.
    max_iteration_number: u64,

    // Current seed for the randomness generator.
    current_seed: T::Hash,
}

impl<T: Trait> Iterator for RandomStorageBucketIdIterator<T> {
    type Item = T::StorageBucketId;

    fn next(&mut self) -> Option<Self::Item> {
        // Cannot create randomness in the initial block (Substrate error).
        if <frame_system::Module<T>>::block_number() == Zero::zero() {
            return None;
        }

        if self.current_iteration >= self.max_iteration_number {
            return None;
        }

        let random_storage_bucket_id = self.random_storage_bucket_id();

        self.current_iteration += 1;
        self.current_seed = T::Randomness::random(self.current_seed.as_ref());

        Some(random_storage_bucket_id)
    }
}

impl<T: Trait> RandomStorageBucketIdIterator<T> {
    // Generate random storage bucket ID using next_storage_bucket_id() as upper_bound.
    // Deleted storage bucket ID are included.
    fn random_storage_bucket_id(&self) -> T::StorageBucketId {
        let total_buckets_number = Module::<T>::next_storage_bucket_id();

        let random_bucket_id: T::StorageBucketId = self
            .random_index(total_buckets_number.saturated_into())
            .saturated_into();

        random_bucket_id
    }

    // Generate random number from zero to upper_bound (excluding).
    fn random_index(&self, upper_bound: u64) -> u64 {
        if upper_bound == 0 {
            return upper_bound;
        }

        let mut rand: u64 = 0;
        for offset in 0..8 {
            rand += (self.current_seed.as_ref()[offset] as u64) << offset;
        }
        rand % upper_bound
    }

    // Creates new iterator.
    pub(crate) fn new() -> Self {
        // Cannot create randomness in the initial block (Substrate error).
        let seed = if <frame_system::Module<T>>::block_number() == Zero::zero() {
            Default::default()
        } else {
            T::Randomness::random_seed()
        };

        Self {
            current_iteration: 0,
            max_iteration_number: T::MaxRandomIterationNumber::get(),
            trait_marker: PhantomData,
            current_seed: seed,
        }
    }
}

// Iterator for sequential storage bucket IDs. It starts from the first possible storage bucket ID
// (zero) and goes up to the last storage bucket IDs (next_storage_bucket_id - excluding).
pub(crate) struct SequentialStorageBucketIdIterator<T: Trait> {
    // Trait marker.
    trait_marker: PhantomData<T>,

    // Storage bucket ID for the current iteration.
    current_bucket_id: T::StorageBucketId,
}

impl<T: Trait> Iterator for SequentialStorageBucketIdIterator<T> {
    type Item = T::StorageBucketId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_bucket_id >= Module::<T>::next_storage_bucket_id() {
            return None;
        }

        let result = self.current_bucket_id;

        self.current_bucket_id += One::one();

        Some(result)
    }
}

impl<T: Trait> SequentialStorageBucketIdIterator<T> {
    // Creates new iterator.
    pub(crate) fn new() -> Self {
        Self {
            current_bucket_id: Zero::zero(),
            trait_marker: PhantomData,
        }
    }
}
