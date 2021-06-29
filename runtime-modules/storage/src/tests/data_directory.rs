#![cfg(test)]

use crate::data_directory::Error;
use common::storage::StorageObjectOwner;
use frame_support::assert_ok;
use frame_support::dispatch::DispatchError;
use frame_system::RawOrigin;

use super::mock::*;

#[test]
fn succeed_adding_content() {
    with_default_mock_builder(|| {
        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let first_content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let second_content_parameters = ContentParameters {
            content_id: 2,
            type_id: 2,
            size: 20,
            ipfs_content_id: vec![1, 2, 7, 9],
        };

        let multi_content = vec![first_content_parameters, second_content_parameters];

        // Register a content with 1234 bytes of type 1, which should be recognized.
        let res = TestDataDirectory::add_content(Origin::signed(sender), owner, multi_content);
        assert!(res.is_ok());
    });
}

#[test]
fn add_content_fails_with_invalid_origin() {
    with_default_mock_builder(|| {
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        // Make an attempt to register a content with 1234 bytes of type 1, which should be recognized.
        let res =
            TestDataDirectory::add_content(RawOrigin::Root.into(), owner, vec![content_parameters]);
        assert_eq!(res, Err(DispatchError::Other("Bad origin")));
    });
}

#[test]
fn add_content_uploading_blocked() {
    ExtBuilder::default()
        .uploading_blocked_status(true)
        .build()
        .execute_with(|| {
            let sender = 1u64;

            let owner = StorageObjectOwner::Member(1u64);

            let content_parameters = ContentParameters {
                content_id: 1,
                type_id: 1234,
                size: 0,
                ipfs_content_id: vec![1, 2, 3, 4],
            };

            // Make an attempt to register a content, when uploading is blocked.
            let res = TestDataDirectory::add_content(
                Origin::signed(sender),
                owner,
                vec![content_parameters],
            );
            assert_eq!(res, Err(Error::<Test>::ContentUploadingBlocked.into()));
        });
}

#[test]
fn add_content_size_limit_reached() {
    with_default_mock_builder(|| {
        let sender = 1u64;

        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: DEFAULT_VOUCHER.get_size_limit() + 1,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        // Make an attempt to register a content, when uploading is blocked.
        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert_eq!(res, Err(Error::<Test>::VoucherSizeLimitExceeded.into()));
    });
}

#[test]
fn add_content_objects_limit_reached() {
    with_default_mock_builder(|| {
        let sender = 1u64;

        let owner = StorageObjectOwner::Member(1u64);

        let mut content = vec![];

        for i in 0..=DEFAULT_VOUCHER.get_objects_limit() {
            let content_parameters = ContentParameters {
                content_id: i + 1,
                type_id: 1234,
                size: 0,
                ipfs_content_id: vec![1, 2, 3, 4],
            };
            content.push(content_parameters);
        }

        // Make an attempt to register a content, when uploading is blocked.
        let res = TestDataDirectory::add_content(Origin::signed(sender), owner, content);
        assert_eq!(res, Err(Error::<Test>::VoucherObjectsLimitExceeded.into()));
    });
}

#[test]
fn add_content_global_size_limit_reached() {
    let global_voucher_size_limit = 0;
    let global_voucher_objects_limit = 50;

    ExtBuilder::default()
        .global_voucher(Voucher::new(
            global_voucher_size_limit,
            global_voucher_objects_limit,
        ))
        .build()
        .execute_with(|| {
            let sender = 1u64;

            let owner = StorageObjectOwner::Member(1u64);

            let content_parameters = ContentParameters {
                content_id: 1,
                type_id: 1234,
                size: global_voucher_size_limit + 1,
                ipfs_content_id: vec![1, 2, 3, 4],
            };

            // Make an attempt to register a content, when uploading is blocked.
            let res = TestDataDirectory::add_content(
                Origin::signed(sender),
                owner,
                vec![content_parameters],
            );
            assert_eq!(res, Err(Error::<Test>::VoucherSizeLimitExceeded.into()));
        });
}

