//! # Proposals discussion module
//! Proposals `discussion` module for the Joystream platform. Version 2.
//! It contains discussion system of the proposals.
//!
//! ## Overview
//!
//! The proposals discussion module is used by the codex module to provide a platform for discussions
//! about different proposals. It allows to create discussion threads and then add and update related
//! posts.
//!
//! ## Supported extrinsics
//! - [add_post](./struct.Module.html#method.add_post) - adds a post to an existing discussion thread
//! - [update_post](./struct.Module.html#method.update_post) - updates existing post
//!
//! ## Public API methods
//! - [create_thread](./struct.Module.html#method.create_thread) - creates a discussion thread
//! - [ensure_can_create_thread](./struct.Module.html#method.ensure_can_create_thread) - ensures safe thread creation
//!
//! ## Usage
//!
//! ```
//! use frame_support::decl_module;
//! use frame_system::ensure_root;
//! use pallet_proposals_discussion::{self as discussions};
//!
//! pub trait Config: discussions::Config + membership::Config {}
//!
//! decl_module! {
//!     pub struct Module<T: Config> for enum Call where origin: T::Origin {
//!         #[weight = 10_000_000]
//!         pub fn create_discussion(origin, title: Vec<u8>, author_id : T::MemberId) {
//!             ensure_root(origin)?;
//!             <discussions::Module<T>>::ensure_can_create_thread(author_id, &title)?;
//!             <discussions::Module<T>>::create_thread(author_id, title)?;
//!         }
//!     }
//! }
//! # fn main() {}
//! ```

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

#[cfg(test)]
mod tests;
mod types;

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::Get;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter};
use sp_std::clone::Clone;
use sp_std::vec::Vec;

use common::origin::ActorOriginValidator;
use types::{DiscussionPost, DiscussionThread, ThreadCounter};

type MemberId<T> = <T as membership::Config>::MemberId;

decl_event!(
    /// Proposals engine events
    pub enum Event<T>
    where
        <T as Config>::ThreadId,
        MemberId = MemberId<T>,
        <T as Config>::PostId,
    {
        /// Emits on thread creation.
        ThreadCreated(ThreadId, MemberId),

        /// Emits on post creation.
        PostCreated(PostId, MemberId),

        /// Emits on post update.
        PostUpdated(PostId, MemberId),
    }
);

/// 'Proposal discussion' substrate module Config
pub trait Config: frame_system::Config + membership::Config {
    /// Discussion event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// Validates post author id and origin combination
    type PostAuthorOriginValidator: ActorOriginValidator<
        Self::Origin,
        MemberId<Self>,
        Self::AccountId,
    >;

    /// Discussion thread Id type
    type ThreadId: From<u64> + Into<u64> + Parameter + Default + Copy;

    /// Post Id type
    type PostId: From<u64> + Parameter + Default + Copy;

    /// Defines post edition number limit.
    type MaxPostEditionNumber: Get<u32>;

    /// Defines thread title length limit.
    type ThreadTitleLengthLimit: Get<u32>;

    /// Defines post length limit.
    type PostLengthLimit: Get<u32>;

    /// Defines max thread by same author in a row number limit.
    type MaxThreadInARowNumber: Get<u32>;
}

decl_error! {
    /// Discussion module predefined errors
    pub enum Error for Module<T: Config> {
        /// Author should match the post creator
        NotAuthor,

        ///  Post edition limit reached
        PostEditionNumberExceeded,

        /// Discussion cannot have an empty title
        EmptyTitleProvided,

        /// Title is too long
        TitleIsTooLong,

        /// Thread doesn't exist
        ThreadDoesntExist,

        /// Post doesn't exist
        PostDoesntExist,

        /// Post cannot be empty
        EmptyPostProvided,

        /// Post is too long
        PostIsTooLong,

        /// Max number of threads by same author in a row limit exceeded
        MaxThreadInARowLimitExceeded,

        /// Require root origin in extrinsics
        RequireRootOrigin,
    }
}

// Storage for the proposals discussion module
decl_storage! {
    pub trait Store for Module<T: Config> as ProposalDiscussion {
        /// Map thread identifier to corresponding thread.
        pub ThreadById get(fn thread_by_id): map hasher(blake2_128_concat)
            T::ThreadId => DiscussionThread<MemberId<T>, T::BlockNumber>;

        /// Count of all threads that have been created.
        pub ThreadCount get(fn thread_count): u64;

        /// Map thread id and post id to corresponding post.
        pub PostThreadIdByPostId:
            double_map hasher(blake2_128_concat) T::ThreadId, hasher(blake2_128_concat) T::PostId =>
                DiscussionPost<MemberId<T>, T::BlockNumber, T::ThreadId>;

        /// Count of all posts that have been created.
        pub PostCount get(fn post_count): u64;

        /// Last author thread counter (part of the antispam mechanism)
        pub LastThreadAuthorCounter get(fn last_thread_author_counter):
            Option<ThreadCounter<MemberId<T>>>;
    }
}

