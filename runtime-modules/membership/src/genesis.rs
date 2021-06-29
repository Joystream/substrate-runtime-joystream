#![cfg(feature = "std")]

use crate::{Config, GenesisConfig};
use common::currency::BalanceOf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Member<MemberId, AccountId, Moment> {
    pub member_id: MemberId,
    pub root_account: AccountId,
    pub controller_account: AccountId,
    pub handle: String,
    pub avatar_uri: String,
    pub about: String,
    pub registered_at_time: Moment,
}

/// Builder fo membership module genesis configuration.
pub struct GenesisConfigBuilder<T: Config> {
    default_paid_membership_fee: BalanceOf<T>,
    members: Vec<(T::MemberId, T::AccountId)>,
}

impl<T: Config> Default for GenesisConfigBuilder<T> {
    fn default() -> Self {
        Self {
            default_paid_membership_fee: BalanceOf::<T>::default(), // Was 100, will this break any tests??
            members: vec![],
        }
    }
}

impl<T: Config> GenesisConfigBuilder<T> {
    /// Assign default paid membeship fee
    pub fn default_paid_membership_fee(
        mut self,
        default_paid_membership_fee: BalanceOf<T>,
    ) -> Self {
        self.default_paid_membership_fee = default_paid_membership_fee;
        self
    }

    /// Assign a collection of MemberId and AccountId pairs, used to derive mock member at genesis
    pub fn members(mut self, members: Vec<(T::MemberId, T::AccountId)>) -> Self {
        self.members = members;
        self
    }

    /// Generates a Vec of `Member`s from pairs of MemberId and AccountId
    fn generate_mock_members(&self) -> Vec<Member<T::MemberId, T::AccountId, T::Moment>> {
        self.members
            .iter()
            .enumerate()
            .map(|(ix, (ref member_id, ref account_id))| Member {
                member_id: *member_id,
                root_account: account_id.clone(),
                controller_account: account_id.clone(),
                // hack to get min handle length to 5
                handle: (10000 + ix).to_string(),
                avatar_uri: "".into(),
                about: "".into(),
                registered_at_time: T::Moment::from(0u32),
            })
            .collect()
    }

    /// Construct GenesisConfig for mocked testing purposes only
    pub fn build(&self) -> GenesisConfig<T> {
        GenesisConfig::<T> {
            default_paid_membership_fee: self.default_paid_membership_fee,
            members: self.generate_mock_members(),
        }
    }
}
