pub use agglayer_bincode as bincode;
pub use agglayer_primitives::Digest;
use agglayer_tries::error::SmtError;
pub use agglayer_tries::roots::{BalanceRoot, LocalExitRoot, NullifierRoot};
use serde::{Deserialize, Serialize};

pub mod aggchain_proof;

pub type EpochNumber = u64;
pub type CertificateId = Digest;

pub use agglayer_primitives as primitives;
// Re-export common primitives again as agglayer-types root types
pub use agglayer_primitives::{Address, Signature, B256, U256, U512};
pub use unified_bridge::{
    BridgeExit, Claim, ClaimFromMainnet, ClaimFromRollup, GlobalIndex, GlobalIndexWithLeafHash,
    ImportedBridgeExit, ImportedBridgeExitCommitmentValues, L1InfoTreeLeaf, L1InfoTreeLeafInner,
    LeafType, MerkleProof, NetworkId, TokenInfo,
};

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    /// The operation cannot be applied on the smt.
    #[error(transparent)]
    InvalidSmtOperation(#[from] SmtError),
}
