use agglayer_interop_types::Digest;
use prost::bytes::Bytes;

use super::Error;
use crate::v1;

impl TryFrom<v1::FixedBytes32> for Digest {
    type Error = Error;

    #[inline]
    fn try_from(value: v1::FixedBytes32) -> Result<Self, Self::Error> {
        Ok(Digest::from(<[u8; 32]>::try_from(value)?))
    }
}

impl From<Digest> for v1::FixedBytes32 {
    #[inline]
    fn from(value: Digest) -> Self {
        v1::FixedBytes32 {
            value: Bytes::copy_from_slice(&value.0),
        }
    }
}
