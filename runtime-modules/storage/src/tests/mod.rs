#![cfg(test)]

mod fixtures;
pub(crate) mod mocks;

use frame_support::dispatch::DispatchError;
use frame_support::traits::Currency;
use frame_support::StorageMap;
use frame_system::RawOrigin;
use sp_runtime::SaturatedConversion;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::iter::FromIterator;

use common::working_group::WorkingGroup;

use crate::{
    BagId, DataObject, DataObjectCreationParameters, DataObjectStorage, DynamicBagCreationPolicy,
    DynamicBagId, DynamicBagType, Error, ModuleAccount, RawEvent, StaticBagId,
    StorageBucketOperatorStatus, StorageTreasury, UploadParameters, Voucher,
};

use mocks::{
    build_test_externalities, Balances, DataObjectDeletionPrize,
    DefaultChannelDynamicBagCreationPolicy, DefaultMemberDynamicBagCreationPolicy,
    InitialStorageBucketsNumberForDynamicBag, MaxNumberOfDataObjectsPerBag,
    MaxRandomIterationNumber, MaxStorageBucketNumber, Storage, Test, ANOTHER_STORAGE_PROVIDER_ID,
    DEFAULT_MEMBER_ACCOUNT_ID, DEFAULT_MEMBER_ID, DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID,
    DEFAULT_STORAGE_PROVIDER_ID, WG_LEADER_ACCOUNT_ID,
};

use fixtures::*;

#[test]
fn create_storage_bucket_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        set_max_voucher_limits();

        let accepting_new_bags = true;
        let size_limit = 20;
        let objects_limit = 1;

        let invite_worker = None;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_accepting_new_bags(accepting_new_bags)
            .with_invite_worker(invite_worker)
            .with_size_limit(size_limit)
            .with_objects_limit(objects_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        let storage_bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(
            storage_bucket.operator_status,
            StorageBucketOperatorStatus::Missing
        );

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketCreated(
            bucket_id,
            invite_worker,
            accepting_new_bags,
            size_limit,
            objects_limit,
        ));
    });
}

#[test]
fn create_storage_bucket_fails_with_invalid_voucher_params() {
    build_test_externalities().execute_with(|| {
        let size_limit = 2000;
        let objects_limit = 10;

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_size_limit(size_limit)
            .call_and_assert(Err(Error::<Test>::VoucherMaxObjectSizeLimitExceeded.into()));

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_objects_limit(objects_limit)
            .call_and_assert(Err(
                Error::<Test>::VoucherMaxObjectNumberLimitExceeded.into()
            ));
    });
}

#[test]
fn create_storage_bucket_succeeded_with_invited_member() {
    build_test_externalities().execute_with(|| {
        let invited_worker_id = 10;
        let accepting_new_bags = true;
        let invite_worker = Some(invited_worker_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_accepting_new_bags(accepting_new_bags)
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let storage_bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(
            storage_bucket.operator_status,
            StorageBucketOperatorStatus::InvitedStorageWorker(invited_worker_id)
        );
    });
}

#[test]
fn create_storage_bucket_fails_with_non_signed_origin() {
    build_test_externalities().execute_with(|| {
        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn create_storage_bucket_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_account_id = 1;

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_account_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn create_storage_bucket_fails_with_exceeding_max_storage_bucket_limit() {
    build_test_externalities().execute_with(|| {
        let buckets_number = MaxStorageBucketNumber::get();
        create_storage_buckets(buckets_number);

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(
                Error::<Test>::MaxStorageBucketNumberLimitExceeded.into()
            ));
    });
}

#[test]
fn create_storage_bucket_fails_with_invalid_storage_provider_id() {
    build_test_externalities().execute_with(|| {
        let invalid_storage_provider_id = 155;

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(invalid_storage_provider_id))
            .call_and_assert(Err(Error::<Test>::StorageProviderOperatorDoesntExist.into()));
    });
}

#[test]
fn accept_storage_bucket_invitation_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketInvitationAccepted(
            bucket_id,
            storage_provider_id,
        ));
    });
}

#[test]
fn accept_storage_bucket_invitation_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_storage_provider_id = 1;

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(non_storage_provider_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn accept_storage_bucket_invitation_fails_with_non_existing_storage_bucket() {
    build_test_externalities().execute_with(|| {
        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn accept_storage_bucket_invitation_fails_with_non_invited_storage_provider() {
    build_test_externalities().execute_with(|| {
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(None)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::NoStorageBucketInvitation.into()));
    });
}

#[test]
fn accept_storage_bucket_invitation_fails_with_different_invited_storage_provider() {
    build_test_externalities().execute_with(|| {
        let different_storage_provider_id = ANOTHER_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(different_storage_provider_id))
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::DifferentStorageProviderInvited.into()));
    });
}

#[test]
fn accept_storage_bucket_invitation_fails_with_already_set_storage_provider() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(storage_provider_id))
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderAlreadySet.into()));
    });
}

