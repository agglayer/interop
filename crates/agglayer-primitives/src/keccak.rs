use tiny_keccak::{Hasher as _, Keccak};

use crate::Digest;

/// Hashes the input data using a Keccak hasher with a 256-bit security level.
#[inline]
pub fn keccak256(data: &[u8]) -> Digest {
    let mut hasher = Keccak::v256();
    hasher.update(data);

    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    Digest(output)
}

/// Hashes the input items using a Keccak hasher with a 256-bit security level.
/// Safety: This function should only be called with fixed-size items to avoid
/// collisions.
#[inline]
pub fn keccak256_combine<I, T>(items: I) -> Digest
where
    I: IntoIterator<Item = T>,
    T: AsRef<[u8]>,
{
    let mut hasher = Keccak::v256();
    for data in items {
        hasher.update(data.as_ref());
    }

    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    Digest(output)
}
