//! # Data directory module
//! Data directory module for the Joystream platform manages IPFS content id, storage providers,
//! owners of the content. It allows to add and accept or reject the content in the frame_system.
//!
//! ## Comments
//!
//! Data object type registry module uses  working group module to authorize actions.
//!
//! ## Supported extrinsics
//!
//! ### Public extrinsic
//! - [add_content](./struct.Module.html#method.add_content) - Adds the content to the frame_system.
//!
//! ### Private extrinsics
//! - accept_content - Storage provider accepts a content.
//! - reject_content - Storage provider rejects a content.
//! - remove_known_content_id - Removes the content id from the list of known content ids. Requires root privileges.
//! - set_known_content_id - Sets the content id from the list of known content ids. Requires root privileges.
//!

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::ensure_root;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use common::origin::ActorOriginValidator;
pub use common::storage::{ContentParameters, StorageObjectOwner};
pub(crate) use common::BlockAndTime;

use crate::data_object_type_registry;
use crate::data_object_type_registry::IsActiveDataObjectType;
use crate::*;

/// The default maximum storage size (bytes) that lead can set on the voucher of an owner
pub const DEFAULT_VOUCHER_SIZE_LIMIT_UPPER_BOUND: u64 = 540_000_000_000;
/// The default maximum number of objects that lead can set on the voucher of an owner
pub const DEFAULT_VOUCHER_OBJECTS_LIMIT_UPPER_BOUND: u64 = 15_000;
/// The default frame_system global storage limits
pub const DEFAULT_GLOBAL_VOUCHER: Voucher = Voucher::new(110_000_000_000_000, 10_000_000);
/// The default initial owner voucher
pub const DEFAULT_VOUCHER: Voucher = Voucher::new(110_000_000_000, 5_000);
/// The default starting upload blocked status
pub const DEFAULT_UPLOADING_BLOCKED_STATUS: bool = false;

/// The _Data directory_ main _Trait_.
pub trait Config:
    pallet_timestamp::Config
    + frame_system::Config
    + data_object_type_registry::Config
    + membership::Config
    + working_group::Config<StorageWorkingGroupInstance>
    + common::MembershipTypes
    + common::StorageOwnership
{
    /// _Data directory_ event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// Active data object type validator.
    type IsActiveDataObjectType: data_object_type_registry::IsActiveDataObjectType<Self>;

    /// Validates member id and origin combination.
    type MemberOriginValidator: ActorOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;
}

decl_error! {
    /// _Data object storage registry_ module predefined errors.
    pub enum Error for Module<T: Config>{
        /// Content with this ID not found.
        CidNotFound,

        /// Cannot create content for inactive or missing data object type.
        DataObjectTypeMustBeActive,

        /// "Data object already added under this content id".
        DataObjectAlreadyAdded,

        /// Require root origin in extrinsics.
        RequireRootOrigin,

        /// DataObject Injection Failed. Too Many DataObjects.
        DataObjectsInjectionExceededLimit,

        /// Contant uploading failed. Actor voucher objects limit exceeded.
        VoucherObjectsLimitExceeded,

        /// Contant uploading failed. Actor voucher size limit exceeded.
        VoucherSizeLimitExceeded,

        /// Voucher size limit upper bound exceeded
        VoucherSizeLimitUpperBoundExceeded,

        /// Voucher objects limit upper bound exceeded
        VoucherObjectsLimitUpperBoundExceeded,

        /// Content uploading blocked.
        ContentUploadingBlocked,

        /// Provided owner should be equal o the data object owner under given content id
        OwnersAreNotEqual,

        /// New voucher limit being set is less than used.
        VoucherLimitLessThanUsed,

        /// Overflow detected when changing
        VoucherOverflow,
    }
}

/// The status of the content which can be updated by a storage provider.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Debug)]
pub enum LiaisonJudgement {
    /// Content awaits for a judgment.
    Pending,

    /// Content accepted.
    Accepted,
}

impl Default for LiaisonJudgement {
    fn default() -> Self {
        LiaisonJudgement::Pending
    }
}

/// Alias for DataObjectInternal
pub type DataObject<T> = DataObjectInternal<
    MemberId<T>,
    ChannelId<T>,
    DAOId<T>,
    <T as frame_system::Config>::BlockNumber,
    <T as pallet_timestamp::Config>::Moment,
    DataObjectTypeId<T>,
    StorageProviderId<T>,