#[test]
fn update_storage_buckets_for_bags_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let static_bag_id = StaticBagId::Council;
        let bag_id = BagId::<Test>::StaticBag(static_bag_id.clone());

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let add_buckets = BTreeSet::from_iter(vec![bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(add_buckets.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(static_bag_id);
        assert_eq!(bag.stored_by, add_buckets);

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketsUpdatedForBag(
            bag_id,
            add_buckets,
            BTreeSet::new(),
        ));
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_non_existing_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let add_buckets = BTreeSet::from_iter(vec![bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(add_buckets.clone())
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_non_accepting_new_bags_bucket() {
    build_test_externalities().execute_with(|| {
        let static_bag_id = StaticBagId::Council;
        let bag_id = BagId::<Test>::StaticBag(static_bag_id.clone());

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(None)
            .with_accepting_new_bags(false)
            .call_and_assert(Ok(()))
            .unwrap();

        let add_buckets = BTreeSet::from_iter(vec![bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(add_buckets.clone())
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntAcceptNewBags.into()));
    });
}

#[test]
fn update_storage_buckets_for_bags_succeeded_with_voucher_usage() {
    build_test_externalities().execute_with(|| {
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        set_update_storage_buckets_per_bag_limit();
        let old_bucket_id = create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let object_creation_list = create_single_data_object();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let objects_limit = 1;
        let size_limit = 100;

        let new_bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_objects_limit(objects_limit)
            .with_size_limit(size_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        let old_buckets = BTreeSet::from_iter(vec![old_bucket_id]);
        let new_buckets = BTreeSet::from_iter(vec![new_bucket_id]);

        let bag = Storage::static_bag(StaticBagId::Council);
        assert_eq!(bag.stored_by, old_buckets);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(new_buckets.clone())
            .with_remove_bucket_ids(old_buckets.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(StaticBagId::Council);
        assert_eq!(bag.stored_by, new_buckets);

        //// Check vouchers
        let old_bucket = Storage::storage_bucket_by_id(old_bucket_id);

        assert_eq!(old_bucket.voucher.objects_used, 0);
        assert_eq!(old_bucket.voucher.size_used, 0);

        let new_bucket = Storage::storage_bucket_by_id(new_bucket_id);
        assert_eq!(new_bucket.voucher.objects_used, 1);
        assert_eq!(new_bucket.voucher.size_used, object_creation_list[0].size);
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_exceeding_the_voucher_objects_number_limit() {
    build_test_externalities().execute_with(|| {
        set_update_storage_buckets_per_bag_limit();

        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let object_creation_list = create_single_data_object();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let new_bucket_objects_limit = 0;
        let new_bucket_size_limit = 100;
        let new_bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_objects_limit(new_bucket_objects_limit)
            .with_size_limit(new_bucket_size_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        let new_buckets = BTreeSet::from_iter(vec![new_bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(new_buckets.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectNumberLimitReached.into()
            ));
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_exceeding_the_voucher_objects_total_size_limit() {
    build_test_externalities().execute_with(|| {
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        set_update_storage_buckets_per_bag_limit();
        create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let object_creation_list = create_single_data_object();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let new_bucket_objects_limit = 1;
        let new_bucket_size_limit = 5;
        let new_bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_objects_limit(new_bucket_objects_limit)
            .with_size_limit(new_bucket_size_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        let new_buckets = BTreeSet::from_iter(vec![new_bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(new_buckets.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectSizeLimitReached.into()
            ));
    });
}

#[test]
fn update_storage_buckets_for_working_group_static_bags_succeeded() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let buckets = BTreeSet::from_iter(vec![bucket_id]);

        let static_bag_id = StaticBagId::WorkingGroup(WorkingGroup::Storage);
        let bag_id = BagId::<Test>::StaticBag(static_bag_id.clone());

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(buckets.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(&static_bag_id);
        assert_eq!(bag.stored_by, buckets);
    });
}

#[test]
fn update_storage_buckets_for_dynamic_bags_succeeded() {
    build_test_externalities().execute_with(|| {
        set_update_storage_buckets_per_bag_limit();

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let buckets = BTreeSet::from_iter(vec![bucket_id]);

        let member_id = 10;
        let dynamic_bag_id = DynamicBagId::<Test>::Member(member_id);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());
        create_dynamic_bag(&dynamic_bag_id);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_remove_bucket_ids(buckets.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::dynamic_bag(&dynamic_bag_id);
        assert_eq!(bag.stored_by, BTreeSet::new());
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_empty_params() {
    build_test_externalities().execute_with(|| {
        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketIdCollectionsAreEmpty.into()));
    });
}

#[test]
fn update_storage_buckets_for_bags_fails_with_non_existing_storage_buckets() {
    build_test_externalities().execute_with(|| {
        let invalid_bucket_id = 11000;
        let buckets = BTreeSet::from_iter(vec![invalid_bucket_id]);
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        // Invalid added bucket ID.
        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(buckets.clone())
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));

        // Invalid removed bucket ID.
        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_remove_bucket_ids(buckets.clone())
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn upload_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // check bag content
        let data_object_id = 0u64;
        let bag = Storage::static_bag(StaticBagId::Council);
        assert_eq!(
            bag.objects.iter().collect::<Vec<_>>(),
            vec![(
                &data_object_id,
                &DataObject {
                    size: upload_params.object_creation_list[0].size,
                    deletion_prize: DataObjectDeletionPrize::get(),
                    accepted: false,
                }
            )]
        );

        // check balances
        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance - DataObjectDeletionPrize::get()
        );
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            DataObjectDeletionPrize::get()
        );

        EventFixture::assert_last_crate_event(RawEvent::DataObjectdUploaded(
            vec![data_object_id],
            upload_params,
        ));
    });
}

#[test]
fn upload_succeeded_with_data_size_fee() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let data_size_fee = 100;

        UpdateDataObjectPerMegabyteFeeFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_fee(data_size_fee)
            .call_and_assert(Ok(()));

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // check balances
        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance - DataObjectDeletionPrize::get() - data_size_fee
        );
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            DataObjectDeletionPrize::get()
        );
    });
}

#[test]
fn upload_succeeded_with_active_storage_bucket_having_voucher() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id,
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        //// Check voucher

        let bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(bucket.voucher.objects_used, 1);
        assert_eq!(bucket.voucher.size_used, object_creation_list[0].size);
    });
}

#[test]
fn upload_fails_with_active_storage_bucket_with_voucher_object_number_limit_exceeding() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let objects_limit = 1;
        let size_limit = 100;

        create_storage_bucket_and_assign_to_bag(bag_id.clone(), None, objects_limit, size_limit);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id,
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // Check storage bucket voucher: object number limit.
        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectNumberLimitReached.into()
            ));
    });
}

#[test]
fn upload_fails_with_active_storage_bucket_with_voucher_object_size_limit_exceeding() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let objects_limit = 1;
        let size_limit = 1;

        create_storage_bucket_and_assign_to_bag(bag_id.clone(), None, objects_limit, size_limit);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id,
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        // Check storage bucket voucher: object size limit.
        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectSizeLimitReached.into()
            ));
    });
}

#[test]
fn upload_succeeded_with_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::DynamicBag(dynamic_bag_id.clone()),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // check bag content
        let data_object_id = 0u64;
        let bag = Storage::dynamic_bag(&dynamic_bag_id);

        assert_eq!(
            bag.objects.iter().collect::<Vec<_>>(),
            vec![(
                &data_object_id,
                &DataObject {
                    size: upload_params.object_creation_list[0].size,
                    deletion_prize: DataObjectDeletionPrize::get(),
                    accepted: false,
                }
            )]
        );

        assert_eq!(bag.deletion_prize, DataObjectDeletionPrize::get());
    });
}

#[test]
fn upload_fails_with_non_existent_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::DynamicBag(dynamic_bag_id.clone()),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn upload_succeeded_with_non_empty_bag() {
    build_test_externalities().execute_with(|| {
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, 1000);

        let upload_params1 = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_data_object_candidates(1, 2),
        };

        UploadFixture::default()
            .with_params(upload_params1.clone())
            .call_and_assert(Ok(()));

        let upload_params2 = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_data_object_candidates(3, 2),
        };

        UploadFixture::default()
            .with_params(upload_params2.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(StaticBagId::Council);
        assert_eq!(bag.objects.len(), 4);
    });
}

#[test]
fn upload_fails_with_empty_params_object() {
    build_test_externalities().execute_with(|| {
        let upload_params = UploadParameters::<Test>::default();

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::NoObjectsOnUpload.into()));
    });
}

#[test]
fn upload_fails_with_zero_object_size() {
    build_test_externalities().execute_with(|| {
        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: vec![DataObjectCreationParameters {
                ipfs_content_id: vec![1],
                size: 0,
            }],
        };

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::ZeroObjectSize.into()));
    });
}

#[test]
fn upload_fails_with_empty_object_cid() {
    build_test_externalities().execute_with(|| {
        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: vec![DataObjectCreationParameters {
                ipfs_content_id: Vec::new(),
                size: 220,
            }],
        };

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::EmptyContentId.into()));
    });
}

#[test]
fn upload_fails_with_max_data_object_size_exceeded() {
    build_test_externalities().execute_with(|| {
        let max_object_size = MaxNumberOfDataObjectsPerBag::get();
        let invalid_object_number: u8 = (max_object_size + 1).saturated_into();

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_data_object_candidates(1, invalid_object_number),
        };

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::DataObjectsPerBagLimitExceeded.into()));
    });
}

#[test]
fn upload_fails_with_insufficient_balance_for_deletion_prize() {
    build_test_externalities().execute_with(|| {
        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::InsufficientBalance.into()));
    });
}

