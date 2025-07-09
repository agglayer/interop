use agglayer_primitives::{address, Address};
use agglayer_tries::proof::ToBits;
use serde::{Deserialize, Serialize};

use crate::NetworkId;

pub const L1_ETH: TokenInfo = TokenInfo {
    origin_network: NetworkId::ETH_L1,
    origin_token_address: address!("0000000000000000000000000000000000000000"),
};

/// Encapsulates the information to uniquely identify a token on the origin
/// network.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Copy,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct TokenInfo {
    /// Network which the token originates from
    pub origin_network: NetworkId,

    /// The address of the token on the origin network
    #[rkyv(with = crate::bridge_exit::AddressDef)]
    pub origin_token_address: Address,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub enum LeafType {
    Transfer = 0,
    Message = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid leaf type number")]
pub struct LeafTypeFromU8Error;

impl TryFrom<u8> for LeafType {
    type Error = LeafTypeFromU8Error;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Transfer),
            1 => Ok(Self::Message),
            _ => Err(LeafTypeFromU8Error),
        }
    }
}

impl ToBits<192> for TokenInfo {
    #[inline]
    fn to_bits(&self) -> [bool; 192] {
        let address_bytes = self.origin_token_address.0;
        // Security: We assume here that `address_bytes` is a fixed-size array of
        // 20 bytes. The following code could panic otherwise.
        std::array::from_fn(|i| {
            if i < 32 {
                (self.origin_network.to_u32() >> i) & 1 == 1
            } else {
                ((address_bytes[(i - 32) / 8]) >> (i % 8)) & 1 == 1
            }
        })
    }
}
