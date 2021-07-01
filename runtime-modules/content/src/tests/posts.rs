#![cfg(test)]

use super::curators;
use super::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};
use sp_runtime::traits::Hash;

pub const UNKNOWN_MEMBER_ID: u64 = 7777;
pub const UNKNOWN_CURATOR_GROUP_ID: u64 = 7777;
pub const UNKNOWN_CURATOR_ID: u64 = 7777;
pub const UNKNOWN_REPLY_ID: u64 = 7777;
pub const UNKNOWN_POST_ID: u64 = 7777;
pub const UNKNOWN_VIDEO_ID: u64 = 7777;

fn setup_testing_scenario() -> (
    <tests::mock::Test as Trait>::VideoId,
    <tests::mock::Test as Trait>::VideoId,
) {
    // scenario A:
    // - 1 member (FIRST_MEMBER_ID),
    // - 1 curator group (FIRST_CURATOR_GROUP_ID),
    // - 2 channels (channel_id.0 & channel_id.1, one for each actor),
    // - 2 videos (video_id.0 & video_id.1 one for each channel)

    let member_channel_id = Content::next_channel_id();

    assert_ok!(Content::create_channel(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        ContentActor::Member(FIRST_MEMBER_ID),
        ChannelCreationParameters {
            assets: vec![],
            meta: vec![],
            reward_account: None,
        },
    ));
    let curator_channel_id = Content::next_channel_id();

    println!("member channel done");

    let group_id = curators::add_curator_to_new_group(FIRST_CURATOR_ID);
    assert_eq!(FIRST_CURATOR_GROUP_ID, group_id);

    assert_ok!(Content::create_channel(
        Origin::signed(FIRST_CURATOR_ORIGIN),
        ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
        ChannelCreationParameters {
            assets: vec![],
            meta: vec![],
            reward_account: None,
        },
    ));
    println!("curator channel done");

    let member_video_id = Content::next_video_id();

    assert_ok!(Content::create_video(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        ContentActor::Member(FIRST_MEMBER_ID),
        member_channel_id,
        VideoCreationParameters {
            assets: vec![NewAsset::Urls(vec![b"https://somewhere.com/".to_vec()])],
            meta: b"metablob".to_vec(),
        },
    ));

    let curator_video_id = Content::next_video_id();

    assert_ok!(Content::create_video(
        Origin::signed(FIRST_CURATOR_ORIGIN),
        ContentActor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
        curator_channel_id,
        VideoCreationParameters {
            assets: vec![NewAsset::Urls(vec![b"https://somewhere.com/".to_vec()])],
            meta: b"metablob".to_vec(),
        },
    ));

    (member_video_id, curator_video_id)
}

// create post test
#[test]
fn cannot_create_post_with_nonexistent_video() {
    with_default_mock_builder(|| {
        //        run_to_block(1);

        let (_member_video_id, _curator_video_id) = setup_testing_scenario();

        assert_err!(
            Content::create_post(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as Trait>::VideoId::from(UNKNOWN_VIDEO_ID),
                ContentActor::Member(FIRST_MEMBER_ID),
            ),
            Error::<Test>::VideoDoesNotExist,
        );
    })
}

#[test]
fn non_authorized_actor_cannot_create_post() {
    with_default_mock_builder(|| {
        let (member_video_id, curator_video_id) = setup_testing_scenario();

        // non channel owner cannot create post
        assert_err!(
            Content::create_post(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                <tests::mock::Test as Trait>::VideoId::from(member_video_id),
                ContentActor::Member(SECOND_MEMBER_ID),
            ),
            Error::<Test>::ActorNotAuthorized,
        );

        assert_err!(
            Content::create_post(
                Origin::signed(SECOND_CURATOR_ORIGIN),
                <tests::mock::Test as Trait>::VideoId::from(curator_video_id),
                ContentActor::Curator(SECOND_CURATOR_GROUP_ID, SECOND_CURATOR_ID),
            ),
            Error::<Test>::CuratorGroupIsNotActive,
        );

        // unknown actor/member cannot create post
        assert_err!(
            Content::create_post(
                Origin::signed(UNKNOWN_ORIGIN),
                <tests::mock::Test as Trait>::VideoId::from(member_video_id),
                ContentActor::Member(UNKNOWN_MEMBER_ID),
            ),
            Error::<Test>::MemberAuthFailed,
        );

        assert_err!(
            Content::create_post(
                Origin::signed(UNKNOWN_ORIGIN),
                <tests::mock::Test as Trait>::VideoId::from(curator_video_id),
                ContentActor::Curator(UNKNOWN_CURATOR_GROUP_ID, UNKNOWN_CURATOR_ID),
            ),
            Error::<Test>::CuratorAuthFailed,
        );
    })
}
// Edit post tests
fn setup_testing_scenario_with_post() -> (
    <tests::mock::Test as Trait>::PostId,
    <tests::mock::Test as Trait>::VideoId,
) {
    // scenario B: previous scenario + one post made by FIRST_MEMBER_ID

    let (member_video_id, _curator_video_id) = setup_testing_scenario();
    let post_id = Content::next_post_id();

    assert_ok!(Content::create_post(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        <tests::mock::Test as Trait>::VideoId::from(member_video_id),
        ContentActor::Member(FIRST_MEMBER_ID),
    ));

    assert_eq!(Content::next_post_id(), 1);

    (post_id, member_video_id)
}

