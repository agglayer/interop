use agglayer_primitives::digest::Digest;

pub mod empty_hash;
pub mod smt;

pub use agglayer_primitives::utils::FromBool;
pub use agglayer_primitives::utils::FromU256;

/// Trait for objects that can be hashed.
pub trait Hashable {
    /// Hashes the object to a [`Digest`].
    fn hash(&self) -> Digest;
}
