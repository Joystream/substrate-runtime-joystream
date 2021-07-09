// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

mod errors;
mod functions;
mod types;
pub use common::{origin::ActorOriginValidator, MembershipTypes};
pub use content::{ContentActor, ContentActorAuthenticator};
use errors::*;
use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::{DispatchError, DispatchResult},
    ensure, Parameter,
};
pub use frame_system::ensure_signed;
pub use functions::*;
#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::{BaseArithmetic, One, Zero};
use sp_runtime::traits::{MaybeSerializeDeserialize, Member};
use types::*;

use codec::Codec;
pub use codec::{Decode, Encode};
use frame_support::traits::{Currency, ReservableCurrency};
pub use sp_runtime::Perbill;

pub trait NumericIdentifier:
    Parameter
    + Member
    + BaseArithmetic
    + Codec
    + Default
    + Copy
    + Clone
    + MaybeSerializeDeserialize
    + Eq
    + PartialEq
    + Ord
    + Zero
{
}

impl NumericIdentifier for u64 {}

type VideoId<T> = <T as content::Trait>::VideoId;

pub trait Trait:
    timestamp::Trait + content::Trait + content::ContentActorAuthenticator + MembershipTypes
{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    // NFT provider pallet
    type VNFTId: NumericIdentifier;

    // Payment provider
    type NftCurrencyProvider: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    // Member origin validator
    type MemberOriginValidator: ActorOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Content {

        /// Map, representing  VideoId -> VNFTId relation
        pub VNFTIdByVideo get(fn vnft_id_by_video_id): map hasher(blake2_128_concat) VideoId<T> => T::VNFTId;

        /// Map, representing  VNFTId -> VNFT relation
        pub VNFTById get(fn vnft_by_vnft_id): map hasher(blake2_128_concat) T::VNFTId => VNFT<T::AccountId>;

        /// Map, representing AuctionId -> Auction relation
        pub AuctionById get (fn auction_by_id): map hasher(blake2_128_concat) AuctionId<VideoId<T>, T::VNFTId> => Auction<T>;

        /// Representation for pending vNFT transfers, vnft_id => receiver
        pub PendingTransfers get (fn pending_transfers): double_map
            hasher(blake2_128_concat) T::VNFTId,
            hasher(blake2_128_concat) MemberId<T> => ();

        /// Next vNFT id
        pub NextVNFTId get(fn next_video_nft_id) config(): T::VNFTId;

        /// Min auction round time
        pub MinRoundTime get(fn min_round_time) config(): T::Moment;

        /// Max auction round time
        pub MaxRoundTime get(fn max_round_time) config(): T::Moment;

        /// Min auction staring price
        pub MinStartingPrice get(fn min_starting_price) config(): BalanceOf<T>;

        /// Max auction staring price
        pub MaxStartingPrice get(fn max_starting_price) config(): BalanceOf<T>;

        /// Min creator royalty
        pub MinCreatorRoyalty get(fn min_creator_royalty) config(): Perbill;

        /// Max creator royalty
        pub MaxCreatorRoyalty get(fn max_creator_royalty) config(): Perbill;

        /// Min auction bid step
        pub MinBidStep get(fn min_bid_step) config(): BalanceOf<T>;

        /// Max auction bid step
        pub MaxBidStep get(fn max_bid_step) config(): BalanceOf<T>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Initializing events
        fn deposit_event() = default;

        /// Start video auction
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn start_video_auction(
            origin,
            auctioneer: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            auction_params: AuctionParams<T::VNFTId, VideoId<T>, <T as timestamp::Trait>::Moment, BalanceOf<T>>,
        ) {

            let auctioneer_account_id = Self::authorize_auctioneer(origin, &auctioneer, &auction_params)?;

            // Validate round_time & starting_price
            Self::validate_auction_params(&auction_params)?;

            let auction_id = auction_params.auction_mode.get_auction_id();

            // Try complete auction
            if Self::is_auction_exist(auction_id) {

                let auction = Self::auction_by_id(auction_id);

                // Try finalize already completed auction (issues new nft if required)
                ensure!(Self::try_complete_auction(&auction), Error::<T>::AuctionAlreadyStarted);
                return Ok(())
            }

            //
            // == MUTATION SAFE ==
            //

            // Create new auction
            let auction = AuctionRecord::new(auctioneer, auctioneer_account_id, auction_params.clone());

            <AuctionById<T>>::insert(auction_id, auction);

            // Trigger event
            Self::deposit_event(RawEvent::AuctionStarted(auctioneer, auction_params));
        }

        /// Cancel video auction
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_auction(
            origin,
            auctioneer: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            auction_id: AuctionId<VideoId<T>, T::VNFTId>,
        ) {

            Self::authorize_content_actor(origin, &auctioneer)?;

            // Ensure auction for given video id exists
            let auction = Self::ensure_auction_exists(auction_id)?;

            // Ensure given auction has no participants
            auction.ensure_is_not_active::<T>()?;

            // Ensure given conntent actor is auctioneer
            auction.ensure_is_auctioneer::<T>(&auctioneer)?;

            //
            // == MUTATION SAFE ==
            //

            // Try complete previous auction
            if Self::try_complete_auction(&auction) {
                return Ok(())
            }

            <AuctionById<T>>::remove(auction_id);

            // Trigger event
            Self::deposit_event(RawEvent::AuctionCancelled(auctioneer, auction_id));
        }

        /// Make auction bid
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn make_bid(
            origin,
            participant: MemberId<T>,
            auction_id: AuctionId<VideoId<T>, T::VNFTId>,
            bid: BalanceOf<T>,
        ) {

            let auction_participant = Self::authorize_participant(origin, participant)?;

            // Ensure auction exists
            let auction = Self::ensure_auction_exists(auction_id)?;


            // Ensure bidder have sufficient balance amount to reserve for bid
            Self::ensure_has_sufficient_balance(&auction_participant, bid)?;

            // Ensure new bid is greater then last bid + minimal bid step
            auction.ensure_is_valid_bid::<T>(bid)?;

            //
            // == MUTATION SAFE ==
            //

            // Try complete previous auction
            if Self::try_complete_auction(&auction) {
                return Ok(())
            }

            let last_bid_time = timestamp::Module::<T>::now();

            // Unreserve previous bidder balance
            T::NftCurrencyProvider::unreserve(&auction.last_bidder, auction.last_bid);

            // Make auction bid & update auction data
            let auction = match *&auction.buy_now_price {
                // Instantly complete auction if bid is greater or equal then buy now price
                Some(buy_now_price) if bid >= buy_now_price => {
                    // Reseve balance for current bid
                    // Can not fail, needed check made
                    T::NftCurrencyProvider::reserve(&auction_participant, buy_now_price)?;

                    let auction = auction.make_bid::<T>(auction_participant, buy_now_price, last_bid_time);
                    Self::complete_auction(&auction);
                    return Ok(())
                }
                _ => {
                    // Reseve balance for current bid
                    // Can not fail, needed check made
                    T::NftCurrencyProvider::reserve(&auction_participant, bid)?;
                    auction.make_bid::<T>(auction_participant, bid, last_bid_time)
                }
            };

            <AuctionById<T>>::insert(auction_id, auction);

            // Trigger event
            Self::deposit_event(RawEvent::AuctionBidMade(participant, auction_id, bid));
        }

        /// Issue vNFT
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn issue(
            origin,
            auctioneer: ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
            video_id: T::VideoId,
            royalty: Option<Royalty>,
        ) {

            // Authorize content actor
            Self::authorize_content_actor(origin.clone(), &auctioneer)?;

            let content_actor_account_id = ensure_signed(origin)?;

            Self::ensure_vnft_does_not_exist(video_id)?;

            // Enure royalty bounds satisfied, if provided
            if let Some(royalty) = royalty {
                Self::ensure_royalty_bounds_satisfied(royalty)?;
            }

            //
            // == MUTATION SAFE ==
            //

            let auction_id = AuctionId::VideoId(video_id);

            // Try complete auction
            if Self::is_auction_exist(auction_id) {

                let auction = Self::auction_by_id(auction_id);

                // Try finalize already completed auction (issues new nft if required)
                Self::try_complete_auction(&auction);
                return Ok(())
            }

            // Issue vNFT

            let royalty = royalty.map(|royalty| (content_actor_account_id.clone(), royalty));

            Self::issue_vnft(content_actor_account_id, video_id, royalty);
        }

        /// Start vNFT transfer
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn start_transfer(
            origin,
            vnft_id: T::VNFTId,
            from: MemberId<T>,
            to: MemberId<T>,
        ) {

            // Authorize participant under given member id
            let from_account_id = Self::authorize_participant(origin, from)?;

            // Ensure given vnft exists
            let vnft = Self::ensure_vnft_exists(vnft_id)?;

            // Ensure from_account_id is vnft owner
            vnft.ensure_ownership::<T>(&from_account_id)?;

            // Ensure there is no auction for given vnft
            Self::ensure_auction_does_not_exist(AuctionId::VNFTId(vnft_id))?;

            // Ensure pending transfer isn`t started
            Self::ensure_pending_transfer_does_not_exist(vnft_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Add vNFT transfer data to pending transfers storage
            <PendingTransfers<T>>::insert(vnft_id, to, ());

            // Trigger event
            Self::deposit_event(RawEvent::TransferStarted(vnft_id, from, to));
        }

        /// Cancel vNFT transfer
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn cancel_transfer(
            origin,
            vnft_id: T::VNFTId,
            participant: MemberId<T>,
        ) {

            let participant_account_id = Self::authorize_participant(origin, participant)?;

            Self::ensure_pending_transfer_exists(vnft_id)?;

            let vnft = Self::ensure_vnft_exists(vnft_id)?;
            vnft.ensure_ownership::<T>(&participant_account_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove vNFT transfer data from pending transfers storage
            // Safe to call, because we always have one transfers per vnft_id
            <PendingTransfers<T>>::remove_prefix(vnft_id);

            // Trigger event
            Self::deposit_event(RawEvent::TransferCancelled(vnft_id, participant));
        }

        /// Accept incoming vNFT transfer
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn accept_incoming_vnft(
            origin,
            vnft_id: T::VNFTId,
            participant: MemberId<T>,
        ) {

            let participant_account_id = Self::authorize_participant(origin, participant)?;

            Self::ensure_pending_transfer_exists(vnft_id)?;

            // Ensure new pending transfer available to proceed
            Self::ensure_new_pending_transfer_available(vnft_id, participant)?;

            let mut vnft = Self::ensure_vnft_exists(vnft_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove vNFT transfer data from pending transfers storage
            // Safe to call, because we always have one transfers per vnft_id
            <PendingTransfers<T>>::remove_prefix(vnft_id);


            vnft.owner = participant_account_id;
            <VNFTById<T>>::insert(vnft_id, vnft);

            // Trigger event
            Self::deposit_event(RawEvent::TransferAccepted(vnft_id, participant));
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        VideoId = VideoId<T>,
        AuctionId = AuctionId<<T as content::Trait>::VideoId, <T as Trait>::VNFTId>,
        VNFTId = <T as Trait>::VNFTId,
        Member = MemberId<T>,
        Balance = BalanceOf<T>,
        ContentActor = ContentActor<CuratorGroupId<T>, CuratorId<T>, MemberId<T>>,
        AuctionParams = AuctionParams<
            <T as Trait>::VNFTId,
            VideoId<T>,
            <T as timestamp::Trait>::Moment,
            BalanceOf<T>,
        >,
        Auction = AuctionRecord<
            <T as frame_system::Trait>::AccountId,
            VideoId<T>,
            <T as Trait>::VNFTId,
            <T as timestamp::Trait>::Moment,
            CuratorGroupId<T>,
            CuratorId<T>,
            MemberId<T>,
            BalanceOf<T>,
        >,
        CreatorRoyalty = Option<(<T as frame_system::Trait>::AccountId, Royalty)>,
    {
        AuctionStarted(ContentActor, AuctionParams),
        NftIssued(VideoId, VNFTId, CreatorRoyalty),
        AuctionBidMade(Member, AuctionId, Balance),
        AuctionCancelled(ContentActor, AuctionId),
        AuctionCompleted(Auction),
        TransferStarted(VNFTId, Member, Member),
        TransferCancelled(VNFTId, Member),
        TransferAccepted(VNFTId, Member),
    }
);
