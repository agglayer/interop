#![allow(clippy::needless_range_loop)]
use std::fmt::Debug;

use agglayer_primitives::{keccak::keccak256_combine, Digest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LETMerkleProof<const TREE_DEPTH: usize = 32> {
    #[serde_as(as = "[_; TREE_DEPTH]")]
    pub siblings: [Digest; TREE_DEPTH],
}

#[cfg(feature = "testutils")]
impl<'a, const TREE_DEPTH: usize> arbitrary::Arbitrary<'a> for LETMerkleProof<TREE_DEPTH> {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let siblings = <[Digest; TREE_DEPTH]>::arbitrary(u)?;
        Ok(Self { siblings })
    }
}

impl<const TREE_DEPTH: usize> LETMerkleProof<TREE_DEPTH> {
    #[inline]
    pub fn verify(&self, leaf: Digest, leaf_index: u32, root: Digest) -> bool {
        let mut entry = leaf;
        let mut index = leaf_index;
        for &sibling in &self.siblings {
            entry = if index & 1 == 0 {
                keccak256_combine([&entry, &sibling])
            } else {
                keccak256_combine([&sibling, &entry])
            };
            index >>= 1;
        }
        if index != 0 {
            return false;
        }

        entry == root
    }
}
