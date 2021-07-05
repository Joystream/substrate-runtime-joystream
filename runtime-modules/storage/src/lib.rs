//! # Storage module
//! Storage module for the Joystream platform.
//!
//! Initial spec links:
//! - [spec](https://github.com/Joystream/joystream/issues/2224)
//! - [utilization model](https://github.com/Joystream/joystream/issues/2359)
//!
//! Pallet functionality could be split in three distinct groups:
//! - extrinsics for the working group leader
//! - extrinsics for the storage provider
//! - public methods for the pallet integration
//!
//! #### Working group leader extrinsics
//! - [create_storage_bucket](./struct.Module.html#method.create_storage_bucket) - creates storage
//! bucket.
//! - [update_storage_buckets_for_bag](./struct.Module.html#method.update_storage_buckets_for_bag) -
//! updates storage buckets for a bag.
//! - [delete_storage_bucket](./struct.Module.html#method.delete_storage_bucket) - deletes storage
//! bucket.
//! - [invite_storage_bucket_operator](./struct.Module.html#method.invite_storage_bucket_operator) -
//! invites storage bucket operator.
//! - [cancel_storage_bucket_operator_invite](./struct.Module.html#method.cancel_storage_bucket_operator_invite) -
//! cancels pending storage bucket invite.
//! - [remove_storage_bucket_operator](./struct.Module.html#method.remove_storage_bucket_operator) -
//! removes storage bucket operator.
//! - [update_uploading_blocked_status](./struct.Module.html#method.update_uploading_blocked_status) -
//! updates whether uploading is globally blocked.
//! - [update_data_size_fee](./struct.Module.html#method.update_data_size_fee) - updates size-based
//! pricing of new objects uploaded.
//! - [update_storage_buckets_per_bag_limit](./struct.Module.html#method.update_storage_buckets_per_bag_limit) -
//! updates "Storage buckets per bag" number limit.
//! - [update_storage_buckets_voucher_max_limits](./struct.Module.html#method.update_storage_buckets_voucher_max_limits) -
//! updates "Storage buckets voucher max limits".
//! - [update_number_of_storage_buckets_in_dynamic_bag_creation_policy](./struct.Module.html#method.update_number_of_storage_buckets_in_dynamic_bag_creation_policy) -
//! updates number of storage buckets used in given dynamic bag creation policy.
//! - [update_blacklist](./struct.Module.html#method.update_blacklist) - adds and removes hashes to
//! the current blacklist.
//!
//! #### Storage provider extrinsics
//!
//! - [accept_storage_bucket_invitation](./struct.Module.html#method.accept_storage_bucket_invitation) -
//! accepts the storage bucket invitation.
//! - [set_storage_operator_metadata](./struct.Module.html#method.set_storage_operator_metadata) -
//! sets storage operator metadata.
//! - [set_storage_bucket_voucher_limits](./struct.Module.html#method.set_storage_bucket_voucher_limits) -
//! sets storage bucket voucher limits.
//! - [update_storage_bucket_status](./struct.Module.html#method.update_storage_bucket_status) -
//! updates whether new bags are being accepted for storage.
//! - [accept_pending_data_objects](./struct.Module.html#method.accept_pending_data_objects) - a
//! storage provider signals that the data object was successfully uploaded to its storage.
//!
//! #### Public methods
//! Public integration methods are exposed via the [DataObjectStorage](./trait.DataObjectStorage.html)
//! - can_upload_data_objects
//! - upload_data_objects
//! - can_move_data_objects
//! - move_data_objects
//! - can_delete_data_objects
//! - delete_data_objects
//! - can_delete_dynamic_bag
//! - delete_dynamic_bag
//! - can_create_dynamic_bag
//! - create_dynamic_bag
//!
//! ### Pallet constants
//! - MaxStorageBucketNumber
//! - MaxNumberOfDataObjectsPerBag
//! - DataObjectDeletionPrize
//! - BlacklistSizeLimit
//! - StorageBucketsPerBagValueConstraint
//! - DefaultMemberDynamicBagCreationPolicy
//! - DefaultChannelDynamicBagCreationPolicy
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod bag_manager;
pub(crate) mod storage_bucket_picker;

use codec::{Codec, Decode, Encode};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, ExistenceRequirement, Get, Randomness};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{AccountIdConversion, MaybeSerialize, Member, Saturating};
use sp_runtime::{ModuleId, SaturatedConversion};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::iter;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;
use frame_system::ensure_root;

use common::constraints::BoundedValueConstraint;
use common::origin::ActorOriginValidator;
use common::working_group::WorkingGroup;

use bag_manager::BagManager;
use storage_bucket_picker::StorageBucketPicker;

/// Public interface for the storage module.
pub trait DataObjectStorage<T: Trait> {
    /// Validates upload parameters and conditions (like global uploading block).
    /// Validates voucher usage for affected buckets.
    fn can_upload_data_objects(params: &UploadParameters<T>) -> DispatchResult;

    /// Upload new data objects.
    fn upload_data_objects(params: UploadParameters<T>) -> DispatchResult;

    /// Validates moving objects parameters.
    /// Validates voucher usage for affected buckets.
    fn can_move_data_objects(
        src_bag_id: &BagId<T>,
        dest_bag_id: &BagId<T>,
        objects: &BTreeSet<T::DataObjectId>,
    ) -> DispatchResult;

    /// Move data objects to a new bag.
    fn move_data_objects(
        src_bag_id: BagId<T>,
        dest_bag_id: BagId<T>,
        objects: BTreeSet<T::DataObjectId>,
    ) -> DispatchResult;

    /// Validates `delete_data_objects` parameters.
    /// Validates voucher usage for affected buckets.
    fn can_delete_data_objects(
        bag_id: &BagId<T>,
        objects: &BTreeSet<T::DataObjectId>,
    ) -> DispatchResult;

    /// Delete storage objects. Transfer deletion prize to the provided account.
    fn delete_data_objects(
        deletion_prize_account_id: T::AccountId,
        bag_id: BagId<T>,
        objects: BTreeSet<T::DataObjectId>,
    ) -> DispatchResult;

    /// Delete dynamic bag. Updates related storage bucket vouchers.
    fn delete_dynamic_bag(
        deletion_prize_account_id: T::AccountId,
        bag_id: DynamicBagId<T>,
    ) -> DispatchResult;

    /// Validates `delete_dynamic_bag` parameters and conditions.
    fn can_delete_dynamic_bag(bag_id: &DynamicBagId<T>) -> DispatchResult;

    /// Creates dynamic bag. BagId should provide the caller.
    fn create_dynamic_bag(bag_id: DynamicBagId<T>) -> DispatchResult;

    /// Validates `create_dynamic_bag` parameters and conditions.
    fn can_create_dynamic_bag(bag_id: &DynamicBagId<T>) -> DispatchResult;
}

/// Storage trait.
pub trait Trait: frame_system::Trait + balances::Trait + membership::Trait {
    /// Storage event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Data object ID type.
    type DataObjectId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Storage bucket ID type.
    type StorageBucketId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Distribution bucket ID type.
    type DistributionBucketId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Channel ID type (part of the dynamic bag ID).
    type ChannelId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Defines max allowed storage bucket number.
    type MaxStorageBucketNumber: Get<u64>;

    /// Defines max number of data objects per bag.
    type MaxNumberOfDataObjectsPerBag: Get<u64>;

    /// Defines a prize for a data object deletion.
    type DataObjectDeletionPrize: Get<BalanceOf<Self>>;

    /// Defines maximum size of the "hash blacklist" collection.
    type BlacklistSizeLimit: Get<u64>;

    /// The module id, used for deriving its sovereign account ID.
    type ModuleId: Get<ModuleId>;

    /// Validates member id and origin combination.
    type MemberOriginValidator: ActorOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;

    /// "Storage buckets per bag" value constraint.
    type StorageBucketsPerBagValueConstraint: Get<StorageBucketsPerBagValueConstraint>;

    /// Defines the default dynamic bag creation policy for members.
    type DefaultMemberDynamicBagCreationPolicy: Get<DynamicBagCreationPolicy>;

    /// Defines the default dynamic bag creation policy for channels.
    type DefaultChannelDynamicBagCreationPolicy: Get<DynamicBagCreationPolicy>;

    /// Defines max random iteration number (eg.: when picking the storage buckets).
    type MaxRandomIterationNumber: Get<u64>;

    /// Something that provides randomness in the runtime.
    type Randomness: Randomness<Self::Hash>;

    /// Demand the working group leader authorization.
    /// TODO: Refactor after merging with the Olympia release.
    fn ensure_working_group_leader_origin(origin: Self::Origin) -> DispatchResult;

    /// Validate origin for the worker.
    /// TODO: Refactor after merging with the Olympia release.
    fn ensure_worker_origin(origin: Self::Origin, worker_id: WorkerId<Self>) -> DispatchResult;

    /// Validate worker existence.
    /// TODO: Refactor after merging with the Olympia release.
    fn ensure_worker_exists(worker_id: &WorkerId<Self>) -> DispatchResult;
}

/// Operations with local pallet account.
pub trait ModuleAccount<T: balances::Trait> {
    /// The module id, used for deriving its sovereign account ID.
    type ModuleId: Get<ModuleId>;

    /// The account ID of the module account.
    fn module_account_id() -> T::AccountId {
        Self::ModuleId::get().into_sub_account(Vec::<u8>::new())
    }