#[test]
fn upload_fails_with_insufficient_balance_for_data_size_fee() {
    build_test_externalities().execute_with(|| {
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, DataObjectDeletionPrize::get());

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        // Check that balance is sufficient for the deletion prize.
        assert_eq!(Storage::can_upload_data_objects(&upload_params), Ok(()));

        let data_size_fee = 1000;

        UpdateDataObjectPerMegabyteFeeFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_fee(data_size_fee)
            .call_and_assert(Ok(()));

        UploadFixture::default()
            .with_params(upload_params)
            .call_and_assert(Err(Error::<Test>::InsufficientBalance.into()));
    });
}

#[test]
fn upload_failed_with_blocked_uploading() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        let new_blocking_status = true;
        UpdateUploadingBlockedStatusFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_status(new_blocking_status)
            .call_and_assert(Ok(()));

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Err(Error::<Test>::UploadingBlocked.into()));
    });
}

#[test]
fn upload_failed_with_blacklisted_data_object() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();
        let hash = object_creation_list[0].ipfs_content_id.clone();
        let add_hashes = BTreeSet::from_iter(vec![hash]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes)
            .call_and_assert(Ok(()));

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::StaticBag(StaticBagId::Council),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list,
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Err(Error::<Test>::DataObjectBlacklisted.into()));
    });
}

#[test]
fn set_storage_operator_metadata_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let metadata = b"http://localhost:4000".to_vec();

        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_metadata(metadata.clone())
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageOperatorMetadataSet(
            bucket_id,
            storage_provider_id,
            metadata,
        ));
    });
}

#[test]
fn set_storage_operator_metadata_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn set_storage_operator_metadata_fails_with_invalid_storage_bucket() {
    build_test_externalities().execute_with(|| {
        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn set_storage_operator_metadata_fails_with_invalid_storage_association() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        // Missing invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));

        // Not accepted invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));

        // Invitation accepted. Incorrect storage provider.
        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let incorrect_storage_provider_id = 888;
        SetStorageOperatorMetadataFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(incorrect_storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));
    });
}

#[test]
fn accept_pending_data_objects_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let objects_limit = 1;
        let size_limit = 100;
        set_max_voucher_limits();

        let static_bag_id = StaticBagId::Council;
        let bag_id = BagId::<Test>::StaticBag(static_bag_id.clone());

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .with_size_limit(size_limit)
            .with_objects_limit(objects_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let buckets = BTreeSet::from_iter(vec![bucket_id]);

        UpdateStorageBucketForBagsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_add_bucket_ids(buckets.clone())
            .call_and_assert(Ok(()));

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0; // just uploaded data object

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        let bag = Storage::static_bag(static_bag_id.clone());
        // Check `accepted` flag for the fist data object in the bag.
        assert_eq!(bag.objects.iter().collect::<Vec<_>>()[0].1.accepted, false);

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(static_bag_id);
        // Check `accepted` flag for the fist data object in the bag.
        assert_eq!(bag.objects.iter().collect::<Vec<_>>()[0].1.accepted, true);

        EventFixture::assert_last_crate_event(RawEvent::PendingDataObjectsAccepted(
            bucket_id,
            storage_provider_id,
            bag_id,
            data_object_ids,
        ));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_unrelated_storage_bucket() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let static_bag_id = StaticBagId::Council;
        let bag_id = BagId::<Test>::StaticBag(static_bag_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0;

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids)
            .call_and_assert(Err(Error::<Test>::StorageBucketIsNotBoundToBag.into()));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_non_existing_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());

        let data_object_id = 0;

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids)
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn accept_pending_data_objects_succeeded_with_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        set_max_voucher_limits();

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);
        let objects_limit = 1;
        let size_limit = 100;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .with_objects_limit(objects_limit)
            .with_size_limit(size_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());
        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0; // just uploaded data object

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids)
            .call_and_assert(Ok(()));

        let bag = Storage::dynamic_bag(&dynamic_bag_id);
        // Check `accepted` flag for the fist data object in the bag.
        assert_eq!(bag.objects.iter().collect::<Vec<_>>()[0].1.accepted, true);
    });
}

#[test]
fn accept_pending_data_objects_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_empty_params() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let objects_limit = 1;
        let size_limit = 100;
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_storage_bucket_and_assign_to_bag(
            bag_id.clone(),
            Some(storage_provider_id),
            objects_limit,
            size_limit,
        );

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .with_data_object_ids(BTreeSet::new())
            .call_and_assert(Err(Error::<Test>::DataObjectIdParamsAreEmpty.into()));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_non_existing_data_object() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let objects_limit = 1;
        let size_limit = 100;
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_storage_bucket_and_assign_to_bag(
            bag_id.clone(),
            Some(storage_provider_id),
            objects_limit,
            size_limit,
        );

        let data_object_id = 0;

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids)
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::DataObjectDoesntExist.into()));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_invalid_storage_provider() {
    build_test_externalities().execute_with(|| {
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));
    });
}

#[test]
fn accept_pending_data_objects_fails_with_non_existing_bucket_id() {
    build_test_externalities().execute_with(|| {
        AcceptPendingDataObjectsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn cancel_storage_bucket_operator_invite_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        CancelStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketInvitationCancelled(
            bucket_id,
        ));
    });
}

#[test]
fn cancel_storage_bucket_operator_invite_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_storage_provider_id = 1;

        CancelStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(non_storage_provider_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn cancel_storage_bucket_operator_invite_fails_with_non_existing_storage_bucket() {
    build_test_externalities().execute_with(|| {
        CancelStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn cancel_storage_bucket_operator_invite_fails_with_non_invited_storage_provider() {
    build_test_externalities().execute_with(|| {
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(None)
            .call_and_assert(Ok(()))
            .unwrap();

        CancelStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::NoStorageBucketInvitation.into()));
    });
}

#[test]
fn cancel_storage_bucket_operator_invite_fails_with_already_set_storage_provider() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(storage_provider_id))
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        CancelStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderAlreadySet.into()));
    });
}

#[test]
fn invite_storage_bucket_operator_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        InviteStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_operator_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketOperatorInvited(
            bucket_id,
            storage_provider_id,
        ));
    });
}

#[test]
fn invite_storage_bucket_operator_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        InviteStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn invite_storage_bucket_operator_fails_with_non_existing_storage_bucket() {
    build_test_externalities().execute_with(|| {
        InviteStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn invite_storage_bucket_operator_fails_with_non_missing_invitation() {
    build_test_externalities().execute_with(|| {
        let invited_worker_id = DEFAULT_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(invited_worker_id))
            .call_and_assert(Ok(()))
            .unwrap();

        InviteStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::InvitedStorageProvider.into()));
    });
}

#[test]
fn invite_storage_bucket_operator_fails_with_invalid_storage_provider_id() {
    build_test_externalities().execute_with(|| {
        let invalid_storage_provider_id = 155;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        InviteStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_operator_worker_id(invalid_storage_provider_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderOperatorDoesntExist.into()));
    });
}

#[test]
fn update_uploading_blocked_status_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let new_blocking_status = true;

        UpdateUploadingBlockedStatusFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_status(new_blocking_status)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::UploadingBlockStatusUpdated(
            new_blocking_status,
        ));
    });
}