#[test]
fn verify_create_post_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (post_id, member_video_id) = setup_testing_scenario_with_post();

        // the actual post that should be created
        let post: Post<Test> = Post_ {
            author: ChannelOwner::Member(FIRST_MEMBER_ID),
            cleanup_pay_off: BalanceOf::<Test>::zero(),
            replies_count: <Test as Trait>::ReplyId::zero(),
            last_edited: frame_system::Module::<Test>::block_number(),
            video: member_video_id,
        };

        // next post increased by one
        let new_post_id = Content::next_post_id();

        // Effects:
        // Content::NextPostId increased by 1
        assert_eq!(new_post_id - post_id, 1);

        // Post is added to the storage
        assert_eq!(Content::post_by_id(post_id), post);

        // Event::<Test>::PostCreated is deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::PostCreated(
                ContentActor::Member(FIRST_MEMBER_ID),
                member_video_id,
                post_id
            ))
        );
    })
}

#[test]
fn non_authorized_actor_cannot_edit_post() {
    with_default_mock_builder(|| {
        let (post_id, member_video_id) = setup_testing_scenario_with_post();

        // non owner cannot edit post
        assert_err!(
            Content::edit_post(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                post_id,
                <tests::mock::Test as Trait>::VideoId::from(member_video_id + 1),
                ContentActor::Member(SECOND_MEMBER_ID),
            ),
            Error::<Test>::ActorNotAuthorized,
        );

        // unknown member/actor cannot edit posts
        assert_err!(
            Content::edit_post(
                Origin::signed(UNKNOWN_ORIGIN),
                post_id,
                <tests::mock::Test as Trait>::VideoId::from(member_video_id + 1),
                ContentActor::Member(UNKNOWN_MEMBER_ID),
            ),
            Error::<Test>::MemberAuthFailed,
        );
    })
}

#[test]
fn cannot_edit_nonexistent_post() {
    with_default_mock_builder(|| {
        let (_post_id, member_video_id) = setup_testing_scenario_with_post();

        assert_err!(
            Content::edit_post(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                UNKNOWN_POST_ID,
                <tests::mock::Test as Trait>::VideoId::from(member_video_id + 1),
                ContentActor::Member(FIRST_MEMBER_ID),
            ),
            Error::<Test>::PostDoesNotExist,
        );
    })
}

#[test]
fn verify_edit_post_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (post_id, member_video_id) = setup_testing_scenario_with_post();

        assert_ok!(Content::edit_post(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            post_id,
            <tests::mock::Test as Trait>::VideoId::from(member_video_id + 1),
            ContentActor::Member(FIRST_MEMBER_ID)
        ));

        // post video is updated
        assert_eq!(Content::post_by_id(post_id).video, member_video_id + 1);

        // event is deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::PostModified(
                ContentActor::Member(FIRST_MEMBER_ID),
                member_video_id + 1,
                post_id,
            ))
        );
    })
}

// delete post tests
#[test]
fn cannot_delete_nonexistent_post() {
    with_default_mock_builder(|| {
        let (_post_id, _member_video_id) = setup_testing_scenario_with_post();
        assert_err!(
            Content::delete_post(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                UNKNOWN_POST_ID,
                ContentActor::Member(FIRST_MEMBER_ID),
            ),
            Error::<Test>::PostDoesNotExist,
        );
    })
}

