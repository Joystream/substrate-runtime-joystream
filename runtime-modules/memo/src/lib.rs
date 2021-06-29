// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;
use frame_support::{decl_event, decl_module, decl_storage, ensure};
use frame_system::ensure_signed;
use sp_arithmetic::traits::Zero;
use sp_std::vec::Vec;

use common::currency::GovernanceCurrency;

pub trait Config: frame_system::Config + GovernanceCurrency {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

pub type MemoText = Vec<u8>;

decl_storage! {
    trait Store for Module<T: Config> as Memo {
        Memo get(fn memo) : map hasher(blake2_128_concat) T::AccountId => MemoText;
        MaxMemoLength get(fn max_memo_length) : u32 = 4096;
    }
}

decl_event! {
    pub enum Event<T> where <T as frame_system::Config>::AccountId {
        MemoUpdated(AccountId),
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000_000] // TODO: adjust weight
        fn update_memo(origin, memo: MemoText) {
            let sender = ensure_signed(origin)?;

            ensure!(!T::Currency::total_balance(&sender).is_zero(), "account must have a balance");
            ensure!(memo.len() as u32 <= Self::max_memo_length(), "memo too long");

            <Memo<T>>::insert(&sender, memo);
            Self::deposit_event(RawEvent::MemoUpdated(sender));
        }
    }
}