#[test]
fn update_uploading_blocked_status_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateUploadingBlockedStatusFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn move_data_objects_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let src_dynamic_bag_id = DynamicBagId::<Test>::Member(1u64);
        let src_bag_id = BagId::<Test>::DynamicBag(src_dynamic_bag_id.clone());
        create_dynamic_bag(&src_dynamic_bag_id);

        let dest_dynamic_bag_id = DynamicBagId::<Test>::Member(2u64);
        let dest_bag_id = BagId::<Test>::DynamicBag(dest_dynamic_bag_id.clone());
        create_dynamic_bag(&dest_dynamic_bag_id);

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: src_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        // Pre-checks
        let src_bag = Storage::dynamic_bag(&src_dynamic_bag_id.clone());
        let dest_bag = Storage::dynamic_bag(&dest_dynamic_bag_id.clone());

        assert!(src_bag.objects.contains_key(&data_object_id));
        assert!(!dest_bag.objects.contains_key(&data_object_id));

        assert_eq!(src_bag.deletion_prize, DataObjectDeletionPrize::get());
        assert_eq!(dest_bag.deletion_prize, 0);

        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Ok(()));

        // Post-checks
        let src_bag = Storage::dynamic_bag(&src_dynamic_bag_id.clone());
        let dest_bag = Storage::dynamic_bag(&dest_dynamic_bag_id.clone());

        assert!(!src_bag.objects.contains_key(&data_object_id));
        assert!(dest_bag.objects.contains_key(&data_object_id));

        assert_eq!(src_bag.deletion_prize, 0);
        assert_eq!(dest_bag.deletion_prize, DataObjectDeletionPrize::get());

        EventFixture::assert_last_crate_event(RawEvent::DataObjectsMoved(
            src_bag_id,
            dest_bag_id,
            ids,
        ));
    });
}

#[test]
fn move_data_objects_fails_with_non_existing_dynamic_bags() {
    build_test_externalities().execute_with(|| {
        let src_dynamic_bag_id = DynamicBagId::<Test>::Member(1u64);
        let src_bag_id = BagId::<Test>::DynamicBag(src_dynamic_bag_id.clone());

        let dest_dynamic_bag_id = DynamicBagId::<Test>::Member(2u64);
        let dest_bag_id = BagId::<Test>::DynamicBag(dest_dynamic_bag_id.clone());

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        // Neigher src bag nor dest bag exist.
        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));

        create_dynamic_bag(&src_dynamic_bag_id);

        // Src bag exists, dest doesn't
        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn move_data_objects_succeeded_having_voucher() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let working_group = WorkingGroup::Storage;
        let src_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let dest_bag_id = BagId::<Test>::StaticBag(StaticBagId::WorkingGroup(working_group));

        let src_bucket_id = create_default_storage_bucket_and_assign_to_bag(src_bag_id.clone());
        let dest_bucket_id = create_default_storage_bucket_and_assign_to_bag(dest_bag_id.clone());

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id: src_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        let src_bucket = Storage::storage_bucket_by_id(src_bucket_id);
        let dest_bucket = Storage::storage_bucket_by_id(dest_bucket_id);
        assert_eq!(dest_bucket.voucher.objects_used, 0);
        assert_eq!(dest_bucket.voucher.size_used, 0);

        assert_eq!(src_bucket.voucher.objects_used, 1);
        assert_eq!(src_bucket.voucher.size_used, object_creation_list[0].size);

        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Ok(()));

        //// Check vouchers
        let src_bucket = Storage::storage_bucket_by_id(src_bucket_id);
        let dest_bucket = Storage::storage_bucket_by_id(dest_bucket_id);

        assert_eq!(src_bucket.voucher.objects_used, 0);
        assert_eq!(src_bucket.voucher.size_used, 0);

        assert_eq!(dest_bucket.voucher.objects_used, 1);
        assert_eq!(dest_bucket.voucher.size_used, object_creation_list[0].size);
    });
}

#[test]
fn move_data_objects_fails_with_exceeding_voucher_object_number_limit() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let working_group = WorkingGroup::Storage;
        let src_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let dest_bag_id = BagId::<Test>::StaticBag(StaticBagId::WorkingGroup(working_group));

        let src_objects_limit = 1;
        let src_size_limit = 100;

        let dest_objects_limit = 0;
        let dest_size_limit = 100;

        create_storage_bucket_and_assign_to_bag(
            src_bag_id.clone(),
            None,
            src_objects_limit,
            src_size_limit,
        );
        create_storage_bucket_and_assign_to_bag(
            dest_bag_id.clone(),
            None,
            dest_objects_limit,
            dest_size_limit,
        );

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id: src_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectNumberLimitReached.into()
            ));
    });
}

#[test]
fn move_data_objects_fails_with_exceeding_voucher_objects_size_limit() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let working_group = WorkingGroup::Storage;
        let src_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let dest_bag_id = BagId::<Test>::StaticBag(StaticBagId::WorkingGroup(working_group));

        let src_objects_limit = 1;
        let src_size_limit = 100;

        let dest_objects_limit = 1;
        let dest_size_limit = 1;

        create_storage_bucket_and_assign_to_bag(
            src_bag_id.clone(),
            None,
            src_objects_limit,
            src_size_limit,
        );
        create_storage_bucket_and_assign_to_bag(
            dest_bag_id.clone(),
            None,
            dest_objects_limit,
            dest_size_limit,
        );

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id: src_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id.clone())
            .with_dest_bag_id(dest_bag_id.clone())
            .with_data_object_ids(ids.clone())
            .call_and_assert(Err(
                Error::<Test>::StorageBucketObjectSizeLimitReached.into()
            ));
    });
}

#[test]
fn move_data_objects_fails_with_empty_data_collection() {
    build_test_externalities().execute_with(|| {
        let dest_bag_id =
            BagId::<Test>::StaticBag(StaticBagId::WorkingGroup(WorkingGroup::Storage));

        MoveDataObjectsFixture::default()
            .with_dest_bag_id(dest_bag_id)
            .call_and_assert(Err(Error::<Test>::DataObjectIdCollectionIsEmpty.into()));
    });
}

#[test]
fn move_data_objects_fails_with_non_existent_data() {
    build_test_externalities().execute_with(|| {
        let dest_bag_id =
            BagId::<Test>::StaticBag(StaticBagId::WorkingGroup(WorkingGroup::Storage));

        let data_object_id = 0u64;
        let ids = BTreeSet::from_iter(vec![data_object_id]);

        MoveDataObjectsFixture::default()
            .with_dest_bag_id(dest_bag_id)
            .with_data_object_ids(ids)
            .call_and_assert(Err(Error::<Test>::DataObjectDoesntExist.into()));
    });
}

#[test]
fn move_data_objects_fails_with_same_bag() {
    build_test_externalities().execute_with(|| {
        let src_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let dest_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        MoveDataObjectsFixture::default()
            .with_src_bag_id(src_bag_id)
            .with_dest_bag_id(dest_bag_id)
            .call_and_assert(Err(Error::<Test>::SourceAndDestinationBagsAreEqual.into()));
    });
}

