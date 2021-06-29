//! Council Elections Manager
//!
//! # Election Parameters:
//! We don't currently handle zero periods, zero council term, zero council size and candidacy
//! limit in any special way. The behaviour in such cases:
//!
//! - Setting any period to 0 will mean the election getting stuck in that stage, until force changing
//! the state.
//!
//! - Council Size of 0 - no limit to size of council, all applicants that move beyond
//! announcing stage would become council members, so effectively the candidacy limit will
//! be the size of the council, voting and revealing have no impact on final results.
//!
//! - If candidacy limit is zero and council size > 0, council_size number of applicants will reach the voting stage.
//! and become council members, voting will have no impact on final results.
//!
//! - If both candidacy limit and council size are zero then all applicant become council members
//! since no filtering occurs at end of announcing stage.
//!
//! We only guard against these edge cases in the [`set_election_parameters`] call.
//!
//! [`set_election_parameters`]: struct.Module.html#method.set_election_parameters

// Clippy linter warning
#![allow(clippy::type_complexity)]
// disable it because of possible frontend API break
// TODO: remove post-Constaninople

// Clippy linter warning
#![allow(clippy::redundant_closure_call)] // disable it because of the substrate lib design
#![allow(clippy::match_like_matches_macro)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{decl_event, decl_module, decl_storage, ensure};
use frame_system::{ensure_root, ensure_signed};
use sp_arithmetic::traits::Zero;
use sp_runtime::traits::Hash;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::ops::Add;
use sp_std::vec;
use sp_std::vec::Vec;

use super::sealed_vote::SealedVote;
use super::stake::Stake;

use super::council;
use crate::election_params::ElectionParameters;
pub use common::currency::{BalanceOf, GovernanceCurrency};

use crate::DispatchResult;

pub trait Config:
    frame_system::Config + council::Config + GovernanceCurrency + membership::Config
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type CouncilElected: CouncilElected<Seats<Self::AccountId, BalanceOf<Self>>, Self::BlockNumber>;
}

pub static MSG_CANNOT_CHANGE_PARAMS_DURING_ELECTION: &str = "CannotChangeParamsDuringElection";

