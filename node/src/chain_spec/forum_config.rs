use codec::Decode;
use joystream_node_runtime::{
    common::constraints::InputValidationLengthConstraint,
    forum::{Category, CategoryId, Post, Thread},
    AccountId, BlockNumber, ForumConfig, Moment, PostId, ThreadId,
};
use serde::Deserialize;
use std::{fs, path::Path};

fn new_validation(min: u16, max_min_diff: u16) -> InputValidationLengthConstraint {
    InputValidationLengthConstraint { min, max_min_diff }
}

#[derive(Decode)]
struct ForumData {
    categories: Vec<Category<BlockNumber, Moment, AccountId>>,
    posts: Vec<Post<BlockNumber, Moment, AccountId, ThreadId, PostId>>,
    threads: Vec<Thread<BlockNumber, Moment, AccountId, ThreadId>>,
}

#[derive(Deserialize)]
struct EncodedForumData {
    /// hex encoded categories
    categories: Vec<String>,
    /// hex encoded posts
    posts: Vec<String>,
    /// hex encoded threads
    threads: Vec<String>,
}

impl EncodedForumData {
    fn decode(&self) -> ForumData {
        ForumData {
            categories: self
                .categories
                .iter()
                .map(|category| {
                    let encoded_category = hex::decode(&category[2..].as_bytes())
                        .expect("failed to parse category hex string");
                    Decode::decode(&mut encoded_category.as_slice()).unwrap()
                })
                .collect(),
            posts: self
                .posts
                .iter()
                .map(|post| {
                    let encoded_post = hex::decode(&post[2..].as_bytes())
                        .expect("failed to parse post hex string");
                    Decode::decode(&mut encoded_post.as_slice()).unwrap()
                })
                .collect(),
            threads: self
                .threads
                .iter()
                .map(|thread| {
                    let encoded_thread = hex::decode(&thread[2..].as_bytes())
                        .expect("failed to parse thread hex string");
                    Decode::decode(&mut encoded_thread.as_slice()).unwrap()
                })
                .collect(),
        }
    }
}

fn parse_forum_json(data_file: &Path) -> EncodedForumData {
    let data = fs::read_to_string(data_file).expect("Failed reading file");
    serde_json::from_str(&data).expect("failed parsing members data")
}

/// Generates a `ForumConfig` geneis config pre-populated with
/// categories, threads and posts parsed
/// from a json file serialized as `EncodedForumData`
pub fn from_json(forum_sudo: AccountId, data_file: &Path) -> ForumConfig {
    let forum_data = parse_forum_json(data_file);
    create(forum_sudo, forum_data)
}

/// Generates a basic empty `ForumConfig` geneis config
pub fn empty(forum_sudo: AccountId) -> ForumConfig {
    let forum_data = EncodedForumData {
        categories: vec![],
        threads: vec![],
        posts: vec![],
    };
    create(forum_sudo, forum_data)
}

fn create(forum_sudo: AccountId, forum_data: EncodedForumData) -> ForumConfig {
    let first_id = 1;
    let forum_data = forum_data.decode();

    let next_category_id: CategoryId = forum_data
        .categories
        .last()
        .map_or(first_id, |category| category.id + 1);

    assert_eq!(
        next_category_id,
        (forum_data.categories.len() + 1) as CategoryId
    );

    let next_thread_id: ThreadId = forum_data
        .threads
        .last()
        .map_or(first_id, |thread| thread.id + 1);

    assert_eq!(next_thread_id, (forum_data.threads.len() + 1) as ThreadId);

    let next_post_id: PostId = forum_data.posts.last().map_or(first_id, |post| post.id + 1);

    assert_eq!(next_post_id, (forum_data.posts.len() + 1) as PostId);

    ForumConfig {
        category_by_id: forum_data
            .categories
            .into_iter()
            .map(|encoded_category| {
                let category = encoded_category;
                (category.id, category)
            })
            .collect(),
        thread_by_id: forum_data
            .threads
            .into_iter()
            .map(|encoded_thread| {
                let thread = encoded_thread;
                (thread.id, thread)
            })
            .collect(),
        post_by_id: forum_data
            .posts
            .into_iter()
            .map(|encoded_post| {
                let post = encoded_post;
                (post.id, post)
            })
            .collect(),
        next_category_id,
        next_thread_id,
        next_post_id,
        forum_sudo,
        category_title_constraint: new_validation(10, 90),
        category_description_constraint: new_validation(10, 490),
        thread_title_constraint: new_validation(10, 90),
        post_text_constraint: new_validation(10, 2990),
        thread_moderation_rationale_constraint: new_validation(10, 290),
        post_moderation_rationale_constraint: new_validation(10, 290),
    }
}