#[test]
fn add_content_global_objects_limit_reached() {
    let global_voucher_size_limit = 50000;
    let global_voucher_objects_limit = 0;

    ExtBuilder::default()
        .global_voucher(Voucher::new(
            global_voucher_size_limit,
            global_voucher_objects_limit,
        ))
        .build()
        .execute_with(|| {
            let sender = 1u64;

            let owner = StorageObjectOwner::Member(1u64);

            let content_parameters = ContentParameters {
                content_id: 1,
                type_id: 1234,
                size: 0,
                ipfs_content_id: vec![1, 2, 3, 4],
            };

            // Make an attempt to register a content, when uploading is blocked.
            let res = TestDataDirectory::add_content(
                Origin::signed(sender),
                owner,
                vec![content_parameters],
            );
            assert_eq!(res, Err(Error::<Test>::VoucherObjectsLimitExceeded.into()));
        });
}

#[test]
fn delete_content() {
    with_default_mock_builder(|| {
        let sender = 1u64;

        let owner = StorageObjectOwner::Member(1u64);

        let content_id = 1;

        let content_parameters = ContentParameters {
            content_id,
            type_id: 1234,
            size: 1,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        // Register a content with 1234 bytes of type 1, which should be recognized.

        TestDataDirectory::add_content(
            Origin::signed(sender),
            owner.clone(),
            vec![content_parameters],
        )
        .unwrap();

        let res =
            TestDataDirectory::remove_content(Origin::signed(sender), owner, vec![content_id]);

        assert!(res.is_ok());
    });
}

#[test]
fn delete_content_id_not_found() {
    with_default_mock_builder(|| {
        let sender = 1u64;

        let content_id = 1;

        let owner = StorageObjectOwner::Member(1u64);

        // Make an attempt to remove content under non existent id
        let res =
            TestDataDirectory::remove_content(Origin::signed(sender), owner, vec![content_id]);

        assert_eq!(res, Err(Error::<Test>::CidNotFound.into()));
    });
}

#[test]
fn delete_content_owners_are_not_equal() {
    with_default_mock_builder(|| {
        let sender = 1u64;

        let owner = StorageObjectOwner::Member(1u64);

        let second_owner = StorageObjectOwner::Member(10u64);

        let content_id = 1;

        let content_parameters = ContentParameters {
            content_id,
            type_id: 1234,
            size: 1,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        // Register a content with 1234 bytes of type 1, which should be recognized.

        TestDataDirectory::add_content(
            Origin::signed(sender),
            owner.clone(),
            vec![content_parameters],
        )
        .unwrap();

        // Make an attempt to remove content, providing a wrong origin
        let res = TestDataDirectory::remove_content(
            Origin::signed(sender),
            second_owner,
            vec![content_id],
        );

        assert_eq!(res, Err(Error::<Test>::OwnersAreNotEqual.into()));
    });
}

#[test]
fn update_content_uploading_status() {
    with_default_mock_builder(|| {
        SetLeadFixture::set_default_lead();

        let res = TestDataDirectory::update_content_uploading_status(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            true,
        );

        assert!(res.is_ok());

        assert_eq!(TestDataDirectory::uploading_blocked(), true);
    });
}

#[test]
fn update_storage_object_owner_voucher_objects_limit() {
    with_default_mock_builder(|| {
        SetLeadFixture::set_default_lead();

        let owner = StorageObjectOwner::Member(1u64);

        let new_objects_limit = 20;

        let res = TestDataDirectory::update_storage_object_owner_voucher_objects_limit(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            owner.clone(),
            new_objects_limit,
        );

        assert!(res.is_ok());

        assert_eq!(
            TestDataDirectory::vouchers(owner).get_objects_limit(),
            new_objects_limit
        );
    });
}

#[test]
fn update_storage_object_owner_voucher_objects_limit_upper_bound_exceeded() {
    with_default_mock_builder(|| {
        SetLeadFixture::set_default_lead();

        let owner = StorageObjectOwner::Member(1u64);

        let new_objects_limit = TestDataDirectory::voucher_objects_limit_upper_bound() + 1;

        // Make an attempt to update storage object owner voucher objects limit, providing value, which exceeds upper bound.
        let res = TestDataDirectory::update_storage_object_owner_voucher_objects_limit(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            owner.clone(),
            new_objects_limit,
        );

        assert_eq!(
            res,
            Err(Error::<Test>::VoucherObjectsLimitUpperBoundExceeded.into())
        );
    });
}

#[test]
fn update_storage_object_owner_voucher_size_limit() {
    with_default_mock_builder(|| {
        SetLeadFixture::set_default_lead();

        let owner = StorageObjectOwner::Member(1u64);

        let new_objects_total_size_limit = 100;

        let res = TestDataDirectory::update_storage_object_owner_voucher_size_limit(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            owner.clone(),
            new_objects_total_size_limit,
        );

        assert!(res.is_ok());

        assert_eq!(
            TestDataDirectory::vouchers(owner).get_size_limit(),
            new_objects_total_size_limit
        );
    });
}

#[test]
fn update_storage_object_owner_voucher_size_limit_upper_bound_exceeded() {
    with_default_mock_builder(|| {
        SetLeadFixture::set_default_lead();

        let owner = StorageObjectOwner::Member(1u64);

        let new_objects_total_size_limit = TestDataDirectory::voucher_size_limit_upper_bound() + 1;

        // Make an attempt to update storage object owner voucher size limit, providing value, which exceeds upper bound.
        let res = TestDataDirectory::update_storage_object_owner_voucher_size_limit(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            owner.clone(),
            new_objects_total_size_limit,
        );

        assert_eq!(
            res,
            Err(Error::<Test>::VoucherSizeLimitUpperBoundExceeded.into())
        );
    });
}

#[test]
fn accept_and_reject_content_fail_with_invalid_storage_provider() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert!(res.is_ok());

        let content_id = match &System::events().last().unwrap().event {
            Event::data_directory(data_directory::RawEvent::ContentAdded(content, _)) => {
                content[0].content_id
            }
            _ => 0u64,
        };

        //  invalid data
        let (storage_provider_account_id, storage_provider_id) = (1, 5);

        let res = TestDataDirectory::accept_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Err(working_group::Error::<Test, crate::StorageWorkingGroupInstance>::WorkerDoesNotExist.into()));
    });
}

