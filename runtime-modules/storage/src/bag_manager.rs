#![warn(missing_docs)]

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::{ensure, StorageMap};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::marker::PhantomData;

use crate::{BagId, BalanceOf, DataObject, DynamicBag, Error, Module, StaticBag, Trait};

// Static and dynamic bags abstraction.
pub(crate) struct BagManager<T> {
    trait_marker: PhantomData<T>,
}

impl<T: Trait> BagManager<T> {
    // Accept data objects for a bag.
    pub(crate) fn accept_data_objects(bag_id: &BagId<T>, data_object_id: &T::DataObjectId) {
        Self::mutate(
            &bag_id,
            |bag| {
                let data_object = bag.objects.get_mut(data_object_id);

                if let Some(data_object) = data_object {
                    data_object.accepted = true;
                }
            },
            |bag| {
                let data_object = bag.objects.get_mut(data_object_id);

                if let Some(data_object) = data_object {
                    data_object.accepted = true;
                }
            },
        );
    }

    // Delete data object for a bag.
    pub(crate) fn delete_data_object(bag_id: &BagId<T>, data_object_id: &T::DataObjectId) {
        Self::mutate(
            &bag_id,
            |bag| {
                bag.objects.remove(data_object_id);
            },
            |bag| {
                bag.objects.remove(data_object_id);
            },
        );
    }

    // Adds several data objects to bag.
    pub(crate) fn append_data_objects(
        bag_id: &BagId<T>,
        data_objects: &BTreeMap<T::DataObjectId, DataObject<BalanceOf<T>>>,
    ) {
        Self::mutate(
            &bag_id,
            |bag| {
                bag.objects.append(&mut data_objects.clone());
            },
            |bag| {
                bag.objects.append(&mut data_objects.clone());
            },
        );
    }

    // Insert a single data object to bag.
    pub(crate) fn insert_data_object(
        bag_id: &BagId<T>,
        data_object_id: T::DataObjectId,
        data_object: &DataObject<BalanceOf<T>>,
    ) {
        Self::mutate(
            &bag_id,
            |bag| {
                bag.objects.insert(data_object_id, data_object.clone());
            },
            |bag| {
                bag.objects.insert(data_object_id, data_object.clone());
            },
        );
    }
    // Move data objects between bags.
    pub(crate) fn move_data_objects(
        src_bag_id: &BagId<T>,
        dest_bag_id: &BagId<T>,
        object_ids: &BTreeSet<T::DataObjectId>,
    ) {
        Self::mutate(
            &src_bag_id,
            |bag| {
                for object_id in object_ids.iter() {
                    let data_object = bag.objects.remove(object_id);

                    if let Some(data_object) = data_object {
                        Self::insert_data_object(dest_bag_id, *object_id, &data_object);
                    }
                }
            },
            |bag| {
                for object_id in object_ids.iter() {
                    let data_object = bag.objects.remove(object_id);

                    if let Some(data_object) = data_object {
                        Self::insert_data_object(dest_bag_id, *object_id, &data_object);
                    }
                }
            },
        );
    }

    // Add storage buckets to a bag.
    pub(crate) fn add_storage_buckets(bag_id: &BagId<T>, buckets: BTreeSet<T::StorageBucketId>) {
        Self::mutate(
            &bag_id,
            |bag| {
                bag.stored_by.append(&mut buckets.clone());
            },
            |bag| {
                bag.stored_by.append(&mut buckets.clone());
            },
        );
    }

    // Remove storage buckets from a bag.
    pub(crate) fn remove_storage_buckets(bag_id: &BagId<T>, buckets: BTreeSet<T::StorageBucketId>) {
        Self::mutate(
            &bag_id,
            |bag| {
                for bucket_id in buckets.iter() {
                    bag.stored_by.remove(bucket_id);
                }
            },
            |bag| {
                for bucket_id in buckets.iter() {
                    bag.stored_by.remove(bucket_id);
                }
            },
        );
    }