    /// Transfer tokens from the module account to the destination account (spends from
    /// module account).
    fn withdraw(dest_account_id: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        <Balances<T> as Currency<T::AccountId>>::transfer(
            &Self::module_account_id(),
            dest_account_id,
            amount,
            ExistenceRequirement::AllowDeath,
        )
    }

    /// Transfer tokens from the destination account to the module account (fills module account).
    fn deposit(src_account_id: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        <Balances<T> as Currency<T::AccountId>>::transfer(
            src_account_id,
            &Self::module_account_id(),
            amount,
            ExistenceRequirement::AllowDeath,
        )
    }

    /// Displays usable balance for the module account.
    fn usable_balance() -> BalanceOf<T> {
        <Balances<T>>::usable_balance(&Self::module_account_id())
    }
}

/// Implementation of the ModuleAccountHandler.
pub struct ModuleAccountHandler<T: balances::Trait, ModId: Get<ModuleId>> {
    /// Phantom marker for the trait.
    trait_marker: PhantomData<T>,

    /// Phantom marker for the module id type.
    module_id_marker: PhantomData<ModId>,
}

impl<T: balances::Trait, ModId: Get<ModuleId>> ModuleAccount<T> for ModuleAccountHandler<T, ModId> {
    type ModuleId = ModId;
}

/// Holds parameter values impacting how exactly the creation of a new dynamic bag occurs,
/// and there is one such policy for each type of dynamic bag.
/// It describes how many storage buckets should store the bag.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct DynamicBagCreationPolicy {
    /// The number of storage buckets which should replicate the new bag.
    pub number_of_storage_buckets: u64,
}

impl DynamicBagCreationPolicy {
    // Verifies non-zero number of storage buckets.
    pub(crate) fn no_storage_buckets_required(&self) -> bool {
        self.number_of_storage_buckets == 0
    }
}

/// "Storage buckets per bag" value constraint type.
pub type StorageBucketsPerBagValueConstraint = BoundedValueConstraint<u64>;

/// Local module account handler.
pub type StorageTreasury<T> = ModuleAccountHandler<T, <T as Trait>::ModuleId>;

/// IPFS hash type alias.
pub type ContentId = Vec<u8>;

// Alias for the Substrate balances pallet.
type Balances<T> = balances::Module<T>;

/// Alias for the member id.
pub type MemberId<T> = <T as membership::Trait>::MemberId;

/// Type identifier for worker role, which must be same as membership actor identifier
pub type WorkerId<T> = <T as membership::Trait>::ActorId;

/// Balance alias for `balances` module.
pub type BalanceOf<T> = <T as balances::Trait>::Balance;

/// The fundamental concept in the system, which represents single static binary object in the
/// system. The main goal of the system is to retain an index of all such objects, including who
/// owns them, and information about what actors are currently tasked with storing and distributing
/// them to end users. The system is unaware of the underlying content represented by such an
/// object, as it is used by different parts of the Joystream system.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct DataObject<Balance> {
    /// Defines whether the data object was accepted by a liaison.
    pub accepted: bool,

    /// A reward for the data object deletion.
    pub deletion_prize: Balance,

    /// Object size in bytes.
    pub size: u64,
}

/// Type alias for the StaticBagObject.
pub type StaticBag<T> = StaticBagObject<
    <T as Trait>::DataObjectId,
    <T as Trait>::StorageBucketId,
    <T as Trait>::DistributionBucketId,
    BalanceOf<T>,
>;

/// Static bag container.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct StaticBagObject<
    DataObjectId: Ord,
    StorageBucketId: Ord,
    DistributionBucketId: Ord,
    Balance,
> {
    /// Associated data objects.
    pub objects: BTreeMap<DataObjectId, DataObject<Balance>>,

    /// Associated storage buckets.
    pub stored_by: BTreeSet<StorageBucketId>,

    /// Associated distribution buckets.
    pub distributed_by: BTreeSet<DistributionBucketId>,
}

impl<DataObjectId: Ord, StorageBucketId: Ord, DistributionBucketId: Ord, Balance>
    StaticBagObject<DataObjectId, StorageBucketId, DistributionBucketId, Balance>
{
    // Calculates total object size for static bag.
    pub(crate) fn objects_total_size(&self) -> u64 {
        self.objects.values().map(|obj| obj.size).sum()
    }

    // Calculates total objects number for static bag.
    pub(crate) fn objects_number(&self) -> u64 {
        self.objects.len().saturated_into()
    }
}

/// Parameters for the data object creation.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct DataObjectCreationParameters {
    /// Object size in bytes.
    pub size: u64,

    /// Content identifier presented as IPFS hash.
    pub ipfs_content_id: Vec<u8>,
}

/// Type alias for the BagIdType.
pub type BagId<T> = BagIdType<MemberId<T>, <T as Trait>::ChannelId>;

/// Identifier for a bag.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum BagIdType<MemberId, ChannelId> {
    /// Static bag type.
    StaticBag(StaticBagId),

    /// Dynamic bag type.
    DynamicBag(DynamicBagIdType<MemberId, ChannelId>),
}

impl<MemberId, ChannelId> Default for BagIdType<MemberId, ChannelId> {
    fn default() -> Self {
        Self::StaticBag(Default::default())
    }
}

/// Define dynamic bag types.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Copy)]
pub enum DynamicBagType {
    /// Member dynamic bag type.
    Member,

    /// Channel dynamic bag type.
    Channel,
}

impl Default for DynamicBagType {
    fn default() -> Self {
        Self::Member
    }
}

/// A type for static bags ID.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum StaticBagId {
    /// Dedicated bag for a council.
    Council,

    /// Dedicated bag for some working group.
    WorkingGroup(WorkingGroup),
}

impl Default for StaticBagId {
    fn default() -> Self {
        Self::Council
    }
}

/// Type alias for the DynamicBagIdType.
pub type DynamicBagId<T> = DynamicBagIdType<MemberId<T>, <T as Trait>::ChannelId>;

/// A type for dynamic bags ID.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum DynamicBagIdType<MemberId, ChannelId> {
    /// Dynamic bag assigned to a member.
    Member(MemberId),

    /// Dynamic bag assigned to media channel.
    Channel(ChannelId),
}

impl<MemberId: Default, ChannelId> Default for DynamicBagIdType<MemberId, ChannelId> {
    fn default() -> Self {
        Self::Member(Default::default())
    }
}

#[allow(clippy::from_over_into)] // Cannot implement From using these types.
impl<MemberId: Default, ChannelId> Into<DynamicBagType> for DynamicBagIdType<MemberId, ChannelId> {
    fn into(self) -> DynamicBagType {
        match self {
            DynamicBagIdType::Member(_) => DynamicBagType::Member,
            DynamicBagIdType::Channel(_) => DynamicBagType::Channel,
        }
    }
}

/// Alias for the UploadParametersObject
pub type UploadParameters<T> = UploadParametersObject<
    MemberId<T>,
    <T as Trait>::ChannelId,
    <T as frame_system::Trait>::AccountId,
>;

/// Data wrapper structure. Helps passing the parameters to the `upload` extrinsic.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct UploadParametersObject<MemberId, ChannelId, AccountId> {
    /// Public key used authentication in upload to liaison.
    pub authentication_key: Vec<u8>,

    /// Static or dynamic bag to upload data.
    pub bag_id: BagIdType<MemberId, ChannelId>,

    /// Data object parameters.
    pub object_creation_list: Vec<DataObjectCreationParameters>,

    /// Account for the data object deletion prize.
    pub deletion_prize_source_account_id: AccountId,
}

/// Defines storage bucket parameters.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Voucher {
    /// Total size limit.
    pub size_limit: u64,

    /// Object number limit.
    pub objects_limit: u64,

    /// Current size.
    pub size_used: u64,

    /// Current object number.
    pub objects_used: u64,
}

// Defines whether we should increase or decrease parameters during some operation.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
enum OperationType {
    // Increase parameters.
    Increase,

    // Decrease parameters.
    Decrease,
}

// Helper-struct - defines voucher changes.
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
struct VoucherUpdate {
    /// Total number.
    pub objects_number: u64,

    /// Total objects size sum.
    pub objects_total_size: u64,
}

impl VoucherUpdate {
    fn get_updated_voucher(&self, voucher: &Voucher, voucher_operation: OperationType) -> Voucher {
        let (objects_used, size_used) = match voucher_operation {
            OperationType::Increase => (
                voucher.objects_used.saturating_add(self.objects_number),
                voucher.size_used.saturating_add(self.objects_total_size),
            ),
            OperationType::Decrease => (
                voucher.objects_used.saturating_sub(self.objects_number),
                voucher.size_used.saturating_sub(self.objects_total_size),
            ),
        };

        Voucher {
            objects_used,
            size_used,
            ..voucher.clone()
        }
    }

    // Adds a single object data to the voucher update (updates objects size and number).
    fn add_object(&mut self, size: u64) {
        self.objects_number = self.objects_number.saturating_add(1);
        self.objects_total_size = self.objects_total_size.saturating_add(size);
    }
}

/// Defines the storage bucket connection to the storage operator (storage WG worker).
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum StorageBucketOperatorStatus<WorkerId> {
    /// No connection.
    Missing,

    /// Storage operator was invited.
    InvitedStorageWorker(WorkerId),

    /// Storage operator accepted the invitation.
    StorageWorker(WorkerId),
}

