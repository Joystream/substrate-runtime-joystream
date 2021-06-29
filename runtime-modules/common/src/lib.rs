// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod constraints;
pub mod currency;
pub mod origin;
pub mod storage;
pub mod working_group;

use codec::{Codec, Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::Parameter;
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::{MaybeSerialize, Member};
use sp_std::vec::Vec;

/// Member id type alias
pub type MemberId<T> = <T as MembershipTypes>::MemberId;

/// Actor id type alias
pub type ActorId<T> = <T as MembershipTypes>::ActorId;

/// HTTP Url string
pub type Url = Vec<u8>;

/// Generic trait for membership dependent pallets.
pub trait MembershipTypes: frame_system::Config {
    /// Describes the common type for the members.
    type MemberId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Ord
        + PartialEq
        + Eq;

    /// Describes the common type for the working group members (workers).
    type ActorId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Ord
        + PartialEq
        + Eq;
}

/// Generic trait for strorage ownership dependent pallets.
pub trait StorageOwnership {
    /// Channel id representation.
    type ChannelId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Ord
        + PartialEq;

    /// DAO id representation.
    type DAOId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Ord
        + PartialEq;

    /// Content id representation.
    type ContentId: Parameter + Member + Codec + Default + Copy + MaybeSerialize + Ord + PartialEq;

    /// Data object type id.
    type DataObjectTypeId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + Ord
        + PartialEq;
}

/// Defines time in both block number and substrate time abstraction.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug, Default)]
pub struct BlockAndTime<BlockNumber, Moment> {
    /// Defines chain block
    pub block: BlockNumber,

    /// Defines time
    pub time: Moment,
}

/// Gathers current block and time information for the runtime.
/// If this function is used inside a config() at genesis the timestamp will be 0
/// because the timestamp is actually produced by validators.
pub fn current_block_time<T: frame_system::Config + pallet_timestamp::Config>(
) -> BlockAndTime<T::BlockNumber, T::Moment> {
    BlockAndTime {
        block: <frame_system::Pallet<T>>::block_number(),
        time: <pallet_timestamp::Pallet<T>>::now(),
    }
}