>;

/// Manages content ids, type and storage provider decision about it.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Debug, Default)]
pub struct DataObjectInternal<
    MemberId,
    ChannelId,
    DAOId,
    BlockNumber,
    Moment,
    DataObjectTypeId,
    StorageProviderId,
> {
    /// Content owner.
    pub owner: StorageObjectOwner<MemberId, ChannelId, DAOId>,

    /// Content added at.
    pub added_at: BlockAndTime<BlockNumber, Moment>,

    /// Content type id.
    pub type_id: DataObjectTypeId,

    /// Content size in bytes.
    pub size: u64,

    /// Storage provider which first accepted the content.
    pub liaison: Option<StorageProviderId>,

    /// The liaison judgment.
    pub liaison_judgement: LiaisonJudgement,

    /// IPFS content id.
    pub ipfs_content_id: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct Delta {
    pub size: u64,
    pub objects: u64,
}

/// Uploading voucher for StorageObjectOwner
/// All fields are private. All changes should be done through the methods
/// to avoid invalid state.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, Debug, Default)]
pub struct Voucher {
    // Total objects size limit per StorageObjectOwner
    size_limit: u64,
    // Total objects number limit per StorageObjectOwner
    objects_limit: u64,
    size_used: u64,
    objects_used: u64,
}

impl Voucher {
    /// Create new voucher with provided size & objects limits
    pub const fn new(size_limit: u64, objects_limit: u64) -> Self {
        Self {
            size_limit,
            objects_limit,
            size_used: 0,
            objects_used: 0,
        }
    }

    pub fn get_size_limit(&self) -> u64 {
        self.size_limit
    }

    pub fn get_objects_limit(&self) -> u64 {
        self.objects_limit
    }

    pub fn get_size_used(&self) -> u64 {
        self.size_used
    }

    pub fn get_objects_used(&self) -> u64 {
        self.objects_used
    }

    /// Calculate voucher delta. We don't do checked_sub as the other methods
    /// protect against state which would result in underflow.
    pub fn calculate_delta(&self) -> Delta {
        Delta {
            size: self.size_limit - self.size_used,
            objects: self.objects_limit - self.objects_used,
        }
    }

    /// Attempts to fill voucher and returns an updated Voucher if no overlflows occur
    /// or limits are exceeded. Error otherwise.
    pub fn fill_voucher<T: Config>(self, voucher_delta: Delta) -> Result<Self, Error<T>> {
        if let Some(size_used) = self.size_used.checked_add(voucher_delta.size) {
            // Ensure size limit not exceeded
            ensure!(
                size_used <= self.size_limit,
                Error::<T>::VoucherSizeLimitExceeded
            );
            if let Some(objects_used) = self.objects_used.checked_add(voucher_delta.objects) {
                // Ensure objects limit not exceeded
                ensure!(
                    objects_used <= self.objects_limit,
                    Error::<T>::VoucherObjectsLimitExceeded
                );
                return Ok(Self {
                    size_used,
                    objects_used,
                    ..self
                });
            }
        }
        Err(Error::<T>::VoucherOverflow)
    }

    pub fn release_voucher<T: Config>(self, voucher_delta: Delta) -> Result<Self, Error<T>> {
        if let Some(size_used) = self.size_used.checked_sub(voucher_delta.size) {
            if let Some(objects_used) = self.objects_used.checked_sub(voucher_delta.objects) {
                return Ok(Self {
                    size_used,
                    objects_used,
                    ..self
                });
            }
        }
        Err(Error::<T>::VoucherOverflow)
    }

    pub fn set_new_size_limit<T: Config>(&mut self, new_size_limit: u64) -> Result<(), Error<T>> {
        if self.size_used > new_size_limit {
            Err(Error::<T>::VoucherLimitLessThanUsed)
        } else {
            self.size_limit = new_size_limit;
            Ok(())
        }
    }

    pub fn set_new_objects_limit<T: Config>(
        &mut self,
        new_objects_limit: u64,
    ) -> Result<(), Error<T>> {
        if self.objects_used > new_objects_limit {
            Err(Error::<T>::VoucherLimitLessThanUsed)
        } else {
            self.objects_limit = new_objects_limit;
            Ok(())
        }
    }
}

/// A map collection of unique DataObjects keyed by the ContentId
pub type DataObjectsMap<T> = BTreeMap<ContentId<T>, DataObject<T>>;

decl_storage! {
    trait Store for Module<T: Config> as DataDirectory {

        /// Maps data objects by their content id.
        pub DataByContentId get(fn data_object_by_content_id) config():
            map hasher(blake2_128_concat) T::ContentId => DataObject<T>;

        /// Maps storage owner to it`s voucher. Created when the first upload by the new actor occured.
        pub Vouchers get(fn vouchers) config():
            map hasher(blake2_128_concat) ObjectOwner<T> => Voucher;

        /// Upper bound for the Voucher size limit.
        pub VoucherSizeLimitUpperBound get(fn voucher_size_limit_upper_bound) config(): u64 = DEFAULT_VOUCHER_SIZE_LIMIT_UPPER_BOUND;

        /// Upper bound for the Voucher objects number limit.
        pub VoucherObjectsLimitUpperBound get(fn voucher_objects_limit_upper_bound) config(): u64 = DEFAULT_VOUCHER_OBJECTS_LIMIT_UPPER_BOUND;

        /// Default content voucher for all actors.
        pub DefaultVoucher get(fn default_voucher) config(): Voucher = DEFAULT_VOUCHER;

        /// Global voucher.
        pub GlobalVoucher get(fn global_voucher) config(): Voucher = DEFAULT_GLOBAL_VOUCHER;

        /// If all new uploads blocked
        pub UploadingBlocked get(fn uploading_blocked) config(): bool = DEFAULT_UPLOADING_BLOCKED_STATUS;

    }
}

decl_event! {
    /// _Data directory_ events
    pub enum Event<T> where
        StorageObjectOwner = ObjectOwner<T>,
        StorageProviderId = StorageProviderId<T>,
        ContentId = ContentId<T>,
        ContentParameters = ContentParameters<ContentId<T>, DataObjectTypeId<T>>,
        VoucherLimit = u64,
        UploadingStatus = bool
    {
        /// Emits on adding of the content.
        /// Params:
        /// - Content parameters representation.
        /// - StorageObjectOwner enum.
        ContentAdded(Vec<ContentParameters>, StorageObjectOwner),

        /// Emits on content removal.
        /// Params:
        /// - Content parameters representation.
        /// - StorageObjectOwner enum.
        ContentRemoved(Vec<ContentId>, StorageObjectOwner),

        /// Emits when the storage provider accepts a content.
        /// Params:
        /// - Id of the relationship.
        /// - Id of the storage provider.
        ContentAccepted(ContentId, StorageProviderId),

        /// Emits when the storage provider rejects a content.
        /// Params:
        /// - Id of the relationship.
        /// - Id of the storage provider.
        ContentRejected(ContentId, StorageProviderId),

        /// Emits when the storage object owner voucher size limit update performed.
        /// Params:
        /// - StorageObjectOwner enum.
        /// - voucher size limit.
        StorageObjectOwnerVoucherSizeLimitUpdated(StorageObjectOwner, VoucherLimit),

        /// Emits when the storage object owner voucher objects limit update performed.
        /// Params:
        /// - StorageObjectOwner enum.
        /// - voucher objects limit.
        StorageObjectOwnerVoucherObjectsLimitUpdated(StorageObjectOwner, VoucherLimit),

        /// Emits when the content uploading status update performed.
        /// Params:
        /// - UploadingStatus bool flag.
        ContentUploadingStatusUpdated(UploadingStatus),

        /// Emits when the global voucher size limit is updated.
        /// Params:
        /// - New limit
        GlobalVoucherSizeLimitUpdated(u64),

        /// Emits when the global voucher objects limit is updated.
        /// Params:
        /// - New limit
        GlobalVoucherObjectsLimitUpdated(u64),

        /// Emits when the size limit upper bound is updated.
        /// Params:
        /// - New Upper bound
        VoucherSizeLimitUpperBoundUpdated(u64),

        /// Emits when the objects limit upper bound is updated.
        /// Params:
        /// - New Upper bound
        VoucherObjectsLimitUpperBoundUpdated(u64),

        /// Emits when the lead sets a new default voucher
        /// Params:
        /// - New size limit
        /// - New objects limit
        DefaultVoucherUpdated(u64, u64),
    }
}

decl_module! {
    /// _Data directory_ substrate module.
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        /// Default deposit_event() handler
        fn deposit_event() = default;

        /// Predefined errors.
        type Error = Error<T>;

        /// Adds the content to the frame_system. The created DataObject
        /// awaits liaison to accept it.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_content(
            origin,
            owner: ObjectOwner<T>,
            content: Vec<ContentParameters<ContentId<T>, DataObjectTypeId<T>>>
        ) {

            // Ensure given origin can perform operation under specific storage object owner
            Self::ensure_storage_object_owner_origin(origin, &owner)?;

            Self::ensure_uploading_is_not_blocked()?;

            Self::ensure_content_is_valid(&content)?;

            // Ensure owner and global voucher constraints satisfied.
            let (new_owner_voucher, new_global_voucher) = Self::ensure_voucher_constraints_satisfied(&owner, &content)?;

            //
            // == MUTATION SAFE ==
            //

            // Updade or create owner voucher.
            <Vouchers<T>>::insert(&owner, new_owner_voucher);

            // Update global voucher
            <GlobalVoucher>::put(new_global_voucher);

            Self::upload_content(content.clone(), owner.clone());

            Self::deposit_event(RawEvent::ContentAdded(content, owner));
        }

        /// Remove the content from the frame_system.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_content(
            origin,
            owner: ObjectOwner<T>,
            content_ids: Vec<ContentId<T>>
        ) {

            // Ensure given origin can perform operation under specific storage object owner
            Self::ensure_storage_object_owner_origin(origin, &owner)?;

            // Ensure content under given content ids can be successfully removed
            let content = Self::ensure_content_can_be_removed(&content_ids, &owner)?;

            let owner_voucher = Self::get_voucher(&owner);
            let removal_voucher = Self::calculate_content_voucher(content);
            let new_owner_voucher = owner_voucher.release_voucher::<T>(removal_voucher)?;
            let new_global_voucher = Self::global_voucher().release_voucher::<T>(removal_voucher)?;

            //
            // == MUTATION SAFE ==
            //

            // Updade owner voucher
            <Vouchers<T>>::insert(&owner, new_owner_voucher);

            // Update global voucher
            <GlobalVoucher>::put(new_global_voucher);

            // Let's remove content
            for content_id in &content_ids {
                <DataByContentId<T>>::remove(content_id);
            }

            Self::deposit_event(RawEvent::ContentRemoved(content_ids, owner));
        }

        /// Updates storage object owner voucher objects limit. Requires leader privileges.
        /// New limit cannot be less that used value.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_object_owner_voucher_objects_limit(
            origin,
            object_owner: ObjectOwner<T>,
            new_voucher_objects_limit: u64
        ) {
            <StorageWorkingGroup<T>>::ensure_origin_is_active_leader(origin)?;
            ensure!(
                new_voucher_objects_limit <= Self::voucher_objects_limit_upper_bound(),
                Error::<T>::VoucherObjectsLimitUpperBoundExceeded
            );

            let mut voucher = Self::get_voucher(&object_owner);

            voucher.set_new_objects_limit::<T>(new_voucher_objects_limit)?;

            //
            // == MUTATION SAFE ==
            //

            <Vouchers<T>>::insert(&object_owner, voucher);

            Self::deposit_event(RawEvent::StorageObjectOwnerVoucherObjectsLimitUpdated(object_owner, new_voucher_objects_limit));
        }

        /// Updates storage object owner voucher size limit. Requires leader privileges.
        /// New limit cannot be less that used value.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_object_owner_voucher_size_limit(
            origin,
            object_owner: ObjectOwner<T>,
            new_voucher_size_limit: u64
        ) {
            <StorageWorkingGroup<T>>::ensure_origin_is_active_leader(origin)?;
            ensure!(
                new_voucher_size_limit <= Self::voucher_size_limit_upper_bound(),
                Error::<T>::VoucherSizeLimitUpperBoundExceeded
            );

            let mut voucher = Self::get_voucher(&object_owner);

            voucher.set_new_size_limit::<T>(new_voucher_size_limit)?;

            //
            // == MUTATION SAFE ==
            //

            <Vouchers<T>>::insert(&object_owner, voucher);

            Self::deposit_event(RawEvent::StorageObjectOwnerVoucherSizeLimitUpdated(object_owner, new_voucher_size_limit));
        }

        /// Sets global voucher size limit. Requires root privileges.
        /// New limit cannot be less that used value.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_global_voucher_size_limit(
            origin,
            new_size_limit: u64
        ) {
            ensure_root(origin)?;

            let mut voucher =  Self::global_voucher();
            voucher.set_new_size_limit::<T>(new_size_limit)?;

            //
            // == MUTATION SAFE ==
            //

            GlobalVoucher::put(voucher);

            Self::deposit_event(RawEvent::GlobalVoucherSizeLimitUpdated(new_size_limit));
        }

        /// Sets global voucher objects limit. Requires root privileges.
        /// New limit cannot be less that used value.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_global_voucher_objects_limit(
            origin,
            new_objects_limit: u64
        ) {
            ensure_root(origin)?;

            let mut voucher = Self::global_voucher();
            voucher.set_new_objects_limit::<T>(new_objects_limit)?;

            //
            // == MUTATION SAFE ==
            //

            GlobalVoucher::put(voucher);

            Self::deposit_event(RawEvent::GlobalVoucherObjectsLimitUpdated(new_objects_limit));
        }

        /// Sets VoucherSizeLimitUpperBound. Requires root privileges.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_voucher_size_limit_upper_bound(
            origin,
            new_upper_bound: u64
        ) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            VoucherSizeLimitUpperBound::put(new_upper_bound);

            Self::deposit_event(RawEvent::VoucherSizeLimitUpperBoundUpdated(new_upper_bound));
        }

        /// Sets VoucherObjectsLimitUpperBound. Requires root privileges.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_voucher_objects_limit_upper_bound(
            origin,
            new_upper_bound: u64
        ) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            VoucherObjectsLimitUpperBound::put(new_upper_bound);

            Self::deposit_event(RawEvent::VoucherObjectsLimitUpperBoundUpdated(new_upper_bound));
        }

        /// Set the default owner voucher
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_default_voucher(
            origin,
            size_limit: u64,
            objects_limit: u64
        ) {
            <StorageWorkingGroup<T>>::ensure_origin_is_active_leader(origin)?;
            // constrain to upper bounds
            ensure!(
                size_limit <= Self::voucher_size_limit_upper_bound(),
                Error::<T>::VoucherSizeLimitUpperBoundExceeded
            );
            ensure!(
                objects_limit <= Self::voucher_objects_limit_upper_bound(),
                Error::<T>::VoucherObjectsLimitUpperBoundExceeded
            );

            // == MUTATION SAFE ==
            DefaultVoucher::put(Voucher::new(size_limit, objects_limit));

            Self::deposit_event(RawEvent::DefaultVoucherUpdated(size_limit, objects_limit));
        }

        /// Storage provider accepts a content. Requires signed storage provider account and its id.
        /// The LiaisonJudgement can only be updated once from Pending to Accepted.
        /// Subsequent calls are a no-op.
        #[weight = 10_000_000] // TODO: adjust weight
        pub(crate) fn accept_content(
            origin,
            storage_provider_id: StorageProviderId<T>,
            content_id: T::ContentId
        ) {
            <StorageWorkingGroup<T>>::ensure_worker_signed(origin, &storage_provider_id)?;

            let mut data = Self::get_data_object(&content_id)?;

            // == MUTATION SAFE ==

            if data.liaison_judgement == LiaisonJudgement::Pending {
                // Set the liaison which is updating the judgement
                data.liaison = Some(storage_provider_id);

                // Set the judgement
                data.liaison_judgement = LiaisonJudgement::Accepted;
                <DataByContentId<T>>::insert(content_id, data);
                Self::deposit_event(RawEvent::ContentAccepted(content_id, storage_provider_id));
            }
        }

        /// Locks / unlocks content uploading
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_content_uploading_status(origin, is_blocked: bool) {
            <StorageWorkingGroup<T>>::ensure_origin_is_active_leader(origin)?;

            // == MUTATION SAFE ==

            <UploadingBlocked>::put(is_blocked);
            Self::deposit_event(RawEvent::ContentUploadingStatusUpdated(is_blocked));
        }
    }
}

