use std::{fmt, ops::Deref};

use alloy_primitives::B256;
use hex::FromHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    utils::{FromBool, FromU256},
    U256,
};

#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct Digest(pub [u8; 32]);

impl Deref for Digest {
    type Target = [u8; 32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for Digest {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Digest {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:#x}")
    }
}

impl fmt::Debug for Digest {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:x}")
    }
}

impl fmt::UpperHex for Digest {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{byte:02X}")?;
        }

        Ok(())
    }
}

impl Digest {
    pub const ZERO: Digest = Digest([0u8; 32]);

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for Digest {
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<B256> for Digest {
    #[inline]
    fn from(bytes: B256) -> Self {
        Self(bytes.into())
    }
}

impl From<Digest> for B256 {
    #[inline]
    fn from(bytes: Digest) -> Self {
        Self::from(bytes.0)
    }
}

impl fmt::LowerHex for Digest {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for Digest {
    #[inline]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;

            let s = s.trim_start_matches("0x");
            let s = <[u8; 32]>::from_hex(s).map_err(serde::de::Error::custom)?;

            Ok(Digest(s))
        } else {
            #[derive(::serde::Deserialize)]
            #[serde(rename = "NewDigest")]
            struct Value([u8; 32]);

            let value = Value::deserialize(deserializer)?;
            Ok(Digest(value.0))
        }
    }
}

impl Serialize for Digest {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            format!("{self:#x}").serialize(serializer)
        } else {
            serializer.serialize_newtype_struct("NewDigest", &self.0)
        }
    }
}

const DIGEST_FROM_BOOL_TRUE: Digest = Digest([
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

const DIGEST_FROM_BOOL_FALSE: Digest = Digest([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

impl FromBool for Digest {
    #[inline]
    fn from_bool(b: bool) -> Self {
        if b {
            DIGEST_FROM_BOOL_TRUE
        } else {
            DIGEST_FROM_BOOL_FALSE
        }
    }
}

impl FromU256 for Digest {
    #[inline]
    fn from_u256(u: U256) -> Self {
        Self(u.to_be_bytes())
    }
}

impl TryFrom<&[u8]> for Digest {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(value).map(Digest)
    }
}

impl TryFrom<Vec<u8>> for Digest {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}

impl From<Digest> for Vec<u8> {
    #[inline]
    fn from(value: Digest) -> Self {
        value.0.to_vec()
    }
}

#[cfg(any(test, feature = "testutils"))]
impl rand::distr::Distribution<Digest> for rand::distr::StandardUniform {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Digest {
        let raw: [u8; 32] = rng.random();
        Digest(raw)
    }
}
