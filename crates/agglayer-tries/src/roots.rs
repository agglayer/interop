use std::fmt;

use agglayer_primitives::{Digest, B256};
use serde::{Deserialize, Serialize};

macro_rules! define_root {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
        )]
        #[serde(transparent)]
        pub struct $name(Digest);

        impl $name {
            pub const fn new(digest: Digest) -> Self {
                Self(digest)
            }
        }

        impl AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.0.as_slice()
            }
        }

        impl AsRef<Digest> for $name {
            #[inline]
            fn as_ref(&self) -> &Digest {
                &self.0
            }
        }

        impl AsRef<[u8; 32]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8; 32] {
                &self.0 .0
            }
        }

        impl From<Digest> for $name {
            #[inline]
            fn from(digest: Digest) -> Self {
                Self(digest)
            }
        }

        impl From<$name> for Digest {
            #[inline]
            fn from(it: $name) -> Self {
                it.0
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = std::array::TryFromSliceError;

            #[inline]
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Digest::try_from(value).map(Self)
            }
        }

        impl From<B256> for $name {
            #[inline]
            fn from(value: B256) -> Self {
                Self(Digest::from(value))
            }
        }

        impl From<$name> for B256 {
            #[inline]
            fn from(value: $name) -> Self {
                B256::from(value.0)
            }
        }

        impl fmt::UpperHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl fmt::LowerHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

define_root!(BalanceRoot);
define_root!(NullifierRoot);
define_root!(LocalExitRoot);