#[test]
fn delete_data_objects_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());

        create_dynamic_bag(&dynamic_bag_id);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0; // just uploaded data object

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        // pre-checks
        let bag = Storage::dynamic_bag(&dynamic_bag_id);
        assert!(bag.objects.contains_key(&data_object_id));
        assert_eq!(bag.deletion_prize, DataObjectDeletionPrize::get());

        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance - DataObjectDeletionPrize::get()
        );
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            DataObjectDeletionPrize::get()
        );

        DeleteDataObjectsFixture::default()
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Ok(()));

        // post-checks
        let bag = Storage::dynamic_bag(&dynamic_bag_id);
        assert!(!bag.objects.contains_key(&data_object_id));
        assert_eq!(bag.deletion_prize, 0);

        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance
        );
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            0
        );

        EventFixture::assert_last_crate_event(RawEvent::DataObjectsDeleted(
            DEFAULT_MEMBER_ACCOUNT_ID,
            bag_id,
            data_object_ids,
        ));
    });
}

#[test]
fn delete_data_objects_fails_with_non_existent_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());

        let data_object_id = 0;

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        DeleteDataObjectsFixture::default()
            .with_bag_id(bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn delete_data_objects_fails_with_invalid_treasury_balance() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let council_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);
        let upload_params = UploadParameters::<Test> {
            bag_id: council_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0; // just uploaded data object

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        // Corrupt module balance.
        let _ = Balances::slash(
            &<StorageTreasury<Test>>::module_account_id(),
            <StorageTreasury<Test>>::usable_balance(),
        );

        DeleteDataObjectsFixture::default()
            .with_bag_id(council_bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Err(Error::<Test>::InsufficientTreasuryBalance.into()));
    });
}

#[test]
fn delete_data_objects_succeeded_with_voucher_usage() {
    build_test_externalities().execute_with(|| {
        let council_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_default_storage_bucket_and_assign_to_bag(council_bag_id.clone());

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id: council_bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        let data_object_id = 0; // just uploaded data object

        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        //// Pre-check voucher
        let bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(bucket.voucher.objects_used, 1);
        assert_eq!(bucket.voucher.size_used, object_creation_list[0].size);

        DeleteDataObjectsFixture::default()
            .with_bag_id(council_bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .call_and_assert(Ok(()));

        let bag = Storage::static_bag(StaticBagId::Council);
        assert!(!bag.objects.contains_key(&data_object_id));

        //// Post-check voucher
        let bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(bucket.voucher.objects_used, 0);
        assert_eq!(bucket.voucher.size_used, 0);
    });
}

#[test]
fn delete_data_objects_fails_with_empty_params() {
    build_test_externalities().execute_with(|| {
        DeleteDataObjectsFixture::default()
            .call_and_assert(Err(Error::<Test>::DataObjectIdParamsAreEmpty.into()));
    });
}

#[test]
fn delete_data_objects_fails_with_non_existing_data_object() {
    build_test_externalities().execute_with(|| {
        let council_bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let data_object_id = 0;
        let data_object_ids = BTreeSet::from_iter(vec![data_object_id]);

        DeleteDataObjectsFixture::default()
            .with_bag_id(council_bag_id.clone())
            .with_data_object_ids(data_object_ids.clone())
            .call_and_assert(Err(Error::<Test>::DataObjectDoesntExist.into()));
    });
}

#[test]
fn update_storage_bucket_status_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let new_status = true;
        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_new_status(new_status)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketStatusUpdated(
            bucket_id,
            storage_provider_id,
            new_status,
        ));
    });
}

#[test]
fn update_storage_bucket_status_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_storage_bucket_status_fails_with_invalid_storage_bucket() {
    build_test_externalities().execute_with(|| {
        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn update_storage_bucket_status_fails_with_invalid_storage_association() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        // Missing invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));

        // Not accepted invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));

        // Invitation accepted. Incorrect storage provider.
        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let incorrect_storage_provider_id = 888;
        UpdateStorageBucketStatusFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(incorrect_storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));
    });
}

#[test]
fn update_blacklist_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let cid1 = vec![1];
        let cid2 = vec![2];

        let add_hashes = BTreeSet::from_iter(vec![cid1.clone()]);
        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .call_and_assert(Ok(()));

        assert!(crate::Blacklist::contains_key(&cid1));
        assert_eq!(Storage::current_blacklist_size(), 1);

        let remove_hashes = BTreeSet::from_iter(vec![cid1.clone()]);
        let add_hashes = BTreeSet::from_iter(vec![cid2.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .with_remove_hashes(remove_hashes.clone())
            .call_and_assert(Ok(()));

        assert!(!crate::Blacklist::contains_key(&cid1));
        assert!(crate::Blacklist::contains_key(&cid2));
        assert_eq!(Storage::current_blacklist_size(), 1);

        EventFixture::assert_last_crate_event(RawEvent::UpdateBlacklist(remove_hashes, add_hashes));
    });
}

#[test]
fn update_blacklist_failed_with_exceeding_size_limit() {
    build_test_externalities().execute_with(|| {
        let cid1 = vec![1];
        let cid2 = vec![2];
        let cid3 = vec![3];

        let add_hashes = BTreeSet::from_iter(vec![cid1.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .call_and_assert(Ok(()));

        let remove_hashes = BTreeSet::from_iter(vec![cid1.clone()]);
        let add_hashes = BTreeSet::from_iter(vec![cid2.clone(), cid3.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .with_remove_hashes(remove_hashes.clone())
            .call_and_assert(Err(Error::<Test>::BlacklistSizeLimitExceeded.into()));

        assert!(crate::Blacklist::contains_key(&cid1));
        assert!(!crate::Blacklist::contains_key(&cid2));
        assert!(!crate::Blacklist::contains_key(&cid3));
    });
}

#[test]
fn update_blacklist_failed_with_exceeding_size_limit_with_non_existent_remove_hashes() {
    build_test_externalities().execute_with(|| {
        let cid1 = vec![1];
        let cid2 = vec![2];
        let cid3 = vec![3];

        let add_hashes = BTreeSet::from_iter(vec![cid1.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .call_and_assert(Ok(()));

        let remove_hashes = BTreeSet::from_iter(vec![cid3.clone()]);
        let add_hashes = BTreeSet::from_iter(vec![cid2.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .with_remove_hashes(remove_hashes.clone())
            .call_and_assert(Err(Error::<Test>::BlacklistSizeLimitExceeded.into()));

        assert!(crate::Blacklist::contains_key(&cid1));
        assert!(!crate::Blacklist::contains_key(&cid2));
        assert!(!crate::Blacklist::contains_key(&cid3));
    });
}

#[test]
fn update_blacklist_succeeds_with_existent_remove_hashes() {
    build_test_externalities().execute_with(|| {
        let cid1 = vec![1];

        let add_hashes = BTreeSet::from_iter(vec![cid1.clone()]);

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .call_and_assert(Ok(()));

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_add_hashes(add_hashes.clone())
            .call_and_assert(Ok(()));

        assert!(crate::Blacklist::contains_key(&cid1));
        assert_eq!(Storage::current_blacklist_size(), 1);
    });
}

#[test]
fn update_blacklist_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateBlacklistFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

fn create_default_storage_bucket_and_assign_to_bag(bag_id: BagId<Test>) -> u64 {
    let objects_limit = 1;
    let size_limit = 100;

    create_storage_bucket_and_assign_to_bag(bag_id, None, objects_limit, size_limit)
}

fn create_storage_bucket_and_assign_to_bag(
    bag_id: BagId<Test>,
    storage_provider_id: Option<u64>,
    objects_limit: u64,
    size_limit: u64,
) -> u64 {
    set_max_voucher_limits();

    let bucket_id = CreateStorageBucketFixture::default()
        .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
        .with_invite_worker(storage_provider_id)
        .with_objects_limit(objects_limit)
        .with_size_limit(size_limit)
        .call_and_assert(Ok(()))
        .unwrap();

    let buckets = BTreeSet::from_iter(vec![bucket_id]);

    UpdateStorageBucketForBagsFixture::default()
        .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
        .with_bag_id(bag_id.clone())
        .with_add_bucket_ids(buckets.clone())
        .call_and_assert(Ok(()));

    if let Some(storage_provider_id) = storage_provider_id {
        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));
    }

    bucket_id
}

#[test]
fn delete_dynamic_bags_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::DynamicBag(dynamic_bag_id.clone()),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // pre-check balances
        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance - DataObjectDeletionPrize::get()
        );
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            DataObjectDeletionPrize::get()
        );

        DeleteDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Ok(()));

        // post-check balances
        assert_eq!(
            Balances::usable_balance(&<StorageTreasury<Test>>::module_account_id()),
            0
        );
        assert_eq!(
            Balances::usable_balance(&DEFAULT_MEMBER_ACCOUNT_ID),
            initial_balance
        );

        EventFixture::assert_last_crate_event(RawEvent::DynamicBagDeleted(
            DEFAULT_MEMBER_ACCOUNT_ID,
            dynamic_bag_id,
        ));
    });
}

