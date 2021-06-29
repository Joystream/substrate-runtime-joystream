// Clippy linter warning. TODO: remove after the Constaninople release
#![allow(clippy::type_complexity)]
// disable it because of possible frontend API break

// Clippy linter warning. TODO: refactor the Option<Option<>>
#![allow(clippy::option_option)] // disable it because of possible API break
#![allow(clippy::result_unit_err)]
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::storage::IterableStorageMap;
use frame_support::{decl_module, decl_storage, ensure, Parameter};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{MaybeSerialize, Member};

use minting::BalanceOf;

mod mock;
mod tests;

pub trait Config: frame_system::Config + minting::Config {
    type PayoutStatusHandler: PayoutStatusHandler<Self>;

    /// Type of identifier for recipients.
    type RecipientId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;

    /// Type for identifier for relationship representing that a recipient recieves recurring reward from a token mint
    type RewardRelationshipId: Parameter
        + Member
        + BaseArithmetic
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq;
}

/// Handler for aftermath of a payout attempt
pub trait PayoutStatusHandler<T: Config> {
    fn payout_succeeded(
        id: T::RewardRelationshipId,
        destination_account: &T::AccountId,
        amount: BalanceOf<T>,
    );

    fn payout_failed(
        id: T::RewardRelationshipId,
        destination_account: &T::AccountId,
        amount: BalanceOf<T>,
    );
}

/// Makes `()` empty tuple, a PayoutStatusHandler that does nothing.
impl<T: Config> PayoutStatusHandler<T> for () {
    fn payout_succeeded(
        _id: T::RewardRelationshipId,
        _destination_account: &T::AccountId,
        _amount: BalanceOf<T>,
    ) {
    }

    fn payout_failed(
        _id: T::RewardRelationshipId,
        _destination_account: &T::AccountId,
        _amount: BalanceOf<T>,
    ) {
    }
}

/// A recipient of recurring rewards
#[derive(Encode, Decode, Copy, Clone, Debug, Default)]
pub struct Recipient<Balance> {
    // stats
    /// Total payout received by this recipient
    total_reward_received: Balance,

    /// Total payout missed for this recipient
    total_reward_missed: Balance,
}

#[derive(Encode, Decode, Copy, Clone, Debug, Default)]
pub struct RewardRelationship<AccountId, Balance, BlockNumber, MintId, RecipientId> {
    /// Identifier for receiver
    recipient: RecipientId,

    /// Identifier for reward source
    mint_id: MintId,

    /// Destination account for reward
    pub account: AccountId,

    /// The payout amount at the next payout
    pub amount_per_payout: Balance,

    /// When set, identifies block when next payout should be processed,
    /// otherwise there is no pending payout
    next_payment_at_block: Option<BlockNumber>,

    /// When set, will be the basis for automatically setting next payment,
    /// otherwise any upcoming payout will be a one off.
    payout_interval: Option<BlockNumber>,

    // stats
    /// Total payout received in this relationship
    total_reward_received: Balance,

    /// Total payout failed in this relationship
    total_reward_missed: Balance,
}

