use agglayer_primitives::keccak::Digest;
use serde::{Deserialize, Serialize};
use sha2::{Digest as Sha256Digest, Sha256};

/// Public values to verify the SP1 aggchain proof.
#[derive(Serialize, Deserialize)]
pub struct AggchainProofPublicValues {
    /// Previous local exit root.
    pub prev_local_exit_root: Digest,

    /// New local exit root.
    pub new_local_exit_root: Digest,

    /// L1 info root used to import bridge exits.
    pub l1_info_root: Digest,

    /// Origin network for which the proof was generated.
    pub origin_network: u32,

    /// Commitment to the imported bridge exits indexes.
    pub commit_imported_bridge_exits: Digest,

    /// Chain-specific commitment forwarded by the PP.
    pub aggchain_params: Digest,
}

impl AggchainProofPublicValues {
    pub fn hash(&self) -> [u8; 32] {
        let AggchainProofPublicValues {
            prev_local_exit_root,
            new_local_exit_root,
            l1_info_root,
            origin_network,
            commit_imported_bridge_exits,
            aggchain_params,
        } = self;

        let public_values = [
            prev_local_exit_root.as_slice(),
            new_local_exit_root.as_slice(),
            l1_info_root.as_slice(),
            &origin_network.to_le_bytes(),
            commit_imported_bridge_exits.as_slice(),
            aggchain_params.as_slice(),
        ]
        .concat();

        Sha256::digest(&public_values).into()
    }
}