impl<WorkerId> Default for StorageBucketOperatorStatus<WorkerId> {
    fn default() -> Self {
        Self::Missing
    }
}

/// A commitment to hold some set of bags for long term storage. A bucket may have a bucket
/// operator, which is a single worker in the storage working group.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct StorageBucket<WorkerId> {
    /// Current storage operator status.
    pub operator_status: StorageBucketOperatorStatus<WorkerId>,

    /// Defines whether the bucket accepts new bags.
    pub accepting_new_bags: bool,

    /// Defines limits for a bucket.
    pub voucher: Voucher,

    /// Defines storage bucket medata (like current storage provider URL).
    pub metadata: Vec<u8>,
}

// Helper-struct for the data object uploading.
#[derive(Default, Clone, Debug)]
struct DataObjectCandidates<T: Trait> {
    // next data object ID to be saved in the storage.
    next_data_object_id: T::DataObjectId,

    // 'ID-data object' map.
    data_objects_map: BTreeMap<T::DataObjectId, DataObject<BalanceOf<T>>>,
}

/// Type alias for the DynamicBagObject.
pub type DynamicBag<T> = DynamicBagObject<
    <T as Trait>::DataObjectId,
    <T as Trait>::StorageBucketId,
    <T as Trait>::DistributionBucketId,
    BalanceOf<T>,
>;

/// Dynamic bag container.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct DynamicBagObject<
    DataObjectId: Ord,
    StorageBucketId: Ord,
    DistributionBucketId: Ord,
    Balance,
> {
    /// Associated data objects.
    pub objects: BTreeMap<DataObjectId, DataObject<Balance>>,

    /// Associated storage buckets.
    pub stored_by: BTreeSet<StorageBucketId>,

    /// Associated distribution buckets.
    pub distributed_by: BTreeSet<DistributionBucketId>,

    /// Dynamic bag deletion prize.
    pub deletion_prize: Balance,
}

impl<DataObjectId: Ord, StorageBucketId: Ord, DistributionBucketId: Ord, Balance>
    DynamicBagObject<DataObjectId, StorageBucketId, DistributionBucketId, Balance>
{
    // Calculates total object size for dynamic bag.
    pub(crate) fn objects_total_size(&self) -> u64 {
        self.objects.values().map(|obj| obj.size).sum()
    }

    // Calculates total objects number for dynamic bag.
    pub(crate) fn objects_number(&self) -> u64 {
        self.objects.len().saturated_into()
    }
}

// Helper struct for the dynamic bag changing.
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
struct BagChangeInfo<Balance> {
    // Voucher update for data objects
    voucher_update: VoucherUpdate,

    // Total deletion prize for data objects.
    total_deletion_prize: Balance,
}

impl<Balance: Saturating + Copy> BagChangeInfo<Balance> {
    // Adds a single object data to the voucher update (updates objects size, number)
    // and deletion prize.
    fn add_object(&mut self, size: u64, deletion_prize: Balance) {
        self.voucher_update.add_object(size);
        self.total_deletion_prize = self.total_deletion_prize.saturating_add(deletion_prize);
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Storage {
        /// Defines whether all new uploads blocked
        pub UploadingBlocked get(fn uploading_blocked): bool;

        /// Working groups' and council's bags storage map.
        pub StaticBags get(fn static_bag): map hasher(blake2_128_concat)
            StaticBagId => StaticBag<T>;

        /// Dynamic bag storage map.
        pub DynamicBags get (fn dynamic_bag): map hasher(blake2_128_concat)
            DynamicBagId<T> => DynamicBag<T>;

        /// Storage bucket id counter. Starts at zero.
        pub NextStorageBucketId get(fn next_storage_bucket_id): T::StorageBucketId;

        /// Data object id counter. Starts at zero.
        pub NextDataObjectId get(fn next_data_object_id): T::DataObjectId;

        /// Total number of the storage buckets in the system.
        pub StorageBucketsNumber get(fn storage_buckets_number): u64;

        /// Storage buckets.
        pub StorageBucketById get (fn storage_bucket_by_id): map hasher(blake2_128_concat)
            T::StorageBucketId => StorageBucket<WorkerId<T>>;

        /// Blacklisted data object hashes.
        pub Blacklist get (fn blacklist): map hasher(blake2_128_concat) ContentId => ();

        /// Blacklist collection counter.
        pub CurrentBlacklistSize get (fn current_blacklist_size): u64;

        /// Size based pricing of new objects uploaded.
        pub DataObjectPerMegabyteFee get (fn data_object_per_mega_byte_fee): BalanceOf<T>;

        /// "Storage buckets per bag" number limit.
        pub StorageBucketsPerBagLimit get (fn storage_buckets_per_bag_limit): u64;

        /// "Max objects size for a storage bucket voucher" number limit.
        pub VoucherMaxObjectsSizeLimit get (fn voucher_max_objects_size_limit): u64;

        /// "Max objects number for a storage bucket voucher" number limit.
        pub VoucherMaxObjectsNumberLimit get (fn voucher_max_objects_number_limit): u64;

        /// DynamicBagCreationPolicy by bag type storage map.
        pub DynamicBagCreationPolicies get (fn dynamic_bag_creation_policy):
            map hasher(blake2_128_concat) DynamicBagType => DynamicBagCreationPolicy;
    }
}

decl_event! {
    /// Storage events
 pub enum Event<T>
    where
        <T as Trait>::StorageBucketId,
        WorkerId = WorkerId<T>,
        <T as Trait>::DataObjectId,
        UploadParameters = UploadParameters<T>,
        BagId = BagId<T>,
        DynamicBagId = DynamicBagId<T>,
        <T as frame_system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        /// Emits on creating the storage bucket.
        /// Params
        /// - storage bucket ID
        /// - invited worker
        /// - flag "accepting_new_bags"
        /// - size limit for voucher,
        /// - objects limit for voucher,
        StorageBucketCreated(StorageBucketId, Option<WorkerId>, bool, u64, u64),

        /// Emits on accepting the storage bucket invitation.
        /// Params
        /// - storage bucket ID
        /// - invited worker ID
        StorageBucketInvitationAccepted(StorageBucketId, WorkerId),

        /// Emits on updating storage buckets for bag.
        /// Params
        /// - bag ID
        /// - storage buckets to add ID collection
        /// - storage buckets to remove ID collection
        StorageBucketsUpdatedForBag(BagId, BTreeSet<StorageBucketId>, BTreeSet<StorageBucketId>),

        /// Emits on uploading data objects.
        /// Params
        /// - data objects IDs
        /// - initial uploading parameters
        DataObjectdUploaded(Vec<DataObjectId>, UploadParameters),

        /// Emits on setting the storage operator metadata.
        /// Params
        /// - storage bucket ID
        /// - invited worker ID
        /// - metadata
        StorageOperatorMetadataSet(StorageBucketId, WorkerId, Vec<u8>),

        /// Emits on setting the storage bucket voucher limits.
        /// Params
        /// - storage bucket ID
        /// - invited worker ID
        /// - new total objects size limit
        /// - new total objects number limit
        StorageBucketVoucherLimitsSet(StorageBucketId, WorkerId, u64, u64),

        /// Emits on accepting pending data objects.
        /// Params
        /// - storage bucket ID
        /// - worker ID (storage provider ID)
        /// - bag ID
        /// - pending data objects
        PendingDataObjectsAccepted(StorageBucketId, WorkerId, BagId, BTreeSet<DataObjectId>),

        /// Emits on cancelling the storage bucket invitation.
        /// Params
        /// - storage bucket ID
        StorageBucketInvitationCancelled(StorageBucketId),

        /// Emits on the storage bucket operator invitation.
        /// Params
        /// - storage bucket ID
        /// - operator worker ID (storage provider ID)
        StorageBucketOperatorInvited(StorageBucketId, WorkerId),

        /// Emits on the storage bucket operator removal.
        /// Params
        /// - storage bucket ID
        StorageBucketOperatorRemoved(StorageBucketId),

        /// Emits on changing the size-based pricing of new objects uploaded.
        /// Params
        /// - new status
        UploadingBlockStatusUpdated(bool),

        /// Emits on changing the size-based pricing of new objects uploaded.
        /// Params
        /// - new data size fee
        DataObjectPerMegabyteFeeUpdated(Balance),

        /// Emits on changing the "Storage buckets per bag" number limit.
        /// Params
        /// - new limit
        StorageBucketsPerBagLimitUpdated(u64),

        /// Emits on changing the "Storage buckets voucher max limits".
        /// Params
        /// - new objects size limit
        /// - new objects number limit
        StorageBucketsVoucherMaxLimitsUpdated(u64, u64),

        /// Emits on moving data objects between bags.
        /// Params
        /// - source bag ID
        /// - destination bag ID
        /// - data object IDs
        DataObjectsMoved(BagId, BagId, BTreeSet<DataObjectId>),

        /// Emits on data objects deletion from bags.
        /// Params
        /// - account ID for the deletion prize
        /// - bag ID
        /// - data object IDs
        DataObjectsDeleted(AccountId, BagId, BTreeSet<DataObjectId>),

        /// Emits on storage bucket status update.
        /// Params
        /// - storage bucket ID
        /// - worker ID (storage provider ID)
        /// - new status
        StorageBucketStatusUpdated(StorageBucketId, WorkerId, bool),

        /// Emits on updating the blacklist with data hashes.
        /// Params
        /// - hashes to remove from the blacklist
        /// - hashes to add to the blacklist
        UpdateBlacklist(BTreeSet<ContentId>, BTreeSet<ContentId>),

        /// Emits on deleting a dynamic bag.
        /// Params
        /// - account ID for the deletion prize
        /// - dynamic bag ID
        DynamicBagDeleted(AccountId, DynamicBagId),

        /// Emits on creating a dynamic bag.
        /// Params
        /// - dynamic bag ID
        DynamicBagCreated(DynamicBagId),

        /// Emits on changing the deletion prize for a dynamic bag.
        /// Params
        /// - dynamic bag ID
        /// - new deletion prize
        DeletionPrizeChanged(DynamicBagId, Balance),

        /// Emits on changing the voucher for a storage bucket.
        /// Params
        /// - storage bucket ID
        /// - new voucher
        VoucherChanged(StorageBucketId, Voucher),

        /// Emits on storage bucket deleting.
        /// Params
        /// - storage bucket ID
        StorageBucketDeleted(StorageBucketId),

        /// Emits on updating the number of storage buckets in dynamic bag creation policy.
        /// Params
        /// - dynamic bag type
        /// - new number of storage buckets
        NumberOfStorageBucketsInDynamicBagCreationPolicyUpdated(DynamicBagType, u64),
    }
}