#[test]
fn non_authorized_actor_cannot_delete_post() {
    with_default_mock_builder(|| {
        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        // non authorized member
        assert_err!(
            Content::delete_post(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                post_id,
                ContentActor::Member(SECOND_MEMBER_ID),
            ),
            Error::<Test>::ActorNotAuthorized,
        );

        // nonexistent member
        assert_err!(
            Content::delete_post(
                Origin::signed(UNKNOWN_ORIGIN),
                post_id,
                ContentActor::Member(UNKNOWN_MEMBER_ID),
            ),
            Error::<Test>::MemberAuthFailed,
        );
    })
}

#[test]
fn verify_delete_post_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        assert_ok!(Content::delete_post(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            post_id,
            ContentActor::Member(FIRST_MEMBER_ID)
        ));

        // event is deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::PostDeleted(
                ContentActor::Member(FIRST_MEMBER_ID),
                post_id
            ))
        );
    })
}

// create reply tests

#[test]
fn nonexistent_member_cannot_reply() {
    with_default_mock_builder(|| {
        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        assert_err!(
            Content::create_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(UNKNOWN_MEMBER_ID),
                post_id,
                None,
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
            ),
            Error::<Test>::MemberAuthFailed,
        );
    })
}

#[test]
fn cannot_reply_to_nonexistent_post_or_reply() {
    with_default_mock_builder(|| {
        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        assert_err!(
            Content::create_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                <tests::mock::Test as Trait>::PostId::from(UNKNOWN_POST_ID),
                None,
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
            ),
            Error::<Test>::PostDoesNotExist,
        );

        assert_err!(
            Content::create_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                <tests::mock::Test as Trait>::PostId::from(post_id),
                Some(<tests::mock::Test as Trait>::ReplyId::from(
                    UNKNOWN_REPLY_ID
                )),
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
            ),
            Error::<Test>::ReplyDoesNotExist,
        );
    })
}

fn setup_testing_scenario_with_replies() -> (
    <tests::mock::Test as Trait>::ReplyId,
    <tests::mock::Test as Trait>::PostId,
) {
    // scenario C : scenario B + reply (reply.1) to the post by FIRST_MEMBER_ID and reply to reply.1
    // by FIRST_MEMBER_ID
    let (post_id, _member_video_id) = setup_testing_scenario_with_post();

    assert_ok!(Content::create_reply(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
        post_id,
        None,
        <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
    ));

    let reply_id = Content::post_by_id(post_id).replies_count;

    assert_ok!(Content::create_reply(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
        post_id,
        Some(reply_id),
        <tests::mock::Test as frame_system::Trait>::Hashing::hash(&reply_id.encode()),
    ));
    (reply_id, post_id)
}

#[test]
fn verify_create_reply_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        // replies count increased
        assert_eq!(Content::post_by_id(post_id).replies_count - reply_id, 1);

        // event deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ReplyCreated(
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                Content::post_by_id(post_id).replies_count,
            ))
        );
    })
}

// edit_reply
#[test]
fn cannot_edit_nonexistent_reply() {
    with_default_mock_builder(|| {
        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        // reply to an nonexistent reply
        assert_err!(
            Content::edit_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                <tests::mock::Test as Trait>::ReplyId::from(UNKNOWN_REPLY_ID),
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&reply_id.encode()),
            ),
            Error::<Test>::ReplyDoesNotExist,
        );

        // reply to a nonexistent post
        assert_err!(
            Content::edit_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                <tests::mock::Test as Trait>::PostId::from(UNKNOWN_POST_ID),
                reply_id,
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&reply_id.encode()),
            ),
            Error::<Test>::ReplyDoesNotExist,
        );
    })
}

#[test]
fn non_authorized_member_cannot_edit_reply() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        // non owner cannot edit reply
        assert_err!(
            Content::edit_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(SECOND_MEMBER_ID),
                post_id,
                reply_id,
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&reply_id.encode()),
            ),
            Error::<Test>::MemberAuthFailed,
        );

        // non existing member cannot edit reply
        assert_err!(
            Content::edit_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(UNKNOWN_MEMBER_ID),
                post_id,
                reply_id,
                <tests::mock::Test as frame_system::Trait>::Hashing::hash(&reply_id.encode()),
            ),
            Error::<Test>::MemberAuthFailed,
        );
    })
}