impl<T: Config> Module<T> {
    // Used to initialize data_directory runtime storage on runtime upgrade
    pub fn initialize_data_directory(
        vouchers: Vec<(ObjectOwner<T>, Voucher)>,
        voucher_size_limit_upper_bound: u64,
        voucher_objects_limit_upper_bound: u64,
        global_voucher: Voucher,
        default_voucher: Voucher,
        uploading_blocked: bool,
    ) {
        for (storage_object_owner, voucher) in vouchers {
            <Vouchers<T>>::insert(storage_object_owner, voucher);
        }

        <VoucherSizeLimitUpperBound>::put(voucher_size_limit_upper_bound);
        <VoucherObjectsLimitUpperBound>::put(voucher_objects_limit_upper_bound);
        <GlobalVoucher>::put(global_voucher);
        <DefaultVoucher>::put(default_voucher);
        <UploadingBlocked>::put(uploading_blocked);
    }

    // Ensure given origin can perform operation under specific storage object owner
    fn ensure_storage_object_owner_origin(
        origin: T::Origin,
        owner: &ObjectOwner<T>,
    ) -> DispatchResult {
        if let StorageObjectOwner::Member(member_id) = owner {
            T::MemberOriginValidator::ensure_actor_origin(origin, *member_id)?;
        } else {
            ensure_root(origin)?;
        };
        Ok(())
    }

