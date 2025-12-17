use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use agglayer_primitives::Digest;

use crate::{
    error::SmtError,
    node::Node,
    proof::{SmtMerkleProof, SmtNonInclusionProof, ToBits},
    utils::{empty_hash_array_at_height, empty_hash_at_height, EMPTY_HASH_ARRAY_AT_193},
};

/// An SMT consistent with a zero-initialized Merkle tree
#[derive(Clone, Debug)]
pub struct Smt<const DEPTH: usize> {
    /// The SMT root
    pub root: Digest,

    /// A map from node hash to node
    pub tree: HashMap<Digest, Node>,
}

impl<const DEPTH: usize> Default for Smt<DEPTH> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const DEPTH: usize> Smt<DEPTH> {
    pub(crate) const EMPTY_HASH_ARRAY_AT_HEIGHT: &[Digest; DEPTH] =
        empty_hash_array_at_height::<DEPTH>();

    #[inline]
    pub fn new() -> Self {
        let root = Node {
            left: empty_hash_at_height::<DEPTH>(),
            right: empty_hash_at_height::<DEPTH>(),
        };
        Self::new_with_nodes(root.hash(), &[root])
    }

    #[inline]
    pub fn new_with_nodes(root: Digest, nodes: &[Node]) -> Self {
        Smt {
            root,
            tree: nodes.iter().map(|n| (n.hash(), *n)).collect(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root == EMPTY_HASH_ARRAY_AT_193[DEPTH]
    }

    #[inline]
    pub fn get<K>(&self, key: K) -> Option<Digest>
    where
        K: ToBits<DEPTH>,
    {
        let mut hash = self.root;
        for b in key.to_bits() {
            hash = if b {
                self.tree.get(&hash)?.right
            } else {
                self.tree.get(&hash)?.left
            };
        }

        Some(hash)
    }

    fn insert_helper(
        &mut self,
        hash: Digest,
        depth: usize,
        bits: &[bool; DEPTH],
        value: Digest,
        // If true, update the value at the key.
        // If false, insert the value at the key and error if the key is present.
        update: bool,
    ) -> Result<Digest, SmtError> {
        if depth == DEPTH {
            return if !update && hash != Self::EMPTY_HASH_ARRAY_AT_HEIGHT[0] {
                Err(SmtError::KeyAlreadyPresent)
            } else {
                Ok(value)
            };
        }
        let node = self.tree.get(&hash);
        let mut node = node.copied().unwrap_or(Node {
            left: Self::empty_hash_at_depth_from_root(depth)?,
            right: Self::empty_hash_at_depth_from_root(depth)?,
        });
        let child_hash = if bits[depth] {
            self.insert_helper(node.right, depth + 1, bits, value, update)
        } else {
            self.insert_helper(node.left, depth + 1, bits, value, update)
        }?;
        if bits[depth] {
            node.right = child_hash;
        } else {
            node.left = child_hash;
        }

        let new_hash = node.hash();
        self.tree.insert(new_hash, node);

        Ok(new_hash)
    }

    /// Returns all the key value pairs contained in the SMT
    pub fn entries(&self) -> Result<Vec<([bool; DEPTH], Digest)>, SmtError> {
        let mut entries = Vec::new();
        let mut current_path = [false; DEPTH];

        self.entries_helper(self.root, 0, &mut current_path, &mut entries)?;

        Ok(entries)
    }

    /// Accumulates all the keypairs through dfs
    fn entries_helper(
        &self,
        hash: Digest,
        depth: usize,
        path: &mut [bool; DEPTH],
        acc: &mut Vec<([bool; DEPTH], Digest)>,
    ) -> Result<(), SmtError> {
        // Reached a leaf, adds it only if non-null
        if depth == DEPTH {
            if hash != Digest::ZERO {
                acc.push((*path, hash));
            }
            return Ok(());
        }

        // Reached an empty sub-tree
        if hash == Self::empty_hash_at_depth_from_root(depth)? {
            return Ok(());
        }

        // Reached an intermediary node
        if let Some(node) = self.tree.get(&hash) {
            // traverse left
            path[depth] = false;
            self.entries_helper(node.left, depth + 1, path, acc)?;

            // traverse right
            path[depth] = true;
            self.entries_helper(node.right, depth + 1, path, acc)?;
        }

        Ok(())
    }

    #[inline]
    pub fn insert<K>(&mut self, key: K, value: Digest) -> Result<(), SmtError>
    where
        K: ToBits<DEPTH>,
    {
        let new_root = self.insert_helper(self.root, 0, &key.to_bits(), value, false)?;
        self.root = new_root;

        Ok(())
    }

    #[inline]
    pub fn update<K>(&mut self, key: K, value: Digest) -> Result<(), SmtError>
    where
        K: ToBits<DEPTH>,
    {
        let new_root = self.insert_helper(self.root, 0, &key.to_bits(), value, true)?;
        self.root = new_root;

        Ok(())
    }

    fn traverse_helper(
        &self,
        hash: Digest,
        depth: usize,
        nodes: &mut HashSet<Digest>,
    ) -> Result<(), SmtError> {
        nodes.insert(hash);

        if depth == DEPTH {
            // We've reached a leaf.
            return Ok(());
        }

        let node = self.tree.get(&hash).ok_or(SmtError::KeyNotPresent)?;
        if node.left != Self::empty_hash_at_depth_from_root(depth)? {
            self.traverse_helper(node.left, depth + 1, nodes)?;
        }
        if node.right != Self::empty_hash_at_depth_from_root(depth)? {
            self.traverse_helper(node.right, depth + 1, nodes)?;
        }

        Ok(())
    }

    #[inline]
    const fn empty_hash_at_depth_from_root(depth: usize) -> Result<Digest, SmtError> {
        if depth > DEPTH {
            return Err(SmtError::DepthOutOfBounds);
        }
        // We are calculating the depth from the leaf to the root,
        // hence we need to subtract the depth from the tree height.
        Ok(Self::EMPTY_HASH_ARRAY_AT_HEIGHT[(DEPTH - 1) - depth])
    }

    /// Traverse the SMT and prune all stale nodes.
    #[inline]
    pub fn traverse_and_prune(&mut self) -> Result<(), SmtError>
    where
        Digest: Eq + Hash,
    {
        let mut seen_nodes = HashSet::new();
        self.traverse_helper(self.root, 0, &mut seen_nodes)?;
        self.tree.retain(|k, _v| seen_nodes.contains(k));

        Ok(())
    }

    fn get_inclusion_proof_helper<K>(
        &self,
        key: K,
        zero_allowed: bool,
    ) -> Result<SmtMerkleProof<DEPTH>, SmtError>
    where
        K: ToBits<DEPTH>,
    {
        let mut siblings = [Self::EMPTY_HASH_ARRAY_AT_HEIGHT[0]; DEPTH];
        let mut hash = self.root;
        let bits = key.to_bits();
        for i in 0..DEPTH {
            let node = self.tree.get(&hash).ok_or(SmtError::KeyNotPresent)?;
            siblings[DEPTH - i - 1] = if bits[i] { node.left } else { node.right };
            hash = if bits[i] { node.right } else { node.left };
        }
        if !zero_allowed && hash == Self::EMPTY_HASH_ARRAY_AT_HEIGHT[0] {
            return Err(SmtError::KeyNotPresent);
        }

        Ok(SmtMerkleProof { siblings })
    }

    #[inline]
    pub fn get_inclusion_proof<K>(&self, key: K) -> Result<SmtMerkleProof<DEPTH>, SmtError>
    where
        K: ToBits<DEPTH>,
    {
        self.get_inclusion_proof_helper(key, false)
    }

    /// Returns an inclusion proof that the key is not in the SMT.
    /// This has the same purpose as a non-inclusion proof, but with the same
    /// format as an inclusion proof. Use case: In the balance tree, we use
    /// inclusion proofs to verify the balance of a token in the tree and
    /// update it. If the token is not already in the tree, we still want an
    /// inclusion proof, so we use this function.
    #[inline]
    pub fn get_inclusion_proof_zero<K>(&mut self, key: K) -> Result<SmtMerkleProof<DEPTH>, SmtError>
    where
        K: Copy + ToBits<DEPTH>,
    {
        // Hack: We use `insert` to insert all the necessary nodes in the SMT.
        // This will return an error if the key is in the SMT.
        self.insert(key, Self::EMPTY_HASH_ARRAY_AT_HEIGHT[0])?;
        self.get_inclusion_proof_helper(key, true)
    }

    pub fn get_non_inclusion_proof<K>(
        &self,
        key: K,
    ) -> Result<SmtNonInclusionProof<DEPTH>, SmtError>
    where
        K: ToBits<DEPTH>,
    {
        let mut siblings = vec![];
        let mut hash = self.root;
        let bits = key.to_bits();

        for (depth, bit) in bits.iter().take(DEPTH).enumerate() {
            if Self::empty_hash_at_depth_from_root(depth).map_err(|_| SmtError::DepthOutOfBounds)?
                == hash
            {
                return Ok(SmtNonInclusionProof { siblings });
            }
            let Some(node) = self.tree.get(&hash) else {
                return Ok(SmtNonInclusionProof { siblings });
            };
            siblings.push(if *bit { node.left } else { node.right });
            hash = if *bit { node.right } else { node.left };
        }
        if hash != Self::EMPTY_HASH_ARRAY_AT_HEIGHT[0] {
            return Err(SmtError::KeyPresent);
        }

        Ok(SmtNonInclusionProof { siblings })
    }
}
