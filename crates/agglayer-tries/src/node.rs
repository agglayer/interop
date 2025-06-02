use agglayer_primitives::keccak::Hasher;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;

/// A node in an SMT
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Node<H>
where
    H: Hasher,
    H::Digest: Serialize + DeserializeOwned,
{
    #[serde_as(as = "_")]
    pub left: H::Digest,

    #[serde_as(as = "_")]
    pub right: H::Digest,
}

impl<H> Clone for Node<H>
where
    H: Hasher,
    H::Digest: Copy + Serialize + DeserializeOwned,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<H> Copy for Node<H>
where
    H: Hasher,
    H::Digest: Copy + Serialize + DeserializeOwned,
{
}

impl<H> Node<H>
where
    H: Hasher,
    H::Digest: Serialize + DeserializeOwned,
{
    #[inline]
    pub fn hash(&self) -> H::Digest {
        H::merge(&self.left, &self.right)
    }
}
