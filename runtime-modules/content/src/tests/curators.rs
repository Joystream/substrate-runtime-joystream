#![cfg(test)]

use super::mock::{Event, *};
use crate::*;
use frame_support::{assert_err, assert_ok};

pub fn add_curator_to_new_group(curator_id: CuratorId) -> CuratorGroupId {
    let curator_group_id = Content::next_curator_group_id();
    // create new group and add curator id to it
    assert_ok!(Content::create_curator_group(Origin::signed(LEAD_ORIGIN)));
    assert_ok!(Content::add_curator_to_group(
        Origin::signed(LEAD_ORIGIN),
        curator_group_id,
        curator_id
    ));
    // make group active
    assert_ok!(Content::set_curator_group_status(
        Origin::signed(LEAD_ORIGIN),
        curator_group_id,
        true
    ));
    curator_group_id
}

#[test]
fn curator_group_management() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        let curator_group_id = Content::next_curator_group_id();
        assert_ok!(Content::create_curator_group(Origin::signed(LEAD_ORIGIN)));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::CuratorGroupCreated(curator_group_id))
        );

        let group = Content::curator_group_by_id(curator_group_id);

        // By default group is empty and not active
        assert_eq!(group.is_active(), false);
        assert_eq!(group.get_curators().len(), 0);

        // Activate group
        assert_ok!(Content::set_curator_group_status(
            Origin::signed(LEAD_ORIGIN),
            curator_group_id,
            true
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::CuratorGroupStatusSet(curator_group_id, true))
        );

        let group = Content::curator_group_by_id(curator_group_id);
        assert_eq!(group.is_active(), true);

        // Cannot add non curators into group
        assert_err!(
            Content::add_curator_to_group(
                Origin::signed(LEAD_ORIGIN),
                curator_group_id,
                MEMBERS_COUNT + 1 // not a curator
            ),
            Error::<Test>::CuratorIdInvalid
        );

        // Add curator to group
        assert_ok!(Content::add_curator_to_group(
            Origin::signed(LEAD_ORIGIN),
            curator_group_id,
            FIRST_CURATOR_ID
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::CuratorAdded(curator_group_id, FIRST_CURATOR_ID))
        );

        // Ensure curator is in group
        let group = Content::curator_group_by_id(curator_group_id);
        assert!(group.has_curator(&FIRST_CURATOR_ID));

        // Cannot add same curator again
        assert_err!(
            Content::add_curator_to_group(
                Origin::signed(LEAD_ORIGIN),
                curator_group_id,
                FIRST_CURATOR_ID
            ),
            Error::<Test>::CuratorIsAlreadyAMemberOfGivenCuratorGroup
        );

        // Cannot remove curator if not in group
        assert_err!(
            Content::remove_curator_from_group(
                Origin::signed(LEAD_ORIGIN),
                curator_group_id,
                MEMBERS_COUNT + 1 // not a curator
            ),
            Error::<Test>::CuratorIsNotAMemberOfGivenCuratorGroup
        );

        // Remove curator from group
        assert_ok!(Content::remove_curator_from_group(
            Origin::signed(LEAD_ORIGIN),
            curator_group_id,
            FIRST_CURATOR_ID
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::CuratorRemoved(curator_group_id, FIRST_CURATOR_ID))
        );

        let group = Content::curator_group_by_id(curator_group_id);
        assert!(!group.has_curator(&FIRST_CURATOR_ID));

        // Already removed cannot remove again
        assert_err!(
            Content::remove_curator_from_group(
                Origin::signed(LEAD_ORIGIN),
                curator_group_id,
                FIRST_CURATOR_ID
            ),
            Error::<Test>::CuratorIsNotAMemberOfGivenCuratorGroup
        );
    })
}