impl<AccountId: Clone, Balance: Clone, BlockNumber: Clone, MintId: Clone, RecipientId: Clone>
    RewardRelationship<AccountId, Balance, BlockNumber, MintId, RecipientId>
{
    /// Verifies whether relationship is active
    pub fn is_active(&self) -> bool {
        self.next_payment_at_block.is_some()
    }

    /// Make clone which is activated.
    pub fn clone_activated(&self, start_at: &BlockNumber) -> Self {
        Self {
            next_payment_at_block: Some((*start_at).clone()),
            ..((*self).clone())
        }
    }

    /// Make clone which is deactivated
    pub fn clone_deactivated(&self) -> Self {
        Self {
            next_payment_at_block: None,
            ..((*self).clone())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Config> as RecurringReward {
        Recipients get(fn recipients): map hasher(blake2_128_concat)
            T::RecipientId => Recipient<BalanceOf<T>>;

        RecipientsCreated get(fn recipients_created): T::RecipientId;

        pub RewardRelationships get(fn reward_relationships): map hasher(blake2_128_concat)
            T::RewardRelationshipId => RewardRelationship<T::AccountId, BalanceOf<T>, T::BlockNumber, T::MintId, T::RecipientId>;

        RewardRelationshipsCreated get(fn reward_relationships_created): T::RewardRelationshipId;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        fn on_finalize(now: T::BlockNumber) {
            Self::do_payouts(now);
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum RewardsError {
    RecipientNotFound,
    RewardSourceNotFound,
    NextPaymentNotInFuture,
    RewardRelationshipNotFound,
}

impl<T: Config> Module<T> {
    /// Adds a new Recipient and returns new recipient identifier.
    pub fn add_recipient() -> T::RecipientId {
        let next_id = Self::recipients_created();
        <RecipientsCreated<T>>::put(next_id + One::one());
        <Recipients<T>>::insert(&next_id, Recipient::default());
        next_id
    }

    /// Adds a new RewardRelationship, for a given source mint, recipient, account.
    pub fn add_reward_relationship(
        mint_id: T::MintId,
        recipient: T::RecipientId,
        account: T::AccountId,
        amount_per_payout: BalanceOf<T>,
        next_payment_at_block: T::BlockNumber,
        payout_interval: Option<T::BlockNumber>,
    ) -> Result<T::RewardRelationshipId, RewardsError> {
        ensure!(
            <minting::Module<T>>::mint_exists(mint_id),
            RewardsError::RewardSourceNotFound
        );
        ensure!(
            <Recipients<T>>::contains_key(recipient),
            RewardsError::RecipientNotFound
        );
        ensure!(
            next_payment_at_block > <frame_system::Pallet<T>>::block_number(),
            RewardsError::NextPaymentNotInFuture
        );

        let relationship_id = Self::reward_relationships_created();
        <RewardRelationshipsCreated<T>>::put(relationship_id + One::one());
        <RewardRelationships<T>>::insert(
            relationship_id,
            RewardRelationship {
                mint_id,
                recipient,
                account,
                amount_per_payout,
                next_payment_at_block: Some(next_payment_at_block),
                payout_interval,
                total_reward_received: Zero::zero(),
                total_reward_missed: Zero::zero(),
            },
        );
        Ok(relationship_id)
    }

    /// Removes a relationship from RewardRelashionships and its recipient.
    pub fn remove_reward_relationship(id: T::RewardRelationshipId) {
        if <RewardRelationships<T>>::contains_key(&id) {
            <Recipients<T>>::remove(<RewardRelationships<T>>::take(&id).recipient);
        }
    }

    /// Will attempt to activat a deactivated reward relationship.
    pub fn try_to_activate_relationship(
        id: T::RewardRelationshipId,
        next_payment_at_block: T::BlockNumber,
    ) -> Result<bool, ()> {
        // Ensure relationship exists
        let reward_relationship = Self::ensure_reward_relationship_exists(&id)?;

        let activated = if reward_relationship.is_active() {
            // Was not activated
            false
        } else {
            // Update as activated
            let activated_relationship =
                reward_relationship.clone_activated(&next_payment_at_block);

            RewardRelationships::<T>::insert(id, activated_relationship);

            // We activated
            true
        };

        Ok(activated)
    }

    /// Will attempt to deactivat a activated reward relationship.
    pub fn try_to_deactivate_relationship(id: T::RewardRelationshipId) -> Result<bool, ()> {
        // Ensure relationship exists
        let reward_relationship = Self::ensure_reward_relationship_exists(&id)?;

        let deactivated = if reward_relationship.is_active() {
            let deactivated_relationship = reward_relationship.clone_deactivated();

            RewardRelationships::<T>::insert(id, deactivated_relationship);

            // Was deactivated
            true
        } else {
            // Was not deactivated
            false
        };

        Ok(deactivated)
    }

    // For reward relationship found with given identifier, new values can be set for
    // account, payout, block number when next payout will be made and the new interval after
    // the next scheduled payout. All values are optional, but updating values are combined in this
    // single method to ensure atomic updates.
    pub fn set_reward_relationship(
        id: T::RewardRelationshipId,
        new_account: Option<T::AccountId>,
        new_payout: Option<BalanceOf<T>>,
        new_next_payment_at: Option<Option<T::BlockNumber>>,
        new_payout_interval: Option<Option<T::BlockNumber>>,
    ) -> Result<(), RewardsError> {
        ensure!(
            <RewardRelationships<T>>::contains_key(&id),
            RewardsError::RewardRelationshipNotFound
        );

        let mut relationship = Self::reward_relationships(&id);

        if let Some(account) = new_account {
            relationship.account = account;
        }
        if let Some(payout) = new_payout {
            relationship.amount_per_payout = payout;
        }
        if let Some(next_payout_at_block) = new_next_payment_at {
            if let Some(blocknumber) = next_payout_at_block {
                ensure!(
                    blocknumber > <frame_system::Pallet<T>>::block_number(),
                    RewardsError::NextPaymentNotInFuture
                );
            }
            relationship.next_payment_at_block = next_payout_at_block;
        }
        if let Some(payout_interval) = new_payout_interval {
            relationship.payout_interval = payout_interval;
        }

        <RewardRelationships<T>>::insert(&id, relationship);
        Ok(())
    }

    /*
    For all relationships where next_payment_at_block is set and matches current block height,
    a call to pay_reward is made for the suitable amount, recipient and source.
    The next_payment_in_block is updated based on payout_interval.
    If the call succeeds, total_reward_received is incremented on both
    recipient and dependency with amount_per_payout, and a call to T::PayoutStatusHandler is made.
    Otherwise, analogous steps for failure.
    */
    fn do_payouts(now: T::BlockNumber) {
        for (relationship_id, ref mut relationship) in <RewardRelationships<T>>::iter() {
            assert!(<Recipients<T>>::contains_key(&relationship.recipient));

            let mut recipient = Self::recipients(relationship.recipient);

            if let Some(next_payment_at_block) = relationship.next_payment_at_block {
                if next_payment_at_block != now {
                    continue;
                }

                // Add the missed payout and try to pay those in addition to scheduled payout?
                // let payout = relationship.total_reward_missed + relationship.amount_per_payout;
                let payout = relationship.amount_per_payout;

                // try to make payment
                if <minting::Module<T>>::transfer_tokens(
                    relationship.mint_id,
                    payout,
                    &relationship.account,
                )
                .is_err()
                {
                    // add only newly scheduled payout to total missed payout
                    relationship.total_reward_missed += relationship.amount_per_payout;

                    // update recipient stats
                    recipient.total_reward_missed += relationship.amount_per_payout;

                    T::PayoutStatusHandler::payout_failed(
                        relationship_id,
                        &relationship.account,
                        payout,
                    );
                } else {
                    // update payout received stats
                    relationship.total_reward_received += payout;
                    recipient.total_reward_received += payout;

                    // update missed payout stats
                    // if relationship.total_reward_missed != Zero::zero() {
                    //     // update recipient stats
                    //     recipient.total_reward_missed -= relationship.total_reward_missed;

                    //     // clear missed reward on relationship
                    //     relationship.total_reward_missed = Zero::zero();
                    // }
                    T::PayoutStatusHandler::payout_succeeded(
                        relationship_id,
                        &relationship.account,
                        payout,
                    );
                }

                // update next payout blocknumber at interval if set
                if let Some(payout_interval) = relationship.payout_interval {
                    relationship.next_payment_at_block = Some(now + payout_interval);
                } else {
                    relationship.next_payment_at_block = None;
                }

                <Recipients<T>>::insert(relationship.recipient, recipient);
                <RewardRelationships<T>>::insert(relationship_id, relationship);
            }
        }
    }
}

impl<T: Config> Module<T> {
    fn ensure_reward_relationship_exists(
        id: &T::RewardRelationshipId,
    ) -> Result<
        RewardRelationship<T::AccountId, BalanceOf<T>, T::BlockNumber, T::MintId, T::RecipientId>,
        (),
    > {
        ensure!(RewardRelationships::<T>::contains_key(id), ());

        let relationship = RewardRelationships::<T>::get(id);

        Ok(relationship)
    }
}
