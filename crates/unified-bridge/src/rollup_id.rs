use std::{fmt::Display, num::NonZeroU32};

use serde::{Deserialize, Serialize};

use crate::{NetworkId, RollupIndex};

/// A rollup ID.
///
/// Rollups are numbered from 1 to `u32::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct RollupId(NonZeroU32);

impl Display for RollupId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid rollup id")]
pub struct InvalidRollupIdError;

impl RollupId {
    pub const BITS: usize = u32::BITS as usize;

    #[inline]
    pub const fn new(value: u32) -> Result<RollupId, InvalidRollupIdError> {
        match NonZeroU32::new(value) {
            Some(v) => Ok(RollupId(v)),
            None => Err(InvalidRollupIdError),
        }
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0.get()
    }

    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.get().to_be_bytes()
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 4] {
        self.0.get().to_le_bytes()
    }
}

impl TryFrom<u32> for RollupId {
    type Error = InvalidRollupIdError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RollupId> for u32 {
    #[inline]
    fn from(value: RollupId) -> Self {
        value.0.get()
    }
}

impl TryFrom<NetworkId> for RollupId {
    type Error = InvalidRollupIdError;

    #[inline]
    fn try_from(value: NetworkId) -> Result<Self, Self::Error> {
        RollupId::new(value.to_u32())
    }
}

impl From<RollupId> for NetworkId {
    #[inline]
    fn from(value: RollupId) -> Self {
        NetworkId::new(value.0.get())
    }
}

impl From<RollupIndex> for RollupId {
    #[inline]
    fn from(value: RollupIndex) -> Self {
        RollupId(NonZeroU32::new(value.to_u32() + 1).unwrap())
    }
}
