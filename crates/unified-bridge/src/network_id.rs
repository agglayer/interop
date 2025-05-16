use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A network ID.
///
/// 0 refers to ethereum, and rollups are numbered from 1 to `u32::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct NetworkId(u32);

impl Display for NetworkId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl NetworkId {
    pub const BITS: usize = u32::BITS as usize;

    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl From<u32> for NetworkId {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<NetworkId> for u32 {
    #[inline]
    fn from(value: NetworkId) -> Self {
        value.0
    }
}