decl_module! {
    /// 'Proposal discussion' substrate module
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Emits an event. Default substrate implementation.
        fn deposit_event() = default;

        /// Exports post edition number limit const.
        const MaxPostEditionNumber: u32 = T::MaxPostEditionNumber::get();

        /// Exports thread title length limit const.
        const ThreadTitleLengthLimit: u32 = T::ThreadTitleLengthLimit::get();

        /// Exports post length limit const.
        const PostLengthLimit: u32 = T::PostLengthLimit::get();

        /// Exports max thread by same author in a row number limit const.
        const MaxThreadInARowNumber: u32 = T::MaxThreadInARowNumber::get();

        /// Adds a post with author origin check.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_post(
            origin,
            post_author_id: MemberId<T>,
            thread_id : T::ThreadId,
            text : Vec<u8>
        ) {
            T::PostAuthorOriginValidator::ensure_actor_origin(
                origin,
                post_author_id,
            )?;
            ensure!(<ThreadById<T>>::contains_key(thread_id), Error::<T>::ThreadDoesntExist);

            ensure!(!text.is_empty(),Error::<T>::EmptyPostProvided);
            ensure!(
                text.len() as u32 <= T::PostLengthLimit::get(),
                Error::<T>::PostIsTooLong
            );

            // mutation

            let next_post_count_value = Self::post_count() + 1;
            let new_post_id = next_post_count_value;

            let new_post = DiscussionPost {
                text,
                created_at: Self::current_block(),
                updated_at: Self::current_block(),
                author_id: post_author_id,
                edition_number : 0,
                thread_id,
            };

            let post_id = T::PostId::from(new_post_id);
            <PostThreadIdByPostId<T>>::insert(thread_id, post_id, new_post);
            PostCount::put(next_post_count_value);
            Self::deposit_event(RawEvent::PostCreated(post_id, post_author_id));
       }

        /// Updates a post with author origin check. Update attempts number is limited.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_post(
            origin,
            post_author_id: MemberId<T>,
            thread_id: T::ThreadId,
            post_id : T::PostId,
            text : Vec<u8>
        ){
            T::PostAuthorOriginValidator::ensure_actor_origin(
                origin,
                post_author_id,
            )?;

            ensure!(<ThreadById<T>>::contains_key(thread_id), Error::<T>::ThreadDoesntExist);
            ensure!(<PostThreadIdByPostId<T>>::contains_key(thread_id, post_id), Error::<T>::PostDoesntExist);

            ensure!(!text.is_empty(), Error::<T>::EmptyPostProvided);
            ensure!(
                text.len() as u32 <= T::PostLengthLimit::get(),
                Error::<T>::PostIsTooLong
            );

            let post = <PostThreadIdByPostId<T>>::get(&thread_id, &post_id);

            ensure!(post.author_id == post_author_id, Error::<T>::NotAuthor);
            ensure!(post.edition_number < T::MaxPostEditionNumber::get(),
                Error::<T>::PostEditionNumberExceeded);

            let new_post = DiscussionPost {
                text,
                updated_at: Self::current_block(),
                edition_number: post.edition_number + 1,
                ..post
            };

            // mutation

            <PostThreadIdByPostId<T>>::insert(thread_id, post_id, new_post);
            Self::deposit_event(RawEvent::PostUpdated(post_id, post_author_id));
       }
    }
}

impl<T: Config> Module<T> {
    /// Create the discussion thread. Cannot add more threads than 'predefined limit = MaxThreadInARowNumber'
    /// times in a row by the same author.
    pub fn create_thread(
        thread_author_id: MemberId<T>,
        title: Vec<u8>,
    ) -> Result<T::ThreadId, DispatchError> {
        Self::ensure_can_create_thread(thread_author_id, &title)?;

        let next_thread_count_value = Self::thread_count() + 1;
        let new_thread_id = next_thread_count_value;

        let new_thread = DiscussionThread {
            title,
            created_at: Self::current_block(),
            author_id: thread_author_id,
        };

        // get new 'threads in a row' counter for the author
        let current_thread_counter = Self::get_updated_thread_counter(thread_author_id);

        // mutation

        let thread_id = T::ThreadId::from(new_thread_id);
        <ThreadById<T>>::insert(thread_id, new_thread);
        ThreadCount::put(next_thread_count_value);
        <LastThreadAuthorCounter<T>>::put(current_thread_counter);
        Self::deposit_event(RawEvent::ThreadCreated(thread_id, thread_author_id));

        Ok(thread_id)
    }

    /// Ensures thread can be created.
    /// Checks:
    /// - title is valid
    /// - max thread in a row by the same author
    pub fn ensure_can_create_thread(thread_author_id: MemberId<T>, title: &[u8]) -> DispatchResult {
        ensure!(!title.is_empty(), Error::<T>::EmptyTitleProvided);
        ensure!(
            title.len() as u32 <= T::ThreadTitleLengthLimit::get(),
            Error::<T>::TitleIsTooLong
        );

        // get new 'threads in a row' counter for the author
        let current_thread_counter = Self::get_updated_thread_counter(thread_author_id);

        ensure!(
            current_thread_counter.counter as u32 <= T::MaxThreadInARowNumber::get(),
            Error::<T>::MaxThreadInARowLimitExceeded
        );

        Ok(())
    }
}

impl<T: Config> Module<T> {
    // Wrapper-function over frame_system::block_number()
    fn current_block() -> T::BlockNumber {
        <frame_system::Pallet<T>>::block_number()
    }

    // returns incremented thread counter if last thread author equals with provided parameter
    fn get_updated_thread_counter(author_id: MemberId<T>) -> ThreadCounter<MemberId<T>> {
        // if thread counter exists
        if let Some(last_thread_author_counter) = Self::last_thread_author_counter() {
            // if last(previous) author is the same as current author
            if last_thread_author_counter.author_id == author_id {
                return last_thread_author_counter.increment();
            }
        }

        // else return new counter (set with 1 thread number)
        ThreadCounter::new(author_id)
    }
}
