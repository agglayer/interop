use std::fmt;

use agglayer_primitives::Digest;
use serde::{Deserialize, Serialize};

pub trait TreeRoot {
    fn as_slice(&self) -> &[u8];
    fn as_digest(&self) -> &Digest;
}

macro_rules! define_root {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
        #[serde(transparent)]
        pub struct $name(Digest);

        impl $name {
            pub const fn new(digest: Digest) -> Self {
                Self(digest)
            }
        }

        impl TreeRoot for $name {
            #[inline]
            fn as_slice(&self) -> &[u8] {
                self.0.as_slice()
            }

            #[inline]
            fn as_digest(&self) -> &Digest {
                &self.0
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
