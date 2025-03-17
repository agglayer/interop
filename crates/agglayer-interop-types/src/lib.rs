pub use agglayer_primitives::digest::Digest;
use pessimistic_proof::core::commitment::PPRootVersion;
use pessimistic_proof::local_balance_tree::{LocalBalanceTree, LOCAL_BALANCE_TREE_DEPTH};
use pessimistic_proof::local_exit_tree::hasher::Keccak256Hasher;
use pessimistic_proof::local_exit_tree::{LocalExitTree, LocalExitTreeError};
use pessimistic_proof::local_state::StateCommitment;
use pessimistic_proof::nullifier_tree::{NullifierTree, NULLIFIER_TREE_DEPTH};
use pessimistic_proof::utils::smt::{Smt, SmtError};
use pessimistic_proof::LocalNetworkState;
use serde::{Deserialize, Serialize};

pub mod aggchain_proof;

pub type EpochNumber = u64;
pub type CertificateId = Digest;

pub use agglayer_primitives as primitives;
// Re-export common primitives again as agglayer-types root types
pub use agglayer_primitives::{Address, Signature, B256, U256, U512};
pub use pessimistic_proof::bridge_exit::{BridgeExit, LeafType, NetworkId, TokenInfo};
pub use pessimistic_proof::global_index::GlobalIndex;
pub use pessimistic_proof::imported_bridge_exit::{
    Claim, ClaimFromMainnet, ClaimFromRollup, ImportedBridgeExit, L1InfoTreeLeaf,
    L1InfoTreeLeafInner, MerkleProof,
};
pub use pessimistic_proof::proof::Proof;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    /// The imported bridge exits should refer to one and the same L1 info root.
    #[error("Imported bridge exits refer to multiple L1 info root")]
    MultipleL1InfoRoot,
    /// The certificate refers to a new local exit root which differ from the
    /// one computed by the agglayer.
    #[error(
        "Mismatch on the certificate new local exit root. declared: {declared:?}, computed: \
         {computed:?}"
    )]
    MismatchNewLocalExitRoot { computed: Digest, declared: Digest },
    /// The given token balance cannot overflow.
    #[error("Token balance cannot overflow. token: {0:?}")]
    BalanceOverflow(TokenInfo),
    /// The given token balance cannot be negative.
    #[error("Token balance cannot be negative. token: {0:?}")]
    BalanceUnderflow(TokenInfo),
    /// The balance proof for the given token cannot be generated.
    #[error("Unable to generate the balance proof. token: {token:?}, error: {source}")]
    BalanceProofGenerationFailed {
        source: pessimistic_proof::utils::smt::SmtError,
        token: TokenInfo,
    },
    /// The nullifier path for the given imported bridge exit cannot be
    /// generated.
    #[error(
        "Unable to generate the nullifier path. global_index: {global_index:?}, error: {source}"
    )]
    NullifierPathGenerationFailed {
        source: pessimistic_proof::utils::smt::SmtError,
        global_index: GlobalIndex,
    },
    /// The operation cannot be applied on the local exit tree.
    #[error(transparent)]
    InvalidLocalExitTreeOperation(#[from] LocalExitTreeError),
    #[error(
        "Incorrect L1 Info Root for the leaf count {leaf_count}. declared: {declared}, retrieved \
         from L1: {retrieved}"
    )]
    /// Invalid or unsettled L1 Info Root
    L1InfoRootIncorrect {
        leaf_count: u32,
        declared: Digest,
        retrieved: Digest,
    },
    #[error(
        "Incorrect declared L1 Info Tree information: l1_leaf: {l1_leaf:?}, l1_root: \
         {l1_info_root:?}"
    )]
    InconsistentL1InfoTreeInformation {
        l1_leaf: Option<u32>,
        l1_info_root: Option<Digest>,
    },
    /// The operation cannot be applied on the smt.
    #[error(transparent)]
    InvalidSmtOperation(#[from] SmtError),

    #[error("AggchainVkey missing")]
    MissingAggchainVkey,

    #[error(
        "Invalid custom chain data length expected at least {expected_at_least}, actual {actual}"
    )]
    InvalidCustomChainDataLength {
        expected_at_least: usize,
        actual: usize,
    },
}

/// Local state data of one network.
/// The AggLayer tracks the [`LocalNetworkStateData`] for all networks.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LocalNetworkStateData {
    /// The local exit tree without leaves.
    pub exit_tree: LocalExitTree<Keccak256Hasher>,
    /// The full local balance tree.
    pub balance_tree: Smt<Keccak256Hasher, LOCAL_BALANCE_TREE_DEPTH>,
    /// The full nullifier tree.
    pub nullifier_tree: Smt<Keccak256Hasher, NULLIFIER_TREE_DEPTH>,
}

impl From<LocalNetworkStateData> for LocalNetworkState {
    fn from(state: LocalNetworkStateData) -> Self {
        LocalNetworkState {
            exit_tree: state.exit_tree,
            balance_tree: LocalBalanceTree::new_with_root(state.balance_tree.root),
            nullifier_tree: NullifierTree::new_with_root(state.nullifier_tree.root),
        }
    }
}

impl From<LocalNetworkStateData> for pessimistic_proof::NetworkState {
    fn from(state: LocalNetworkStateData) -> Self {
        LocalNetworkState::from(state).into()
    }
}

/// The last pessimistic root can be either fetched from L1 or recomputed for a
/// given version.
pub enum PessimisticRootInput {
    /// Computed from the given version.
    Computed(PPRootVersion),
    /// Fetched from the L1.
    Fetched(Digest),
}

impl LocalNetworkStateData {
    /// Prune the SMTs
    pub fn prune_stale_nodes(&mut self) -> Result<(), Error> {
        self.balance_tree.traverse_and_prune()?;
        self.nullifier_tree.traverse_and_prune()?;

        Ok(())
    }

    pub fn get_roots(&self) -> StateCommitment {
        StateCommitment {
            exit_root: self.exit_tree.get_root(),
            ler_leaf_count: self.exit_tree.leaf_count(),
            balance_root: self.balance_tree.root,
            nullifier_root: self.nullifier_tree.root,
        }
    }
}