    // Get owner voucher if exists, otherwise return default one.
    fn get_voucher(owner: &ObjectOwner<T>) -> Voucher {
        if <Vouchers<T>>::contains_key(owner) {
            Self::vouchers(owner)
        } else {
            Self::default_voucher()
        }
    }

    // Ensure content uploading is not blocked
    fn ensure_uploading_is_not_blocked() -> DispatchResult {
        ensure!(
            !Self::uploading_blocked(),
            Error::<T>::ContentUploadingBlocked
        );
        Ok(())
    }

    // Ensure content under given content ids can be successfully removed
    fn ensure_content_can_be_removed(
        content_ids: &[T::ContentId],
        owner: &ObjectOwner<T>,
    ) -> Result<Vec<DataObject<T>>, Error<T>> {
        let mut content = Vec::new();
        for content_id in content_ids {
            let data_object = Self::get_data_object(content_id)?;
            ensure!(data_object.owner == *owner, Error::<T>::OwnersAreNotEqual);
            content.push(data_object);
        }

        Ok(content)
    }

    /// Calculates content voucher delta of existing data objects
    fn calculate_content_voucher(content: Vec<DataObject<T>>) -> Delta {
        let content_length = content.len() as u64;

        let content_size = content
            .into_iter()
            .fold(0, |total_size, content| total_size + content.size);

        Delta {
            size: content_size,
            objects: content_length,
        }
    }