    // Check the bag existence. Static bags always exist.
    pub(crate) fn ensure_bag_exists(bag_id: &BagId<T>) -> DispatchResult {
        // Static bags could be auto-created on absence.
        // We must check only dynamic bags.

        if let BagId::<T>::DynamicBag(dynamic_bag_id) = bag_id {
            ensure!(
                <crate::DynamicBags<T>>::contains_key(dynamic_bag_id),
                Error::<T>::DynamicBagDoesntExist
            );
        }

        Ok(())
    }

    // Check the data object existence inside a bag.
    pub(crate) fn ensure_data_object_existence(
        bag_id: &BagId<T>,
        data_object_id: &T::DataObjectId,
    ) -> Result<DataObject<BalanceOf<T>>, DispatchError> {
        Self::query(
            bag_id,
            |bag| bag.objects.get(data_object_id).cloned(),
            |bag| bag.objects.get(data_object_id).cloned(),
        )
        .ok_or_else(|| Error::<T>::DataObjectDoesntExist.into())
    }

    // Check the storage bucket binding for a bag.
    pub(crate) fn ensure_storage_bucket_bound(
        bag_id: &BagId<T>,
        storage_bucket_id: &T::StorageBucketId,
    ) -> DispatchResult {
        let storage_bucket_bound = Self::query(
            bag_id,
            |bag| bag.stored_by.contains(storage_bucket_id),
            |bag| bag.stored_by.contains(storage_bucket_id),
        );

        ensure!(
            storage_bucket_bound,
            Error::<T>::StorageBucketIsNotBoundToBag
        );

        Ok(())
    }

    // Gets data object number from the bag container.
    pub(crate) fn get_data_objects_number(bag_id: &BagId<T>) -> u64 {
        Self::query(
            bag_id,
            |bag| bag.objects_number(),
            |bag| bag.objects_number(),
        )
    }

    // Gets data objects total size from the bag container.
    pub(crate) fn get_data_objects_total_size(bag_id: &BagId<T>) -> u64 {
        Self::query(
            bag_id,
            |bag| bag.objects_total_size(),
            |bag| bag.objects_total_size(),
        )
    }

    // Gets storage bucket ID set from the bag container.
    pub(crate) fn get_storage_bucket_ids(bag_id: &BagId<T>) -> BTreeSet<T::StorageBucketId> {
        Self::query(
            bag_id,
            |bag| bag.stored_by.clone(),
            |bag| bag.stored_by.clone(),
        )
    }

    // Abstract bag query function. Accepts two closures that should have similar result type.
    fn query<
        Res,
        StaticBagQuery: Fn(&StaticBag<T>) -> Res,
        DynamicBagQuery: Fn(&DynamicBag<T>) -> Res,
    >(
        bag_id: &BagId<T>,
        static_bag_query: StaticBagQuery,
        dynamic_bag_query: DynamicBagQuery,
    ) -> Res {
        match bag_id {
            BagId::<T>::StaticBag(static_bag_id) => {
                let bag = Module::<T>::static_bag(&static_bag_id);

                static_bag_query(&bag)
            }
            BagId::<T>::DynamicBag(dynamic_bag_id) => {
                let bag = Module::<T>::dynamic_bag(dynamic_bag_id);

                dynamic_bag_query(&bag)
            }
        }
    }

    // Abstract bag mutation function. Accept a closure for each static and dynamic bag types.
    // Optional return value.
    fn mutate<
        Res,
        StaticBagMutation: Fn(&mut StaticBag<T>) -> Res,
        DynamicBagMutation: Fn(&mut DynamicBag<T>) -> Res,
    >(
        bag_id: &BagId<T>,
        static_bag_mutation: StaticBagMutation,
        dynamic_bag_mutation: DynamicBagMutation,
    ) {
        match bag_id {
            BagId::<T>::StaticBag(static_bag_id) => {
                <crate::StaticBags<T>>::mutate(static_bag_id, static_bag_mutation);
            }
            BagId::<T>::DynamicBag(dynamic_bag_id) => {
                <crate::DynamicBags<T>>::mutate(dynamic_bag_id, dynamic_bag_mutation);
            }
        }
    }
}