// edit_reply
#[test]
fn cannot_delete_reply_to_nonexistent_reply_or_post() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        // reply id is not valid
        assert_err!(
            Content::delete_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                <tests::mock::Test as Trait>::ReplyId::from(UNKNOWN_REPLY_ID),
            ),
            Error::<Test>::ReplyDoesNotExist,
        );

        // postid is not valid
        assert_err!(
            Content::delete_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                <tests::mock::Test as Trait>::PostId::from(UNKNOWN_POST_ID),
                reply_id,
            ),
            Error::<Test>::ReplyDoesNotExist,
        );
    })
}

#[test]
fn non_authorized_member_cannot_delete_reply() {
    with_default_mock_builder(|| {
        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        // non owner member cannot delete reply
        assert_err!(
            Content::delete_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(SECOND_MEMBER_ID),
                post_id,
                reply_id,
            ),
            Error::<Test>::MemberAuthFailed,
        );

        // non existing member cannot delete reply
        assert_err!(
            Content::delete_reply(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(UNKNOWN_MEMBER_ID),
                post_id,
                reply_id,
            ),
            Error::<Test>::MemberAuthFailed,
        );
    })
}

#[test]
fn verify_edit_reply_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (reply_id, post_id) = setup_testing_scenario_with_replies();

        assert_ok!(Content::edit_reply(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
            post_id,
            reply_id,
            <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
        ));

        // replies count increased
        assert_eq!(
            Content::reply_by_id(post_id, reply_id).text,
            <tests::mock::Test as frame_system::Trait>::Hashing::hash(&post_id.encode()),
        );

        // event deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ReplyModified(
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                reply_id,
            ))
        );
    })
}

// delete reply
// behavior on unsatisfied preconditions is the same as edit_reply, testing effects only...
#[test]
fn verify_delete_sub_reply_effects() {
    with_default_mock_builder(|| {
        // deleting (the last) reply to a reply
        run_to_block(1);

        let (_reply_id, post_id) = setup_testing_scenario_with_replies();

        let replies_count_pre = Content::post_by_id(post_id).replies_count;

        assert_ok!(Content::delete_reply(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
            post_id,
            replies_count_pre,
        ));

        let replies_count_post = Content::post_by_id(post_id).replies_count;

        // replies count decreased
        assert_eq!(replies_count_pre - replies_count_post, 1);

        // event deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ReplyDeleted(
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                replies_count_pre,
            ))
        );
    })
}

#[test]
fn verify_delete_parent_reply_effects() {
    with_default_mock_builder(|| {
        // deleting parent reply
        run_to_block(1);

        let (parent_reply_id, post_id) = setup_testing_scenario_with_replies();

        let replies_count_pre = Content::post_by_id(post_id).replies_count;

        assert_ok!(Content::delete_reply(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
            post_id,
            parent_reply_id,
        ));

        let replies_count_post = Content::post_by_id(post_id).replies_count;

        // replies count decreased
        assert_eq!(replies_count_pre - replies_count_post, 1);

        // event deposited
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ReplyDeleted(
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                parent_reply_id,
            ))
        );
    })
}

#[test]
fn nonexistent_member_cannot_react_to_post() {
    with_default_mock_builder(|| {
        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        assert_err!(
            Content::react_to_post(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                <tests::mock::Test as MembershipTypes>::MemberId::from(UNKNOWN_MEMBER_ID),
                post_id,
                <tests::mock::Test as Trait>::PostReactionId::from(1u64),
            ),
            Error::<Test>::MemberAuthFailed
        );
    })
}

#[test]
fn verify_react_to_post_effects() {
    with_default_mock_builder(|| {
        run_to_block(1);

        let (post_id, _member_video_id) = setup_testing_scenario_with_post();

        let reaction_id = <tests::mock::Test as Trait>::PostReactionId::from(1u64);

        assert_ok!(Content::react_to_post(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
            post_id,
            reaction_id
        ));

        // deposit event
        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::content(RawEvent::ReactionToPost(
                <tests::mock::Test as MembershipTypes>::MemberId::from(FIRST_MEMBER_ID),
                post_id,
                reaction_id,
            ))
        );
    })
}
