pub use agglayer_primitives::Digest;
use agglayer_tries::error::SmtError;
use serde::{Deserialize, Serialize};

pub mod aggchain_proof;

pub type EpochNumber = u64;
pub type CertificateId = Digest;

pub use agglayer_primitives as primitives;
// Re-export common primitives again as agglayer-types root types
pub use agglayer_primitives::{Address, Signature, B256, U256, U512};
pub use unified_bridge::BridgeExit;
pub use unified_bridge::GlobalIndex;
pub use unified_bridge::LeafType;
pub use unified_bridge::NetworkId;
pub use unified_bridge::TokenInfo;
pub use unified_bridge::{
    Claim, ClaimFromMainnet, ClaimFromRollup, GlobalIndexWithLeafHash, ImportedBridgeExit,
    ImportedBridgeExitCommitmentValues, L1InfoTreeLeaf, L1InfoTreeLeafInner, MerkleProof,
};

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    /// The operation cannot be applied on the smt.
    #[error(transparent)]
    InvalidSmtOperation(#[from] SmtError),
}
