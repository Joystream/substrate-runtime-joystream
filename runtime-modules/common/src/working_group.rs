use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use strum_macros::EnumIter;

/// Defines well-known working groups.
/// Additional integer values are set to maintain the index of the enum variants after its
/// modifying. The 'isize' suffix is required by the 'clippy' linter. We should revisit it after we
/// upgrade the rust compiler version (current version is "nightly-2021-02-20-x86_64").
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, EnumIter))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Copy, Debug, PartialOrd, Ord)]
pub enum WorkingGroup {
    /* Reserved
        /// Forum working group: working_group::Instance1.
        Forum,
    */
    /// Storage working group: working_group::Instance2.
    Storage = 2isize,

    /// Storage working group: working_group::Instance3.
    Content = 3isize,
}
