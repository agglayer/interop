use crate::{digest::Digest, U256};

/// Allows for the conversion of a boolean to a type
/// This trait is used in tree to properly bound the type of the digest.
pub trait FromBool {
    fn from_bool(b: bool) -> Self;
}

pub trait FromU256 {
    fn from_u256(u: U256) -> Self;
}

/// Trait for objects that can be hashed.
pub trait Hashable {
    /// Hashes the object to a [`Digest`].
    fn hash(&self) -> Digest;
}
