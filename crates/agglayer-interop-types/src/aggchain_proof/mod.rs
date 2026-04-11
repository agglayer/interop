use agglayer_primitives::Signature;
use serde::{Deserialize, Serialize};
pub use unified_bridge::AggchainProofPublicValues;

use crate::Digest;

// Aggchain data submitted via the [`Certificate`].
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary, Eq, PartialEq))]
#[serde(untagged)]
pub enum AggchainData {
    ECDSA {
        signature: Signature,
    },
    Generic {
        /// proof of the aggchain proof.
        proof: Proof,
        /// Chain-specific commitment forwarded through the PP.
        aggchain_params: Digest,
        /// Signature of the aggchain proof.
        signature: Option<Box<Signature>>,
        /// Optional aggchain proof public values.
        public_values: Option<Box<AggchainProofPublicValues>>,
    },
    MultisigOnly {
        multisig: MultisigPayload,
    },
    MultisigAndAggchainProof {
        multisig: MultisigPayload,
        aggchain_proof: AggchainProof,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary, Eq, PartialEq))]
pub struct MultisigPayload(pub Vec<Option<Signature>>);

// Aggchain proof data submitted via the Certificate.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary, Eq, PartialEq))]
pub struct AggchainProof {
    /// proof of the aggchain proof.
    pub proof: Proof,
    /// Chain-specific commitment forwarded through the PP.
    pub aggchain_params: Digest,
    /// Optional aggchain proof public values.
    pub public_values: Option<Box<AggchainProofPublicValues>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary, Eq, PartialEq))]
pub struct SP1StarkWithContext {
    pub proof: Vec<u8>,
    pub vkey: Vec<u8>,
    pub version: String,
}

/// Proof that is part of the aggchain proof submitted via the [`Certificate`].
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary, Eq, PartialEq))]
pub enum Proof {
    SP1Stark(SP1StarkWithContext),
}

#[cfg(test)]
mod test;
