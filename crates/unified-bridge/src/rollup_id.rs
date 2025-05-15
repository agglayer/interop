use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::{NetworkId, RollupIndex};

/// A rollup ID.
///
/// Rollups are numbered from 1 to `u32::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct RollupId(
    #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1..=u32::MAX))] u32,
);

impl Display for RollupId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for RollupId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = u32::deserialize(deserializer)?;
        if id == 0 {
            return Err(serde::de::Error::custom("Rollup ID cannot be 0"));
        }
        Ok(RollupId(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid rollup id")]
pub struct InvalidRollupIdError;

impl RollupId {
    pub const BITS: usize = u32::BITS as usize;

    #[inline]
    pub const fn new(value: u32) -> Result<RollupId, InvalidRollupIdError> {
        if value == 0 {
            return Err(InvalidRollupIdError);
        }
        Ok(RollupId(value))
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for RollupId {
    type Error = InvalidRollupIdError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(InvalidRollupIdError);
        }
        Ok(RollupId(value))
    }
}

impl From<RollupId> for u32 {
    #[inline]
    fn from(value: RollupId) -> Self {
        value.0
    }
}

impl TryFrom<NetworkId> for RollupId {
    type Error = InvalidRollupIdError;

    #[inline]
    fn try_from(value: NetworkId) -> Result<Self, Self::Error> {
        if value.to_u32() == 0 {
            return Err(InvalidRollupIdError);
        }
        Ok(RollupId(value.to_u32()))
    }
}

impl From<RollupId> for NetworkId {
    #[inline]
    fn from(value: RollupId) -> Self {
        NetworkId::new(value.0)
    }
}

impl From<RollupIndex> for RollupId {
    #[inline]
    fn from(value: RollupIndex) -> Self {
        RollupId(value.to_u32() + 1)
    }
}

impl Deref for RollupId {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