#[test]
fn delete_dynamic_bags_fails_with_non_existent_dynamic_bag() {
    build_test_externalities().execute_with(|| {
        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);

        DeleteDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Err(Error::<Test>::DynamicBagDoesntExist.into()));
    });
}

#[test]
fn delete_dynamic_bags_succeeded_having_voucher() {
    build_test_externalities().execute_with(|| {
        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());

        let bucket_id = create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let object_creation_list = create_single_data_object();

        let upload_params = UploadParameters::<Test> {
            bag_id,
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // Pre-checks for voucher
        let bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(bucket.voucher.size_used, object_creation_list[0].size);
        assert_eq!(bucket.voucher.objects_used, 1);

        DeleteDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Ok(()));

        // Post-checks for voucher
        let bucket = Storage::storage_bucket_by_id(bucket_id);

        assert_eq!(bucket.voucher.size_used, 0);
        assert_eq!(bucket.voucher.objects_used, 0);
    });
}

#[test]
fn delete_dynamic_bags_fails_with_insufficient_balance_for_deletion_prize() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::DynamicBag(dynamic_bag_id.clone()),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        // Corrupt module balance.
        let _ = Balances::slash(
            &<StorageTreasury<Test>>::module_account_id(),
            <StorageTreasury<Test>>::usable_balance(),
        );

        DeleteDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .with_deletion_account_id(DEFAULT_MEMBER_ACCOUNT_ID)
            .call_and_assert(Err(Error::<Test>::InsufficientTreasuryBalance.into()));
    });
}

#[test]
fn delete_storage_bucket_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        DeleteStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketDeleted(bucket_id));
    });
}

#[test]
fn delete_storage_bucket_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        DeleteStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn delete_storage_bucket_fails_with_non_existing_storage_bucket() {
    build_test_externalities().execute_with(|| {
        DeleteStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn delete_storage_bucket_fails_with_non_missing_invitation() {
    build_test_externalities().execute_with(|| {
        let invited_worker_id = DEFAULT_STORAGE_PROVIDER_ID;

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(Some(invited_worker_id))
            .call_and_assert(Ok(()))
            .unwrap();

        DeleteStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::InvitedStorageProvider.into()));
    });
}

#[test]
fn delete_storage_bucket_fails_with_non_empty_bucket() {
    build_test_externalities().execute_with(|| {
        let bag_id = BagId::<Test>::StaticBag(StaticBagId::Council);

        let bucket_id = create_default_storage_bucket_and_assign_to_bag(bag_id.clone());

        let object_creation_list = create_single_data_object();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        DeleteStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::CannotDeleteNonEmptyStorageBucket.into()));
    });
}

#[test]
fn remove_storage_bucket_operator_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        RemoveStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketOperatorRemoved(bucket_id));
    });
}

#[test]
fn remove_storage_bucket_operator_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_storage_provider_id = 1;

        RemoveStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(non_storage_provider_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn remove_storage_bucket_operator_fails_with_non_existing_storage_bucket() {
    build_test_externalities().execute_with(|| {
        RemoveStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn remove_storage_bucket_operator_fails_with_non_accepted_storage_provider() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        RemoveStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));
    });
}

#[test]
fn remove_storage_bucket_operator_fails_with_missing_storage_provider() {
    build_test_externalities().execute_with(|| {
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(None)
            .call_and_assert(Ok(()))
            .unwrap();

        RemoveStorageBucketOperatorFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));
    });
}

#[test]
fn update_data_size_fee_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let new_fee = 1000;

        UpdateDataObjectPerMegabyteFeeFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_fee(new_fee)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::DataObjectPerMegabyteFeeUpdated(new_fee));
    });
}

#[test]
fn update_data_size_fee_fails_with_non_leader_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateDataObjectPerMegabyteFeeFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn data_size_fee_calculation_works_properly() {
    build_test_externalities().execute_with(|| {
        const ONE_MB: u64 = 1_048_576;

        // Fee set to zero.
        assert_eq!(Storage::calculate_data_storage_fee(ONE_MB), 0);

        let data_size_fee = 1000;

        UpdateDataObjectPerMegabyteFeeFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_fee(data_size_fee)
            .call_and_assert(Ok(()));

        // Fee set.
        assert_eq!(Storage::calculate_data_storage_fee(ONE_MB), data_size_fee);
        assert_eq!(
            Storage::calculate_data_storage_fee(2 * ONE_MB),
            2 * data_size_fee
        );

        // Rounding works correctly.
        assert_eq!(
            Storage::calculate_data_storage_fee(ONE_MB + 1),
            2 * data_size_fee
        );
    });
}

#[test]
fn deletion_prize_changed_event_fired() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        create_dynamic_bag(&dynamic_bag_id);

        let upload_params = UploadParameters::<Test> {
            bag_id: BagId::<Test>::DynamicBag(dynamic_bag_id.clone()),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: create_single_data_object(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        EventFixture::contains_crate_event(RawEvent::DeletionPrizeChanged(
            dynamic_bag_id,
            DataObjectDeletionPrize::get(),
        ));
    });
}

#[test]
fn storage_bucket_voucher_changed_event_fired() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);
        let bag_id = BagId::<Test>::DynamicBag(dynamic_bag_id.clone());
        create_dynamic_bag(&dynamic_bag_id);

        let objects_limit = 1;
        let size_limit = 100;

        let bucket_id = create_storage_bucket_and_assign_to_bag(
            bag_id.clone(),
            None,
            objects_limit,
            size_limit,
        );

        let object_creation_list = create_single_data_object();

        let initial_balance = 1000;
        increase_account_balance(&DEFAULT_MEMBER_ACCOUNT_ID, initial_balance);

        let upload_params = UploadParameters::<Test> {
            bag_id: bag_id.clone(),
            authentication_key: Vec::new(),
            deletion_prize_source_account_id: DEFAULT_MEMBER_ACCOUNT_ID,
            object_creation_list: object_creation_list.clone(),
        };

        UploadFixture::default()
            .with_params(upload_params.clone())
            .call_and_assert(Ok(()));

        EventFixture::contains_crate_event(RawEvent::VoucherChanged(
            bucket_id,
            Voucher {
                objects_limit,
                size_limit,
                objects_used: 1,
                size_used: object_creation_list[0].size,
            },
        ));
    });
}

