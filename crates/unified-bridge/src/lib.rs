mod aggchain_proof;
mod bridge_exit;
mod global_index;
mod imported_bridge_exit;
mod local_exit_tree;
mod network_id;
mod rollup_id;
mod rollup_index;
mod token_info;

pub use aggchain_proof::AggchainProofPublicValues;
pub use bridge_exit::BridgeExit;
pub use global_index::GlobalIndex;
pub use imported_bridge_exit::{
    Claim, ClaimFromMainnet, ClaimFromRollup, GlobalIndexWithLeafHash, ImportedBridgeExit,
    ImportedBridgeExitCommitmentValues, L1InfoTreeLeaf, L1InfoTreeLeafInner, MerkleProof,
};
pub use local_exit_tree::{proof::LETMerkleProof, LocalExitTree};
pub use network_id::NetworkId;
pub use rollup_id::{InvalidRollupIdError, RollupId};
pub use rollup_index::{InvalidRollupIndexError, RollupIndex};
pub use token_info::{LeafType, TokenInfo, L1_ETH};

#[derive(Debug, Clone, Copy)]
pub enum CommitmentVersion {
    V2,
    V3,
}
