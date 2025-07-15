use agglayer_primitives::{keccak::keccak256_combine, Digest};

/// Returns an array whose `i`th element is the root of an empty Merkle tree of
/// depth `i`.
#[inline]
pub fn empty_hash_at_height<const DEPTH: usize>() -> [Digest; DEPTH] {
    let mut empty_hash_at_height = [Digest::default(); DEPTH];
    for height in 1..DEPTH {
        empty_hash_at_height[height] = keccak256_combine([
            &empty_hash_at_height[height - 1],
            &empty_hash_at_height[height - 1],
        ]);
    }
    empty_hash_at_height
}