    /// Calculates content voucher delta for new content being added
    fn calculate_new_content_voucher(
        content: &[ContentParameters<T::ContentId, DataObjectTypeId<T>>],
    ) -> Delta {
        let objects = content.len() as u64;
        let size = content
            .iter()
            .fold(0, |total_size, content| total_size + content.size);

        Delta { size, objects }
    }

    /// Ensures new content satisfies both owner and global vouchers. Returns new vouchers
    /// if constraints are satisfied, Error otherwise.
    fn ensure_voucher_constraints_satisfied(
        owner: &ObjectOwner<T>,
        content: &[ContentParameters<T::ContentId, DataObjectTypeId<T>>],
    ) -> Result<(Voucher, Voucher), Error<T>> {
        let owner_voucher = Self::get_voucher(owner);
        let global_voucher = Self::global_voucher();
        let content_voucher_delta = Self::calculate_new_content_voucher(content);

        let new_owner_voucher = owner_voucher.fill_voucher::<T>(content_voucher_delta)?;
        let new_global_voucher = global_voucher.fill_voucher::<T>(content_voucher_delta)?;

        Ok((new_owner_voucher, new_global_voucher))
    }

    // Complete content upload
    fn upload_content(
        multi_content: Vec<ContentParameters<T::ContentId, DataObjectTypeId<T>>>,
        owner: ObjectOwner<T>,
    ) {
        for content in multi_content {
            let data: DataObject<T> = DataObjectInternal {
                type_id: content.type_id,
                size: content.size,
                added_at: common::current_block_time::<T>(),
                owner: owner.clone(),
                liaison: None,
                liaison_judgement: LiaisonJudgement::Pending,
                ipfs_content_id: content.ipfs_content_id,
            };

            <DataByContentId<T>>::insert(content.content_id, data);
        }
    }