decl_error! {
    /// Storage module predefined errors
    pub enum Error for Module<T: Trait>{
        /// Max storage bucket number limit exceeded.
        MaxStorageBucketNumberLimitExceeded,

        /// Empty "data object creation" collection.
        NoObjectsOnUpload,

        /// The requested storage bucket doesn't exist.
        StorageBucketDoesntExist,

        /// The requested storage bucket is not bound to a bag.
        StorageBucketIsNotBoundToBag,

        /// The requested storage bucket is already bound to a bag.
        StorageBucketIsBoundToBag,

        /// Invalid operation with invites: there is no storage bucket invitation.
        NoStorageBucketInvitation,

        /// Invalid operation with invites: storage provider was already set.
        StorageProviderAlreadySet,

        /// Storage provider must be set.
        StorageProviderMustBeSet,

        /// Invalid operation with invites: another storage provider was invited.
        DifferentStorageProviderInvited,

        /// Invalid operation with invites: storage provider was already invited.
        InvitedStorageProvider,

        /// Storage bucket id collections are empty.
        StorageBucketIdCollectionsAreEmpty,

        /// Upload data error: empty content ID provided.
        EmptyContentId,

        /// Upload data error: zero object size.
        ZeroObjectSize,

        /// Upload data error: invalid deletion prize source account.
        InvalidDeletionPrizeSourceAccount,

        /// Upload data error: data objects per bag limit exceeded.
        DataObjectsPerBagLimitExceeded,

        /// Invalid storage provider for bucket.
        InvalidStorageProvider,

        /// Insufficient balance for an operation.
        InsufficientBalance,

        /// Data object doesn't exist.
        DataObjectDoesntExist,

        /// Uploading of the new object is blocked.
        UploadingBlocked,

        /// Data object id collection is empty.
        DataObjectIdCollectionIsEmpty,

        /// Cannot move objects within the same bag.
        SourceAndDestinationBagsAreEqual,

        /// Data object hash is part of the blacklist.
        DataObjectBlacklisted,

        /// Blacklist size limit exceeded.
        BlacklistSizeLimitExceeded,

        /// Max object size limit exceeded for voucher.
        VoucherMaxObjectSizeLimitExceeded,

        /// Max object number limit exceeded for voucher.
        VoucherMaxObjectNumberLimitExceeded,

        /// Object number limit for the storage bucket reached.
        StorageBucketObjectNumberLimitReached,

        /// Objects total size limit for the storage bucket reached.
        StorageBucketObjectSizeLimitReached,

        /// Insufficient module treasury balance for an operation.
        InsufficientTreasuryBalance,

        /// Cannot delete a non-empty storage bucket.
        CannotDeleteNonEmptyStorageBucket,

        /// The `data_object_ids` extrinsic parameter collection is empty.
        DataObjectIdParamsAreEmpty,

        /// The new `StorageBucketsPerBagLimit` number is too low.
        StorageBucketsPerBagLimitTooLow,

        /// The new `StorageBucketsPerBagLimit` number is too high.
        StorageBucketsPerBagLimitTooHigh,

        /// `StorageBucketsPerBagLimit` was exceeded for a bag.
        StorageBucketPerBagLimitExceeded,

        /// The storage bucket doesn't accept new bags.
        StorageBucketDoesntAcceptNewBags,

        /// Cannot create the dynamic bag: dynamic bag exists.
        DynamicBagExists,

        /// Dynamic bag doesn't exist.
        DynamicBagDoesntExist,

        /// Storage provider operator doesn't exist.
        StorageProviderOperatorDoesntExist,
    }
}

