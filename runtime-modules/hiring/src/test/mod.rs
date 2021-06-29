#![cfg(test)]

mod public_api;
mod smoke;
mod staking_module;
use public_api::*;

use crate::mock::Test;
use crate::*;

use sp_std::cell::RefCell;
use sp_std::rc::Rc;

use std::panic;

pub(crate) type OpeningId = <Test as Config>::OpeningId;
pub(crate) type ApplicationId = <Test as Config>::ApplicationId;
pub(crate) type BlockNumber = <Test as frame_system::Config>::BlockNumber;
pub(crate) type StakeId = <Test as stake::Config>::StakeId;
pub(crate) type Balance = <<Test as stake::Config>::Currency as Currency<
    <Test as frame_system::Config>::AccountId,
>>::Balance;

// Debug test object content. Recurring temporary usage - do not delete.
#[allow(dead_code)]
pub fn debug_print<T: sp_std::fmt::Debug>(obj: T) {
    println!("{:?}", obj);
}
