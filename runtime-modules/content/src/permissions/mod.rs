mod curator_group;

pub use curator_group::*;

pub use crate::errors::*;
use crate::*;
pub use codec::{Codec, Decode, Encode};
pub use common::MembershipTypes;
use core::fmt::Debug;
use frame_support::{ensure, Parameter};
#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::{MaybeSerializeDeserialize, Member};
// use frame_system::ensure_root;

/// Model of authentication manager.
pub trait ContentActorAuthenticator: frame_system::Config + MembershipTypes {
    /// Curator identifier
    type CuratorId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    /// Curator group identifier
    type CuratorGroupId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    /// Authorize actor as lead
    fn is_lead(account_id: &Self::AccountId) -> bool;

    /// Checks if Id represents a worker id in the working group
    fn is_valid_curator_id(curator_id: &Self::CuratorId) -> bool;

    /// Authorize actor as curator
    fn is_curator(curator_id: &Self::CuratorId, account_id: &Self::AccountId) -> bool;

    /// Authorize actor as member
    fn is_member(member_id: &Self::MemberId, account_id: &Self::AccountId) -> bool;
}

pub fn ensure_is_valid_curator_id<T: Config>(curator_id: &T::CuratorId) -> DispatchResult {
    ensure!(
        T::is_valid_curator_id(curator_id),
        Error::<T>::CuratorIdInvalid
    );
    Ok(())
}

/// Ensure curator authorization performed succesfully
pub fn ensure_curator_auth_success<T: Config>(
    curator_id: &T::CuratorId,
    account_id: &T::AccountId,
) -> DispatchResult {
    ensure!(
        T::is_curator(curator_id, account_id),
        Error::<T>::CuratorAuthFailed
    );
    Ok(())
}

/// Ensure member authorization performed succesfully
pub fn ensure_member_auth_success<T: Config>(
    member_id: &T::MemberId,
    account_id: &T::AccountId,
) -> DispatchResult {
    ensure!(
        T::is_member(member_id, account_id),
        Error::<T>::MemberAuthFailed
    );
    Ok(())
}

/// Ensure lead authorization performed succesfully
pub fn ensure_lead_auth_success<T: Config>(account_id: &T::AccountId) -> DispatchResult {
    ensure!(T::is_lead(account_id), Error::<T>::LeadAuthFailed);
    Ok(())
}

/// Ensure given `Origin` is lead
pub fn ensure_is_lead<T: Config>(origin: T::Origin) -> DispatchResult {
    let account_id = ensure_signed(origin)?;
    ensure_lead_auth_success::<T>(&account_id)
}

pub fn ensure_actor_authorized_to_create_channel<T: Config>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    match actor {
        // Lead should use their member or curator role to create or update channel assets.
        ContentActor::Lead => {
            Err(Error::<T>::ActorCannotOwnChannel.into())
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            let sender = ensure_signed(origin)?;

            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )
        }
        ContentActor::Member(member_id) => {
            let sender = ensure_signed(origin)?;

            ensure_member_auth_success::<T>(member_id, &sender)
        }
        // TODO:
        // ContentActor::Dao(_daoId) => ...,
    }
}

// Enure actor can update channels and videos in the channel
pub fn ensure_actor_authorized_to_update_channel<T: Config>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
) -> DispatchResult {
    // Only owner of a channel can update and delete channel assets.
    // Lead can update and delete curator group owned channel assets.
    match actor {
        ContentActor::Lead => {
            let sender = ensure_signed(origin)?;
            ensure_lead_auth_success::<T>(&sender)?;
            if let ChannelOwner::CuratorGroup(_) = owner {
                Ok(())
            } else {
                Err(Error::<T>::ActorNotAuthorized.into())
            }
        }
        ContentActor::Curator(curator_group_id, curator_id) => {
            let sender = ensure_signed(origin)?;

            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )?;

            // Ensure curator group is the channel owner.
            ensure!(
                *owner == ChannelOwner::CuratorGroup(*curator_group_id),
                Error::<T>::ActorNotAuthorized
            );

            Ok(())
        }
        ContentActor::Member(member_id) => {
            let sender = ensure_signed(origin)?;

            ensure_member_auth_success::<T>(member_id, &sender)?;

            // Ensure the member is the channel owner.
            ensure!(
                *owner == ChannelOwner::Member(*member_id),
                Error::<T>::ActorNotAuthorized
            );

            Ok(())
        }
        // TODO:
        // ContentActor::Dao(_daoId) => ...,
    }
}