#[test]
fn accept_content_as_liaison() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let sender = 1u64;
        let owner = StorageObjectOwner::Member(1u64);

        let content_parameters = ContentParameters {
            content_id: 1,
            type_id: 1234,
            size: 0,
            ipfs_content_id: vec![1, 2, 3, 4],
        };

        let res =
            TestDataDirectory::add_content(Origin::signed(sender), owner, vec![content_parameters]);
        assert!(res.is_ok());

        // An appropriate event should have been fired.
        let (content_id, creator) = match &System::events().last().unwrap().event {
            Event::data_directory(data_directory::RawEvent::ContentAdded(content, creator)) => {
                (content[0].content_id, creator.clone())
            }
            _ => (0u64, StorageObjectOwner::Member(0xdeadbeefu64)), // invalid value, unlikely to match
        };
        assert_ne!(creator, StorageObjectOwner::Member(0xdeadbeefu64));
        assert_eq!(creator, StorageObjectOwner::Member(sender));

        let (storage_provider_account_id, storage_provider_id) = hire_storage_provider();

        // Accepting content should not work with some random origin
        let res =
            TestDataDirectory::accept_content(Origin::signed(55), storage_provider_id, content_id);
        assert!(res.is_err());

        // However, with the liaison as origin it should.
        let res = TestDataDirectory::accept_content(
            Origin::signed(storage_provider_account_id),
            storage_provider_id,
            content_id,
        );
        assert_eq!(res, Ok(()));
    });
}