decl_module! {
    /// _Storage_ substrate module.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Default deposit_event() handler
        fn deposit_event() = default;

        /// Predefined errors.
        type Error = Error<T>;

        /// Exports const - max allowed storage bucket number.
        const MaxStorageBucketNumber: u64 = T::MaxStorageBucketNumber::get();

        /// Exports const - max number of data objects per bag.
        const MaxNumberOfDataObjectsPerBag: u64 = T::MaxNumberOfDataObjectsPerBag::get();

        /// Exports const - a prize for a data object deletion.
        const DataObjectDeletionPrize: BalanceOf<T> = T::DataObjectDeletionPrize::get();

        /// Exports const - maximum size of the "hash blacklist" collection.
        const BlacklistSizeLimit: u64 = T::BlacklistSizeLimit::get();

        /// Exports const - "Storage buckets per bag" value constraint.
        const StorageBucketsPerBagValueConstraint: StorageBucketsPerBagValueConstraint =
            T::StorageBucketsPerBagValueConstraint::get();

        /// Exports const - the default dynamic bag creation policy for members.
        const DefaultMemberDynamicBagCreationPolicy: DynamicBagCreationPolicy =
            T::DefaultMemberDynamicBagCreationPolicy::get();

        /// Exports const - the default dynamic bag creation policy for channels.
        const DefaultChannelDynamicBagCreationPolicy: DynamicBagCreationPolicy =
            T::DefaultChannelDynamicBagCreationPolicy::get();

        // ===== Storage Lead actions =====

        /// Delete storage bucket. Must be empty. Storage operator must be missing.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn delete_storage_bucket(
            origin,
            storage_bucket_id: T::StorageBucketId,
        ){
            T::ensure_working_group_leader_origin(origin)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_missing_invitation_status(&bucket)?;

            ensure!(
                bucket.voucher.objects_used == 0,
                Error::<T>::CannotDeleteNonEmptyStorageBucket
            );

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::remove(storage_bucket_id);

            Self::deposit_event(
                RawEvent::StorageBucketDeleted(storage_bucket_id)
            );
        }

        /// Update whether uploading is globally blocked.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_uploading_blocked_status(origin, new_status: bool) {
            T::ensure_working_group_leader_origin(origin)?;

            //
            // == MUTATION SAFE ==
            //

            UploadingBlocked::put(new_status);

            Self::deposit_event(RawEvent::UploadingBlockStatusUpdated(new_status));
        }

        /// Updates size-based pricing of new objects uploaded.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_data_size_fee(origin, new_data_size_fee: BalanceOf<T>) {
            T::ensure_working_group_leader_origin(origin)?;

            //
            // == MUTATION SAFE ==
            //

            DataObjectPerMegabyteFee::<T>::put(new_data_size_fee);

            Self::deposit_event(RawEvent::DataObjectPerMegabyteFeeUpdated(new_data_size_fee));
        }

        /// Updates "Storage buckets per bag" number limit.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_buckets_per_bag_limit(origin, new_limit: u64) {
            T::ensure_working_group_leader_origin(origin)?;

            T::StorageBucketsPerBagValueConstraint::get().ensure_valid(
                new_limit,
                Error::<T>::StorageBucketsPerBagLimitTooLow,
                Error::<T>::StorageBucketsPerBagLimitTooHigh,
            )?;

            //
            // == MUTATION SAFE ==
            //

            StorageBucketsPerBagLimit::put(new_limit);

            Self::deposit_event(RawEvent::StorageBucketsPerBagLimitUpdated(new_limit));
        }

        /// Updates "Storage buckets voucher max limits".
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_buckets_voucher_max_limits(
            origin,
            new_objects_size: u64,
            new_objects_number: u64,
        ) {
            T::ensure_working_group_leader_origin(origin)?;

            //
            // == MUTATION SAFE ==
            //

            VoucherMaxObjectsSizeLimit::put(new_objects_size);
            VoucherMaxObjectsNumberLimit::put(new_objects_number);

            Self::deposit_event(
                RawEvent::StorageBucketsVoucherMaxLimitsUpdated(new_objects_size, new_objects_number)
            );
        }

        /// Update number of storage buckets used in given dynamic bag creation policy.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy(
            origin,
            dynamic_bag_type: DynamicBagType,
            number_of_storage_buckets: u64,
        ) {
            T::ensure_working_group_leader_origin(origin)?;

            //
            // == MUTATION SAFE ==
            //

            let mut creation_policy = Self::get_dynamic_bag_creation_policy(dynamic_bag_type);

            creation_policy.number_of_storage_buckets = number_of_storage_buckets;

            DynamicBagCreationPolicies::insert(dynamic_bag_type, creation_policy);

            Self::deposit_event(
                RawEvent::NumberOfStorageBucketsInDynamicBagCreationPolicyUpdated(
                    dynamic_bag_type,
                    number_of_storage_buckets
                )
            );
        }

        /// Add and remove hashes to the current blacklist.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_blacklist(
            origin,
            remove_hashes: BTreeSet<ContentId>,
            add_hashes: BTreeSet<ContentId>
        ){
            T::ensure_working_group_leader_origin(origin)?;

            // Get only hashes that exist in the blacklist.
            let verified_remove_hashes = Self::get_existing_hashes(&remove_hashes);

            // Get only hashes that doesn't exist in the blacklist.
            let verified_add_hashes = Self::get_nonexisting_hashes(&add_hashes);

            let updated_blacklist_size: u64 = Self::current_blacklist_size()
                .saturating_add(verified_add_hashes.len().saturated_into::<u64>())
                .saturating_sub(verified_remove_hashes.len().saturated_into::<u64>());

            ensure!(
                updated_blacklist_size <= T::BlacklistSizeLimit::get(),
                Error::<T>::BlacklistSizeLimitExceeded
            );

            //
            // == MUTATION SAFE ==
            //

            for cid in verified_remove_hashes.iter() {
                Blacklist::remove(cid);
            }

            for cid in verified_add_hashes.iter() {
                Blacklist::insert(cid, ());
            }

            CurrentBlacklistSize::put(updated_blacklist_size);

            Self::deposit_event(RawEvent::UpdateBlacklist(remove_hashes, add_hashes));
        }

        /// Create storage bucket.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn create_storage_bucket(
            origin,
            invite_worker: Option<WorkerId<T>>,
            accepting_new_bags: bool,
            size_limit: u64,
            objects_limit: u64,
        ) {
            T::ensure_working_group_leader_origin(origin)?;

            let voucher = Voucher {
                size_limit,
                objects_limit,
                ..Default::default()
            };

            Self::can_create_storage_bucket(&voucher, &invite_worker)?;

            //
            // == MUTATION SAFE ==
            //

            let operator_status = invite_worker
                .map(StorageBucketOperatorStatus::InvitedStorageWorker)
                .unwrap_or(StorageBucketOperatorStatus::Missing);

            let storage_bucket = StorageBucket {
                operator_status,
                accepting_new_bags,
                voucher,
                metadata: Vec::new(),
            };

            let storage_bucket_id = Self::next_storage_bucket_id();

            StorageBucketsNumber::put(Self::storage_buckets_number() + 1);

            <NextStorageBucketId<T>>::put(storage_bucket_id + One::one());

            <StorageBucketById<T>>::insert(storage_bucket_id, storage_bucket);

            Self::deposit_event(
                RawEvent::StorageBucketCreated(
                    storage_bucket_id,
                    invite_worker,
                    accepting_new_bags,
                    size_limit,
                    objects_limit,
                )
            );
        }

        /// Updates storage buckets for a bag..
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_buckets_for_bag(
            origin,
            bag_id: BagId<T>,
            add_buckets: BTreeSet<T::StorageBucketId>,
            remove_buckets: BTreeSet<T::StorageBucketId>,
        ) {
            T::ensure_working_group_leader_origin(origin)?;

            let voucher_update = Self::validate_update_storage_buckets_for_bag_params(
                &bag_id,
                &add_buckets,
                &remove_buckets,
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Update vouchers.
            if !add_buckets.is_empty() {
                BagManager::<T>::add_storage_buckets(&bag_id, add_buckets.clone());

                Self::change_storage_buckets_vouchers(
                    &add_buckets,
                    &voucher_update,
                    OperationType::Increase
                );
            }
            if !remove_buckets.is_empty() {
                BagManager::<T>::remove_storage_buckets(&bag_id, remove_buckets.clone());

                Self::change_storage_buckets_vouchers(
                    &remove_buckets,
                    &voucher_update,
                    OperationType::Decrease
                );
            }

            Self::deposit_event(
                RawEvent::StorageBucketsUpdatedForBag(bag_id, add_buckets, remove_buckets)
            );
        }

        /// Cancel pending storage bucket invite. An invitation must be pending.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_storage_bucket_operator_invite(origin, storage_bucket_id: T::StorageBucketId){
            T::ensure_working_group_leader_origin(origin)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_pending_invitation_status(&bucket)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.operator_status = StorageBucketOperatorStatus::Missing;
            });

            Self::deposit_event(
                RawEvent::StorageBucketInvitationCancelled(storage_bucket_id)
            );
        }

        /// Invite storage bucket operator. Must be missing.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn invite_storage_bucket_operator(
            origin,
            storage_bucket_id: T::StorageBucketId,
            operator_id: WorkerId<T>,
        ){
            T::ensure_working_group_leader_origin(origin)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_missing_invitation_status(&bucket)?;

            Self::ensure_storage_provider_operator_exists(&operator_id)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.operator_status =
                    StorageBucketOperatorStatus::InvitedStorageWorker(operator_id);
            });

            Self::deposit_event(
                RawEvent::StorageBucketOperatorInvited(storage_bucket_id, operator_id)
            );
        }

        /// Removes storage bucket operator. Must be invited.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_storage_bucket_operator(
            origin,
            storage_bucket_id: T::StorageBucketId,
        ){
            T::ensure_working_group_leader_origin(origin)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_storage_provider_invitation_status_for_removal(&bucket)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.operator_status =
                    StorageBucketOperatorStatus::Missing;
            });

            Self::deposit_event(
                RawEvent::StorageBucketOperatorRemoved(storage_bucket_id)
            );
        }

        // ===== Storage Operator actions =====

        /// Accept the storage bucket invitation. An invitation must match the worker_id parameter.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_storage_bucket_invitation(
            origin,
            worker_id: WorkerId<T>,
            storage_bucket_id: T::StorageBucketId
        ) {
            T::ensure_worker_origin(origin, worker_id)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_storage_provider_invitation_status(&bucket, worker_id)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.operator_status = StorageBucketOperatorStatus::StorageWorker(worker_id);
            });

            Self::deposit_event(
                RawEvent::StorageBucketInvitationAccepted(storage_bucket_id, worker_id)
            );
        }

        /// Sets storage operator metadata (eg.: storage node URL).
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_storage_operator_metadata(
            origin,
            worker_id: WorkerId<T>,
            storage_bucket_id: T::StorageBucketId,
            metadata: Vec<u8>
        ) {
            T::ensure_worker_origin(origin, worker_id)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_invitation_accepted(&bucket, worker_id)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.metadata = metadata.clone();
            });

            Self::deposit_event(
                RawEvent::StorageOperatorMetadataSet(storage_bucket_id, worker_id, metadata)
            );
        }

        /// Sets storage bucket voucher limits.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_storage_bucket_voucher_limits(
            origin,
            worker_id: WorkerId<T>,
            storage_bucket_id: T::StorageBucketId,
            new_objects_size_limit: u64,
            new_objects_number_limit: u64,
        ) {
            T::ensure_worker_origin(origin, worker_id)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_invitation_accepted(&bucket, worker_id)?;

            ensure!(
                new_objects_size_limit <= Self::voucher_max_objects_size_limit(),
                Error::<T>::VoucherMaxObjectSizeLimitExceeded
            );

            ensure!(
                new_objects_number_limit <= Self::voucher_max_objects_number_limit(),
                Error::<T>::VoucherMaxObjectNumberLimitExceeded
            );

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.voucher = Voucher{
                    size_limit: new_objects_size_limit,
                    objects_limit: new_objects_number_limit,
                    ..bucket.voucher
                };
            });

            Self::deposit_event(
                RawEvent::StorageBucketVoucherLimitsSet(
                    storage_bucket_id,
                    worker_id,
                    new_objects_size_limit,
                    new_objects_number_limit
                )
            );
        }

        /// A storage provider signals that the data object was successfully uploaded to its storage.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_pending_data_objects(
            origin,
            worker_id: WorkerId<T>,
            storage_bucket_id: T::StorageBucketId,
            bag_id: BagId<T>,
            data_objects: BTreeSet<T::DataObjectId>,
        ) {
            T::ensure_worker_origin(origin, worker_id)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_invitation_accepted(&bucket, worker_id)?;

            Self::validate_accept_pending_data_objects_params(
                &bag_id,
                &data_objects,
                &storage_bucket_id
            )?;

            //
            // == MUTATION SAFE ==
            //

            for data_object_id in data_objects.iter() {
                BagManager::<T>::accept_data_objects(&bag_id, &data_object_id);
            }

            Self::deposit_event(
                    RawEvent::PendingDataObjectsAccepted(storage_bucket_id, worker_id, bag_id, data_objects)
            );
        }

        /// Update whether new bags are being accepted for storage.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_storage_bucket_status(
            origin,
            worker_id: WorkerId<T>,
            storage_bucket_id: T::StorageBucketId,
            accepting_new_bags: bool
        ) {
            T::ensure_worker_origin(origin, worker_id)?;

            let bucket = Self::ensure_storage_bucket_exists(&storage_bucket_id)?;

            Self::ensure_bucket_invitation_accepted(&bucket, worker_id)?;

            //
            // == MUTATION SAFE ==
            //

            <StorageBucketById<T>>::mutate(storage_bucket_id, |bucket| {
                bucket.accepting_new_bags = accepting_new_bags;
            });

            Self::deposit_event(
                RawEvent::StorageBucketStatusUpdated(
                    storage_bucket_id,
                    worker_id,
                    accepting_new_bags
                )
            );
        }

        /// Upload new data objects. Development mode.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn sudo_upload_data_objects(origin, params: UploadParameters<T>) {
          ensure_root(origin)?;

          Self::upload_data_objects(params)?;
        }

        /// Create a dynamic bag. Development mode.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn sudo_create_dynamic_bag(origin, bag_id: DynamicBagId<T>) {
          ensure_root(origin)?;

          Self::create_dynamic_bag(bag_id)?;
        }
    }
}