// Enure actor can update or delete channels and videos
pub fn ensure_actor_authorized_to_set_featured_videos<T: Config>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    // Only Lead authorized to set featured videos
    if let ContentActor::Lead = actor {
        let sender = ensure_signed(origin)?;
        ensure_lead_auth_success::<T>(&sender)
    } else {
        Err(Error::<T>::ActorNotAuthorized.into())
    }
}

pub fn ensure_actor_authorized_to_censor<T: Config>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
    owner: &ChannelOwner<T::MemberId, T::CuratorGroupId, T::DAOId>,
) -> DispatchResult {
    // Only lead and curators can censor channels and videos
    // Only lead can censor curator group owned channels and videos
    match actor {
        ContentActor::Lead => {
            let sender = ensure_signed(origin)?;
            ensure_lead_auth_success::<T>(&sender)
        },
        ContentActor::Curator(curator_group_id, curator_id) => {
            let sender = ensure_signed(origin)?;

            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )?;

            // Curators cannot censor curator group channels
            if let ChannelOwner::CuratorGroup(_) = owner {
                Err(Error::<T>::CannotCensoreCuratorGroupOwnedChannels.into())
            } else {
                Ok(())
            }
        },
        ContentActor::Member(_) => {
            // Members cannot censore channels!
            Err(Error::<T>::ActorNotAuthorized.into())
        }
        // TODO:
        // ContentActor::Dao(_daoId) => ...,
    }
}

pub fn ensure_actor_authorized_to_manage_categories<T: Config>(
    origin: T::Origin,
    actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
) -> DispatchResult {
    // Only lead and curators can manage categories
    match actor {
        ContentActor::Lead => {
            let sender = ensure_signed(origin)?;
            ensure_lead_auth_success::<T>(&sender)
        },
        ContentActor::Curator(curator_group_id, curator_id) => {
            let sender = ensure_signed(origin)?;

            // Authorize curator, performing all checks to ensure curator can act
            CuratorGroup::<T>::perform_curator_in_group_auth(
                curator_id,
                curator_group_id,
                &sender,
            )
        },
        ContentActor::Member(_) => {
            // Members cannot censore channels!
            Err(Error::<T>::ActorNotAuthorized.into())
        }
        // TODO:
        // ContentActor::Dao(_daoId) => ...,
    }
}

// pub fn ensure_actor_authorized_to_delete_stale_assets<T: Config>(
//     origin: T::Origin,
//     actor: &ContentActor<T::CuratorGroupId, T::CuratorId, T::MemberId>,
// ) -> DispatchResult {
//     // Only Lead and (sudo) can delete assets no longer associated with a channel or person.
//     if let ContentActor::Lead = actor {
//         let sender = ensure_signed(origin)?;
//         ensure_lead_auth_success::<T>(&sender)
//     } else {
//         ensure_root(origin)?;
//         Ok(())
//     }
// }

/// Enum, representing all possible `Actor`s
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ContentActor<
    CuratorGroupId: Default + Clone + Copy,
    CuratorId: Default + Clone + Copy,
    MemberId: Default + Clone + Copy,
> {
    Curator(CuratorGroupId, CuratorId),
    Member(MemberId),
    Lead,
    // Dao,
}

impl<
        CuratorGroupId: Default + Clone + Copy,
        CuratorId: Default + Clone + Copy,
        MemberId: Default + Clone + Copy,
    > Default for ContentActor<CuratorGroupId, CuratorId, MemberId>
{
    fn default() -> Self {
        Self::Lead
    }
}
