use joystream_node_runtime::{AccountId, Balance};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct SerializedInitialBalances {
    balances: Vec<(AccountId, Balance)>,
}

fn parse_json(data_file: &Path) -> SerializedInitialBalances {
    let data = fs::read_to_string(data_file).expect("Failed reading file");
    serde_json::from_str(&data).expect("failed parsing balances data")
}

/// Deserializes initial balances from json file
pub fn from_json(data_file: &Path) -> Vec<(AccountId, Balance)> {
    parse_json(data_file).balances
}
