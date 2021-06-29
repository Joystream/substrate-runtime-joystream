mod mock;

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_system::RawOrigin;
use frame_system::{EventRecord, Phase};

use crate::*;
use mock::{Event, *};

struct EventFixture;
impl EventFixture {
    fn assert_events(expected_raw_events: Vec<RawEvent<u64, u64, u64>>) {
        let expected_events = expected_raw_events
            .iter()
            .map(|ev| EventRecord {
                phase: Phase::Initialization,
                event: Event::proposals_discussion(ev.clone()),
                topics: vec![],
            })
            .collect::<Vec<EventRecord<_, _>>>();

        assert_eq!(System::events(), expected_events);
    }
}

struct TestPostEntry {
    pub post_id: u64,
    pub text: Vec<u8>,
    pub edition_number: u32,
}

struct TestThreadEntry {
    pub thread_id: u64,
    pub title: Vec<u8>,
}

fn assert_thread_content(thread_entry: TestThreadEntry, post_entries: Vec<TestPostEntry>) {
    assert!(<ThreadById<Test>>::contains_key(thread_entry.thread_id));

    let actual_thread = <ThreadById<Test>>::get(thread_entry.thread_id);
    let expected_thread = DiscussionThread {
        title: thread_entry.title,
        created_at: 0,
        author_id: 1,
    };
    assert_eq!(actual_thread, expected_thread);

    for post_entry in post_entries {
        let actual_post =
            <PostThreadIdByPostId<Test>>::get(thread_entry.thread_id, post_entry.post_id);
        let expected_post = DiscussionPost {
            text: post_entry.text,
            created_at: 0,
            updated_at: 0,
            author_id: 1,
            thread_id: thread_entry.thread_id,
            edition_number: post_entry.edition_number,
        };

        assert_eq!(actual_post, expected_post);
    }
}

struct DiscussionFixture {
    pub title: Vec<u8>,
    pub origin: RawOrigin<u64>,
    pub author_id: u64,
}

impl Default for DiscussionFixture {
    fn default() -> Self {
        DiscussionFixture {
            title: b"title".to_vec(),
            origin: RawOrigin::Signed(1),
            author_id: 1,
        }
    }
}

impl DiscussionFixture {
    fn with_title(self, title: Vec<u8>) -> Self {
        DiscussionFixture { title, ..self }
    }

    fn create_discussion_and_assert(&self, result: Result<u64, DispatchError>) -> Option<u64> {
        let create_discussion_result =
            Discussions::create_thread(self.author_id, self.title.clone());

        assert_eq!(create_discussion_result, result);

        create_discussion_result.ok()
    }
}

struct PostFixture {
    pub text: Vec<u8>,
    pub origin: RawOrigin<u64>,
    pub thread_id: u64,
    pub post_id: Option<u64>,
    pub author_id: u64,
}

impl PostFixture {
    fn default_for_thread(thread_id: u64) -> Self {
        PostFixture {
            text: b"text".to_vec(),
            author_id: 1,
            thread_id,
            origin: RawOrigin::Signed(1),
            post_id: None,
        }
    }

    fn with_text(self, text: Vec<u8>) -> Self {
        PostFixture { text, ..self }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        PostFixture { origin, ..self }
    }

    fn with_author(self, author_id: u64) -> Self {
        PostFixture { author_id, ..self }
    }

    fn change_thread_id(self, thread_id: u64) -> Self {
        PostFixture { thread_id, ..self }
    }

    fn change_post_id(self, post_id: u64) -> Self {
        PostFixture {
            post_id: Some(post_id),
            ..self
        }
    }

    fn add_post_and_assert(&mut self, result: DispatchResult) -> Option<u64> {
        let add_post_result = Discussions::add_post(
            self.origin.clone().into(),
            self.author_id,
            self.thread_id,
            self.text.clone(),
        );

        assert_eq!(add_post_result, result);

        if result.is_ok() {
            self.post_id = Some(<PostCount>::get());
        }

        self.post_id
    }

    fn update_post_with_text_and_assert(&mut self, new_text: Vec<u8>, result: DispatchResult) {
        let add_post_result = Discussions::update_post(
            self.origin.clone().into(),
            self.author_id,
            self.thread_id,
            self.post_id.unwrap(),
            new_text,
        );

        assert_eq!(add_post_result, result);
    }

    fn update_post_and_assert(&mut self, result: DispatchResult) {
        self.update_post_with_text_and_assert(self.text.clone(), result);
    }
}

#[test]
fn create_discussion_call_succeeds() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();

        discussion_fixture.create_discussion_and_assert(Ok(1));
    });
}

#[test]
fn create_post_call_succeeds() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();

        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture = PostFixture::default_for_thread(thread_id);

        post_fixture.add_post_and_assert(Ok(()));
    });
}

#[test]
fn update_post_call_succeeds() {
    initial_test_ext().execute_with(|| {
        /*
           Events are not emitted on block 0.
           So any dispatchable calls made during genesis block formation will have no events emitted.
           https://substrate.dev/recipes/2-appetizers/4-events.html
        */
        run_to_block(1);

        let discussion_fixture = DiscussionFixture::default();

        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture = PostFixture::default_for_thread(thread_id);

        post_fixture.add_post_and_assert(Ok(()));
        post_fixture.update_post_and_assert(Ok(()));

        EventFixture::assert_events(vec![
            RawEvent::ThreadCreated(1, 1),
            RawEvent::PostCreated(1, 1),
            RawEvent::PostUpdated(1, 1),
        ]);
    });
}

