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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Copy)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct TokenInfo {
    /// Network which the token originates from
    pub origin_network: NetworkId,

    /// The address of the token on the origin network
    pub origin_token_address: Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        let address_bytes = self.origin_token_address.as_slice();
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

impl TokenInfo {
    /// Reconstructs a [`TokenInfo`] from its bit representation (the SMT path)
    pub fn from_bits(bits: &[bool; 192]) -> Self {
        // reconstruct the NetworkId from the bits 0..32
        let mut network_id_u32 = 0u32;
        for (i, &bit) in bits[0..32].iter().enumerate() {
            if bit {
                network_id_u32 |= 1 << i;
            }
        }

        // reconstruct the Address from the bits 32..192
        let mut address_bytes = [0u8; 20];
        for (i, &bit) in bits[32..192].iter().enumerate() {
            if bit {
                let byte_index = i / 8;
                let bit_offset = i % 8;
                address_bytes[byte_index] |= 1 << bit_offset;
            }
        }

        TokenInfo {
            origin_network: NetworkId::from(network_id_u32),
            origin_token_address: Address::from(address_bytes),
        }
    }
}

#[test]
fn test_token_info_round_trip() {
    let initial = TokenInfo {
        origin_network: NetworkId::from(1234),
        origin_token_address: Address::from([0xab; 20]),
    };

    assert_eq!(initial, TokenInfo::from_bits(&initial.to_bits()));
}