#[test]
fn update_storage_buckets_per_bag_limit_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let new_limit = 4;

        UpdateStorageBucketsPerBagLimitFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_limit(new_limit)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketsPerBagLimitUpdated(
            new_limit,
        ));
    });
}

#[test]
fn update_storage_buckets_per_bag_limit_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateStorageBucketsPerBagLimitFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_storage_buckets_per_bag_limit_fails_with_incorrect_value() {
    build_test_externalities().execute_with(|| {
        let new_limit = 0;

        UpdateStorageBucketsPerBagLimitFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_limit(new_limit)
            .call_and_assert(Err(Error::<Test>::StorageBucketsPerBagLimitTooLow.into()));

        let new_limit = 100;

        UpdateStorageBucketsPerBagLimitFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_limit(new_limit)
            .call_and_assert(Err(Error::<Test>::StorageBucketsPerBagLimitTooHigh.into()));
    });
}

fn set_update_storage_buckets_per_bag_limit() {
    let new_limit = 7;

    UpdateStorageBucketsPerBagLimitFixture::default()
        .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
        .with_new_limit(new_limit)
        .call_and_assert(Ok(()))
}

#[test]
fn set_storage_bucket_voucher_limits_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        set_max_voucher_limits();

        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let new_objects_size_limit = 1;
        let new_objects_number_limit = 1;

        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_new_objects_number_limit(new_objects_number_limit)
            .with_new_objects_size_limit(new_objects_size_limit)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketVoucherLimitsSet(
            bucket_id,
            storage_provider_id,
            new_objects_size_limit,
            new_objects_number_limit,
        ));
    });
}

#[test]
fn set_storage_bucket_voucher_limits_fails_with_invalid_values() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let invalid_objects_size_limit = 1000;
        let invalid_objects_number_limit = 1000;

        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_new_objects_size_limit(invalid_objects_size_limit)
            .call_and_assert(Err(Error::<Test>::VoucherMaxObjectSizeLimitExceeded.into()));

        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_worker_id(storage_provider_id)
            .with_storage_bucket_id(bucket_id)
            .with_new_objects_number_limit(invalid_objects_number_limit)
            .call_and_assert(Err(
                Error::<Test>::VoucherMaxObjectNumberLimitExceeded.into()
            ));
    });
}

#[test]
fn set_storage_bucket_voucher_limits_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Root)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn set_storage_bucket_voucher_limits_fails_with_invalid_storage_bucket() {
    build_test_externalities().execute_with(|| {
        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .call_and_assert(Err(Error::<Test>::StorageBucketDoesntExist.into()));
    });
}

#[test]
fn set_storage_bucket_voucher_limits_fails_with_invalid_storage_association() {
    build_test_externalities().execute_with(|| {
        let storage_provider_id = DEFAULT_STORAGE_PROVIDER_ID;
        let invite_worker = Some(storage_provider_id);

        // Missing invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .call_and_assert(Ok(()))
            .unwrap();

        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::StorageProviderMustBeSet.into()));

        // Not accepted invitation
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(invite_worker)
            .call_and_assert(Ok(()))
            .unwrap();

        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));

        // Invitation accepted. Incorrect storage provider.
        AcceptStorageBucketInvitationFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(storage_provider_id)
            .call_and_assert(Ok(()));

        let incorrect_storage_provider_id = 888;
        SetStorageBucketVoucherLimitsFixture::default()
            .with_origin(RawOrigin::Signed(DEFAULT_STORAGE_PROVIDER_ACCOUNT_ID))
            .with_storage_bucket_id(bucket_id)
            .with_worker_id(incorrect_storage_provider_id)
            .call_and_assert(Err(Error::<Test>::InvalidStorageProvider.into()));
    });
}

fn set_max_voucher_limits() {
    let new_size_limit = 100;
    let new_objects_limit = 1;

    UpdateStorageBucketsVoucherMaxLimitsFixture::default()
        .with_new_objects_size_limit(new_size_limit)
        .with_new_objects_number_limit(new_objects_limit)
        .call_and_assert(Ok(()));
}

#[test]
fn update_storage_buckets_voucher_max_limits_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let new_size_limit = 14;
        let new_number_limit = 4;

        UpdateStorageBucketsVoucherMaxLimitsFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_objects_number_limit(new_number_limit)
            .with_new_objects_size_limit(new_size_limit)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::StorageBucketsVoucherMaxLimitsUpdated(
            new_size_limit,
            new_number_limit,
        ));
    });
}

#[test]
fn update_storage_buckets_voucher_max_limits_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateStorageBucketsVoucherMaxLimitsFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn create_dynamic_bag_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);

        create_storage_buckets(10);

        CreateDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(RawEvent::DynamicBagCreated(dynamic_bag_id.clone()));

        let bag = Storage::dynamic_bag(dynamic_bag_id.clone());
        // Check that IDs are within possible range.
        assert!(bag
            .stored_by
            .iter()
            .all(|id| { *id < Storage::next_storage_bucket_id() }));

        let creation_policy = Storage::get_dynamic_bag_creation_policy(dynamic_bag_id.into());
        assert_eq!(
            bag.stored_by.len(),
            creation_policy.number_of_storage_buckets as usize
        );
    });
}

#[test]
fn create_dynamic_bag_failed_with_existing_bag() {
    build_test_externalities().execute_with(|| {
        let dynamic_bag_id = DynamicBagId::<Test>::Member(DEFAULT_MEMBER_ID);

        CreateDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .call_and_assert(Ok(()));

        CreateDynamicBagFixture::default()
            .with_bag_id(dynamic_bag_id.clone())
            .call_and_assert(Err(Error::<Test>::DynamicBagExists.into()));
    });
}

fn create_dynamic_bag(dynamic_bag_id: &DynamicBagId<Test>) {
    CreateDynamicBagFixture::default()
        .with_bag_id(dynamic_bag_id.clone())
        .call_and_assert(Ok(()));
}

#[test]
fn test_storage_bucket_picking_for_bag_non_random() {
    build_test_externalities().execute_with(|| {
        // Randomness disabled at the initial block.

        let initial_buckets_number = InitialStorageBucketsNumberForDynamicBag::get();
        // No buckets
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);
        assert_eq!(bucket_ids, BTreeSet::new());

        // Less then initial buckets number
        let buckets_number = initial_buckets_number - 1;
        let created_buckets = create_storage_buckets(buckets_number);
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        assert_eq!(bucket_ids, created_buckets);

        // More then initial buckets number
        let buckets_number = 5;
        create_storage_buckets(buckets_number);
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        assert_eq!(
            bucket_ids,
            BTreeSet::from_iter((0u64..initial_buckets_number).into_iter())
        );

        // Check removed buckets
        let removed_bucket_id = 1;
        <crate::StorageBucketById<Test>>::remove(removed_bucket_id);

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        let mut expected_ids =
            BTreeSet::from_iter((0u64..(initial_buckets_number + 1)).into_iter());
        expected_ids.remove(&removed_bucket_id);

        assert_eq!(bucket_ids, expected_ids);

        // Check disabled buckets
        let disabled_bucket_id = 2;
        <crate::StorageBucketById<Test>>::mutate(disabled_bucket_id, |bucket| {
            bucket.accepting_new_bags = false;
        });

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        let mut expected_ids =
            BTreeSet::from_iter((0u64..(initial_buckets_number + 2)).into_iter());
        expected_ids.remove(&removed_bucket_id);
        expected_ids.remove(&disabled_bucket_id);

        assert_eq!(bucket_ids, expected_ids);

        // No storage buckets required
        crate::DynamicBagCreationPolicies::insert(
            DynamicBagType::Member,
            DynamicBagCreationPolicy::default(),
        );

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);
        assert_eq!(bucket_ids, BTreeSet::new());
    });
}

