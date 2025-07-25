use agglayer_primitives::{keccak::keccak256_combine, Digest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// A node in an SMT
#[serde_as]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Node {
    #[serde_as(as = "_")]
    pub left: Digest,

    #[serde_as(as = "_")]
    pub right: Digest,
}

impl Node {
    #[inline]
    pub fn hash(&self) -> Digest {
        keccak256_combine([self.left.as_ref(), self.right.as_ref()])
    }
}
