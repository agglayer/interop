use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{NetworkId, RollupId};

/// A rollup index.
///
/// Rollups are numbered from 0 to `u32::MAX - 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct RollupIndex(
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(0..=u32::MAX - 1))] u32,
);

// No Display implementation on purpose: the integer here is off-by-one compared
// to NetworkIds

impl<'de> Deserialize<'de> for RollupIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = u32::deserialize(deserializer)?;
        if id == u32::MAX {
            return Err(serde::de::Error::custom("Rollup ID cannot be u32::MAX"));
        }
        Ok(RollupIndex(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid rollup index")]
pub struct InvalidRollupIndexError;

impl RollupIndex {
    pub const BITS: usize = u32::BITS as usize;

    #[inline]
    pub const fn new(value: u32) -> Result<RollupIndex, InvalidRollupIndexError> {
        if value == u32::MAX {
            return Err(InvalidRollupIndexError);
        }
        Ok(RollupIndex(value))
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for RollupIndex {
    type Error = InvalidRollupIndexError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == u32::MAX {
            return Err(InvalidRollupIndexError);
        }
        Ok(RollupIndex(value))
    }
}

impl From<RollupIndex> for u32 {
    #[inline]
    fn from(value: RollupIndex) -> Self {
        value.0
    }
}

impl TryFrom<NetworkId> for RollupIndex {
    type Error = InvalidRollupIndexError;

    #[inline]
    fn try_from(value: NetworkId) -> Result<Self, Self::Error> {
        if value.to_u32() == 0 {
            return Err(InvalidRollupIndexError);
        }
        Ok(RollupIndex(value.to_u32() - 1))
    }
}

impl From<RollupIndex> for NetworkId {
    #[inline]
    fn from(value: RollupIndex) -> Self {
        NetworkId::new(value.0 + 1)
    }
}

impl From<RollupId> for RollupIndex {
    #[inline]
    fn from(value: RollupId) -> Self {
        RollupIndex(value.to_u32() - 1)
    }
}

impl Deref for RollupIndex {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