#[test]
fn set_global_voucher_limits() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let size_limit = TestDataDirectory::global_voucher().get_size_limit();
        let increment_size_limit_by = 10;
        let expected_new_size_limit = size_limit + increment_size_limit_by;

        assert_ok!(TestDataDirectory::set_global_voucher_size_limit(
            RawOrigin::Root.into(),
            expected_new_size_limit
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::data_directory(data_directory::RawEvent::GlobalVoucherSizeLimitUpdated(
                expected_new_size_limit
            ))
        );

        assert_eq!(
            TestDataDirectory::global_voucher().get_size_limit(),
            expected_new_size_limit
        );

        let objects_limit = TestDataDirectory::global_voucher().get_objects_limit();
        let increment_objects_limit_by = 10;
        let expected_new_objects_limit = objects_limit + increment_objects_limit_by;

        assert_ok!(TestDataDirectory::set_global_voucher_objects_limit(
            RawOrigin::Root.into(),
            expected_new_objects_limit
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::data_directory(data_directory::RawEvent::GlobalVoucherObjectsLimitUpdated(
                expected_new_objects_limit
            ))
        );

        assert_eq!(
            TestDataDirectory::global_voucher().get_objects_limit(),
            expected_new_objects_limit
        );
    })
}

#[test]
fn set_limit_upper_bounds() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let size_limit_upper_bound = TestDataDirectory::voucher_size_limit_upper_bound();
        let increment_size_limit_upper_bound_by = 10;
        let expected_new_size_limit_upper_bound =
            size_limit_upper_bound + increment_size_limit_upper_bound_by;

        assert_ok!(TestDataDirectory::set_voucher_size_limit_upper_bound(
            RawOrigin::Root.into(),
            expected_new_size_limit_upper_bound
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::data_directory(data_directory::RawEvent::VoucherSizeLimitUpperBoundUpdated(
                expected_new_size_limit_upper_bound
            ))
        );

        assert_eq!(
            TestDataDirectory::voucher_size_limit_upper_bound(),
            expected_new_size_limit_upper_bound
        );

        let objects_limit_upper_bound = TestDataDirectory::voucher_objects_limit_upper_bound();
        let increment_objects_limit_upper_bound_by = 10;
        let expected_new_objects_limit_upper_bound =
            objects_limit_upper_bound + increment_objects_limit_upper_bound_by;

        assert_ok!(TestDataDirectory::set_voucher_objects_limit_upper_bound(
            RawOrigin::Root.into(),
            expected_new_objects_limit_upper_bound
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::data_directory(
                data_directory::RawEvent::VoucherObjectsLimitUpperBoundUpdated(
                    expected_new_objects_limit_upper_bound
                )
            )
        );

        assert_eq!(
            TestDataDirectory::voucher_objects_limit_upper_bound(),
            expected_new_objects_limit_upper_bound
        );
    })
}

#[test]
fn set_default_voucher() {
    with_default_mock_builder(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        SetLeadFixture::set_default_lead();

        let default_voucher = TestDataDirectory::default_voucher();

        let new_size_limit = default_voucher.get_size_limit() + 1;
        let new_objects_limit = default_voucher.get_objects_limit() + 1;

        assert_ok!(TestDataDirectory::set_default_voucher(
            Origin::signed(DEFAULT_LEADER_ACCOUNT_ID),
            new_size_limit,
            new_objects_limit
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::data_directory(data_directory::RawEvent::DefaultVoucherUpdated(
                new_size_limit,
                new_objects_limit
            ))
        );

        assert_eq!(
            TestDataDirectory::default_voucher(),
            Voucher::new(new_size_limit, new_objects_limit)
        );
    })
}
