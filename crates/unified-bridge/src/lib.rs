pub mod aggchain_proof;
pub mod bridge_exit;
pub mod global_index;
pub mod imported_bridge_exit;
pub mod local_exit_tree;
pub mod token_info;

#[derive(Debug, Clone, Copy)]
pub enum CommitmentVersion {
    V2,
    V3,
}
