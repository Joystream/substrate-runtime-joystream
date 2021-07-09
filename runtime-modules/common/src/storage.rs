use crate::working_group::WorkingGroup;
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::DispatchResult;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
pub struct ContentParameters<ContentId, DataObjectTypeId> {
    pub content_id: ContentId,
    pub type_id: DataObjectTypeId,
    pub size: u64,
    pub ipfs_content_id: Vec<u8>,
}

// New owner type for storage object struct
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
pub enum StorageObjectOwner<MemberId, ChannelId, DAOId> {
    Member(MemberId),
    Channel(ChannelId), // acts through content directory module, where again DAOs can own channels for example
    #[allow(clippy::upper_case_acronyms)]
    DAO(DAOId), // acts through upcoming `content_finance` module
    Council,            // acts through proposal frame_system
    WorkingGroup(WorkingGroup), // acts through new extrinsic in working group
}

impl<MemberId, ChannelId, DAOId> Default for StorageObjectOwner<MemberId, ChannelId, DAOId> {
    fn default() -> Self {
        Self::Council
    }
}
// To be implemented by current storage data_directory runtime module.
// Defined in 'common' package
pub trait StorageSystem<T: crate::StorageOwnership + crate::MembershipTypes> {
    fn atomically_add_content(
        owner: StorageObjectOwner<T::MemberId, T::ChannelId, T::DAOId>,
        content_parameters: Vec<ContentParameters<T::ContentId, T::DataObjectTypeId>>,
    ) -> DispatchResult;

    // Checks if given owner can add provided content to the storage frame_system
    fn can_add_content(
        owner: StorageObjectOwner<T::MemberId, T::ChannelId, T::DAOId>,
        content_parameters: Vec<ContentParameters<T::ContentId, T::DataObjectTypeId>>,
    ) -> DispatchResult;

    fn atomically_remove_content(
        owner: &StorageObjectOwner<T::MemberId, T::ChannelId, T::DAOId>,
        content_ids: &[T::ContentId],
    ) -> DispatchResult;

    // Checks if given owner can remove content under given content ids from the storage frame_system
    fn can_remove_content(
        owner: &StorageObjectOwner<T::MemberId, T::ChannelId, T::DAOId>,
        content_ids: &[T::ContentId],
    ) -> DispatchResult;
}

