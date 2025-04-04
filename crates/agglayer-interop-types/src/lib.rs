pub use agglayer_primitives::digest::Digest;
use agglayer_tries::error::SmtError;
use serde::{Deserialize, Serialize};

pub mod aggchain_proof;

pub type EpochNumber = u64;
pub type CertificateId = Digest;

pub use agglayer_primitives as primitives;
// Re-export common primitives again as agglayer-types root types
pub use agglayer_primitives::{Address, Signature, B256, U256, U512};
pub use unified_bridge::bridge_exit::BridgeExit;
pub use unified_bridge::bridge_exit::NetworkId;
pub use unified_bridge::global_index::GlobalIndex;
pub use unified_bridge::imported_bridge_exit::{
    Claim, ClaimFromMainnet, ClaimFromRollup, CommitmentVersion, GlobalIndexWithLeafHash,
    ImportedBridgeExit, ImportedBridgeExitCommitmentValues, L1InfoTreeLeaf, L1InfoTreeLeafInner,
    MerkleProof,
};
pub use unified_bridge::token_info::LeafType;
pub use unified_bridge::token_info::TokenInfo;
// pub use pessimistic_proof::proof::Proof;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    /// The operation cannot be applied on the smt.
    #[error(transparent)]
    InvalidSmtOperation(#[from] SmtError),
}
