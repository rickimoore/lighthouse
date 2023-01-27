use crate::test_utils::TestRandom;
use crate::*;
use bls::SignatureBytes;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

#[derive(
    arbitrary::Arbitrary,
    Debug,
    PartialEq,
    Hash,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    TestRandom,
)]
pub struct SignedBlsToExecutionChange {
    pub message: BlsToExecutionChange,
    pub signature: SignatureBytes,
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_and_tree_hash_tests!(SignedBlsToExecutionChange);
}
