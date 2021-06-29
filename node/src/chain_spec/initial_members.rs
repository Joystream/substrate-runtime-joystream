use joystream_node_runtime::{membership, AccountId, Moment};
use std::{fs, path::Path};

/// Generates a Vec of genesis members parsed from a json file
pub fn from_json(data_file: &Path) -> Vec<membership::genesis::Member<u64, AccountId, Moment>> {
    let data = fs::read_to_string(data_file).expect("Failed reading file");
    serde_json::from_str(&data).expect("failed parsing members data")
}

/// Generates an empty Vec of genesis members
pub fn none() -> Vec<membership::genesis::Member<u64, AccountId, Moment>> {
    vec![]
}
