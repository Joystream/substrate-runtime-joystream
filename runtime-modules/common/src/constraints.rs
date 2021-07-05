use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_arithmetic::traits::UniqueSaturatedInto;
use sp_std::ops::Add;

/// Length constraint for input validation.
pub type InputValidationLengthConstraint = BoundedValueConstraint<u16>;

/// Value constraint within boundaries.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Copy)]
pub struct BoundedValueConstraint<Val> {
    /// Minimum value
    pub min: Val,

    /// Difference between minimum value and max value.
    /// While having max would have been more direct, this
    /// way makes max < min unrepresentable semantically,
    /// which is safer.
    pub max_min_diff: Val,
}

impl<Val: Copy + Add<Val, Output = Val> + Ord> BoundedValueConstraint<Val> {
    /// Helper for computing max value.
    pub fn max(&self) -> Val {
        self.min + self.max_min_diff
    }

    /// Validates value and its constraints.
    pub fn ensure_valid<E, ValType: UniqueSaturatedInto<Val>>(
        &self,
        value: ValType,
        less_err: E,
        greater_err: E,
    ) -> Result<(), E> {
        let converted_value: Val = value.unique_saturated_into();

        if converted_value < self.min {
            Err(less_err)
        } else if converted_value > self.max() {
            Err(greater_err)
        } else {
            Ok(())
        }
    }

    /// Create new value constraints.
    pub fn new(min: Val, max_min_diff: Val) -> Self {
        Self { min, max_min_diff }
    }
}