    fn ensure_content_is_valid(
        multi_content: &[ContentParameters<T::ContentId, DataObjectTypeId<T>>],
    ) -> DispatchResult {
        for content in multi_content {
            ensure!(
                T::IsActiveDataObjectType::is_active_data_object_type(&content.type_id),
                Error::<T>::DataObjectTypeMustBeActive
            );

            ensure!(
                !<DataByContentId<T>>::contains_key(&content.content_id),
                Error::<T>::DataObjectAlreadyAdded
            );
        }
        Ok(())
    }
}

/// Content access helper.
pub trait ContentIdExists<T: Config> {
    /// Verifies the content existence.
    fn has_content(id: &T::ContentId) -> bool;

    /// Returns the data object for the provided content id.
    fn get_data_object(id: &T::ContentId) -> Result<DataObject<T>, Error<T>>;
}

impl<T: Config> ContentIdExists<T> for Module<T> {
    fn has_content(content_id: &T::ContentId) -> bool {
        <DataByContentId<T>>::contains_key(content_id)
    }

    fn get_data_object(content_id: &T::ContentId) -> Result<DataObject<T>, Error<T>> {
        if Self::has_content(content_id) {
            Ok(Self::data_object_by_content_id(*content_id))
        } else {
            Err(Error::<T>::CidNotFound)
        }
    }
}

