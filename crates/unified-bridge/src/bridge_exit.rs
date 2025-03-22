use std::{fmt::Display, ops::Deref};

use agglayer_primitives::keccak::{keccak256, keccak256_combine};
use agglayer_primitives::utils::Hashable;
use agglayer_primitives::{digest::Digest, Address, U256};
use hex_literal::hex;
use serde::{Deserialize, Serialize};

pub use crate::token_info::{LeafType, TokenInfo, L1_ETH};

const EMPTY_METADATA_HASH: Digest = Digest(hex!(
    "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
));

impl Hashable for TokenInfo {
    fn hash(&self) -> Digest {
        keccak256_combine([
            &self.origin_network.to_be_bytes(),
            self.origin_token_address.as_slice(),
        ])
    }
}

/// Represents a token bridge exit from the network.
// TODO: Change it to an enum depending on `leaf_type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct BridgeExit {
    pub leaf_type: LeafType,

    /// Unique ID for the token being transferred.
    pub token_info: TokenInfo,

    /// Network which the token is transferred to
    pub dest_network: NetworkId,
    /// Address which will own the received token
    pub dest_address: Address,

    /// Token amount sent
    pub amount: U256,

    pub metadata: Option<Digest>,
}

impl BridgeExit {
    /// Creates a new [`BridgeExit`].
    pub fn new(
        leaf_type: LeafType,
        origin_network: NetworkId,
        origin_token_address: Address,
        dest_network: NetworkId,
        dest_address: Address,
        amount: U256,
        metadata: Vec<u8>,
    ) -> Self {
        Self {
            leaf_type,
            token_info: TokenInfo {
                origin_network,
                origin_token_address,
            },
            dest_network,
            dest_address,
            amount,
            metadata: Some(keccak256(metadata.as_slice())),
        }
    }

    pub fn is_transfer(&self) -> bool {
        self.leaf_type == LeafType::Transfer
    }
}

impl Hashable for BridgeExit {
    fn hash(&self) -> Digest {
        keccak256_combine([
            [self.leaf_type as u8].as_slice(),
            &u32::to_be_bytes(*self.token_info.origin_network),
            self.token_info.origin_token_address.as_slice(),
            &u32::to_be_bytes(*self.dest_network),
            self.dest_address.as_slice(),
            &self.amount.to_be_bytes::<32>(),
            &self.metadata.unwrap_or(EMPTY_METADATA_HASH).0,
        ])
    }
}

impl BridgeExit {
    pub fn is_message(&self) -> bool {
        self.leaf_type == LeafType::Message
    }

    /// Returns the [`TokenInfo`] considered for the the given amount.
    /// The amount corresponds to L1 ETH if the bridge exit is a message.
    pub fn amount_token_info(&self) -> TokenInfo {
        match self.leaf_type {
            LeafType::Message => L1_ETH,
            LeafType::Transfer => self.token_info,
        }
    }
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct NetworkId(u32);

impl Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NetworkId {
    pub const BITS: usize = u32::BITS as usize;

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for NetworkId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<NetworkId> for u32 {
    fn from(value: NetworkId) -> Self {
        value.0
    }
}

impl Deref for NetworkId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
