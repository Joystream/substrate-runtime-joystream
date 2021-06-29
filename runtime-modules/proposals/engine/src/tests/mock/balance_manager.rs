#![cfg(test)]

use frame_support::traits::{Currency, Imbalance};
pub use sp_arithmetic::traits::Zero;

use super::*;

/// StakingEventsHandler implementation for the stake::Config. Restores balances after the unstaking
/// and slashes balances if necessary.
pub struct BalanceManagerStakingEventsHandler;
impl stake::StakingEventsHandler<Test> for BalanceManagerStakingEventsHandler {
    fn unstaked(
        _id: &u64,
        _unstaked_amount: stake::BalanceOf<Test>,
        imbalance: stake::NegativeImbalance<Test>,
    ) -> stake::NegativeImbalance<Test> {
        let default_account_id = 1;

        <Test as stake::Config>::Currency::resolve_creating(&default_account_id, imbalance);

        stake::NegativeImbalance::<Test>::zero()
    }

    fn slashed(
        _id: &u64,
        _slash_id: Option<<Test as stake::Config>::SlashId>,
        _slashed_amount: stake::BalanceOf<Test>,
        _remaining_stake: stake::BalanceOf<Test>,
        imbalance: stake::NegativeImbalance<Test>,
    ) -> stake::NegativeImbalance<Test> {
        imbalance
    }
}