impl<T: Config> common::storage::StorageSystem<T> for Module<T> {
    fn atomically_add_content(
        owner: ObjectOwner<T>,
        content: Vec<ContentParameters<T::ContentId, DataObjectTypeId<T>>>,
    ) -> DispatchResult {
        Self::ensure_content_is_valid(&content)?;

        Self::ensure_uploading_is_not_blocked()?;

        // Ensure owner and global vouchers constraints satisfied.
        let (new_owner_voucher, new_global_voucher) =
            Self::ensure_voucher_constraints_satisfied(&owner, &content)?;

        //
        // == MUTATION SAFE ==
        //

        // Updade or create owner voucher.
        <Vouchers<T>>::insert(&owner, new_owner_voucher);

        // Update global voucher
        <GlobalVoucher>::put(new_global_voucher);

        Self::upload_content(content, owner);
        Ok(())
    }

    fn atomically_remove_content(
        owner: &ObjectOwner<T>,
        content_ids: &[T::ContentId],
    ) -> DispatchResult {
        // Ensure content under given content ids can be successfully removed
        let content = Self::ensure_content_can_be_removed(content_ids, owner)?;

        let owner_voucher = Self::get_voucher(&owner);
        let removal_voucher = Self::calculate_content_voucher(content);
        let new_owner_voucher = owner_voucher.release_voucher::<T>(removal_voucher)?;
        let new_global_voucher = Self::global_voucher().release_voucher::<T>(removal_voucher)?;

        //
        // == MUTATION SAFE ==
        //

        // Updade owner voucher.
        <Vouchers<T>>::insert(owner, new_owner_voucher);

        // Update global voucher
        <GlobalVoucher>::put(new_global_voucher);

        // Let's remove content
        for content_id in content_ids {
            <DataByContentId<T>>::remove(content_id);
        }

        Ok(())
    }

    fn can_add_content(
        owner: ObjectOwner<T>,
        content: Vec<ContentParameters<T::ContentId, DataObjectTypeId<T>>>,
    ) -> DispatchResult {
        Self::ensure_uploading_is_not_blocked()?;

        let _ = Self::ensure_voucher_constraints_satisfied(&owner, &content)?;

        Self::ensure_content_is_valid(&content)
    }

    fn can_remove_content(owner: &ObjectOwner<T>, content_ids: &[ContentId<T>]) -> DispatchResult {
        // Ensure content under given content ids can be successfully removed
        Self::ensure_content_can_be_removed(content_ids, &owner)?;

        Ok(())
    }
}
