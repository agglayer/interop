pub mod aggchain_proof;
pub mod bridge_exit;
pub mod global_index;
pub mod imported_bridge_exit;
pub mod local_exit_tree;
mod network_id;
mod rollup_id;
mod rollup_index;
pub mod token_info;

pub use network_id::NetworkId;
pub use rollup_id::RollupId;
pub use rollup_index::RollupIndex;

#[derive(Debug, Clone, Copy)]
pub enum CommitmentVersion {
    V2,
    V3,
}
