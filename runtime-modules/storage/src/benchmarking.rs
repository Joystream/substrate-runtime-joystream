#![cfg(feature = "runtime-benchmarks")]

//TODO: implement benchmarks after merging with the Olympia release.

use frame_benchmarking::benchmarks;
use frame_support::storage::StorageMap;
use frame_system::RawOrigin;
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::Vec;

use crate::{Call, Module, StorageBucketById, Trait};

benchmarks! {
    where_clause {
        where T: balances::Trait,
              T: membership::Trait,
              T: Trait,
    }
    _{ }

    create_storage_bucket {
        let account_id: T::AccountId = Default::default();


    }: _ (RawOrigin::Signed(account_id), None, false, 0, 0)
    verify {
        let storage_bucket_id: T::StorageBucketId = Default::default();

        assert!(StorageBucketById::<T>::contains_key(storage_bucket_id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mocks::{build_test_externalities, Test};
    use frame_support::assert_ok;

    #[ignore] // until enabling the benchmarking for the pallet
    #[test]
    fn create_storage_bucket() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_create_storage_bucket::<Test>());
        });
    }
}
