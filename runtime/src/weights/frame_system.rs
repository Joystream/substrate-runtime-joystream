//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub struct WeightInfo;
impl frame_system::WeightInfo for WeightInfo {
    // WARNING! Some components were not used: ["b"]
    fn remark(_: u32) -> Weight {
        (9_342_000 as Weight)
    }
    fn set_heap_pages() -> Weight {
        (11_274_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // WARNING! Some components were not used: ["d"]
    fn set_changes_trie_config() -> Weight {
        (32_325_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    fn set_storage(i: u32) -> Weight {
        (0 as Weight)
            .saturating_add((2_060_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    fn kill_storage(i: u32) -> Weight {
        (27_686_000 as Weight)
            .saturating_add((1_237_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    fn kill_prefix(p: u32) -> Weight {
        (27_689_000 as Weight)
            .saturating_add((1_205_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
    fn remark_with_event(_: u32) -> u64 {
        todo!()
    }
}
