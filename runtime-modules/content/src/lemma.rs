/// lemma structure for the merkle proof path:
/// `(data, path)` with
/// `path = [ item1 item2 ... itemN ]` and `item = (hash, side)`
/// `data` is the initial value whose proof membership is to be established
/// item contains the hash value required for the proof together with the side, that is the provided
/// hash is to the left or to the right to the current computed hash during verification.
use codec::{Decode, Encode};
use core::fmt::Debug;

#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};

use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Left,
    Right,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct LemmaItem<HashOutput> {
    pub hash: HashOutput,
    pub side: Side,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Proof<Algorithm: Hash, Value> {
    pub data: Value,
    pub path: Vec<LemmaItem<Algorithm::Output>>,
}

pub trait CommitmentProof<Algorithm: Hash> {
    fn verify(&self, root: Algorithm::Output) -> bool;
}

impl<Algorithm: Hash, Value> CommitmentProof<Algorithm> for Proof<Algorithm, Value>
where
    Value: Encode + Decode,
{
    fn verify(&self, root: Algorithm::Output) -> bool {
        let init_hash = <Algorithm as sp_runtime::traits::Hash>::hash(&self.data.encode());
        let candidate = self
            .path
            .iter()
            .fold(init_hash, |hash_v, el| match el.side {
                Side::Right => {
                    <Algorithm as sp_runtime::traits::Hash>::hash(&[hash_v, el.hash].encode())
                }
                Side::Left => {
                    <Algorithm as sp_runtime::traits::Hash>::hash(&[el.hash, hash_v].encode())
                }
            });
        candidate == root
    }
}