// Public methods
impl<T: Trait> DataObjectStorage<T> for Module<T> {
    fn can_upload_data_objects(params: &UploadParameters<T>) -> DispatchResult {
        Self::validate_upload_data_objects_parameters(params).map(|_| ())
    }

    fn upload_data_objects(params: UploadParameters<T>) -> DispatchResult {
        let bag_change = Self::validate_upload_data_objects_parameters(&params)?;

        //
        // == MUTATION SAFE ==
        //

        let data = Self::create_data_objects(params.object_creation_list.clone());

        <StorageTreasury<T>>::deposit(
            &params.deletion_prize_source_account_id,
            bag_change.total_deletion_prize,
        )?;

        Self::slash_data_size_fee(
            &params.deletion_prize_source_account_id,
            bag_change.voucher_update.objects_total_size,
        );

        <NextDataObjectId<T>>::put(data.next_data_object_id);

        BagManager::<T>::append_data_objects(&params.bag_id, &data.data_objects_map);

        let operation_type = OperationType::Increase;

        // Add a deletion prize for the dynamic bag only.
        Self::change_deletion_prize_for_bag(
            &params.bag_id,
            bag_change.total_deletion_prize,
            operation_type,
        );

        Self::change_storage_bucket_vouchers_for_bag(
            &params.bag_id,
            &bag_change.voucher_update,
            operation_type,
        );

        Self::deposit_event(RawEvent::DataObjectdUploaded(
            data.data_objects_map.keys().cloned().collect(),
            params,
        ));

        Ok(())
    }

    fn can_move_data_objects(
        src_bag_id: &BagId<T>,
        dest_bag_id: &BagId<T>,
        objects: &BTreeSet<<T as Trait>::DataObjectId>,
    ) -> DispatchResult {
        Self::validate_data_objects_on_moving(src_bag_id, dest_bag_id, objects).map(|_| ())
    }

    fn move_data_objects(
        src_bag_id: BagId<T>,
        dest_bag_id: BagId<T>,
        objects: BTreeSet<T::DataObjectId>,
    ) -> DispatchResult {
        let bag_change =
            Self::validate_data_objects_on_moving(&src_bag_id, &dest_bag_id, &objects)?;

        //
        // == MUTATION SAFE ==
        //

        BagManager::<T>::move_data_objects(&src_bag_id, &dest_bag_id, &objects);

        // Change source bag.
        let src_operation_type = OperationType::Decrease;
        Self::change_storage_bucket_vouchers_for_bag(
            &src_bag_id,
            &bag_change.voucher_update,
            src_operation_type,
        );
        Self::change_deletion_prize_for_bag(
            &src_bag_id,
            bag_change.total_deletion_prize,
            src_operation_type,
        );

        // Change destination bag.
        let dest_operation_type = OperationType::Increase;
        Self::change_storage_bucket_vouchers_for_bag(
            &dest_bag_id,
            &bag_change.voucher_update,
            dest_operation_type,
        );
        Self::change_deletion_prize_for_bag(
            &dest_bag_id,
            bag_change.total_deletion_prize,
            dest_operation_type,
        );

        Self::deposit_event(RawEvent::DataObjectsMoved(src_bag_id, dest_bag_id, objects));

        Ok(())
    }

    fn can_delete_data_objects(
        bag_id: &BagId<T>,
        objects: &BTreeSet<T::DataObjectId>,
    ) -> DispatchResult {
        Self::validate_delete_data_objects_params(bag_id, objects).map(|_| ())
    }

    fn delete_data_objects(
        deletion_prize_account_id: T::AccountId,
        bag_id: BagId<T>,
        objects: BTreeSet<T::DataObjectId>,
    ) -> DispatchResult {
        let bag_change = Self::validate_delete_data_objects_params(&bag_id, &objects)?;

        //
        // == MUTATION SAFE ==
        //

        <StorageTreasury<T>>::withdraw(
            &deletion_prize_account_id,
            bag_change.total_deletion_prize,
        )?;

        for data_object_id in objects.iter() {
            BagManager::<T>::delete_data_object(&bag_id, &data_object_id);
        }

        let operation_type = OperationType::Decrease;

        Self::change_storage_bucket_vouchers_for_bag(
            &bag_id,
            &bag_change.voucher_update,
            operation_type,
        );

        // Subtract deletion prize for dynamic bags only.
        Self::change_deletion_prize_for_bag(
            &bag_id,
            bag_change.total_deletion_prize,
            operation_type,
        );

        Self::deposit_event(RawEvent::DataObjectsDeleted(
            deletion_prize_account_id,
            bag_id,
            objects,
        ));

        Ok(())
    }

    fn can_delete_dynamic_bag(bag_id: &DynamicBagId<T>) -> DispatchResult {
        Self::validate_delete_dynamic_bag_params(bag_id).map(|_| ())
    }

    fn delete_dynamic_bag(
        deletion_prize_account_id: T::AccountId,
        bag_id: DynamicBagId<T>,
    ) -> DispatchResult {
        let bag_change = Self::validate_delete_dynamic_bag_params(&bag_id)?;

        //
        // == MUTATION SAFE ==
        //

        <StorageTreasury<T>>::withdraw(
            &deletion_prize_account_id,
            bag_change.total_deletion_prize,
        )?;

        let dynamic_bag = Self::dynamic_bag(&bag_id);

        Self::change_storage_buckets_vouchers(
            &dynamic_bag.stored_by,
            &bag_change.voucher_update,
            OperationType::Decrease,
        );

        <DynamicBags<T>>::remove(&bag_id);

        Self::deposit_event(RawEvent::DynamicBagDeleted(
            deletion_prize_account_id,
            bag_id,
        ));

        Ok(())
    }

    fn create_dynamic_bag(bag_id: DynamicBagId<T>) -> DispatchResult {
        Self::validate_create_dynamic_bag_params(&bag_id)?;

        //
        // == MUTATION SAFE ==
        //

        let storage_buckets = Self::pick_storage_buckets_for_dynamic_bag(bag_id.clone().into());

        let bag = DynamicBag::<T> {
            stored_by: storage_buckets,
            ..Default::default()
        };

        <DynamicBags<T>>::insert(&bag_id, bag);

        Self::deposit_event(RawEvent::DynamicBagCreated(bag_id));

        Ok(())
    }

    fn can_create_dynamic_bag(bag_id: &DynamicBagId<T>) -> DispatchResult {
        Self::validate_create_dynamic_bag_params(bag_id)
    }
}

impl<T: Trait> Module<T> {
    // Validates dynamic bag creation params and conditions.
    fn validate_create_dynamic_bag_params(bag_id: &DynamicBagId<T>) -> DispatchResult {
        ensure!(
            !<DynamicBags<T>>::contains_key(&bag_id),
            Error::<T>::DynamicBagExists
        );

        Ok(())
    }

    // Validates dynamic bag deletion params and conditions.
    fn validate_delete_dynamic_bag_params(
        bag_id: &DynamicBagId<T>,
    ) -> Result<BagChangeInfo<BalanceOf<T>>, DispatchError> {
        BagManager::<T>::ensure_bag_exists(&BagId::<T>::DynamicBag(bag_id.clone()))?;

        let dynamic_bag = Self::dynamic_bag(bag_id);

        let voucher_update = VoucherUpdate {
            objects_number: dynamic_bag.objects_number(),
            objects_total_size: dynamic_bag.objects_total_size(),
        };

        ensure!(
            <StorageTreasury<T>>::usable_balance() >= dynamic_bag.deletion_prize,
            Error::<T>::InsufficientTreasuryBalance
        );

        let bag_change = BagChangeInfo {
            voucher_update,
            total_deletion_prize: dynamic_bag.deletion_prize,
        };

        Ok(bag_change)
    }

    // Ensures the existence of the storage bucket.
    // Returns the StorageBucket object or error.
    fn ensure_storage_bucket_exists(
        storage_bucket_id: &T::StorageBucketId,
    ) -> Result<StorageBucket<WorkerId<T>>, Error<T>> {
        ensure!(
            <StorageBucketById<T>>::contains_key(storage_bucket_id),
            Error::<T>::StorageBucketDoesntExist
        );

        Ok(Self::storage_bucket_by_id(storage_bucket_id))
    }

    // Ensures the correct invitation for the storage bucket and storage provider. Storage provider
    // must be invited.
    fn ensure_bucket_storage_provider_invitation_status(
        bucket: &StorageBucket<WorkerId<T>>,
        worker_id: WorkerId<T>,
    ) -> DispatchResult {
        match bucket.operator_status {
            StorageBucketOperatorStatus::Missing => {
                Err(Error::<T>::NoStorageBucketInvitation.into())
            }
            StorageBucketOperatorStatus::StorageWorker(_) => {
                Err(Error::<T>::StorageProviderAlreadySet.into())
            }
            StorageBucketOperatorStatus::InvitedStorageWorker(invited_worker_id) => {
                ensure!(
                    worker_id == invited_worker_id,
                    Error::<T>::DifferentStorageProviderInvited
                );

                Ok(())
            }
        }
    }

