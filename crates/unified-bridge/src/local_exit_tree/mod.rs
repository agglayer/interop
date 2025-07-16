use agglayer_primitives::{keccak::keccak256_combine, Digest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

pub mod proof;

/// Represents a local exit tree as defined by the LxLy bridge.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalExitTree<const TREE_DEPTH: usize = 32> {
    /// The number of inserted (non-empty) leaves.
    pub leaf_count: u32,

    /// The frontier, with hashes from bottom to top, only of the rightmost
    /// non-empty node. This is not a path in the tree, and only contains
    /// full subtrees.
    ///
    /// It contains meaningful values only up to the current root of the tree,
    /// computed by log2(leaf_count). After that, all values are zeroed out.
    #[serde_as(as = "[_; TREE_DEPTH]")]
    pub frontier: [Digest; TREE_DEPTH],
}

#[derive(Clone, Debug, Error, Serialize, Deserialize, PartialEq, Eq)]
pub enum LocalExitTreeError {
    #[error("Leaf index overflow")]
    LeafIndexOverflow,

    #[error("Index out of bounds")]
    IndexOutOfBounds,

    #[error("Frontier index out of bounds")]
    FrontierIndexOutOfBounds,
}

impl<const TREE_DEPTH: usize> Default for LocalExitTree<TREE_DEPTH> {
    #[inline]
    fn default() -> Self {
        Self {
            leaf_count: 0,
            frontier: [Digest::default(); TREE_DEPTH],
        }
    }
}

impl<const TREE_DEPTH: usize> LocalExitTree<TREE_DEPTH> {
    const MAX_NUM_LEAVES: u32 = ((1u64 << TREE_DEPTH) - 1) as u32;

    /// Creates a new empty [`LocalExitTree`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn leaf_count(&self) -> u32 {
        self.leaf_count
    }

    #[inline]
    pub fn frontier(&self) -> [Digest; TREE_DEPTH] {
        self.frontier
    }

    /// Creates a new [`LocalExitTree`] and populates its leaves.
    #[inline]
    pub fn from_leaves(leaves: impl Iterator<Item = Digest>) -> Result<Self, LocalExitTreeError> {
        let mut tree = Self::new();

        for leaf in leaves {
            tree.add_leaf(leaf)?;
        }

        Ok(tree)
    }

    /// Creates a new [`LocalExitTree`] from its parts: leaf count, and
    /// frontier.
    #[inline]
    pub fn from_parts(leaf_count: u32, frontier: [Digest; TREE_DEPTH]) -> Self {
        Self {
            leaf_count,
            frontier,
        }
    }
    /// Appends a leaf to the tree.
    #[inline]
    pub fn add_leaf(&mut self, leaf: Digest) -> Result<u32, LocalExitTreeError> {
        if self.leaf_count >= Self::MAX_NUM_LEAVES {
            return Err(LocalExitTreeError::LeafIndexOverflow);
        }
        // the index at which the new entry will be inserted
        let frontier_insertion_index: usize = {
            let leaf_count_after_insertion = self.leaf_count + 1;

            leaf_count_after_insertion.trailing_zeros() as usize
        };

        // the new entry to be inserted in the frontier
        let new_frontier_entry = {
            let mut entry = leaf;
            for frontier_ele in &self.frontier[0..frontier_insertion_index] {
                entry = keccak256_combine([frontier_ele, &entry]);
            }

            entry
        };

        // update tree
        self.frontier[frontier_insertion_index] = new_frontier_entry;
        self.leaf_count = self
            .leaf_count
            .checked_add(1)
            .ok_or(LocalExitTreeError::LeafIndexOverflow)?;

        Ok(self.leaf_count)
    }

    /// Computes and returns the root of the tree.
    #[inline]
    pub fn get_root(&self) -> Digest {
        // `root` is the hash of the node we’re going to fill next.
        // Here, we compute the root, starting from the next (yet unfilled) leaf hash.
        let mut root = Digest::default();
        let mut empty_hash_at_height = Digest::default();

        for height in 0..TREE_DEPTH {
            if get_bit_at(self.leaf_count, height) == 1 {
                root = keccak256_combine([&self.frontier[height], &root]);
            } else {
                root = keccak256_combine([&root, &empty_hash_at_height]);
            }

            empty_hash_at_height =
                keccak256_combine([&empty_hash_at_height, &empty_hash_at_height]);
        }

        root
    }
}

/// Returns the bit value at index `bit_idx` in `target`
#[inline]
fn get_bit_at(target: u32, bit_idx: usize) -> u32 {
    (target >> bit_idx) & 1
}

#[cfg(test)]
mod tests {
    use agglayer_primitives::{Address, Hashable, U256};

    use crate::{bridge_exit::BridgeExit, local_exit_tree::LocalExitTree, token_info::LeafType};

    #[test]
    fn test_deposit_hash() {
        let mut deposit = BridgeExit::new(
            LeafType::Transfer,
            0.into(),
            Address::ZERO,
            1.into(),
            Address::ZERO,
            U256::default(),
            vec![],
        );

        let amount_bytes = hex::decode("8ac7230489e80000").unwrap_or_default();
        deposit.amount = U256::try_from_be_slice(amount_bytes.as_slice()).unwrap();

        let dest_addr = hex::decode("c949254d682d8c9ad5682521675b8f43b102aec4").unwrap_or_default();
        deposit.dest_address.as_mut().copy_from_slice(&dest_addr);

        let leaf_hash = deposit.hash();
        assert_eq!(
            "22ed288677b4c2afd83a6d7d55f7df7f4eaaf60f7310210c030fd27adacbc5e0",
            hex::encode(leaf_hash)
        );

        let mut dm = LocalExitTree::<32>::new();
        dm.add_leaf(leaf_hash).unwrap();
        let dm_root = dm.get_root();
        assert_eq!(
            "5ba002329b53c11a2f1dfe90b11e031771842056cf2125b43da8103c199dcd7f",
            hex::encode(dm_root.as_slice())
        );
    }
}