#[test]
fn test_storage_bucket_picking_for_bag_with_randomness() {
    build_test_externalities().execute_with(|| {
        // Enable randomness (disabled at the initial block).
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_buckets_number = InitialStorageBucketsNumberForDynamicBag::get();
        // No buckets
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);
        assert_eq!(bucket_ids, BTreeSet::new());

        // Less then initial buckets number
        let buckets_number = initial_buckets_number - 1;
        let created_buckets = create_storage_buckets(buckets_number);
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        assert_eq!(bucket_ids, created_buckets);

        // More then initial buckets number
        let buckets_number = 5;
        create_storage_buckets(buckets_number);
        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        let sequential_random_ids = BTreeSet::from_iter((0u64..initial_buckets_number).into_iter());

        // Check number of generated IDs
        assert_eq!(initial_buckets_number, bucket_ids.len() as u64);
        // Verify that generated IDs differ from sequential ID enumeration.
        assert_ne!(sequential_random_ids, bucket_ids);
        // Check that IDs are within possible range.
        assert!(bucket_ids
            .iter()
            .all(|id| { *id < Storage::next_storage_bucket_id() }));

        // Check removed buckets
        let removed_bucket_id = bucket_ids.iter().next().unwrap();
        <crate::StorageBucketById<Test>>::remove(removed_bucket_id);

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);
        // Check number of generated IDs
        assert_eq!(initial_buckets_number, bucket_ids.len() as u64);
        // Check that IDs are within possible range.
        assert!(bucket_ids
            .iter()
            .all(|id| { *id < Storage::next_storage_bucket_id() }));
        // Check removed bucket
        assert!(!bucket_ids.contains(removed_bucket_id));

        // Check disabled buckets
        let disabled_bucket_id = 2;
        <crate::StorageBucketById<Test>>::mutate(disabled_bucket_id, |bucket| {
            bucket.accepting_new_bags = false;
        });

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);

        let mut expected_ids =
            BTreeSet::from_iter((0u64..(initial_buckets_number + 2)).into_iter());
        expected_ids.remove(&removed_bucket_id);
        expected_ids.remove(&disabled_bucket_id);

        // Check number of generated IDs
        assert_eq!(initial_buckets_number, bucket_ids.len() as u64);
        // Check that IDs are within possible range.
        assert!(bucket_ids
            .iter()
            .all(|id| { *id < Storage::next_storage_bucket_id() }));
        // Check removed bucket
        assert!(!bucket_ids.contains(removed_bucket_id));

        // No storage buckets required
        crate::DynamicBagCreationPolicies::insert(
            DynamicBagType::Member,
            DynamicBagCreationPolicy::default(),
        );

        let bucket_ids = Storage::pick_storage_buckets_for_dynamic_bag(DynamicBagType::Member);
        assert_eq!(bucket_ids, BTreeSet::new());
    });
}

#[test]
fn test_storage_bucket_iterators() {
    build_test_externalities().execute_with(|| {
        // Enable randomness (disabled at the initial block).
        let starting_block = 1;
        run_to_block(starting_block);

        // More then initial buckets number
        let buckets_number = 5;
        create_storage_buckets(buckets_number);

        use crate::storage_bucket_picker::{
            RandomStorageBucketIdIterator as Rand, SequentialStorageBucketIdIterator as Seq,
        };

        let ids = Rand::<Test>::new()
            .chain(Seq::<Test>::new())
            .collect::<Vec<_>>();

        // Check combined iterator length.
        assert_eq!(
            ids.len(),
            (MaxRandomIterationNumber::get() + buckets_number) as usize
        );
        // Check that IDs are within possible range.
        assert!(ids
            .iter()
            .all(|id| { *id < Storage::next_storage_bucket_id() }));
        // Checks all possible entries are present (remove duplicates).
        assert_eq!(
            ids.iter().collect::<BTreeSet<_>>().len(),
            buckets_number as usize
        );
    });
}

fn create_storage_buckets(buckets_number: u64) -> BTreeSet<u64> {
    set_max_voucher_limits();

    let objects_limit = 1;
    let size_limit = 100;

    let mut bucket_ids = BTreeSet::new();

    for _ in 0..buckets_number {
        let bucket_id = CreateStorageBucketFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_invite_worker(None)
            .with_objects_limit(objects_limit)
            .with_size_limit(size_limit)
            .call_and_assert(Ok(()))
            .unwrap();

        bucket_ids.insert(bucket_id);
    }

    bucket_ids
}

#[test]
fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy_succeeded() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let dynamic_bag_type = DynamicBagType::Channel;
        let new_bucket_number = 40;

        UpdateNumberOfStorageBucketsInDynamicBagCreationPolicyFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_storage_buckets_number(new_bucket_number)
            .with_dynamic_bag_type(dynamic_bag_type)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(
            RawEvent::NumberOfStorageBucketsInDynamicBagCreationPolicyUpdated(
                dynamic_bag_type,
                new_bucket_number,
            ),
        );
    });
}

#[test]
fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        let non_leader_id = 1;

        UpdateNumberOfStorageBucketsInDynamicBagCreationPolicyFixture::default()
            .with_origin(RawOrigin::Signed(non_leader_id))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn dynamic_bag_creation_policy_defaults_and_updates_succeeded() {
    build_test_externalities().execute_with(|| {
        let new_bucket_number = 40;

        // Change member dynamic bag creation policy.
        let dynamic_bag_type = DynamicBagType::Member;
        let policy = Storage::get_dynamic_bag_creation_policy(dynamic_bag_type);
        assert_eq!(policy, DefaultMemberDynamicBagCreationPolicy::get());

        UpdateNumberOfStorageBucketsInDynamicBagCreationPolicyFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_storage_buckets_number(new_bucket_number)
            .with_dynamic_bag_type(dynamic_bag_type)
            .call_and_assert(Ok(()));

        let policy = Storage::get_dynamic_bag_creation_policy(dynamic_bag_type);
        assert_eq!(policy.number_of_storage_buckets, new_bucket_number);

        // Change channel dynamic bag creation policy.
        let dynamic_bag_type = DynamicBagType::Channel;
        let policy = Storage::get_dynamic_bag_creation_policy(dynamic_bag_type);
        assert_eq!(policy, DefaultChannelDynamicBagCreationPolicy::get());

        UpdateNumberOfStorageBucketsInDynamicBagCreationPolicyFixture::default()
            .with_origin(RawOrigin::Signed(WG_LEADER_ACCOUNT_ID))
            .with_new_storage_buckets_number(new_bucket_number)
            .with_dynamic_bag_type(dynamic_bag_type)
            .call_and_assert(Ok(()));

        let policy = Storage::get_dynamic_bag_creation_policy(dynamic_bag_type);
        assert_eq!(policy.number_of_storage_buckets, new_bucket_number);
    });
}
