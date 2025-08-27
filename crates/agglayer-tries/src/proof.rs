use agglayer_primitives::{keccak::keccak256_combine, Digest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::EMPTY_HASH_ARRAY_AT_193;

pub trait ToBits<const NUM_BITS: usize> {
    fn to_bits(&self) -> [bool; NUM_BITS];
}

impl ToBits<8> for u8 {
    #[inline]
    fn to_bits(&self) -> [bool; 8] {
        std::array::from_fn(|i| (self >> i) & 1 == 1)
    }
}

impl ToBits<32> for u32 {
    #[inline]
    fn to_bits(&self) -> [bool; 32] {
        std::array::from_fn(|i| (self >> i) & 1 == 1)
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmtMerkleProof<const DEPTH: usize> {
    #[serde_as(as = "[_; DEPTH]")]
    pub siblings: [Digest; DEPTH],
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmtNonInclusionProof<const DEPTH: usize> {
    #[serde_as(as = "Vec<_>")]
    pub siblings: Vec<Digest>,
}

impl<const DEPTH: usize> SmtMerkleProof<DEPTH> {
    pub fn verify<K>(&self, key: K, value: Digest, root: Digest) -> bool
    where
        K: ToBits<DEPTH>,
    {
        let bits = key.to_bits();
        let mut hash = value;
        for i in 0..DEPTH {
            hash = if bits[DEPTH - i - 1] {
                keccak256_combine([&self.siblings[i], &hash])
            } else {
                keccak256_combine([&hash, &self.siblings[i]])
            };
        }

        hash == root
    }

    /// Verify the inclusion proof (i.e. that `(key, old_value)` is in the SMT)
    /// and return the updated root of the SMT with `(key, new_value)`
    /// inserted, or `None` if the inclusion proof is invalid.
    pub fn verify_and_update<K>(
        &self,
        key: K,
        old_value: Digest,
        new_value: Digest,
        root: Digest,
    ) -> Option<Digest>
    where
        K: ToBits<DEPTH> + Copy,
    {
        if !self.verify(key, old_value, root) {
            return None;
        }
        let bits = key.to_bits();
        let mut hash = new_value;
        for i in 0..DEPTH {
            hash = if bits[DEPTH - i - 1] {
                keccak256_combine([&self.siblings[i], &hash])
            } else {
                keccak256_combine([&hash, &self.siblings[i]])
            };
        }

        Some(hash)
    }
}

impl<const DEPTH: usize> SmtNonInclusionProof<DEPTH> {
    pub fn verify<K>(&self, key: K, root: Digest) -> bool
    where
        K: ToBits<DEPTH>,
    {
        if self.siblings.len() > DEPTH {
            return false;
        }
        if self.siblings.is_empty() {
            let empty_root = EMPTY_HASH_ARRAY_AT_193[DEPTH];
            return root == empty_root;
        }
        let bits = key.to_bits();
        let mut entry = EMPTY_HASH_ARRAY_AT_193[DEPTH - self.siblings.len()];
        for i in (0..self.siblings.len()).rev() {
            let sibling = self.siblings[i];
            entry = if bits[i] {
                keccak256_combine([&sibling, &entry])
            } else {
                keccak256_combine([&entry, &sibling])
            };
        }

        entry == root
    }

    /// Verify the non-inclusion proof (i.e. that `key` is not in the SMT) and
    /// return the updated root of the SMT with `(key, value)` inserted, or
    /// `None` if the inclusion proof is invalid.
    pub fn verify_and_update<K>(&self, key: K, new_value: Digest, root: Digest) -> Option<Digest>
    where
        K: Copy + ToBits<DEPTH>,
    {
        if !self.verify(key, root) {
            return None;
        }

        let mut entry = new_value;
        let bits = key.to_bits();
        for i in (self.siblings.len()..DEPTH).rev() {
            let sibling = EMPTY_HASH_ARRAY_AT_193[DEPTH - i - 1];
            entry = if bits[i] {
                keccak256_combine([&sibling, &entry])
            } else {
                keccak256_combine([&entry, &sibling])
            };
        }
        for i in (0..self.siblings.len()).rev() {
            let sibling = self.siblings[i];
            entry = if bits[i] {
                keccak256_combine([&sibling, &entry])
            } else {
                keccak256_combine([&entry, &sibling])
            };
        }

        Some(entry)
    }
}