#[derive(Clone, Copy, Encode, Decode)]
pub enum ElectionStage<BlockNumber> {
    Announcing(BlockNumber),
    Voting(BlockNumber),
    Revealing(BlockNumber),
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Seat<AccountId, Balance> {
    pub member: AccountId,
    pub stake: Balance,
    pub backers: Vec<Backer<AccountId, Balance>>,
}

impl<AccountId, Balance> Seat<AccountId, Balance>
where
    Balance: Add<Output = Balance> + Copy,
{
    pub fn calc_total_stake(&self) -> Balance {
        self.backers
            .iter()
            .fold(self.stake, |acc, backer| acc + backer.stake)
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Backer<AccountId, Balance> {
    pub member: AccountId,
    pub stake: Balance,
}

pub type Seats<AccountId, Balance> = Vec<Seat<AccountId, Balance>>;

// Hook for setting a new council when it is elected
pub trait CouncilElected<Elected, Term> {
    fn council_elected(new_council: Elected, term: Term);
}

impl<Elected, Term> CouncilElected<Elected, Term> for () {
    fn council_elected(_new_council: Elected, _term: Term) {}
}

impl<Elected, Term, X: CouncilElected<Elected, Term>> CouncilElected<Elected, Term> for (X,) {
    fn council_elected(new_council: Elected, term: Term) {
        X::council_elected(new_council, term);
    }
}
// Chain of handlers.
impl<
        Elected: Clone,
        Term: Clone,
        X: CouncilElected<Elected, Term>,
        Y: CouncilElected<Elected, Term>,
    > CouncilElected<Elected, Term> for (X, Y)
{
    fn council_elected(new_council: Elected, term: Term) {
        X::council_elected(new_council.clone(), term.clone());
        Y::council_elected(new_council, term);
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Clone, Copy, Encode, Decode, Default)]
pub struct TransferableStake<Balance> {
    seat: Balance,
    backing: Balance,
}

// can we use a type alias to overcome name clashes of public types with other modules?
pub type ElectionStake<T> = Stake<BalanceOf<T>>;

decl_storage! {
    trait Store for Module<T: Config> as CouncilElection {
        // Flag for wether to automatically start an election after a council term ends
        AutoStart get(fn auto_start) config() : bool = true;

        // Current stage if there is an election running
        Stage get(fn stage): Option<ElectionStage<T::BlockNumber>>;

        // The election round
        Round get(fn round): u32;

        ExistingStakeHolders get(fn existing_stake_holders): Vec<T::AccountId>;
        TransferableStakes get(fn transferable_stakes): map hasher(blake2_128_concat)
            T::AccountId => TransferableStake<BalanceOf<T>>;

        Applicants get(fn applicants): Vec<T::AccountId>;
        ApplicantStakes get(fn applicant_stakes): map hasher(blake2_128_concat)
            T::AccountId => ElectionStake<T>;

        Commitments get(fn commitments): Vec<T::Hash>;

        // TODO value type of this map looks scary, is there any way to simplify the notation?
        Votes get(fn votes): map hasher(blake2_128_concat)
            T::Hash => SealedVote<T::AccountId, ElectionStake<T>, T::Hash, T::AccountId>;

        // Current Election Parameters.
        // Should we replace all the individual values with a single ElectionParameters type?
        // Having them individually makes it more flexible to add and remove new parameters in future
        // without dealing with migration issues.
        AnnouncingPeriod get(fn announcing_period): T::BlockNumber;
        VotingPeriod get(fn voting_period): T::BlockNumber;
        RevealingPeriod get(fn revealing_period): T::BlockNumber;
        CouncilSize get(fn council_size): u32;
        CandidacyLimit get (fn candidacy_limit): u32;
        MinCouncilStake get(fn min_council_stake): BalanceOf<T>;
        NewTermDuration get(fn new_term_duration): T::BlockNumber;
        MinVotingStake get(fn min_voting_stake): BalanceOf<T>;
    }
    add_extra_genesis {
        config(election_parameters): ElectionParameters<BalanceOf<T>, T::BlockNumber>;
        build(|config: &GenesisConfig<T>| {
            config.election_parameters.ensure_valid().expect("Invalid Election Parameters");
            Module::<T>::set_verified_election_parameters(config.election_parameters);
        });
    }
}

// Event for this module.
decl_event!(
    pub enum Event<T> where
    <T as frame_system::Config>::BlockNumber,
    <T as frame_system::Config>::AccountId,
    <T as frame_system::Config>::Hash  {
        /// A new election started
        ElectionStarted(),
        AnnouncingStarted(u32),
        AnnouncingEnded(),
        VotingStarted(),
        VotingEnded(),
        RevealingStarted(),
        RevealingEnded(),
        CouncilElected(BlockNumber),
        Applied(AccountId),
        Voted(AccountId, Hash),
        Revealed(AccountId, Hash, AccountId),
    }
);

impl<T: Config> Module<T> {
    // HELPERS - IMMUTABLES

    fn council_size_usize() -> usize {
        Self::council_size() as usize
    }

    fn candidacy_limit_usize() -> usize {
        Self::candidacy_limit() as usize
    }

    fn current_block_number_plus(length: T::BlockNumber) -> T::BlockNumber {
        <frame_system::Pallet<T>>::block_number() + length
    }

    fn can_participate(sender: &T::AccountId) -> bool {
        !<T as GovernanceCurrency>::Currency::free_balance(sender).is_zero()
            && <membership::Module<T>>::is_member_account(sender)
    }

    // PUBLIC IMMUTABLES

    /// Returns true if an election is running
    pub fn is_election_running() -> bool {
        Self::stage().is_some()
    }

    /// Returns block number at which current stage will end if an election is running.
    pub fn stage_ends_at() -> Option<T::BlockNumber> {
        if let Some(stage) = Self::stage() {
            match stage {
                ElectionStage::Announcing(ends) => Some(ends),
                ElectionStage::Voting(ends) => Some(ends),
                ElectionStage::Revealing(ends) => Some(ends),
            }
        } else {
            None
        }
    }

    // PRIVATE MUTABLES

    /// Starts an election. Will fail if an election is already running
    /// Initializes transferable stakes. Assumes election parameters have already been set.
    fn start_election(current_council: Seats<T::AccountId, BalanceOf<T>>) -> DispatchResult {
        ensure!(!Self::is_election_running(), "election already in progress");
        ensure!(
            Self::existing_stake_holders().is_empty(),
            "stake holders must be empty"
        );
        ensure!(Self::applicants().is_empty(), "applicants must be empty");
        ensure!(Self::commitments().is_empty(), "commitments must be empty");

        // Take snapshot of seat and backing stakes of an existing council
        // Its important to note that the election frame_system takes ownership of these stakes, and is responsible
        // to return any unused stake to original owners at the end of the election.
        Self::initialize_transferable_stakes(current_council);

        Self::deposit_event(RawEvent::ElectionStarted());

        Self::move_to_announcing_stage();
        Ok(())
    }

    /// Sets announcing stage. Can be called from any stage and assumes all preparatory work
    /// for entering the stage has been performed.
    /// Bumps the election round.
    fn move_to_announcing_stage() {
        let next_round = Round::mutate(|n| {
            *n += 1;
            *n
        });

        let new_stage_ends_at = Self::current_block_number_plus(Self::announcing_period());

        <Stage<T>>::put(ElectionStage::Announcing(new_stage_ends_at));

        Self::deposit_event(RawEvent::AnnouncingStarted(next_round));
    }

    /// Sets announcing stage. Can be called from any stage and assumes all preparatory work
    /// for entering the stage has been performed.
    fn move_to_voting_stage() {
        let new_stage_ends_at = Self::current_block_number_plus(Self::voting_period());

        <Stage<T>>::put(ElectionStage::Voting(new_stage_ends_at));

        Self::deposit_event(RawEvent::VotingStarted());
    }

    /// Sets announcing stage. Can be called from any stage and assumes all preparatory work
    /// for entering the stage has been performed.
    fn move_to_revealing_stage() {
        let new_stage_ends_at = Self::current_block_number_plus(Self::revealing_period());

        <Stage<T>>::put(ElectionStage::Revealing(new_stage_ends_at));

        Self::deposit_event(RawEvent::RevealingStarted());
    }

    /// Sorts applicants by stake, and returns slice of applicants with least stake. Applicants not
    /// returned in the slice are the top `len` highest staked.
    fn find_least_staked_applicants(
        applicants: &mut Vec<T::AccountId>,
        len: usize,
    ) -> &[T::AccountId] {
        if len >= applicants.len() {
            &[]
        } else {
            #[allow(clippy::redundant_closure)] // disable incorrect Clippy linter warning
            applicants.sort_by_key(|applicant| Self::applicant_stakes(applicant));
            &applicants[0..applicants.len() - len]
        }
    }

    fn on_announcing_ended() {
        let mut applicants = Self::applicants();

        if applicants.len() < Self::council_size_usize() {
            // Not enough applicants announced candidacy
            Self::move_to_announcing_stage();
        } else {
            // upper limit on applicants that will move to voting stage
            let limit = sp_std::cmp::max(Self::council_size_usize(), Self::candidacy_limit_usize());
            let applicants_to_drop = Self::find_least_staked_applicants(&mut applicants, limit);

            Self::drop_applicants(applicants_to_drop);

            Self::move_to_voting_stage();
        }
    }

    fn on_voting_ended() {
        Self::move_to_revealing_stage();
    }

    fn on_revealing_ended() {
        // tally the revealed votes
        let mut votes = Vec::new();

        for commitment in Self::commitments().iter() {
            votes.push(Self::votes(commitment));
        }

        let mut new_council = Self::tally_votes(&votes);

        // Note here that applicants with zero votes dont appear in the tally.
        // Is an applicant with some votes but less total stake than another applicant with zero votes
        // more qualified to be on the council?
        // Consider implications - if a council can be formed purely by staking are we fine with that?

        for applicant in Self::applicants().iter() {
            if !new_council.contains_key(applicant) {
                new_council.insert(
                    applicant.clone(),
                    Seat {
                        member: applicant.clone(),
                        stake: Self::applicant_stakes(applicant).total(),
                        backers: Vec::new(),
                    },
                );
            }
        }

        match new_council.len() {
            ncl if ncl == Self::council_size_usize() => {
                // all applicants in the tally will form the new council
            }
            ncl if ncl > Self::council_size_usize() => {
                // we have more than enough applicants to form the new council.
                // select top staked
                Self::filter_top_staked(&mut new_council, Self::council_size_usize());
            }
            _ => {
                // Not enough applicants with votes to form a council.
                // This may happen if we didn't add applicants with zero votes to the tally,
                // or in future if we allow applicants to withdraw candidacy during voting or revealing stages.
                // or council size was increased during voting, revealing stages.
            }
        }

        // unless we want to add more filtering criteria to what is considered a successful election
        // other than just the minimum stake for candidacy, we have a new council!

        Self::teardown_election(
            &votes,
            &new_council,
            true, /* unlock transferable stakes */
        );

        let new_council = new_council.into_iter().map(|(_, seat)| seat).collect();
        T::CouncilElected::council_elected(new_council, Self::new_term_duration());

        Self::deposit_event(RawEvent::CouncilElected(
            <frame_system::Pallet<T>>::block_number(),
        ));
    }

    fn teardown_election(
        votes: &[SealedVote<T::AccountId, Stake<BalanceOf<T>>, T::Hash, T::AccountId>],
        new_council: &BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>>,
        unlock_ts: bool,
    ) {
        Self::refund_voting_stakes(&votes, &new_council);
        Self::clear_votes();

        Self::drop_unelected_applicants(&new_council);
        Self::clear_applicants();

        if unlock_ts {
            Self::unlock_transferable_stakes();
        }

        Self::clear_transferable_stakes();

        <Stage<T>>::kill();
    }

    fn unlock_transferable_stakes() {
        // move stakes back to account holder's free balance
        for stakeholder in Self::existing_stake_holders().iter() {
            let stake = Self::transferable_stakes(stakeholder);
            if !stake.seat.is_zero() || !stake.backing.is_zero() {
                <T as GovernanceCurrency>::Currency::unreserve(
                    stakeholder,
                    stake.seat + stake.backing,
                );
            }
        }
    }

    fn clear_transferable_stakes() {
        for stakeholder in Self::existing_stake_holders() {
            <TransferableStakes<T>>::remove(stakeholder);
        }

        <ExistingStakeHolders<T>>::kill();
    }

    fn clear_applicants() {
        for applicant in Self::applicants() {
            <ApplicantStakes<T>>::remove(applicant);
        }
        <Applicants<T>>::kill();
    }

    fn refund_applicant(applicant: &T::AccountId) {
        let stake = <ApplicantStakes<T>>::get(applicant);

        // return new stake to account's free balance
        if !stake.new.is_zero() {
            <T as GovernanceCurrency>::Currency::unreserve(applicant, stake.new);
        }

        // return unused transferable stake
        if !stake.transferred.is_zero() {
            <TransferableStakes<T>>::mutate(applicant, |transferable| {
                (*transferable).seat += stake.transferred
            });
        }
    }

    fn drop_applicants(drop: &[T::AccountId]) {
        let not_dropped: Vec<T::AccountId> = Self::applicants()
            .into_iter()
            .filter(|id| !drop.iter().any(|x| *x == *id))
            .collect();

        for applicant in drop {
            Self::refund_applicant(applicant);
            <ApplicantStakes<T>>::remove(applicant);
        }

        <Applicants<T>>::put(not_dropped);
    }

    fn drop_unelected_applicants(
        new_council: &BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>>,
    ) {
        let applicants_to_drop: Vec<T::AccountId> = Self::applicants()
            .into_iter()
            .filter(|applicant| !new_council.contains_key(&applicant))
            .collect();

        Self::drop_applicants(&applicants_to_drop[..]);
    }

    fn refund_voting_stakes(
        sealed_votes: &[SealedVote<T::AccountId, Stake<BalanceOf<T>>, T::Hash, T::AccountId>],
        new_council: &BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>>,
    ) {
        for sealed_vote in sealed_votes.iter() {
            // Do a refund if commitment was not revealed, or the vote was for applicant that did
            // not get elected to the council
            // TODO critical: shouldn't we slash the stake in such a case? This is the whole idea behid staking on something: people need to decide carefully and be responsible for their bahavior because they can loose their stake
            // See https://github.com/Joystream/substrate-node-joystream/issues/4
            let do_refund = match sealed_vote.get_vote() {
                Some(applicant) => !new_council.contains_key(&applicant),
                None => true,
            };

            if do_refund {
                // return new stake to account's free balance
                let SealedVote { voter, stake, .. } = sealed_vote;
                if !stake.new.is_zero() {
                    <T as GovernanceCurrency>::Currency::unreserve(voter, stake.new);
                }

                // return unused transferable stake
                if !stake.transferred.is_zero() {
                    <TransferableStakes<T>>::mutate(voter, |transferable| {
                        (*transferable).backing += stake.transferred
                    });
                }
            }
        }
    }

    fn clear_votes() {
        for commitment in Self::commitments() {
            <Votes<T>>::remove(commitment);
        }
        <Commitments<T>>::kill();
    }

    fn tally_votes(
        sealed_votes: &[SealedVote<T::AccountId, Stake<BalanceOf<T>>, T::Hash, T::AccountId>],
    ) -> BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>> {
        let mut tally: BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>> = BTreeMap::new();

        for sealed_vote in sealed_votes.iter() {
            if let Some(applicant) = sealed_vote.get_vote() {
                if !tally.contains_key(&applicant) {
                    // Add new seat
                    tally.insert(
                        applicant.clone(),
                        Seat {
                            member: applicant.clone(),
                            stake: Self::applicant_stakes(applicant).total(),
                            backers: vec![],
                        },
                    );
                }
                if let Some(seat) = tally.get_mut(&applicant) {
                    // Add backer to existing seat
                    seat.backers.push(Backer {
                        member: sealed_vote.voter.clone(),
                        stake: sealed_vote.stake.total(),
                    });
                }
            }
        }

        tally
    }

    fn filter_top_staked(
        tally: &mut BTreeMap<T::AccountId, Seat<T::AccountId, BalanceOf<T>>>,
        limit: usize,
    ) {
        if limit >= tally.len() {
            return;
        }

        // use ordering in the applicants vector (not ordering resulting from btreemap iteration)
        let mut seats: Vec<T::AccountId> = Self::applicants()
            .into_iter()
            .filter(|id| tally.contains_key(id))
            .collect();

        // ensure_eq!(seats.len(), tally.len());

        if limit >= seats.len() {
            // Tally is inconsistent with list of applicants!
            return;
        }

        // TODO: order by number of votes, then number of backers

        seats.sort_by_key(|applicant| {
            tally
                .get(&applicant)
                .map_or(Zero::zero(), |seat| seat.calc_total_stake())
        });

        // seats at bottom of list
        let filtered_out_seats = &seats[0..seats.len() - limit];

        for id in filtered_out_seats {
            tally.remove(id);
        }
    }

    /// Checks if the current election stage has ended and calls the stage ended handler
    fn check_if_stage_is_ending(now: T::BlockNumber) {
        if let Some(stage) = Self::stage() {
            match stage {
                ElectionStage::Announcing(ends) => {
                    if ends == now {
                        Self::deposit_event(RawEvent::AnnouncingEnded());
                        Self::on_announcing_ended();
                    }
                }
                ElectionStage::Voting(ends) => {
                    if ends == now {
                        Self::deposit_event(RawEvent::VotingEnded());
                        Self::on_voting_ended();
                    }
                }
                ElectionStage::Revealing(ends) => {
                    if ends == now {
                        Self::deposit_event(RawEvent::RevealingEnded());
                        Self::on_revealing_ended();
                    }
                }
            }
        }
    }

    /// Takes a snapshot of the stakes from the current council
    fn initialize_transferable_stakes(current_council: Seats<T::AccountId, BalanceOf<T>>) {
        let mut stakeholder_accounts: Vec<T::AccountId> = Vec::new();

        for seat in current_council.into_iter() {
            let Seat { member, stake, .. } = seat;

            if <TransferableStakes<T>>::contains_key(&member) {
                <TransferableStakes<T>>::mutate(&member, |transferbale_stake| {
                    *transferbale_stake = TransferableStake {
                        seat: transferbale_stake.seat + stake,
                        backing: transferbale_stake.backing,
                    }
                });
            } else {
                <TransferableStakes<T>>::insert(
                    &member,
                    TransferableStake {
                        seat: stake,
                        backing: BalanceOf::<T>::zero(),
                    },
                );

                stakeholder_accounts.push(member);
            }

            for backer in seat.backers.into_iter() {
                let Backer { member, stake, .. } = backer;

                if <TransferableStakes<T>>::contains_key(&member) {
                    <TransferableStakes<T>>::mutate(&member, |transferbale_stake| {
                        *transferbale_stake = TransferableStake {
                            seat: transferbale_stake.seat,
                            backing: transferbale_stake.backing + stake,
                        }
                    });
                } else {
                    <TransferableStakes<T>>::insert(
                        &member,
                        TransferableStake {
                            seat: BalanceOf::<T>::zero(),
                            backing: stake,
                        },
                    );

                    stakeholder_accounts.push(member);
                }
            }
        }

        <ExistingStakeHolders<T>>::put(stakeholder_accounts);
    }

    fn new_stake_reusing_transferable(
        transferable: &mut BalanceOf<T>,
        new_stake: BalanceOf<T>,
    ) -> Stake<BalanceOf<T>> {
        let transferred = if *transferable >= new_stake {
            new_stake
        } else {
            *transferable
        };

        *transferable -= transferred;

        Stake {
            new: new_stake - transferred,
            transferred,
        }
    }

    fn try_add_applicant(applicant: T::AccountId, stake: BalanceOf<T>) -> DispatchResult {
        let mut transferable_stake = <TransferableStakes<T>>::get(&applicant);

        let new_stake = Self::new_stake_reusing_transferable(&mut transferable_stake.seat, stake);

        ensure!(
            <T as GovernanceCurrency>::Currency::can_reserve(&applicant, new_stake.new),
            "not enough free balance to reserve"
        );

        ensure!(
            <T as GovernanceCurrency>::Currency::reserve(&applicant, new_stake.new).is_ok(),
            "failed to reserve applicant stake!"
        );

        let applicant_stake = <ApplicantStakes<T>>::get(&applicant);
        let total_stake = applicant_stake.add(&new_stake);

        if <TransferableStakes<T>>::contains_key(&applicant) {
            <TransferableStakes<T>>::insert(&applicant, transferable_stake);
        }

        if !<ApplicantStakes<T>>::contains_key(&applicant) {
            // insert element at the begining, this gives priority to early applicants
            // when ordering applicants by stake if stakes are equal
            <Applicants<T>>::mutate(|applicants| applicants.insert(0, applicant.clone()));
        }

        <ApplicantStakes<T>>::insert(applicant, total_stake);

        Ok(())
    }

    fn try_add_vote(
        voter: T::AccountId,
        stake: BalanceOf<T>,
        commitment: T::Hash,
    ) -> DispatchResult {
        ensure!(
            !<Votes<T>>::contains_key(commitment),
            "duplicate commitment"
        );

        let mut transferable_stake = <TransferableStakes<T>>::get(&voter);

        let vote_stake =
            Self::new_stake_reusing_transferable(&mut transferable_stake.backing, stake);

        ensure!(
            <T as GovernanceCurrency>::Currency::can_reserve(&voter, vote_stake.new),
            "not enough free balance to reserve"
        );

        ensure!(
            <T as GovernanceCurrency>::Currency::reserve(&voter, vote_stake.new).is_ok(),
            "failed to reserve voting stake!"
        );

        <Commitments<T>>::mutate(|commitments| commitments.push(commitment));

        <Votes<T>>::insert(
            commitment,
            SealedVote::new(voter.clone(), vote_stake, commitment),
        );

        if <TransferableStakes<T>>::contains_key(&voter) {
            <TransferableStakes<T>>::insert(&voter, transferable_stake);
        }

        Ok(())
    }

    fn try_reveal_vote(
        voter: T::AccountId,
        commitment: T::Hash,
        vote_for: T::AccountId,
        salt: Vec<u8>,
    ) -> DispatchResult {
        ensure!(
            <Votes<T>>::contains_key(&commitment),
            "commitment not found"
        );

        let mut sealed_vote = <Votes<T>>::get(&commitment);

        ensure!(sealed_vote.is_not_revealed(), "vote already revealed");
        // only voter can reveal their own votes
        ensure!(sealed_vote.is_owned_by(voter), "only voter can reveal vote");
        ensure!(
            <ApplicantStakes<T>>::contains_key(&vote_for),
            "vote for non-applicant not allowed"
        );

        let mut salt = salt;

        // Tries to unseal, if salt is invalid will return error
        sealed_vote.unseal(
            vote_for,
            &mut salt,
            <T as frame_system::Config>::Hashing::hash,
        )?;

        // Update the revealed vote
        <Votes<T>>::insert(commitment, sealed_vote);

        Ok(())
    }

    fn set_verified_election_parameters(params: ElectionParameters<BalanceOf<T>, T::BlockNumber>) {
        <AnnouncingPeriod<T>>::put(params.announcing_period);
        <VotingPeriod<T>>::put(params.voting_period);
        <RevealingPeriod<T>>::put(params.revealing_period);
        <MinCouncilStake<T>>::put(params.min_council_stake);
        <NewTermDuration<T>>::put(params.new_term_duration);
        CouncilSize::put(params.council_size);
        CandidacyLimit::put(params.candidacy_limit);
        <MinVotingStake<T>>::put(params.min_voting_stake);
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // No origin so this is a priviledged call
        fn on_finalize(now: T::BlockNumber) {
            Self::check_if_stage_is_ending(now);
        }

        // Member can apply during announcing stage only. On first call a minimum stake will need to be provided.
        // Member can make subsequent calls during announcing stage to increase their stake.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn apply(origin, stake: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;
            ensure!(Self::can_participate(&sender), "Only members can apply to be on council");

            let stage = Self::stage();
            ensure!(Self::stage().is_some(), "election not running");

            let is_announcing = match stage.unwrap() {
                ElectionStage::Announcing(_) => true,
                _ => false
            };
            ensure!(is_announcing, "election not in announcing stage");

            // minimum stake on first attempt to apply
            if !<ApplicantStakes<T>>::contains_key(&sender) {
                ensure!(stake >= Self::min_council_stake(), "minimum stake must be provided");
            }

            Self::try_add_applicant(sender.clone(), stake)?;

            Self::deposit_event(RawEvent::Applied(sender));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn vote(origin, commitment: T::Hash, stake: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;
            ensure!(Self::can_participate(&sender), "Only members can vote for an applicant");

            let stage = Self::stage();
            ensure!(Self::stage().is_some(), "election not running");

            let is_voting = match stage.unwrap() {
                ElectionStage::Voting(_) => true,
                _ => false
            };
            ensure!(is_voting, "election not in voting stage");

            ensure!(stake >= Self::min_voting_stake(), "voting stake too low");
            Self::try_add_vote(sender.clone(), stake, commitment)?;
            Self::deposit_event(RawEvent::Voted(sender, commitment));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn reveal(origin, commitment: T::Hash, vote: T::AccountId, salt: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(salt.len() <= 32, "salt too large"); // at most 256 bits salt

            let stage = Self::stage();
            ensure!(Self::stage().is_some(), "election not running");

            let is_revealing = match stage.unwrap() {
                ElectionStage::Revealing(_) => true,
                _ => false
            };
            ensure!(is_revealing, "election not in revealing stage");

            Self::try_reveal_vote(sender.clone(), commitment, vote.clone(), salt)?;
            Self::deposit_event(RawEvent::Revealed(sender, commitment, vote));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn set_stage_announcing(origin, ends_at: T::BlockNumber) {
            ensure_root(origin)?;
            ensure!(ends_at > <frame_system::Pallet<T>>::block_number(), "must end at future block number");
            <Stage<T>>::put(ElectionStage::Announcing(ends_at));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn set_stage_revealing(origin, ends_at: T::BlockNumber) {
            ensure_root(origin)?;
            ensure!(ends_at > <frame_system::Pallet<T>>::block_number(), "must end at future block number");
            <Stage<T>>::put(ElectionStage::Revealing(ends_at));
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn set_stage_voting(origin, ends_at: T::BlockNumber) {
            ensure_root(origin)?;
            ensure!(ends_at > <frame_system::Pallet<T>>::block_number(), "must end at future block number");
            <Stage<T>>::put(ElectionStage::Voting(ends_at));
        }

        /// Sets new election parameters. Some combination of parameters that are not desirable, so
        /// the parameters are checked for validity.
        /// The call will fail if an election is in progress. If a council is not being elected for some
        /// reaon after multiple rounds, force_stop_election() can be called to stop elections and followed by
        /// set_election_parameters().
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_election_parameters(origin, params: ElectionParameters<BalanceOf<T>, T::BlockNumber>) {
            ensure_root(origin)?;
            ensure!(!Self::is_election_running(), MSG_CANNOT_CHANGE_PARAMS_DURING_ELECTION);
            params.ensure_valid()?;
            Self::set_verified_election_parameters(params);
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn force_stop_election(origin) {
            ensure_root(origin)?;
            ensure!(Self::is_election_running(), "only running election can be stopped");

            let mut votes = Vec::new();
            for commitment in Self::commitments() {
                votes.push(Self::votes(commitment));
            }

            // no council gets elected
            let empty_council = BTreeMap::new();

            Self::teardown_election (
                &votes,
                &empty_council,
                false /* do not unlock transferable stakes */
            );
        }

        #[weight = 10_000_000] // TODO: adjust weight
        pub fn force_start_election(origin) {
            ensure_root(origin)?;
            Self::start_election(<council::Module<T>>::active_council())?;
        }

        #[weight = 10_000_000] // TODO: adjust weight
        fn set_auto_start (origin, flag: bool) {
            ensure_root(origin)?;
            AutoStart::put(flag);
        }
    }
}

impl<T: Config> council::CouncilTermEnded for Module<T> {
    fn council_term_ended() {
        if Self::auto_start() {
            let _ = Self::start_election(<council::Module<T>>::active_council());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use codec::Encode;
    use frame_support::traits::OnFinalize;
    use frame_support::{assert_err, assert_ok};
    use frame_system::RawOrigin;

    #[test]
    fn election_starts_when_council_term_ends() {
        initial_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert!(Council::is_term_ended());
            assert!(Election::stage().is_none());

            <Election as council::CouncilTermEnded>::council_term_ended();

            assert!(Election::stage().is_some());
        });
    }

    #[test]
    fn new_stake_reusing_transferable_works() {
        {
            let mut transferable = 0;
            let additional = 100;
            let new_stake = Election::new_stake_reusing_transferable(&mut transferable, additional);
            assert_eq!(new_stake.new, 100);
            assert_eq!(new_stake.transferred, 0);
        }

        {
            let mut transferable = 40;
            let additional = 60;
            let new_stake = Election::new_stake_reusing_transferable(&mut transferable, additional);
            assert_eq!(new_stake.new, 20);
            assert_eq!(new_stake.transferred, 40);
            assert_eq!(transferable, 0);
        }

        {
            let mut transferable = 1000;
            let additional = 100;
            let new_stake = Election::new_stake_reusing_transferable(&mut transferable, additional);
            assert_eq!(new_stake.new, 0);
            assert_eq!(new_stake.transferred, 100);
            assert_eq!(transferable, 900);
        }
    }

    #[test]
    fn check_default_params() {
        // TODO missing test implementation?
    }

    #[test]
    fn should_not_start_new_election_if_already_started() {
        initial_test_ext().execute_with(|| {
            assert_ok!(Election::start_election(vec![]));
            assert_err!(
                Election::start_election(vec![]),
                "election already in progress"
            );
        });
    }

    fn assert_announcing_period(expected_period: <Test as frame_system::Config>::BlockNumber) {
        assert!(
            Election::is_election_running(),
            "Election Stage was not set"
        );

        let election_stage = Election::stage().unwrap();

        match election_stage {
            election::ElectionStage::Announcing(period) => {
                assert_eq!(period, expected_period, "Election period not set correctly")
            }
            _ => assert!(false, "Election Stage was not correctly set to Announcing"),
        }
    }

    #[test]
    fn start_election_should_work() {
        initial_test_ext().execute_with(|| {
            System::set_block_number(1);
            <AnnouncingPeriod<Test>>::put(20);
            let prev_round = Election::round();

            assert_ok!(Election::start_election(vec![]));

            // election round is bumped
            assert_eq!(Election::round(), prev_round + 1);

            // we enter the announcing stage for a specified period
            assert_announcing_period(1 + Election::announcing_period());
        });
    }

    #[test]
    fn init_transferable_stake_should_work() {
        initial_test_ext().execute_with(|| {
            let existing_council = vec![
                Seat {
                    member: 1,
                    stake: 100,
                    backers: vec![
                        Backer {
                            member: 2,
                            stake: 50,
                        },
                        Backer {
                            member: 3,
                            stake: 40,
                        },
                        Backer {
                            member: 10,
                            stake: 10,
                        },
                    ],
                },
                Seat {
                    member: 2,
                    stake: 200,
                    backers: vec![
                        Backer {
                            member: 1,
                            stake: 10,
                        },
                        Backer {
                            member: 3,
                            stake: 60,
                        },
                        Backer {
                            member: 20,
                            stake: 20,
                        },
                    ],
                },
                Seat {
                    member: 3,
                    stake: 300,
                    backers: vec![
                        Backer {
                            member: 1,
                            stake: 20,
                        },
                        Backer {
                            member: 2,
                            stake: 40,
                        },
                    ],
                },
            ];

            Election::initialize_transferable_stakes(existing_council);
            let mut existing_stake_holders = Election::existing_stake_holders();
            existing_stake_holders.sort();
            assert_eq!(existing_stake_holders, vec![1, 2, 3, 10, 20]);

            assert_eq!(Election::transferable_stakes(&1).seat, 100);
            assert_eq!(Election::transferable_stakes(&1).backing, 30);

            assert_eq!(Election::transferable_stakes(&2).seat, 200);
            assert_eq!(Election::transferable_stakes(&2).backing, 90);

            assert_eq!(Election::transferable_stakes(&3).seat, 300);
            assert_eq!(Election::transferable_stakes(&3).backing, 100);

            assert_eq!(Election::transferable_stakes(&10).seat, 0);
            assert_eq!(Election::transferable_stakes(&10).backing, 10);

            assert_eq!(Election::transferable_stakes(&20).seat, 0);
            assert_eq!(Election::transferable_stakes(&20).backing, 20);
        });
    }

    #[test]
    fn try_add_applicant_should_work() {
        initial_test_ext().execute_with(|| {
            assert!(Election::applicants().len() == 0);

            let applicant = 20 as u64;

            let starting_balance = 1000 as u64;
            let _ = Balances::deposit_creating(&applicant, starting_balance);

            let stake = 100 as u64;

            assert!(Election::try_add_applicant(applicant, stake).is_ok());
            assert_eq!(Election::applicants(), vec![applicant]);

            assert_eq!(Election::applicant_stakes(applicant).new, stake);
            assert_eq!(Election::applicant_stakes(applicant).transferred, 0);

            assert_eq!(Balances::free_balance(&applicant), starting_balance - stake);
        });
    }

    #[test]
    fn increasing_applicant_stake_should_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let starting_stake = 100 as u64;

            <Applicants<Test>>::put(vec![applicant]);
            <ApplicantStakes<Test>>::insert(
                applicant,
                Stake {
                    new: starting_stake,
                    transferred: 0,
                },
            );

            let additional_stake = 100 as u64;
            let _ = Balances::deposit_creating(&applicant, additional_stake);
            assert!(Election::try_add_applicant(applicant, additional_stake).is_ok());

            assert_eq!(
                Election::applicant_stakes(applicant).new,
                starting_stake + additional_stake
            );
            assert_eq!(Election::applicant_stakes(applicant).transferred, 0)
        });
    }

    #[test]
    fn using_transferable_seat_stake_should_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let _ = Balances::deposit_creating(&applicant, 5000);

            <ExistingStakeHolders<Test>>::put(vec![applicant]);
            save_transferable_stake(
                applicant,
                TransferableStake {
                    seat: 1000,
                    backing: 0,
                },
            );

            <Applicants<Test>>::put(vec![applicant]);
            let starting_stake = Stake {
                new: 100,
                transferred: 0,
            };
            <ApplicantStakes<Test>>::insert(applicant, starting_stake);

            // transferable stake covers new stake
            assert!(Election::try_add_applicant(applicant, 600).is_ok());
            assert_eq!(
                Election::applicant_stakes(applicant).new,
                starting_stake.new
            );
            assert_eq!(Election::applicant_stakes(applicant).transferred, 600);
            assert_eq!(Election::transferable_stakes(applicant).seat, 400);
            assert_eq!(Balances::free_balance(applicant), 5000);

            // all remaining transferable stake is consumed and free balance covers remaining stake
            assert!(Election::try_add_applicant(applicant, 1000).is_ok());
            assert_eq!(
                Election::applicant_stakes(applicant).new,
                starting_stake.new + 600
            );
            assert_eq!(Election::applicant_stakes(applicant).transferred, 1000);
            assert_eq!(Election::transferable_stakes(applicant).seat, 0);
            assert_eq!(Balances::free_balance(applicant), 4400);
        });
    }

    #[test]
    fn moving_to_voting_without_enough_applicants_should_not_work() {
        initial_test_ext().execute_with(|| {
            System::set_block_number(1);
            <AnnouncingPeriod<Test>>::put(20);
            CouncilSize::put(10);
            Election::move_to_announcing_stage();
            let round = Election::round();

            // add applicants
            <Applicants<Test>>::put(vec![10, 20, 30]);
            let stake = Stake {
                new: 10,
                transferred: 0,
            };

            let applicants = Election::applicants();

            for applicant in applicants.iter() {
                <ApplicantStakes<Test>>::insert(applicant, stake);
            }

            // make sure we are testing the condition that we don't have enough applicants
            assert!(Election::council_size_usize() > applicants.len());

            // try to move to voting stage
            let ann_ends = Election::stage_ends_at().unwrap();
            System::set_block_number(ann_ends);
            Election::on_announcing_ended();

            // A new round should have been started
            assert_eq!(Election::round(), round + 1);

            // A new announcing period started
            assert_announcing_period(ann_ends + Election::announcing_period());

            // applicants list should be unchanged..
            assert_eq!(Election::applicants(), applicants);
        });
    }

    #[test]
    fn top_applicants_move_to_voting_stage() {
        initial_test_ext().execute_with(|| {
            <Applicants<Test>>::put(vec![10, 20, 30, 40]);
            let mut applicants = Election::applicants();

            for (i, applicant) in applicants.iter().enumerate() {
                <ApplicantStakes<Test>>::insert(
                    applicant,
                    Stake {
                        new: (i * 10) as u64,
                        transferred: 0,
                    },
                );
            }

            let rejected = Election::find_least_staked_applicants(&mut applicants, 3);
            assert_eq!(rejected.to_vec(), vec![10]);

            <Applicants<Test>>::put(vec![40, 30, 20, 10]);
            let mut applicants = Election::applicants();

            for applicant in applicants.iter() {
                <ApplicantStakes<Test>>::insert(
                    applicant,
                    Stake {
                        new: 20,
                        transferred: 0,
                    },
                );
            }

            // stable sort is preserving order when two elements are equivalent
            let rejected = Election::find_least_staked_applicants(&mut applicants, 3);
            assert_eq!(rejected.to_vec(), vec![40]);
        });
    }

    #[test]
    fn refunding_applicant_stakes_should_work() {
        initial_test_ext().execute_with(|| {
            let _ = Balances::deposit_creating(&1, 1000);
            let _ = Balances::deposit_creating(&2, 7000);
            let _ = Balances::reserve(&2, 5000);
            let _ = Balances::deposit_creating(&3, 8000);
            let _ = Balances::reserve(&3, 5000);

            <Applicants<Test>>::put(vec![1, 2, 3]);

            save_transferable_stake(
                1,
                TransferableStake {
                    seat: 50,
                    backing: 0,
                },
            );
            save_transferable_stake(
                2,
                TransferableStake {
                    seat: 0,
                    backing: 0,
                },
            );
            save_transferable_stake(
                3,
                TransferableStake {
                    seat: 0,
                    backing: 0,
                },
            );

            <ApplicantStakes<Test>>::insert(
                1,
                Stake {
                    new: 100,
                    transferred: 200,
                },
            );

            <ApplicantStakes<Test>>::insert(
                2,
                Stake {
                    new: 300,
                    transferred: 400,
                },
            );

            <ApplicantStakes<Test>>::insert(
                3,
                Stake {
                    new: 500,
                    transferred: 600,
                },
            );

            Election::drop_applicants(&vec![2, 3][..]);

            assert_eq!(Election::applicants(), vec![1]);

            assert_eq!(Election::applicant_stakes(1).new, 100);
            assert_eq!(Election::applicant_stakes(1).transferred, 200);
            assert_eq!(Election::transferable_stakes(1).seat, 50);
            assert_eq!(Balances::free_balance(&1), 1000);

            //assert_eq!(Election::applicant_stakes(2), Default::default());
            assert!(!<ApplicantStakes<Test>>::contains_key(2));
            assert_eq!(Election::transferable_stakes(2).seat, 400);
            assert_eq!(Balances::free_balance(&2), 2300);

            //assert_eq!(Election::applicant_stakes(3), Default::default());
            assert!(!<ApplicantStakes<Test>>::contains_key(3));
            assert_eq!(Election::transferable_stakes(3).seat, 600);
            assert_eq!(Balances::free_balance(&3), 3500);
        });
    }

    #[test]
    fn voting_should_work() {
        initial_test_ext().execute_with(|| {
            let _ = Balances::deposit_creating(&20, 1000);
            let payload = vec![10u8];
            let commitment = <Test as frame_system::Config>::Hashing::hash(&payload[..]);

            assert!(Election::try_add_vote(20, 100, commitment).is_ok());

            assert_eq!(Election::commitments(), vec![commitment]);
            assert_eq!(Election::votes(commitment).voter, 20);
            assert_eq!(Election::votes(commitment).commitment, commitment);
            assert_eq!(
                Election::votes(commitment).stake,
                Stake {
                    new: 100,
                    transferred: 0,
                }
            );
            assert_eq!(Balances::free_balance(&20), 900);
        });
    }

    fn save_transferable_stake(id: u64, stake: TransferableStake<u64>) {
        <TransferableStakes<Test>>::insert(id, stake);
    }

    #[test]
    fn votes_can_be_covered_by_transferable_stake() {
        initial_test_ext().execute_with(|| {
            let _ = Balances::deposit_creating(&20, 1000);

            save_transferable_stake(
                20,
                TransferableStake {
                    seat: 0,
                    backing: 500,
                },
            );

            let payload = vec![10u8];
            let commitment = <Test as frame_system::Config>::Hashing::hash(&payload[..]);

            assert!(Election::try_add_vote(20, 100, commitment).is_ok());

            assert_eq!(Election::commitments(), vec![commitment]);
            assert_eq!(Election::votes(commitment).voter, 20);
            assert_eq!(Election::votes(commitment).commitment, commitment);
            assert_eq!(
                Election::votes(commitment).stake,
                Stake {
                    new: 0,
                    transferred: 100,
                }
            );
            assert_eq!(Balances::free_balance(&20), 1000);
        });
    }

    #[test]
    fn voting_without_enough_balance_should_not_work() {
        initial_test_ext().execute_with(|| {
            let _ = Balances::deposit_creating(&20, 100);

            save_transferable_stake(
                20,
                TransferableStake {
                    seat: 0,
                    backing: 500,
                },
            );

            let payload = vec![10u8];
            let commitment = <Test as frame_system::Config>::Hashing::hash(&payload[..]);

            assert!(Election::try_add_vote(20, 1000, commitment).is_err());
            assert_eq!(Election::commitments(), vec![]);
            assert!(!<Votes<Test>>::contains_key(commitment));
            assert_eq!(Balances::free_balance(&20), 100);
        });
    }

    #[test]
    fn voting_with_existing_commitment_should_not_work() {
        initial_test_ext().execute_with(|| {
            let _ = Balances::deposit_creating(&20, 1000);

            save_transferable_stake(
                20,
                TransferableStake {
                    seat: 0,
                    backing: 500,
                },
            );

            let payload = vec![10u8];
            let commitment = <Test as frame_system::Config>::Hashing::hash(&payload[..]);

            assert!(Election::try_add_vote(20, 100, commitment).is_ok());

            assert_eq!(Election::commitments(), vec![commitment]);
            assert_eq!(Election::votes(commitment).voter, 20);
            assert_eq!(Election::votes(commitment).commitment, commitment);
            assert_eq!(
                Election::votes(commitment).stake,
                Stake {
                    new: 0,
                    transferred: 100,
                }
            );
            assert_eq!(Balances::free_balance(&20), 1000);

            assert!(Election::try_add_vote(30, 100, commitment).is_err());
        });
    }

    fn make_commitment_for_applicant(
        applicant: <Test as frame_system::Config>::AccountId,
        salt: &mut Vec<u8>,
    ) -> <Test as frame_system::Config>::Hash {
        let mut payload = applicant.encode();
        payload.append(salt);
        <Test as frame_system::Config>::Hashing::hash(&payload[..])
    }

    #[test]
    fn revealing_vote_works() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let salt = vec![128u8];
            let commitment = make_commitment_for_applicant(applicant, &mut salt.clone());
            let voter = 10 as u64;

            <ApplicantStakes<Test>>::insert(
                &applicant,
                Stake {
                    new: 0,
                    transferred: 0,
                },
            );

            <Votes<Test>>::insert(
                &commitment,
                SealedVote::new(
                    voter,
                    Stake {
                        new: 100,
                        transferred: 0,
                    },
                    commitment,
                ),
            );

            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
            assert!(Election::try_reveal_vote(voter, commitment, applicant, salt).is_ok());
            assert_eq!(
                <Votes<Test>>::get(commitment).get_vote().unwrap(),
                applicant
            );
        });
    }

    #[test]
    fn revealing_with_bad_salt_should_not_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let salt = vec![128u8];
            let commitment = make_commitment_for_applicant(applicant, &mut salt.clone());
            let voter = 10 as u64;

            <ApplicantStakes<Test>>::insert(
                &applicant,
                Stake {
                    new: 0,
                    transferred: 0,
                },
            );

            <Votes<Test>>::insert(
                &commitment,
                SealedVote::new(
                    voter,
                    Stake {
                        new: 100,
                        transferred: 0,
                    },
                    commitment,
                ),
            );

            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
            assert!(Election::try_reveal_vote(voter, commitment, applicant, vec![]).is_err());
            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
        });
    }

    #[test]
    fn revealing_non_matching_commitment_should_not_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let salt = vec![128u8];
            let commitment = make_commitment_for_applicant(100, &mut salt.clone());
            let voter = 10 as u64;

            <ApplicantStakes<Test>>::insert(
                &applicant,
                Stake {
                    new: 0,
                    transferred: 0,
                },
            );

            assert!(Election::try_reveal_vote(voter, commitment, applicant, vec![]).is_err());
        });
    }

    #[test]
    fn revealing_for_non_applicant_should_not_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let salt = vec![128u8];
            let commitment = make_commitment_for_applicant(applicant, &mut salt.clone());
            let voter = 10 as u64;

            <Votes<Test>>::insert(
                &commitment,
                SealedVote::new(
                    voter,
                    Stake {
                        new: 100,
                        transferred: 0,
                    },
                    commitment,
                ),
            );

            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
            assert!(Election::try_reveal_vote(voter, commitment, applicant, vec![]).is_err());
            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
        });
    }

    #[test]
    fn revealing_by_non_committer_should_not_work() {
        initial_test_ext().execute_with(|| {
            let applicant = 20 as u64;
            let salt = vec![128u8];
            let commitment = make_commitment_for_applicant(applicant, &mut salt.clone());
            let voter = 10 as u64;
            let not_voter = 100 as u64;

            <ApplicantStakes<Test>>::insert(
                &applicant,
                Stake {
                    new: 0,
                    transferred: 0,
                },
            );

            <Votes<Test>>::insert(
                &commitment,
                SealedVote::new(
                    voter,
                    Stake {
                        new: 100,
                        transferred: 0,
                    },
                    commitment,
                ),
            );

            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
            assert!(Election::try_reveal_vote(not_voter, commitment, applicant, salt).is_err());
            assert!(<Votes<Test>>::get(commitment).is_not_revealed());
        });
    }

    pub fn mock_votes(
        mock: Vec<(u64, u64, u64, u64)>,
    ) -> Vec<SealedVote<u64, Stake<u64>, sp_core::H256, u64>> {
        let commitment = make_commitment_for_applicant(1, &mut vec![0u8]);

        mock.into_iter()
            .map(|(voter, stake_ref, stake_tran, applicant)| {
                SealedVote::new_unsealed(
                    voter as u64,
                    Stake {
                        new: stake_ref,
                        transferred: stake_tran,
                    },
                    commitment,
                    applicant as u64,
                )
            })
            .collect()
    }

    #[test]
    fn vote_tallying_should_work() {
        initial_test_ext().execute_with(|| {
            let votes = mock_votes(vec![
                //  (voter, stake[new], stake[transferred], applicant)
                (10, 100, 0, 100),
                (10, 150, 0, 100),
                (10, 500, 0, 200),
                (20, 200, 0, 200),
                (30, 300, 0, 300),
                (30, 400, 0, 300),
            ]);

            let tally = Election::tally_votes(&votes);

            assert_eq!(tally.len(), 3);

            assert_eq!(tally.get(&100).unwrap().member, 100);
            assert_eq!(
                tally.get(&100).unwrap().backers,
                vec![
                    Backer {
                        member: 10 as u64,
                        stake: 100 as u64,
                    },
                    Backer {
                        member: 10 as u64,
                        stake: 150 as u64,
                    },
                ]
            );

            assert_eq!(tally.get(&200).unwrap().member, 200);
            assert_eq!(
                tally.get(&200).unwrap().backers,
                vec![
                    Backer {
                        member: 10 as u64,
                        stake: 500 as u64,
                    },
                    Backer {
                        member: 20 as u64,
                        stake: 200 as u64,
                    }
                ]
            );

            assert_eq!(tally.get(&300).unwrap().member, 300);
            assert_eq!(
                tally.get(&300).unwrap().backers,
                vec![
                    Backer {
                        member: 30 as u64,
                        stake: 300 as u64,
                    },
                    Backer {
                        member: 30 as u64,
                        stake: 400 as u64,
                    }
                ]
            );
        });
    }

    #[test]
    fn filter_top_staked_applicants_should_work() {
        initial_test_ext().execute_with(|| {
            // filter_top_staked depends on order of applicants
            <Applicants<Test>>::put(vec![100, 200, 300]);

            {
                let votes = mock_votes(vec![
                    //  (voter, stake[new], stake[transferred], applicant)
                    (10, 100, 0, 100),
                    (10, 150, 0, 100),
                    (10, 500, 0, 200),
                    (20, 200, 0, 200),
                    (30, 300, 0, 300),
                    (30, 400, 0, 300),
                ]);

                let mut tally = Election::tally_votes(&votes);
                assert_eq!(tally.len(), 3);
                Election::filter_top_staked(&mut tally, 3);
                assert_eq!(tally.len(), 3);
            }

            {
                let votes = mock_votes(vec![
                    //  (voter, stake[new], stake[transferred], applicant)
                    (10, 100, 0, 100),
                    (10, 150, 0, 100),
                    (10, 500, 0, 200),
                    (20, 200, 0, 200),
                    (30, 300, 0, 300),
                    (30, 400, 0, 300),
                ]);

                let mut tally = Election::tally_votes(&votes);
                assert_eq!(tally.len(), 3);
                Election::filter_top_staked(&mut tally, 2);
                assert_eq!(tally.len(), 2);
                assert!(tally.get(&200).is_some());
                assert!(tally.get(&300).is_some());
            }
        });
    }

    #[test]
    fn drop_unelected_applicants_should_work() {
        initial_test_ext().execute_with(|| {
            <Applicants<Test>>::put(vec![100, 200, 300]);

            let _ = Balances::deposit_creating(&100, 2000);
            let _ = Balances::reserve(&100, 1000);

            <ApplicantStakes<Test>>::insert(
                100,
                Stake {
                    new: 20 as u64,
                    transferred: 50 as u64,
                },
            );

            save_transferable_stake(
                100,
                TransferableStake {
                    seat: 100,
                    backing: 0,
                },
            );

            let mut new_council: BTreeMap<u64, Seat<u64, u64>> = BTreeMap::new();
            new_council.insert(
                200 as u64,
                Seat {
                    member: 200 as u64,
                    stake: 0 as u64,
                    backers: vec![],
                },
            );
            new_council.insert(
                300 as u64,
                Seat {
                    member: 300 as u64,
                    stake: 0 as u64,
                    backers: vec![],
                },
            );

            Election::drop_unelected_applicants(&new_council);

            // applicant dropped
            assert_eq!(Election::applicants(), vec![200, 300]);
            assert!(!<ApplicantStakes<Test>>::contains_key(100));

            // and refunded
            assert_eq!(Election::transferable_stakes(100).seat, 150);
            assert_eq!(Balances::free_balance(&100), 1020);
            assert_eq!(Balances::reserved_balance(&100), 980);
        });
    }

    #[test]
    fn refunding_voting_stakes_should_work() {
        initial_test_ext().execute_with(|| {
            // voters' balances
            let _ = Balances::deposit_creating(&10, 6000);
            let _ = Balances::reserve(&10, 5000);
            let _ = Balances::deposit_creating(&20, 7000);
            let _ = Balances::reserve(&20, 5000);
            let _ = Balances::deposit_creating(&30, 8000);
            let _ = Balances::reserve(&30, 5000);

            save_transferable_stake(
                10,
                TransferableStake {
                    seat: 0,
                    backing: 100,
                },
            );
            save_transferable_stake(
                20,
                TransferableStake {
                    seat: 0,
                    backing: 200,
                },
            );
            save_transferable_stake(
                30,
                TransferableStake {
                    seat: 0,
                    backing: 300,
                },
            );

            let votes = mock_votes(vec![
                //  (voter, stake[new], stake[transferred], applicant)
                (10, 100, 20, 100),
                (20, 200, 40, 100),
                (30, 300, 60, 100),
                (10, 500, 70, 200),
                (20, 600, 80, 200),
                (30, 700, 90, 200),
                (10, 800, 100, 300),
                (20, 900, 120, 300),
                (30, 1000, 140, 300),
            ]);

            let mut new_council: BTreeMap<u64, Seat<u64, u64>> = BTreeMap::new();
            new_council.insert(
                200 as u64,
                Seat {
                    member: 200 as u64,
                    stake: 0 as u64,
                    backers: vec![],
                },
            );
            new_council.insert(
                300 as u64,
                Seat {
                    member: 300 as u64,
                    stake: 0 as u64,
                    backers: vec![],
                },
            );

            Election::refund_voting_stakes(&votes, &new_council);

            assert_eq!(Balances::free_balance(&10), 1100);
            assert_eq!(Balances::reserved_balance(&10), 4900);
            assert_eq!(Balances::free_balance(&20), 2200);
            assert_eq!(Balances::reserved_balance(&20), 4800);
            assert_eq!(Balances::free_balance(&30), 3300);
            assert_eq!(Balances::reserved_balance(&30), 4700);

            assert_eq!(Election::transferable_stakes(10).backing, 120);
            assert_eq!(Election::transferable_stakes(20).backing, 240);
            assert_eq!(Election::transferable_stakes(30).backing, 360);
        });
    }

    #[test]
    fn unlock_transferable_stakes_should_work() {
        initial_test_ext().execute_with(|| {
            <ExistingStakeHolders<Test>>::put(vec![10, 20, 30]);

            let _ = Balances::deposit_creating(&10, 6000);
            let _ = Balances::reserve(&10, 5000);
            save_transferable_stake(
                10,
                TransferableStake {
                    seat: 50,
                    backing: 100,
                },
            );

            let _ = Balances::deposit_creating(&20, 7000);
            let _ = Balances::reserve(&20, 5000);
            save_transferable_stake(
                20,
                TransferableStake {
                    seat: 60,
                    backing: 200,
                },
            );

            let _ = Balances::deposit_creating(&30, 8000);
            let _ = Balances::reserve(&30, 5000);
            save_transferable_stake(
                30,
                TransferableStake {
                    seat: 70,
                    backing: 300,
                },
            );

            Election::unlock_transferable_stakes();

            assert_eq!(Balances::free_balance(&10), 1150);
            assert_eq!(Balances::free_balance(&20), 2260);
            assert_eq!(Balances::free_balance(&30), 3370);
        });
    }

    #[test]
    fn council_elected_hook_should_work() {
        initial_test_ext().execute_with(|| {
            let mut new_council: BTreeMap<u64, Seat<u64, u64>> = BTreeMap::new();
            new_council.insert(
                200 as u64,
                Seat {
                    member: 200 as u64,
                    stake: 10 as u64,
                    backers: vec![],
                },
            );
            new_council.insert(
                300 as u64,
                Seat {
                    member: 300 as u64,
                    stake: 20 as u64,
                    backers: vec![],
                },
            );

            assert_eq!(Council::active_council().len(), 0);

            let new_council = new_council
                .into_iter()
                .map(|(_, seat)| seat.clone())
                .collect();
            <Test as election::Config>::CouncilElected::council_elected(new_council, 10);

            assert_eq!(Council::active_council().len(), 2);
        });
    }

    #[test]
    fn simulation() {
        initial_test_ext().execute_with(|| {
            assert_eq!(Council::active_council().len(), 0);
            assert!(Election::stage().is_none());

            CouncilSize::put(10);
            <MinCouncilStake<Test>>::put(50);
            <AnnouncingPeriod<Test>>::put(10);
            <VotingPeriod<Test>>::put(10);
            <RevealingPeriod<Test>>::put(10);
            CandidacyLimit::put(20);
            <NewTermDuration<Test>>::put(100);
            <MinVotingStake<Test>>::put(10);

            for i in 1..30 {
                let _ = Balances::deposit_creating(&(i as u64), 50000);
            }

            System::set_block_number(1);
            assert_ok!(Election::start_election(vec![]));

            for i in 1..20 {
                if i < 21 {
                    assert!(Election::apply(Origin::signed(i), 150).is_ok());
                } else {
                    assert!(Election::apply(Origin::signed(i + 1000), 150).is_err()); // not enough free balance
                    assert!(Election::apply(Origin::signed(i), 20).is_err()); // not enough minimum stake
                }
            }

            let n = 1 + Election::announcing_period();
            System::set_block_number(n);
            let _ = Election::on_finalize(n);

            for i in 1..20 {
                assert!(Election::vote(
                    Origin::signed(i),
                    make_commitment_for_applicant(i, &mut vec![40u8]),
                    100
                )
                .is_ok());

                assert!(Election::vote(
                    Origin::signed(i),
                    make_commitment_for_applicant(i, &mut vec![41u8]),
                    100
                )
                .is_ok());

                assert!(Election::vote(
                    Origin::signed(i),
                    make_commitment_for_applicant(i + 1000, &mut vec![42u8]),
                    100
                )
                .is_ok());
            }

            let n = n + Election::voting_period();
            System::set_block_number(n);
            let _ = Election::on_finalize(n);

            for i in 1..20 {
                assert!(Election::reveal(
                    Origin::signed(i),
                    make_commitment_for_applicant(i, &mut vec![40u8]),
                    i,
                    vec![40u8]
                )
                .is_ok());
                //wrong salt
                assert!(Election::reveal(
                    Origin::signed(i),
                    make_commitment_for_applicant(i, &mut vec![41u8]),
                    i,
                    vec![]
                )
                .is_err());
                //vote not for valid applicant
                assert!(Election::reveal(
                    Origin::signed(i),
                    make_commitment_for_applicant(i + 1000, &mut vec![42u8]),
                    i + 1000,
                    vec![42u8]
                )
                .is_err());
            }

            let n = n + Election::revealing_period();
            System::set_block_number(n);
            let _ = Election::on_finalize(n);

            assert_eq!(
                Council::active_council().len(),
                Election::council_size_usize()
            );
            for (i, seat) in Council::active_council().iter().enumerate() {
                assert_eq!(seat.member, (i + 1) as u64);
            }
            assert!(Election::stage().is_none());

            // When council term ends.. start a new election.
            assert_ok!(Election::start_election(vec![]));
        });
    }

    #[test]
    fn setting_election_parameters() {
        initial_test_ext().execute_with(|| {
            let default_parameters: ElectionParameters<u64, u64> = ElectionParameters::default();
            // default all zeros is invalid
            assert!(default_parameters.ensure_valid().is_err());

            let new_parameters = ElectionParameters {
                announcing_period: 1,
                voting_period: 2,
                revealing_period: 3,
                council_size: 4,
                candidacy_limit: 5,
                min_voting_stake: 6,
                min_council_stake: 7,
                new_term_duration: 8,
            };

            assert_ok!(Election::set_election_parameters(
                RawOrigin::Root.into(),
                new_parameters
            ));

            assert_eq!(
                <AnnouncingPeriod<Test>>::get(),
                new_parameters.announcing_period
            );
            assert_eq!(<VotingPeriod<Test>>::get(), new_parameters.voting_period);
            assert_eq!(
                <RevealingPeriod<Test>>::get(),
                new_parameters.revealing_period
            );
            assert_eq!(
                <MinCouncilStake<Test>>::get(),
                new_parameters.min_council_stake
            );
            assert_eq!(
                <NewTermDuration<Test>>::get(),
                new_parameters.new_term_duration
            );
            assert_eq!(CouncilSize::get(), new_parameters.council_size);
            assert_eq!(CandidacyLimit::get(), new_parameters.candidacy_limit);
            assert_eq!(
                <MinVotingStake<Test>>::get(),
                new_parameters.min_voting_stake
            );
        });
    }
}