    // Ensures the correct invitation for the storage bucket and storage provider for removal.
    // Must be invited storage provider.
    fn ensure_bucket_storage_provider_invitation_status_for_removal(
        bucket: &StorageBucket<WorkerId<T>>,
    ) -> DispatchResult {
        if let StorageBucketOperatorStatus::StorageWorker(_) = bucket.operator_status {
            Ok(())
        } else {
            Err(Error::<T>::StorageProviderMustBeSet.into())
        }
    }

    // Ensures the correct invitation for the storage bucket and storage provider. Must be pending.
    fn ensure_bucket_pending_invitation_status(
        bucket: &StorageBucket<WorkerId<T>>,
    ) -> DispatchResult {
        match bucket.operator_status {
            StorageBucketOperatorStatus::Missing => {
                Err(Error::<T>::NoStorageBucketInvitation.into())
            }
            StorageBucketOperatorStatus::StorageWorker(_) => {
                Err(Error::<T>::StorageProviderAlreadySet.into())
            }
            StorageBucketOperatorStatus::InvitedStorageWorker(_) => Ok(()),
        }
    }

    // Ensures the missing invitation for the storage bucket and storage provider.
    fn ensure_bucket_missing_invitation_status(
        bucket: &StorageBucket<WorkerId<T>>,
    ) -> DispatchResult {
        match bucket.operator_status {
            StorageBucketOperatorStatus::Missing => Ok(()),
            StorageBucketOperatorStatus::StorageWorker(_) => {
                Err(Error::<T>::StorageProviderAlreadySet.into())
            }
            StorageBucketOperatorStatus::InvitedStorageWorker(_) => {
                Err(Error::<T>::InvitedStorageProvider.into())
            }
        }
    }

    // Ensures correct storage provider for the storage bucket.
    fn ensure_bucket_invitation_accepted(
        bucket: &StorageBucket<WorkerId<T>>,
        worker_id: WorkerId<T>,
    ) -> DispatchResult {
        match bucket.operator_status {
            StorageBucketOperatorStatus::Missing => {
                Err(Error::<T>::StorageProviderMustBeSet.into())
            }
            StorageBucketOperatorStatus::InvitedStorageWorker(_) => {
                Err(Error::<T>::InvalidStorageProvider.into())
            }
            StorageBucketOperatorStatus::StorageWorker(invited_worker_id) => {
                ensure!(
                    worker_id == invited_worker_id,
                    Error::<T>::InvalidStorageProvider
                );

                Ok(())
            }
        }
    }

    // Create data objects from the creation data.
    fn create_data_objects(
        object_creation_list: Vec<DataObjectCreationParameters>,
    ) -> DataObjectCandidates<T> {
        let deletion_prize = T::DataObjectDeletionPrize::get();

        let data_objects = object_creation_list.iter().cloned().map(|obj| DataObject {
            accepted: false,
            deletion_prize,
            size: obj.size,
        });

        let mut next_data_object_id = Self::next_data_object_id();
        let ids = iter::repeat_with(|| {
            let id = next_data_object_id;
            next_data_object_id += One::one();

            id
        })
        .take(data_objects.len());

        let data_objects_map = ids.zip(data_objects).collect::<BTreeMap<_, _>>();

        DataObjectCandidates {
            next_data_object_id,
            data_objects_map,
        }
    }

    // Ensures validity of the `accept_pending_data_objects` extrinsic parameters
    fn validate_accept_pending_data_objects_params(
        bag_id: &BagId<T>,
        data_objects: &BTreeSet<T::DataObjectId>,
        storage_bucket_id: &T::StorageBucketId,
    ) -> DispatchResult {
        ensure!(
            !data_objects.is_empty(),
            Error::<T>::DataObjectIdParamsAreEmpty
        );

        BagManager::<T>::ensure_bag_exists(bag_id)?;
        BagManager::<T>::ensure_storage_bucket_bound(bag_id, storage_bucket_id)?;

        for data_object_id in data_objects.iter() {
            BagManager::<T>::ensure_data_object_existence(bag_id, data_object_id)?;
        }

        Ok(())
    }

    // Ensures validity of the `update_storage_buckets_for_bag` extrinsic parameters
    fn validate_update_storage_buckets_for_bag_params(
        bag_id: &BagId<T>,
        add_buckets: &BTreeSet<T::StorageBucketId>,
        remove_buckets: &BTreeSet<T::StorageBucketId>,
    ) -> Result<VoucherUpdate, DispatchError> {
        ensure!(
            !add_buckets.is_empty() || !remove_buckets.is_empty(),
            Error::<T>::StorageBucketIdCollectionsAreEmpty
        );

        BagManager::<T>::ensure_bag_exists(&bag_id)?;

        let storage_bucket_ids = BagManager::<T>::get_storage_bucket_ids(bag_id);
        ensure!(
            storage_bucket_ids.len().saturated_into::<u64>()
                <= Self::storage_buckets_per_bag_limit(),
            Error::<T>::StorageBucketPerBagLimitExceeded
        );

        for bucket_id in remove_buckets.iter() {
            ensure!(
                <StorageBucketById<T>>::contains_key(&bucket_id),
                Error::<T>::StorageBucketDoesntExist
            );

            ensure!(
                storage_bucket_ids.contains(&bucket_id),
                Error::<T>::StorageBucketIsNotBoundToBag
            );
        }

        for bucket_id in add_buckets.iter() {
            let bucket = Self::ensure_storage_bucket_exists(bucket_id)?;

            ensure!(
                bucket.accepting_new_bags,
                Error::<T>::StorageBucketDoesntAcceptNewBags
            );

            ensure!(
                !storage_bucket_ids.contains(&bucket_id),
                Error::<T>::StorageBucketIsBoundToBag
            );
        }

        let objects_total_size = BagManager::<T>::get_data_objects_total_size(bag_id);
        let objects_number = BagManager::<T>::get_data_objects_number(bag_id);

        let voucher_update = VoucherUpdate {
            objects_number,
            objects_total_size,
        };

        Self::check_buckets_for_overflow(&add_buckets, &voucher_update)?;

        Ok(voucher_update)
    }

    // Validate the "Move data objects between bags" operation data.
    fn validate_data_objects_on_moving(
        src_bag_id: &BagId<T>,
        dest_bag_id: &BagId<T>,
        object_ids: &BTreeSet<T::DataObjectId>,
    ) -> Result<BagChangeInfo<BalanceOf<T>>, DispatchError> {
        ensure!(
            *src_bag_id != *dest_bag_id,
            Error::<T>::SourceAndDestinationBagsAreEqual
        );

        ensure!(
            !object_ids.is_empty(),
            Error::<T>::DataObjectIdCollectionIsEmpty
        );

        BagManager::<T>::ensure_bag_exists(src_bag_id)?;
        BagManager::<T>::ensure_bag_exists(dest_bag_id)?;

        let mut bag_change = BagChangeInfo::<BalanceOf<T>>::default();

        for object_id in object_ids.iter() {
            let data_object = BagManager::<T>::ensure_data_object_existence(src_bag_id, object_id)?;

            bag_change.add_object(data_object.size, data_object.deletion_prize);
        }

        Self::check_bag_for_buckets_overflow(dest_bag_id, &bag_change.voucher_update)?;

        Ok(bag_change)
    }

    // Returns only existing hashes in the blacklist from the original collection.
    #[allow(clippy::redundant_closure)] // doesn't work with Substrate storage functions.
    fn get_existing_hashes(hashes: &BTreeSet<ContentId>) -> BTreeSet<ContentId> {
        Self::get_hashes_by_predicate(hashes, |cid| Blacklist::contains_key(cid))
    }

    // Returns only nonexisting hashes in the blacklist from the original collection.
    fn get_nonexisting_hashes(hashes: &BTreeSet<ContentId>) -> BTreeSet<ContentId> {
        Self::get_hashes_by_predicate(hashes, |cid| !Blacklist::contains_key(cid))
    }

    // Returns hashes from the original collection selected by predicate.
    fn get_hashes_by_predicate<P: FnMut(&&ContentId) -> bool>(
        hashes: &BTreeSet<ContentId>,
        predicate: P,
    ) -> BTreeSet<ContentId> {
        hashes
            .iter()
            .filter(predicate)
            .cloned()
            .collect::<BTreeSet<_>>()
    }

    // Ensure the new bucket could be created. It also validates some parameters.
    fn can_create_storage_bucket(
        voucher: &Voucher,
        invited_worker: &Option<WorkerId<T>>,
    ) -> DispatchResult {
        ensure!(
            Self::storage_buckets_number() < T::MaxStorageBucketNumber::get(),
            Error::<T>::MaxStorageBucketNumberLimitExceeded
        );

        ensure!(
            voucher.size_limit <= Self::voucher_max_objects_size_limit(),
            Error::<T>::VoucherMaxObjectSizeLimitExceeded
        );

        ensure!(
            voucher.objects_limit <= Self::voucher_max_objects_number_limit(),
            Error::<T>::VoucherMaxObjectNumberLimitExceeded
        );

        if let Some(operator_id) = invited_worker {
            Self::ensure_storage_provider_operator_exists(operator_id)?;
        }

        Ok(())
    }

