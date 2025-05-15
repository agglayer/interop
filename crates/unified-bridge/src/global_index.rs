use agglayer_primitives::{keccak::keccak256, Digest, U256};
use serde::{Deserialize, Serialize};

use crate::{InvalidRollupIndexError, NetworkId, RollupIndex};

/// The [`GlobalIndex`] uniquely references one leaf within one Global Exit
/// Tree.
///
/// Further defined by the LXLY specifications.
/// | 191 bits |    1 bit      |    32 bits   |    32 bits   |
/// |    0     |  mainnet flag | rollup index |  leaf index  |
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct GlobalIndex {
    mainnet_flag: bool,
    rollup_index: RollupIndex,
    leaf_index: u32,
}

impl GlobalIndex {
    const MAINNET_FLAG_OFFSET: usize = 2 * 32;

    #[inline]
    pub fn is_mainnet(&self) -> bool {
        self.mainnet_flag
    }

    #[inline]
    pub fn network_id(&self) -> NetworkId {
        if self.mainnet_flag {
            NetworkId::new(0)
        } else {
            self.rollup_index.into()
        }
    }

    #[inline]
    pub fn rollup_index(&self) -> Option<RollupIndex> {
        if self.mainnet_flag {
            None
        } else {
            Some(self.rollup_index)
        }
    }

    #[inline]
    pub fn leaf_index(&self) -> u32 {
        self.leaf_index
    }

    #[inline]
    pub fn hash(&self) -> Digest {
        let global_index: U256 = (*self).into();
        keccak256(global_index.as_le_slice())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidGlobalIndexError {
    #[error(transparent)]
    InvalidRollupIndex(InvalidRollupIndexError),

    #[error("Global index has the mainnet flag but a non-zero rollup index")]
    MainnetWithNonZeroRollupIndex,
}

impl TryFrom<U256> for GlobalIndex {
    type Error = InvalidGlobalIndexError;

    #[inline]
    fn try_from(value: U256) -> Result<Self, Self::Error> {
        let bytes = value.as_le_slice();

        let mainnet_flag = value.bit(Self::MAINNET_FLAG_OFFSET);
        // Security: This uses the slice to fixed array TryFrom impl in the std
        // library that is technically fallible. However, our range length in
        // both cases is equal to u32::len() so it is safe to disregard the Result
        // and treat this as an infallible conversion.
        let rollup_index = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let leaf_index = u32::from_le_bytes(bytes[0..4].try_into().unwrap());

        if mainnet_flag && rollup_index != 0 {
            return Err(InvalidGlobalIndexError::MainnetWithNonZeroRollupIndex);
        }

        Ok(Self {
            mainnet_flag,
            rollup_index: rollup_index
                .try_into()
                .map_err(InvalidGlobalIndexError::InvalidRollupIndex)?,
            leaf_index,
        })
    }
}

impl From<GlobalIndex> for U256 {
    #[inline]
    fn from(value: GlobalIndex) -> Self {
        let mut bytes = [0u8; 32];

        let leaf_bytes = value.leaf_index.to_le_bytes();
        bytes[0..4].copy_from_slice(&leaf_bytes);

        let rollup_bytes = value.rollup_index.to_le_bytes();
        bytes[4..8].copy_from_slice(&rollup_bytes);

        if value.mainnet_flag {
            bytes[8] |= 0x01;
        }

        U256::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(raw: &str, expected: GlobalIndex) {
        let global_index_u256 = U256::from_str_radix(raw, 10).unwrap();
        assert_eq!(
            global_index_u256,
            GlobalIndex::try_from(global_index_u256).unwrap().into()
        );
        assert_eq!(expected, GlobalIndex::try_from(global_index_u256).unwrap());
    }

    #[test]
    fn test_global_index() {
        // https://bridge-api.zkevm-g-mainnet.com/bridges/0xa1D5E9CB4f6a09fcF8b938435b0DE63270C67537
        check(
            "18446744073709748107",
            GlobalIndex {
                mainnet_flag: true,
                rollup_index: RollupIndex::new(0).unwrap(),
                leaf_index: 196491,
            },
        );

        // https://etherscan.io/tx/0xd9bc7b7de2df86e08221e41806cfa798693d700f1f644810beb0e7c14706b82d
        check(
            "4294968029",
            GlobalIndex {
                mainnet_flag: false,
                rollup_index: RollupIndex::new(1).unwrap(),
                leaf_index: 733,
            },
        );
    }

    #[test]
    fn test_invalid_global_index() {
        assert!(
            GlobalIndex::try_from(U256::from_str_radix("FFFFFFFF12345678", 16).unwrap()).is_err(),
            "Invalid rollup index should fail to parse",
        );
        assert!(
            GlobalIndex::try_from(U256::from_str_radix("10000000A12345678", 16).unwrap()).is_err(),
            "Mainnet with non-zero rollup index should fail to parse",
        );
    }
}
