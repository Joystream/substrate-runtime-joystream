#![cfg(test)]

use super::Test;
use crate::{Config, PayoutStatusHandler};
use std::cell::RefCell;

struct StatusHandlerState<T: Config> {
    successes: Vec<T::RewardRelationshipId>,
    failures: Vec<T::RewardRelationshipId>,
}

impl<T: Config> StatusHandlerState<T> {
    pub fn reset(&mut self) {
        self.successes = vec![];
        self.failures = vec![];
    }
}

impl<T: Config> Default for StatusHandlerState<T> {
    fn default() -> Self {
        Self {
            successes: vec![],
            failures: vec![],
        }
    }
}

thread_local!(static STATUS_HANDLER_STATE: RefCell<StatusHandlerState<Test>> = RefCell::new(Default::default()));

pub struct MockStatusHandler {}
impl MockStatusHandler {
    pub fn reset() {
        STATUS_HANDLER_STATE.with(|cell| {
            cell.borrow_mut().reset();
        });
    }
    pub fn successes() -> usize {
        let mut value = 0;
        STATUS_HANDLER_STATE.with(|cell| {
            value = cell.borrow_mut().successes.len();
        });
        value
    }
    pub fn failures() -> usize {
        let mut value = 0;
        STATUS_HANDLER_STATE.with(|cell| {
            value = cell.borrow_mut().failures.len();
        });
        value
    }
}
impl PayoutStatusHandler<Test> for MockStatusHandler {
    fn payout_succeeded(id: u64, _destination_account: &u64, _amount: u64) {
        STATUS_HANDLER_STATE.with(|cell| {
            cell.borrow_mut().successes.push(id);
        });
    }

    fn payout_failed(id: u64, _destination_account: &u64, _amount: u64) {
        STATUS_HANDLER_STATE.with(|cell| {
            cell.borrow_mut().failures.push(id);
        });
    }
}
