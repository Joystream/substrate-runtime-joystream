//! Low-level types used throughout the Substrate code.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

/// Alias for ContentId, used in various places.
pub type ContentId = sp_core::H256;

/// Alias for DAOId, used in various places.
pub type DAOId = u64;

/// Alias for DataObjectTypeId, used in various places.
pub type DataObjectTypeId = u64;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Moment type
pub type Moment = u64;

/// Content Directory Channel identifier.
pub type ChannelId = u64;

/// Content Directory Channel Category identifier.
pub type ChannelCategoryId = u64;

/// Content Directory Video identifier.
pub type VideoId = u64;

/// Content Directory Video Category identifier.
pub type VideoCategoryId = u64;

/// Content Directory Playlist identifier.
pub type PlaylistId = u64;

/// Content Directory Person identifier.
pub type PersonId = u64;

/// Content Directory Series identifier.
pub type SeriesId = u64;

/// Content Directory Channel transfer request identifier.
pub type ChannelOwnershipTransferRequestId = u64;

/// Represents a thread identifier for both Forum and Proposals Discussion
///
/// Note: Both modules expose type names ThreadId and PostId (which are defined on their Config) and
/// used in state storage and dispatchable method's argument types,
/// and are therefore part of the public API/metadata of the runtime.
/// In the current version the polkadot-js/api that is used and is compatible with the runtime,
/// the type registry has flat namespace and its not possible
/// to register identically named types from different modules, separately. And so we MUST configure
/// the underlying types to be identicaly to avoid issues with encoding/decoding these types on the client side.
pub type ThreadId = u64;

/// Represents a post identifier for both Forum and Proposals Discussion
///
/// See the Note about ThreadId
pub type PostId = u64;

/// Represent an actor in membership group, which is the same in the working groups.
pub type ActorId = u64;

/// Represent an member in membership group, which is the same in the working groups.
pub type MemberId = u64;

/// App-specific crypto used for reporting equivocation/misbehavior in BABE and
/// GRANDPA. Any rewards for misbehavior reporting will be paid out to this
/// account.
pub mod report {
    use super::{Signature, Verify};
    use frame_system::offchain::AppCrypto;
    use sp_core::crypto::{key_types, KeyTypeId};

    /// Key type for the reporting module. Used for reporting BABE and GRANDPA
    /// equivocations.
    pub const KEY_TYPE: KeyTypeId = key_types::REPORTING;

    mod app {
        use sp_application_crypto::{app_crypto, sr25519};
        app_crypto!(sr25519, super::KEY_TYPE);
    }

    /// Identity of the equivocation/misbehavior reporter.
    pub type ReporterId = app::Public;

    /// An `AppCrypto` type to allow submitting signed transactions using the reporting
    /// application key as signer.
    pub struct ReporterAppCrypto;

    impl AppCrypto<<Signature as Verify>::Signer, Signature> for ReporterAppCrypto {
        type RuntimeAppPublic = ReporterId;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}