    // Update total objects size and number for all storage buckets assigned to a bag.
    fn change_storage_bucket_vouchers_for_bag(
        bag_id: &BagId<T>,
        voucher_update: &VoucherUpdate,
        voucher_operation: OperationType,
    ) {
        let bucket_ids = BagManager::<T>::get_storage_bucket_ids(bag_id);

        Self::change_storage_buckets_vouchers(&bucket_ids, voucher_update, voucher_operation);
    }

    // Update total objects size and number for provided storage buckets.
    fn change_storage_buckets_vouchers(
        bucket_ids: &BTreeSet<T::StorageBucketId>,
        voucher_update: &VoucherUpdate,
        voucher_operation: OperationType,
    ) {
        for bucket_id in bucket_ids.iter() {
            <StorageBucketById<T>>::mutate(bucket_id, |bucket| {
                bucket.voucher =
                    voucher_update.get_updated_voucher(&bucket.voucher, voucher_operation);

                Self::deposit_event(RawEvent::VoucherChanged(*bucket_id, bucket.voucher.clone()));
            });
        }
    }

    // Validates `delete_data_objects` parameters.
    // Returns voucher update for an affected bag.
    fn validate_delete_data_objects_params(
        bag_id: &BagId<T>,
        data_object_ids: &BTreeSet<T::DataObjectId>,
    ) -> Result<BagChangeInfo<BalanceOf<T>>, DispatchError> {
        ensure!(
            !data_object_ids.is_empty(),
            Error::<T>::DataObjectIdParamsAreEmpty
        );

        BagManager::<T>::ensure_bag_exists(bag_id)?;

        let mut bag_change = BagChangeInfo::default();

        for data_object_id in data_object_ids.iter() {
            let data_object =
                BagManager::<T>::ensure_data_object_existence(bag_id, data_object_id)?;

            bag_change.add_object(data_object.size, data_object.deletion_prize);
        }

        ensure!(
            <StorageTreasury<T>>::usable_balance() >= bag_change.total_deletion_prize,
            Error::<T>::InsufficientTreasuryBalance
        );

        Ok(bag_change)
    }

    // Validates upload parameters and conditions (like global uploading block).
    // Returns voucher update parameters for the storage buckets.
    fn validate_upload_data_objects_parameters(
        params: &UploadParameters<T>,
    ) -> Result<BagChangeInfo<BalanceOf<T>>, DispatchError> {
        // Check global uploading block.
        ensure!(!Self::uploading_blocked(), Error::<T>::UploadingBlocked);

        // Check object creation list validity.
        ensure!(
            !params.object_creation_list.is_empty(),
            Error::<T>::NoObjectsOnUpload
        );

        BagManager::<T>::ensure_bag_exists(&params.bag_id)?;

        let bag_objects_number = BagManager::<T>::get_data_objects_number(&params.bag_id.clone());
        let new_objects_number: u64 = params.object_creation_list.len().saturated_into();
        let total_possible_data_objects_number: u64 = new_objects_number + bag_objects_number;

        // Check bag capacity.
        ensure!(
            total_possible_data_objects_number <= T::MaxNumberOfDataObjectsPerBag::get(),
            Error::<T>::DataObjectsPerBagLimitExceeded
        );

        let mut bag_change = BagChangeInfo::default();
        // Check data objects.
        for object_params in params.object_creation_list.iter() {
            // Should be non-empty hash.
            ensure!(
                !object_params.ipfs_content_id.is_empty(),
                Error::<T>::EmptyContentId
            );
            // Should be non-zero size.
            ensure!(object_params.size != 0, Error::<T>::ZeroObjectSize);

            // Should not be blacklisted.
            ensure!(
                !Blacklist::contains_key(&object_params.ipfs_content_id),
                Error::<T>::DataObjectBlacklisted,
            );

            bag_change.add_object(object_params.size, T::DataObjectDeletionPrize::get());
        }

        let size_fee =
            Self::calculate_data_storage_fee(bag_change.voucher_update.objects_total_size);
        let usable_balance =
            Balances::<T>::usable_balance(&params.deletion_prize_source_account_id);

        // Check account balance to satisfy deletion prize and storage fee.
        let total_fee = bag_change.total_deletion_prize + size_fee;
        ensure!(usable_balance >= total_fee, Error::<T>::InsufficientBalance);

        // Check buckets.
        Self::check_bag_for_buckets_overflow(&params.bag_id, &bag_change.voucher_update)?;

        Ok(bag_change)
    }

    // Iterates through buckets in the bag. Verifies voucher parameters to fit the new limits:
    // objects number and total objects size.
    fn check_bag_for_buckets_overflow(
        bag_id: &BagId<T>,
        voucher_update: &VoucherUpdate,
    ) -> DispatchResult {
        let bucket_ids = BagManager::<T>::get_storage_bucket_ids(bag_id);

        Self::check_buckets_for_overflow(&bucket_ids, voucher_update)
    }

    // Iterates through buckets. Verifies voucher parameters to fit the new limits:
    // objects number and total objects size.
    fn check_buckets_for_overflow(
        bucket_ids: &BTreeSet<T::StorageBucketId>,
        voucher_update: &VoucherUpdate,
    ) -> DispatchResult {
        for bucket_id in bucket_ids.iter() {
            let bucket = Self::storage_bucket_by_id(bucket_id);

            // Total object number limit is not exceeded.
            ensure!(
                voucher_update.objects_number + bucket.voucher.objects_used
                    <= bucket.voucher.objects_limit,
                Error::<T>::StorageBucketObjectNumberLimitReached
            );

            // Total object size limit is not exceeded.
            ensure!(
                voucher_update.objects_total_size + bucket.voucher.size_used
                    <= bucket.voucher.size_limit,
                Error::<T>::StorageBucketObjectSizeLimitReached
            );
        }

        Ok(())
    }

    // Increase or decrease a deletion prize for a dynamic bag.
    fn change_deletion_prize_for_dynamic_bag(
        dynamic_bag_id: &DynamicBagId<T>,
        deletion_prize: BalanceOf<T>,
        operation: OperationType,
    ) {
        <DynamicBags<T>>::mutate(dynamic_bag_id, |bag| {
            bag.deletion_prize = match operation {
                OperationType::Increase => bag.deletion_prize.saturating_add(deletion_prize),
                OperationType::Decrease => bag.deletion_prize.saturating_sub(deletion_prize),
            };

            Self::deposit_event(RawEvent::DeletionPrizeChanged(
                dynamic_bag_id.clone(),
                bag.deletion_prize,
            ));
        });
    }

    // Increase or decrease a deletion prize for a dynamic bag.
    // Affect dynamic bags only. Skips static bags.
    fn change_deletion_prize_for_bag(
        bag_id: &BagId<T>,
        deletion_prize: BalanceOf<T>,
        operation: OperationType,
    ) {
        if let BagId::<T>::DynamicBag(ref dynamic_bag_id) = bag_id {
            Self::change_deletion_prize_for_dynamic_bag(dynamic_bag_id, deletion_prize, operation);
        }
    }

    // Calculate data storage fee based on size. Fee-value uses megabytes as measure value.
    // Data size will be rounded to nearest greater MB integer.
    pub(crate) fn calculate_data_storage_fee(bytes: u64) -> BalanceOf<T> {
        let mb_fee = Self::data_object_per_mega_byte_fee();

        const ONE_MB: u64 = 1_048_576;

        let mut megabytes = bytes / ONE_MB;

        if bytes % ONE_MB > 0 {
            megabytes += 1; // rounding to the nearest greater integer
        }

        mb_fee.saturating_mul(megabytes.saturated_into())
    }

    // Slash data size fee if fee value is set to non-zero.
    fn slash_data_size_fee(account_id: &T::AccountId, bytes: u64) {
        let fee = Self::calculate_data_storage_fee(bytes);

        if fee != Zero::zero() {
            let _ = Balances::<T>::slash(account_id, fee);
        }
    }

    // Selects storage bucket ID sets to assign to the storage bucket.
    pub(crate) fn pick_storage_buckets_for_dynamic_bag(
        bag_type: DynamicBagType,
    ) -> BTreeSet<T::StorageBucketId> {
        StorageBucketPicker::<T>::pick_storage_buckets(bag_type)
    }

    // Get default dynamic bag policy by bag type.
    fn get_default_dynamic_bag_creation_policy(
        bag_type: DynamicBagType,
    ) -> DynamicBagCreationPolicy {
        match bag_type {
            DynamicBagType::Member => T::DefaultMemberDynamicBagCreationPolicy::get(),
            DynamicBagType::Channel => T::DefaultChannelDynamicBagCreationPolicy::get(),
        }
    }

    // Loads dynamic bag creation policy or use default values.
    pub(crate) fn get_dynamic_bag_creation_policy(
        bag_type: DynamicBagType,
    ) -> DynamicBagCreationPolicy {
        if DynamicBagCreationPolicies::contains_key(bag_type) {
            return Self::dynamic_bag_creation_policy(bag_type);
        }

        Self::get_default_dynamic_bag_creation_policy(bag_type)
    }

    // Verifies storage provider operator existence.
    fn ensure_storage_provider_operator_exists(operator_id: &WorkerId<T>) -> DispatchResult {
        ensure!(
            T::ensure_worker_exists(operator_id).is_ok(),
            Error::<T>::StorageProviderOperatorDoesntExist
        );

        Ok(())
    }
}
