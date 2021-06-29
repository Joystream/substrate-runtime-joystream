#![cfg(test)]

use super::curators;
use super::mock::{Event, *};
use crate::*;
use frame_support::{assert_err, assert_ok};

fn create_member_channel() -> ChannelId {
    let channel_id = Content::next_channel_id();

    // Member can create the channel
    assert_ok!(Content::create_channel(
        Origin::signed(FIRST_MEMBER_ORIGIN),
        ContentActor::Member(FIRST_MEMBER_ID),
        ChannelCreationParameters {
            assets: vec![],
            meta: vec![],
            reward_account: None,
        }
    ));

    channel_id
}

#[test]
fn member_can_create_videos() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);
        let channel_id = create_member_channel();

        let video_id = Content::next_video_id();
        assert_ok!(Content::create_video(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            channel_id,
            VideoCreationParameters {
                assets: vec![NewAsset::Urls(vec![b"https://somewhere.com/".to_vec()])],
                meta: b"metablob".to_vec(),
            }
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::VideoCreated(
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id,
                video_id,
                VideoCreationParameters {
                    assets: vec![NewAsset::Urls(vec![b"https://somewhere.com/".to_vec()])],
                    meta: b"metablob".to_vec(),
                }
            ))
        );

        // Video is created in correct channel
        let video = Content::video_by_id(video_id);
        assert_eq!(channel_id, video.in_channel);

        // Can update own video
        assert_ok!(Content::update_video(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            video_id,
            VideoUpdateParameters {
                assets: Some(vec![NewAsset::Urls(vec![
                    b"https://somewhere-else.com/".to_vec()
                ])]),
                new_meta: Some(b"newmetablob".to_vec()),
            }
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::VideoUpdated(
                ContentActor::Member(FIRST_MEMBER_ID),
                video_id,
                VideoUpdateParameters {
                    assets: Some(vec![NewAsset::Urls(vec![
                        b"https://somewhere-else.com/".to_vec()
                    ])]),
                    new_meta: Some(b"newmetablob".to_vec()),
                }
            ))
        );

        // Member cannot create video in a channel they do not own
        assert_err!(
            Content::create_video(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                ContentActor::Member(SECOND_MEMBER_ID),
                channel_id,
                VideoCreationParameters {
                    assets: vec![],
                    meta: vec![],
                }
            ),
            Error::<Test>::ActorNotAuthorized
        );

        // Member cannot update video in a channel they do not own
        assert_err!(
            Content::update_video(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                ContentActor::Member(SECOND_MEMBER_ID),
                video_id,
                VideoUpdateParameters {
                    assets: None,
                    new_meta: None,
                }
            ),
            Error::<Test>::ActorNotAuthorized
        );

        // Member cannot delete video in a channel they do not own
        assert_err!(
            Content::delete_video(
                Origin::signed(SECOND_MEMBER_ORIGIN),
                ContentActor::Member(SECOND_MEMBER_ID),
                video_id
            ),
            Error::<Test>::ActorNotAuthorized
        );

        // Owner can delete their video
        assert_ok!(Content::delete_video(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            video_id
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::VideoDeleted(
                ContentActor::Member(FIRST_MEMBER_ID),
                video_id
            ))
        );
    })
}

#[test]
fn curators_can_censor_videos() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);
        let channel_id = create_member_channel();

        let video_id = Content::next_video_id();
        assert_ok!(Content::create_video(
            Origin::signed(FIRST_MEMBER_ORIGIN),
            ContentActor::Member(FIRST_MEMBER_ID),
            channel_id,
            VideoCreationParameters {
                assets: vec![NewAsset::Urls(vec![b"https://somewhere.com/".to_vec()])],
                meta: b"metablob".to_vec(),
            }
        ));

        let group_id = curators::add_curator_to_new_group(FIRST_CURATOR_ID);

        // Curator can censor videos
        let is_censored = true;
        assert_ok!(Content::update_video_censorship_status(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(group_id, FIRST_CURATOR_ID),
            video_id,
            is_censored,
            vec![]
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::VideoCensorshipStatusUpdated(
                ContentActor::Curator(group_id, FIRST_CURATOR_ID),
                video_id,
                is_censored,
                vec![]
            ))
        );

        let video = Content::video_by_id(video_id);

        assert!(video.is_censored);

        // Curator can un-censor videos
        let is_censored = false;
        assert_ok!(Content::update_video_censorship_status(
            Origin::signed(FIRST_CURATOR_ORIGIN),
            ContentActor::Curator(group_id, FIRST_CURATOR_ID),
            video_id,
            is_censored,
            vec![]
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::VideoCensorshipStatusUpdated(
                ContentActor::Curator(group_id, FIRST_CURATOR_ID),
                video_id,
                is_censored,
                vec![]
            ))
        );

        let video = Content::video_by_id(video_id);

        assert!(!video.is_censored);

        // Members cannot censor videos
        assert_err!(
            Content::update_video_censorship_status(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                ContentActor::Member(FIRST_MEMBER_ID),
                channel_id,
                true,
                vec![]
            ),
            Error::<Test>::ActorNotAuthorized
        );
    })
}

#[test]
fn featured_videos() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        // Lead can update curator owned channels
        assert_ok!(Content::set_featured_videos(
            Origin::signed(LEAD_ORIGIN),
            ContentActor::Lead,
            vec![1, 2, 3]
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            Event::content(RawEvent::FeaturedVideosSet(
                ContentActor::Lead,
                vec![1, 2, 3]
            ))
        );

        assert_err!(
            Content::set_featured_videos(
                Origin::signed(FIRST_MEMBER_ORIGIN),
                ContentActor::Member(FIRST_MEMBER_ID),
                vec![1, 2, 3]
            ),
            Error::<Test>::ActorNotAuthorized
        );
    })
}