#[test]
fn update_post_call_fails_because_of_post_edition_limit() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();

        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture = PostFixture::default_for_thread(thread_id);

        post_fixture.add_post_and_assert(Ok(()));

        for _ in 1..6 {
            post_fixture.update_post_and_assert(Ok(()));
        }

        post_fixture.update_post_and_assert(Err(Error::<Test>::PostEditionNumberExceeded.into()));
    });
}

#[test]
fn update_post_call_fails_because_of_the_wrong_author() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();

        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture = PostFixture::default_for_thread(thread_id);

        post_fixture.add_post_and_assert(Ok(()));

        post_fixture = post_fixture.with_author(2);

        post_fixture.update_post_and_assert(Err(DispatchError::Other("Invalid author")));

        post_fixture = post_fixture.with_origin(RawOrigin::None).with_author(2);

        post_fixture.update_post_and_assert(Err(Error::<Test>::NotAuthor.into()));
    });
}

#[test]
fn thread_content_check_succeeded() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();

        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id);
        let post_id1 = post_fixture1.add_post_and_assert(Ok(())).unwrap();

        let mut post_fixture2 = PostFixture::default_for_thread(thread_id);
        let post_id2 = post_fixture2.add_post_and_assert(Ok(())).unwrap();
        post_fixture1.update_post_with_text_and_assert(b"new_text".to_vec(), Ok(()));

        assert_thread_content(
            TestThreadEntry {
                thread_id,
                title: b"title".to_vec(),
            },
            vec![
                TestPostEntry {
                    post_id: post_id1,
                    text: b"new_text".to_vec(),
                    edition_number: 1,
                },
                TestPostEntry {
                    post_id: post_id2,
                    text: b"text".to_vec(),
                    edition_number: 0,
                },
            ],
        );
    });
}

#[test]
fn create_discussion_call_with_bad_title_failed() {
    initial_test_ext().execute_with(|| {
        let mut discussion_fixture = DiscussionFixture::default().with_title(Vec::new());
        discussion_fixture
            .create_discussion_and_assert(Err(Error::<Test>::EmptyTitleProvided.into()));

        discussion_fixture = DiscussionFixture::default().with_title([0; 201].to_vec());
        discussion_fixture.create_discussion_and_assert(Err(Error::<Test>::TitleIsTooLong.into()));
    });
}

#[test]
fn add_post_call_with_invalid_thread_failed() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture = PostFixture::default_for_thread(2);
        post_fixture.add_post_and_assert(Err(Error::<Test>::ThreadDoesntExist.into()));
    });
}

#[test]
fn update_post_call_with_invalid_post_failed() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id);
        post_fixture1.add_post_and_assert(Ok(())).unwrap();

        let mut post_fixture2 = post_fixture1.change_post_id(2);
        post_fixture2.update_post_and_assert(Err(Error::<Test>::PostDoesntExist.into()));
    });
}

#[test]
fn update_post_call_with_invalid_thread_failed() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id);
        post_fixture1.add_post_and_assert(Ok(())).unwrap();

        let mut post_fixture2 = post_fixture1.change_thread_id(2);
        post_fixture2.update_post_and_assert(Err(Error::<Test>::ThreadDoesntExist.into()));
    });
}

#[test]
fn add_post_call_with_invalid_text_failed() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id).with_text(Vec::new());
        post_fixture1.add_post_and_assert(Err(Error::<Test>::EmptyPostProvided.into()));

        let mut post_fixture2 =
            PostFixture::default_for_thread(thread_id).with_text([0; 2001].to_vec());
        post_fixture2.add_post_and_assert(Err(Error::<Test>::PostIsTooLong.into()));
    });
}

#[test]
fn update_post_call_with_invalid_text_failed() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id);
        post_fixture1.add_post_and_assert(Ok(()));

        let mut post_fixture2 = post_fixture1.with_text(Vec::new());
        post_fixture2.update_post_and_assert(Err(Error::<Test>::EmptyPostProvided.into()));

        let mut post_fixture3 = post_fixture2.with_text([0; 2001].to_vec());
        post_fixture3.update_post_and_assert(Err(Error::<Test>::PostIsTooLong.into()));
    });
}

#[test]
fn add_discussion_thread_fails_because_of_max_thread_by_same_author_in_a_row_limit_exceeded() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        for idx in 1..=3 {
            discussion_fixture
                .create_discussion_and_assert(Ok(idx))
                .unwrap();
        }

        discussion_fixture
            .create_discussion_and_assert(Err(Error::<Test>::MaxThreadInARowLimitExceeded.into()));
    });
}

#[test]
fn discussion_thread_and_post_counters_are_valid() {
    initial_test_ext().execute_with(|| {
        let discussion_fixture = DiscussionFixture::default();
        let thread_id = discussion_fixture
            .create_discussion_and_assert(Ok(1))
            .unwrap();

        let mut post_fixture1 = PostFixture::default_for_thread(thread_id);
        let _ = post_fixture1.add_post_and_assert(Ok(())).unwrap();

        assert_eq!(Discussions::thread_count(), 1);
        assert_eq!(Discussions::post_count(), 1);
    });
}
